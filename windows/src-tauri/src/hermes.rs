// Hermes Agent — a remote agent, reached over its OpenAI-compatible API server.
// Everything happens here: the bearer token is read out of the Secret Service
// and never crosses the IPC boundary, so the chat window only ever learns
// "configured" or "not configured".

use std::time::{Duration, Instant};

use serde::Serialize;
use serde_json::{json, Value};
use tauri::ipc::Channel;

use crate::claude::{self, Chat, ChatContext, ChatReply, SYSTEM_PROMPT};
use crate::secrets;

/// No overall request timeout: an agent turn runs tools and can take minutes.
/// Only the connection is bounded — and the stream itself is bounded below.
const CONNECT_TIMEOUT: Duration = Duration::from_secs(15);
/// The endpoint has stopped sending. Agent turns are bursty, so be generous.
const IDLE_TIMEOUT: Duration = Duration::from_secs(120);
/// Absolute ceiling for one turn, so a wedged stream cannot pin the window.
const MAX_TURN: Duration = Duration::from_secs(600);
/// `/v1/models` on a stock install lists exactly this.
const MODEL: &str = "hermes-agent";

#[derive(Clone, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum HermesEvent {
    Delta { text: String },
    Done,
}

/// Both halves in place: the settings row and the empty state read this.
pub fn configured() -> bool {
    secrets::get("hermes-url").is_some() && secrets::get("hermes-token").is_some()
}

/// Base URL → chat-completions URL. Accepts the path already being pasted in.
/// Pure, so the test never has to touch the Secret Service.
fn chat_url(raw: Option<&str>) -> Result<String, String> {
    let raw = raw.ok_or_else(|| "Hermes URL missing. Open settings.".to_string())?;
    let base = raw.trim().trim_end_matches('/');
    if base.is_empty() {
        return Err("Hermes URL missing. Open settings.".into());
    }
    // Set by the user, but still untrusted input: no file://, no garbage.
    if !(base.starts_with("http://") || base.starts_with("https://")) {
        return Err("Hermes URL must start with http:// or https://.".into());
    }
    if base.ends_with("/v1/chat/completions") {
        return Ok(base.to_string());
    }
    Ok(format!("{base}/v1/chat/completions"))
}

fn endpoint() -> Result<String, String> {
    chat_url(secrets::get("hermes-url").as_deref())
}

/// One turn. The caller owns the transcript and hands it back in full each time,
/// so nothing has to live in Rust between messages.
pub async fn send(
    history: Vec<Value>,
    prompt: String,
    channel: Channel<HermesEvent>,
) -> Result<(), String> {
    let url = endpoint()?;
    let token = secrets::get("hermes-token")
        .ok_or_else(|| "Hermes key missing. Open settings.".to_string())?;

    let mut messages = history;
    messages.push(json!({ "role": "user", "content": prompt }));

    let client = reqwest::Client::builder()
        .connect_timeout(CONNECT_TIMEOUT)
        .build()
        .map_err(|e| e.to_string())?;

    let response = client
        .post(&url)
        .bearer_auth(token)
        .header("content-type", "application/json")
        .json(&json!({ "model": MODEL, "stream": true, "messages": messages }))
        .send()
        .await
        .map_err(|e| format!("Hermes unreachable: {e}"))?;

    let status = response.status();
    if !status.is_success() {
        return Err(surface(response, status).await);
    }

    let mut response = response;
    let mut pending = String::new();
    let started = Instant::now();
    loop {
        if started.elapsed() > MAX_TURN {
            return Err("Hermes stopped responding (10 minute limit).".into());
        }
        let chunk = tokio::time::timeout(IDLE_TIMEOUT, response.chunk())
            .await
            .map_err(|_| "Hermes went quiet for two minutes.".to_string())?
            .map_err(|e| format!("Connection lost: {e}"))?;

        // Clean end of stream — not every server sends [DONE].
        let Some(chunk) = chunk else { break };
        pending.push_str(&String::from_utf8_lossy(&chunk));

        while let Some(idx) = pending.find('\n') {
            let line: String = pending.drain(..=idx).collect();
            match handle(line.trim_end(), &channel)? {
                Stream::Continue => {}
                Stream::Finished => {
                    let _ = channel.send(HermesEvent::Done);
                    return Ok(());
                }
            }
        }
    }

    let _ = channel.send(HermesEvent::Done);
    Ok(())
}

enum Stream {
    Continue,
    Finished,
}

/// One SSE line. Anything that is not a `data:` line (comments, `event:`) is
/// silently skipped, as the spec allows.
fn handle(line: &str, channel: &Channel<HermesEvent>) -> Result<Stream, String> {
    let Some(data) = line.strip_prefix("data:").map(str::trim_start) else {
        return Ok(Stream::Continue);
    };
    if data == "[DONE]" {
        return Ok(Stream::Finished);
    }
    let Ok(value) = serde_json::from_str::<Value>(data) else {
        return Ok(Stream::Continue);
    };

    // An error object arrives mid-stream instead of as a status code.
    if let Some(message) = value.pointer("/error/message").and_then(Value::as_str) {
        return Err(message.to_string());
    }
    if let Some(text) = value.pointer("/choices/0/delta/content").and_then(Value::as_str) {
        if !text.is_empty() {
            channel
                .send(HermesEvent::Delta { text: text.to_string() })
                .map_err(|_| "window closed".to_string())?;
        }
    }
    Ok(Stream::Continue)
}

/// Surface the server's own message, which is what makes a bad key or a
/// mis-typed URL obvious.
async fn surface(response: reqwest::Response, status: reqwest::StatusCode) -> String {
    let text = response.text().await.unwrap_or_default();
    let detail = serde_json::from_str::<Value>(&text)
        .ok()
        .and_then(|v| {
            v.pointer("/error/message")
                .or_else(|| v.pointer("/message"))
                .and_then(Value::as_str)
                .map(str::to_string)
        })
        .unwrap_or_else(|| text.chars().take(200).collect());
    if detail.is_empty() {
        return format!("Hermes API {status}");
    }
    format!("Hermes API {status}: {detail}")
}

// ── Island chat ───────────────────────────────────────────────────────────────
//
// The island asks one question and waits for the whole reply: no stream, no
// channel, no transcript of its own. Everything else about the turn matches
// `claude::send` so that a user-chosen fallback can switch engines without the
// history having to be rebuilt.

/// One island turn against the remote agent.
pub async fn chat(
    chat: &Chat,
    query: String,
    context: Option<ChatContext>,
) -> Result<ChatReply, String> {
    let url = endpoint()?;
    let token = secrets::get("hermes-token")
        .ok_or_else(|| "Hermes key missing. Open settings.".to_string())?;

    // File / window context rides along with the first message only, exactly
    // like claude::send().
    let mut content: Vec<Value> = Vec::new();
    if chat.is_empty() {
        match &context {
            Some(ChatContext::File { name, path }) => {
                if let Some(block) = claude::file_block(path) {
                    content.push(block);
                }
                content.push(json!({ "type": "text", "text": format!("File: {name}") }));
            }
            Some(ChatContext::Window { app_name, title, url }) => {
                let mut text = format!("Context — App: {app_name}, Window: {title}");
                if let Some(url) = url {
                    text.push_str(&format!(", URL: {url}"));
                }
                content.push(json!({ "type": "text", "text": text }));
            }
            None => {}
        }
    }
    content.push(json!({ "type": "text", "text": query }));

    chat.push(json!({ "role": "user", "content": content }));

    let request = json!({ "model": MODEL, "messages": turn(&chat.snapshot()) });

    let client = reqwest::Client::builder()
        .connect_timeout(CONNECT_TIMEOUT)
        .timeout(MAX_TURN)
        .build()
        .map_err(|e| e.to_string())?;

    let response = client
        .post(&url)
        .bearer_auth(token)
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("Hermes unreachable: {e}"))?;

    let status = response.status();
    if !status.is_success() {
        let why = surface(response, status).await;
        chat.pop();
        return Err(why);
    }

    let value: Value = response.json().await.map_err(|e| format!("Bad API response: {e}"))?;
    if let Some(why) = value.pointer("/error/message").and_then(Value::as_str) {
        chat.pop();
        return Err(why.to_string());
    }

    let text = value
        .pointer("/choices/0/message/content")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim()
        .to_string();
    if text.is_empty() {
        chat.pop();
        return Err("No response text.".into());
    }

    // Store it in the same shape claude::send() would, so the next turn — on
    // either engine — reads a history it recognises.
    chat.push(json!({ "role": "assistant", "content": [{ "type": "text", "text": text }] }));
    Ok(ChatReply { text })
}

/// Stored history → OpenAI messages. The system prompt rides every request.
fn turn(history: &[Value]) -> Vec<Value> {
    let mut out = Vec::with_capacity(history.len() + 1);
    out.push(json!({ "role": "system", "content": SYSTEM_PROMPT }));
    for message in history {
        out.push(json!({
            "role": message.get("role").and_then(Value::as_str).unwrap_or("user"),
            "content": parts(message.get("content")),
        }));
    }
    out
}

fn parts(content: Option<&Value>) -> Value {
    match content {
        Some(Value::Array(blocks)) => Value::Array(blocks.iter().map(part).collect()),
        Some(Value::String(text)) => json!(text),
        _ => json!(""),
    }
}

/// One stored block → one OpenAI part. Text goes across as text; images and
/// PDFs are inlined, because a remote agent cannot open a path on this machine.
fn part(block: &Value) -> Value {
    let kind = block.get("type").and_then(Value::as_str).unwrap_or("");
    match kind {
        "image" | "document" => {
            let source = &block["source"];
            let media = source["media_type"].as_str().unwrap_or("application/octet-stream");
            let url = format!("data:{media};base64,{}", source["data"].as_str().unwrap_or(""));
            if kind == "image" {
                json!({ "type": "image_url", "image_url": { "url": url } })
            } else {
                // OpenAI's file part. Unverified against Hermes' server: if it
                // rejects the block the turn fails loudly instead of silently.
                json!({ "type": "file", "file": { "filename": "attachment", "file_data": url } })
            }
        }
        // Only present when the other engine held the earlier turns — a
        // fallback must not lose them.
        "tool_use" => json!({
            "type": "text",
            "text": format!("Tool call {}: {}", block["name"].as_str().unwrap_or("?"), block["input"])
        }),
        "tool_result" => json!({ "type": "text", "text": tool_text(block) }),
        _ => json!({ "type": "text", "text": block["text"].as_str().unwrap_or("") }),
    }
}

/// tool_result content is a string, or a list of blocks.
fn tool_text(block: &Value) -> String {
    match block.get("content") {
        Some(Value::String(text)) => text.clone(),
        Some(Value::Array(items)) => items
            .iter()
            .map(|item| item["text"].as_str().unwrap_or(""))
            .collect::<Vec<_>>()
            .join("\n"),
        _ => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::{chat_url, part, turn};
    use serde_json::json;

    /// The URL field is user input; only http(s) survives it.
    #[test]
    fn chat_url_requires_http_and_accepts_a_pasted_path() {
        assert!(chat_url(None).is_err(), "no URL configured");
        assert!(chat_url(Some("   ")).is_err(), "blank URL refused");
        assert!(chat_url(Some("file:///etc")).is_err(), "non-http scheme refused");

        assert_eq!(
            chat_url(Some("https://agent.example:8642/")).unwrap(),
            "https://agent.example:8642/v1/chat/completions"
        );
        assert_eq!(
            chat_url(Some("http://127.0.0.1:8642")).unwrap(),
            "http://127.0.0.1:8642/v1/chat/completions"
        );
        assert_eq!(
            chat_url(Some("https://agent.example:8642/v1/chat/completions")).unwrap(),
            "https://agent.example:8642/v1/chat/completions"
        );
    }

    /// The island stores Anthropic blocks; Hermes speaks OpenAI parts. Every
    /// block the Claude path can write has to come out the other side.
    #[test]
    fn history_translates_from_anthropic_blocks_to_openai_parts() {
        assert_eq!(
            part(&json!({"type": "text", "text": "hi"})),
            json!({"type": "text", "text": "hi"})
        );
        assert_eq!(
            part(&json!({"type": "image", "source": {"type": "base64", "media_type": "image/png", "data": "AAA"}})),
            json!({"type": "image_url", "image_url": {"url": "data:image/png;base64,AAA"}})
        );
        assert_eq!(
            part(&json!({"type": "document", "source": {"type": "base64", "media_type": "application/pdf", "data": "BBB"}})),
            json!({"type": "file", "file": {"filename": "attachment", "file_data": "data:application/pdf;base64,BBB"}})
        );
        assert_eq!(
            part(&json!({"type": "tool_use", "name": "web_search", "input": {"q": "x"}})),
            json!({"type": "text", "text": "Tool call web_search: {\"q\":\"x\"}"})
        );

        let messages = turn(&[json!({"role": "user", "content": [{"type": "text", "text": "hello"}]})]);
        assert_eq!(messages[0]["role"], "system");
        assert_eq!(messages[1]["role"], "user");
        assert_eq!(messages[1]["content"][0]["text"], "hello");
    }
}
