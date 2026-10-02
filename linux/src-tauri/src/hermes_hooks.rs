// Hermes plugin installation — the same discipline as `hooks.rs`, aimed at
// ~/.hermes instead of ~/.claude: read the config, take a dated backup, merge
// only our own keys, show the diff, write after an explicit click. Uninstall
// removes Coucou's entries and nothing else.
//
// Why the YAML is edited by hand and never round-tripped through a parser:
// config.yaml carries comments, anchors and ordering that belong to the user
// (and to Hermes), and a parse/serialise cycle would rewrite all of it. So the
// file is read line by line, one known key is set or removed, and every other
// line goes back untouched. Anything we do not recognise is an error rather
// than a guess.
//
// Install carries the whole connection, not half of it: config.yaml, the
// plugin files, ~/.hermes/.env — the API server will not listen without it,
// because API_SERVER_ENABLED is off by default and the server insists on a
// key — and the URL/key in the system keyring. Every file gets the same
// treatment: dated backup, only our own keys, the diff first, write after the
// click. Uninstall takes our config keys and the plugin away and leaves .env
// alone: API_SERVER_ENABLED is Hermes's own setting and a key the user wrote
// is theirs. (ponytail: uninstall .env too, but only while the two lines still
// hold what we wrote — add when somebody asks.)

use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::settings;

/// Directory name under ~/.hermes/plugins and the transport name it registers.
pub const PLUGIN_NAME: &str = "coucou";

/// The plugin ships inside the binary so install needs no network and no
/// second copy of the code. Both files live in the repo at `hermes-plugin/`.
const PLUGIN_YAML: &str = include_str!("../../../hermes-plugin/plugin.yaml");
const PLUGIN_PY: &str = include_str!("../../../hermes-plugin/__init__.py");

/// ~/.hermes/.env — where the API server reads its own settings.
const ENV_ENABLED: &str = "API_SERVER_ENABLED";
const ENV_KEY: &str = "API_SERVER_KEY";
/// Shown instead of a key the user does not have yet. The real one is
/// generated at write time and goes straight to the keyring: never printed,
/// never in a diff, never in a log.
const ENV_PLACEHOLDER: &str = "<generated>";
/// Where the server listens unless .env says otherwise — the documented default.
const HERMES_URL: &str = "http://127.0.0.1:8642";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HermesHookStatus {
    pub installed: bool,
    /// The relay the plugin forwards to. Without it events go nowhere —
    /// harmless, but the settings window says so instead of looking broken.
    pub hook_ready: bool,
    pub config_path: String,
    pub plugin_path: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HermesHookPreview {
    pub diff: String,
    pub backup: String,
    pub config_path: String,
    pub plugin_path: String,
    /// Identifies the bytes this diff was computed from.
    pub fingerprint: String,
}

pub fn config_path() -> PathBuf {
    settings::home().join(".hermes").join("config.yaml")
}

pub fn plugin_dir() -> PathBuf {
    settings::home()
        .join(".hermes")
        .join("plugins")
        .join(PLUGIN_NAME)
}

fn backup_path() -> PathBuf {
    let name = format!("config.yaml.bak-{}", crate::hooks::stamp());
    config_path().with_file_name(name)
}

/// Only "the file is not there" means "start from nothing". An unreadable
/// config must stop us, never be replaced by an empty one.
fn read_config() -> Result<String, String> {
    match std::fs::read(config_path()) {
        Ok(bytes) => String::from_utf8(bytes)
            .map_err(|_| "config.yaml is not UTF-8 — nothing was written.".to_string()),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(String::new()),
        Err(err) => Err(format!("{}: {err}", config_path().display())),
    }
}

// ── ~/.hermes/.env ───────────────────────────────────────────────────────────

fn env_path() -> PathBuf {
    settings::home().join(".hermes").join(".env")
}

fn env_backup_path() -> PathBuf {
    env_path().with_file_name(format!(".env.bak-{}", crate::hooks::stamp()))
}

/// Only "the file is not there" means "start from nothing": an unreadable
/// .env must stop us, never be replaced by an empty one.
fn read_env() -> Result<String, String> {
    match std::fs::read(env_path()) {
        Ok(bytes) => String::from_utf8(bytes)
            .map_err(|_| ".env is not UTF-8 — nothing was written.".to_string()),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(String::new()),
        Err(err) => Err(format!("{}: {err}", env_path().display())),
    }
}

/// The key of a `KEY=value` line: `export` tolerated, comments skipped, and
/// the match stops at the `=` so `API_SERVER_KEYS` is never taken for ours.
fn env_line_key(line: &str) -> Option<&str> {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
        return None;
    }
    let line = line.strip_prefix("export ").unwrap_or(line);
    let key = line.split('=').next()?.trim();
    (!key.is_empty() && !key.chars().any(char::is_whitespace)).then_some(key)
}

/// The value of `key`, unquoted. `None` means "not set" — which is what an
/// install looks for before it generates one.
fn env_value<'a>(text: &'a str, key: &str) -> Option<&'a str> {
    text.lines().find_map(|line| {
        if env_line_key(line) != Some(key) {
            return None;
        }
        Some(strip_quotes(line.split_once('=')?.1.trim()))
    })
}

/// Sets `key` (or appends it), keeping every other line byte for byte, and
/// leaving a line alone whose value already matches — so a `export ` prefix,
/// the spacing and the comments survive an install the user had done by hand.
fn set_env_value(original: &str, key: &str, value: &str) -> String {
    let terminated = original.ends_with('\n');
    let mut out: Vec<String> = Vec::new();
    let mut seen = false;
    for line in original.lines() {
        if env_line_key(line) != Some(key) {
            out.push(line.to_string());
            continue;
        }
        if seen {
            continue; // a second copy of our key: the first one wins
        }
        seen = true;
        let same = line
            .split_once('=')
            .is_some_and(|(_, v)| strip_quotes(v.trim()) == value);
        out.push(if same {
            line.to_string()
        } else {
            format!("{key}={value}")
        });
    }
    let added = !seen;
    if added {
        out.push(format!("{key}={value}"));
    }
    if out.is_empty() {
        return String::new();
    }
    let mut next = out.join("\n");
    // The newline the file already had, or one of our own when we are the ones
    // creating it — a file we do not have to change must not change.
    if added || terminated {
        next.push('\n');
    }
    next
}

/// What `.env` must say for the server to listen at all, with `key` as its key.
fn env_after(current: &str, key: &str) -> String {
    let next = set_env_value(current, ENV_ENABLED, "true");
    set_env_value(&next, ENV_KEY, key)
}

/// Both sides of the `.env` diff with the secret masked. The preview is read
/// before the key exists and before the keyring has it, so the value has no
/// business on a screen — or in a screenshot of one.
fn redact_env(text: &str) -> String {
    let mask = format!("{ENV_KEY}=••••••••");
    let mut out: Vec<&str> = Vec::new();
    for line in text.lines() {
        out.push(if env_line_key(line) == Some(ENV_KEY) {
            mask.as_str()
        } else {
            line
        });
    }
    out.join("\n")
}

/// Both files this button is about, hashed together: config.yaml moving or
/// .env moving must both invalidate a preview.
fn current_fingerprint() -> Result<String, String> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(read_config()?.as_bytes());
    bytes.push(0); // a config that ends where .env begins cannot collide
    bytes.extend_from_slice(read_env()?.as_bytes());
    Ok(crate::hooks::fingerprint(&bytes))
}

// ── The line-oriented YAML editor ────────────────────────────────────────────

fn indent_of(line: &str) -> usize {
    line.len() - line.trim_start().len()
}

fn skippable(line: &str) -> bool {
    let t = line.trim();
    t.is_empty() || t.starts_with('#')
}

/// `("key", "rest of the line")` for a `key: value` line. A mapping line has an
/// empty rest; comments and list items have no key at all.
fn key_of(line: &str) -> Option<(&str, &str)> {
    let t = line.trim_start();
    if t.is_empty() || t.starts_with('#') || t.starts_with('-') {
        return None;
    }
    let idx = t.find(':')?;
    let key = t[..idx].trim_end();
    if key.is_empty() {
        return None;
    }
    Some((key, t[idx + 1..].trim()))
}

fn find_toplevel(lines: &[String], key: &str) -> Option<usize> {
    lines.iter().position(|line| {
        indent_of(line) == 0 && key_of(line).map(|(k, _)| k == key).unwrap_or(false)
    })
}

/// The line after the last content line of the block starting at `start`.
/// Trailing comments belong to whatever comes next, so they are not swallowed.
fn block_end(lines: &[String], start: usize) -> usize {
    let base = indent_of(&lines[start]);
    let mut last = start;
    for i in (start + 1)..lines.len() {
        let line = &lines[i];
        if skippable(line) {
            continue;
        }
        if indent_of(line) <= base {
            break;
        }
        last = i;
    }
    last + 1
}

/// Index of `key` anywhere inside the block at `start`, at any deeper indent.
fn find_key(lines: &[String], start: usize, key: &str) -> Option<usize> {
    let base = indent_of(&lines[start]);
    for i in (start + 1)..lines.len() {
        let line = &lines[i];
        if skippable(line) {
            continue;
        }
        if indent_of(line) <= base {
            break;
        }
        if key_of(line).map(|(k, _)| k == key).unwrap_or(false) {
            return Some(i);
        }
    }
    None
}

/// Indexes of the content lines nested inside the block at `start`.
fn child_lines(lines: &[String], start: usize) -> Vec<usize> {
    let base = indent_of(&lines[start]);
    ((start + 1)..block_end(lines, start))
        .filter(|&i| !skippable(&lines[i]) && indent_of(&lines[i]) > base)
        .collect()
}

/// Index of the key at `path`, creating nothing. `None` means "not there",
/// which is exactly what an uninstall wants to hear.
fn lookup(lines: &[String], path: &[&str]) -> Option<usize> {
    let (last, parents) = path.split_last()?;
    let mut cur: Option<usize> = None;
    for key in parents {
        cur = match cur {
            None => find_toplevel(lines, key),
            Some(parent) => find_key(lines, parent, key),
        };
        cur?;
    }
    match cur {
        None => find_toplevel(lines, last),
        Some(parent) => find_key(lines, parent, last),
    }
}

/// Walks `path` to its parent mapping, creating whatever is missing.
/// Returns the index of the last key on the path, or None for "top level".
fn ensure_parents(lines: &mut Vec<String>, path: &[&str]) -> Result<Option<usize>, String> {
    let mut cur: Option<usize> = None;
    // Top-level keys sit at column 0, so the imaginary parent starts at -2.
    let mut cur_indent: isize = -2;
    for key in path {
        let found = match cur {
            None => find_toplevel(lines, key),
            Some(parent) => find_key(lines, parent, key),
        };
        if let Some(idx) = found {
            if let Some((_, value)) = key_of(&lines[idx]) {
                if !value.is_empty() {
                    return Err(format!(
                        "`{key}` in {} is not a mapping — nothing was written.",
                        config_path().display()
                    ));
                }
            }
            cur_indent = indent_of(&lines[idx]) as isize;
            cur = Some(idx);
        } else {
            let indent = (cur_indent + 2).max(0) as usize;
            let pos = match cur {
                Some(parent) => block_end(lines, parent),
                // Trailing empty lines are the file's own terminator, not a
                // place for our block: leaving them last is what lets a
                // removal round-trip byte for byte.
                None => {
                    let mut pos = lines.len();
                    while pos > 0 && lines[pos - 1].is_empty() {
                        pos -= 1;
                    }
                    pos
                }
            };
            lines.insert(pos, format!("{}{}:", " ".repeat(indent), key));
            cur_indent = indent as isize;
            cur = Some(pos);
        }
    }
    Ok(cur)
}

/// Sets one scalar key inside its parent mapping (or at the top level).
fn set_key(
    lines: &mut Vec<String>,
    parent: Option<usize>,
    key: &str,
    value: &str,
) -> Result<(), String> {
    let found = match parent {
        Some(p) => find_key(lines, p, key),
        None => find_toplevel(lines, key),
    };
    if let Some(idx) = found {
        if !child_lines(lines, idx).is_empty() {
            return Err(format!(
                "`{key}` holds a mapping of its own — nothing was written."
            ));
        }
        let indent = indent_of(&lines[idx]);
        lines[idx] = format!("{}{key}: {value}", " ".repeat(indent));
        return Ok(());
    }
    let (pos, indent) = match parent {
        Some(p) => (block_end(lines, p), indent_of(&lines[p]) + 2),
        None => (lines.len(), 0),
    };
    lines.insert(pos, format!("{}{key}: {value}", " ".repeat(indent)));
    Ok(())
}

/// Drops a scalar key, but only when it still holds the value we wrote:
/// somebody who changed it by hand keeps their setting.
fn remove_key(lines: &mut Vec<String>, parent: Option<usize>, key: &str, value: &str) -> bool {
    let found = match parent {
        Some(p) => find_key(lines, p, key),
        None => find_toplevel(lines, key),
    };
    match found {
        Some(idx)
            if key_of(&lines[idx])
                .map(|(_, v)| v == value)
                .unwrap_or(false) =>
        {
            lines.remove(idx);
            true
        }
        _ => false,
    }
}

/// `- item` inside a (possibly inline) list at `path`.
fn set_list_item(lines: &mut Vec<String>, path: &[&str], item: &str) -> Result<(), String> {
    let (last, parents) = path.split_last().ok_or_else(|| "empty path".to_string())?;
    let parent = ensure_parents(lines, parents)?;

    let Some(idx) = (match parent {
        Some(p) => find_key(lines, p, last),
        None => find_toplevel(lines, last),
    }) else {
        // No `enabled:` yet: write the whole block under the parent we have.
        let (pos, indent) = match parent {
            Some(p) => (block_end(lines, p), indent_of(&lines[p]) + 2),
            None => (lines.len(), 0),
        };
        // Key first: it shifts the terminator down, so the item then lands
        // between the two instead of after the end of the file.
        lines.insert(pos, format!("{}{last}:", " ".repeat(indent)));
        lines.insert(pos + 1, format!("{}  - {item}", " ".repeat(indent)));
        return Ok(());
    };

    // Copied out: the assignment below mutates `lines`, which `key`/`value`
    // would otherwise still be borrowing.
    let (key, value) = match key_of(&lines[idx]) {
        Some((key, value)) => (key.to_string(), value.to_string()),
        None => return Ok(()),
    };

    // Inline form: `enabled: [a, b]` — extended where it stands.
    if value.starts_with('[') {
        if inline_has(&value, item) {
            return Ok(());
        }
        let close = value
            .rfind(']')
            .ok_or_else(|| "`enabled` has an unterminated inline list.".to_string())?;
        let inner = value[..close].trim_end();
        let joined = if inner.ends_with('[') {
            format!("{inner}{item}]")
        } else {
            format!("{inner}, {item}]")
        };
        let indent = indent_of(&lines[idx]);
        lines[idx] = format!("{}{key}: {joined}", " ".repeat(indent));
        return Ok(());
    }
    if !value.is_empty() {
        return Err(format!(
            "`{}` is not a list — nothing was written.",
            path.join(".")
        ));
    }

    // Block form: nested lines starting with `-`.
    let key_indent = indent_of(&lines[idx]);
    let mut last_item: Option<usize> = None;
    for i in (idx + 1)..block_end(lines, idx) {
        let line = &lines[i];
        if skippable(line) || indent_of(line) <= key_indent {
            continue;
        }
        if line.trim_start().starts_with('-') {
            let text = line.trim_start().trim_start_matches('-').trim();
            if strip_quotes(text) == item {
                return Ok(());
            }
            last_item = Some(i);
        }
    }
    let pos = last_item.map(|i| i + 1).unwrap_or(idx + 1);
    lines.insert(pos, format!("{}- {}", " ".repeat(key_indent + 2), item));
    Ok(())
}

fn strip_quotes(value: &str) -> &str {
    let v = value.trim();
    for quote in ['"', '\''] {
        if v.len() >= 2 && v.starts_with(quote) && v.ends_with(quote) {
            return &v[1..v.len() - 1];
        }
    }
    v
}

fn inline_has(value: &str, item: &str) -> bool {
    let inner = value.trim_start_matches('[').trim_end_matches(']');
    inner.split(',').any(|part| strip_quotes(part) == item)
}

/// Removes `- item` from the list at `path`, creating nothing.
fn remove_list_item(lines: &mut Vec<String>, path: &[&str], item: &str) {
    let Some(idx) = lookup(lines, path) else {
        return;
    };
    let (key, value) = match key_of(&lines[idx]) {
        Some((key, value)) => (key.to_string(), value.to_string()),
        None => return,
    };

    if value.starts_with('[') {
        if !inline_has(&value, item) {
            return;
        }
        let kept = value
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(str::trim)
            .filter(|part| strip_quotes(part) != item)
            .collect::<Vec<_>>()
            .join(", ");
        let indent = indent_of(&lines[idx]);
        lines[idx] = format!("{}{key}: [{kept}]", " ".repeat(indent));
        return;
    }

    let key_indent = indent_of(&lines[idx]);
    let mut i = idx + 1;
    while i < block_end(lines, idx) {
        let line = &lines[i];
        let is_item =
            !skippable(line) && indent_of(line) > key_indent && line.trim_start().starts_with('-');
        if is_item && strip_quotes(line.trim_start().trim_start_matches('-').trim()) == item {
            lines.remove(i);
            continue;
        }
        i += 1;
    }

    // An emptied `enabled:` (or `plugins:`) goes away with it: a bare key is
    // YAML null, and leaving our nulls behind would be our edit, not theirs.
    if value.is_empty() && child_lines(lines, idx).is_empty() {
        lines.remove(idx);
        drop_empty_mapping(lines, parent_keys_of(path));
    }
}

/// The mappings above the leaf, outermost first — used to tidy up what we
/// emptied.
fn parent_keys_of<'a>(path: &[&'a str]) -> Vec<&'a str> {
    path[..path.len().saturating_sub(1)].to_vec()
}

/// Removes `keys` from the outermost inwards while they hold no children.
fn drop_empty_mapping(lines: &mut Vec<String>, keys: Vec<&str>) {
    let mut parent: Option<usize> = None;
    for key in keys {
        let idx = match parent {
            Some(p) => find_key(lines, p, key),
            None => find_toplevel(lines, key),
        };
        let Some(idx) = idx else { return };
        if child_lines(lines, idx).is_empty() {
            lines.remove(idx);
            parent = None;
        } else {
            parent = Some(idx);
        }
    }
}

// ── Install / uninstall ──────────────────────────────────────────────────────

fn split_lines(text: &str) -> Vec<String> {
    if text.is_empty() {
        Vec::new()
    } else {
        text.split('\n').map(str::to_string).collect()
    }
}

/// Puts the file back exactly as found: same trailing newline, or a fresh
/// newline when we are the ones creating it.
fn join(lines: Vec<String>, original: &str) -> String {
    let mut text = lines.join("\n");
    if text.is_empty() {
        return String::new();
    }
    if (original.is_empty() || original.ends_with('\n')) && !text.ends_with('\n') {
        text.push('\n');
    }
    text
}

fn with_keys(text: &str) -> Result<String, String> {
    let mut lines = split_lines(text);
    let parent = ensure_parents(&mut lines, &["security", "approval"])?;
    set_key(&mut lines, parent, "transport", PLUGIN_NAME)?;
    set_key(&mut lines, parent, "transport_fallback", "builtin")?;
    set_list_item(&mut lines, &["plugins", "enabled"], PLUGIN_NAME)?;
    Ok(join(lines, text))
}

fn without_keys(text: &str) -> Result<String, String> {
    if text.is_empty() {
        return Ok(String::new());
    }
    let mut lines = split_lines(text);

    if let Some(approval) = lookup(&lines, &["security", "approval"]) {
        // `transport_fallback: builtin` only ever mattered next to our
        // transport, so it goes the moment transport does.
        if remove_key(&mut lines, Some(approval), "transport", PLUGIN_NAME) {
            remove_key(&mut lines, Some(approval), "transport_fallback", "builtin");
            if child_lines(&lines, approval).is_empty() {
                lines.remove(approval);
                drop_empty_mapping(&mut lines, vec!["security"]);
            }
        }
    }

    remove_list_item(&mut lines, &["plugins", "enabled"], PLUGIN_NAME);
    Ok(join(lines, text))
}

// ── Public API ───────────────────────────────────────────────────────────────

pub fn status() -> HermesHookStatus {
    let config = read_config().unwrap_or_default();
    let installed = plugin_dir().join("__init__.py").exists() && has_transport(&config);
    let hook_path = settings::hook_exe_path();
    HermesHookStatus {
        installed,
        hook_ready: hook_path.exists(),
        config_path: config_path().to_string_lossy().to_string(),
        plugin_path: plugin_dir().to_string_lossy().to_string(),
    }
}

fn has_transport(config: &str) -> bool {
    config.split('\n').any(|line| {
        key_of(line)
            .map(|(k, v)| k == "transport" && strip_quotes(v) == PLUGIN_NAME)
            .unwrap_or(false)
    })
}

pub fn preview(install: bool) -> Result<HermesHookPreview, String> {
    let current = read_config()?;
    let next = if install {
        with_keys(&current)?
    } else {
        without_keys(&current)?
    };

    let env = read_env()?;
    let existing_key = env_value(&env, ENV_KEY);
    let env_next = if install {
        env_after(&env, existing_key.unwrap_or(ENV_PLACEHOLDER))
    } else {
        env.clone()
    };

    // Two files, two headers: the diff is what the user is agreeing to, and
    // half a recipe is how somebody ends up with a server that never starts.
    let mut diff = String::new();
    diff.push_str(&format!("── {}\n", config_path().display()));
    diff.push_str(&crate::hooks::unified_diff(&current, &next));
    if env_next != env {
        diff.push_str(&format!(
            "\n── {} — a dated copy is taken first\n",
            env_path().display()
        ));
        diff.push_str(&crate::hooks::unified_diff(
            &redact_env(&env),
            &redact_env(&env_next),
        ));
        diff.push('\n');
        diff.push_str(if existing_key.is_some() {
            "→ your existing API_SERVER_KEY is reused\n"
        } else {
            "→ a fresh API_SERVER_KEY is generated and kept in your keyring, never printed\n"
        });
    }
    if install {
        diff.push_str("→ remembered in the system keyring, only what you have not already set:\n");
        diff.push_str("    hermes-url    = http://127.0.0.1:8642\n");
        diff.push_str("    hermes-token  = that API_SERVER_KEY\n");
    }

    Ok(HermesHookPreview {
        diff,
        backup: backup_path().to_string_lossy().to_string(),
        config_path: config_path().to_string_lossy().to_string(),
        plugin_path: plugin_dir().to_string_lossy().to_string(),
        fingerprint: current_fingerprint()?,
    })
}

/// Writes the merged config, `.env` and the plugin files, each behind its own
/// dated backup. Returns the sentence the settings window shows, so the UI
/// never has to know which of those files happened to exist.
pub fn write(install: bool, fingerprint: &str) -> Result<String, String> {
    let current = read_config()?;
    let env = read_env()?;
    if current_fingerprint()? != fingerprint {
        return Err(format!(
            "{} or {} changed since the preview. Nothing was written — review the new diff.",
            config_path().display(),
            env_path().display()
        ));
    }

    let next = if install {
        with_keys(&current)?
    } else {
        without_keys(&current)?
    };

    let hermes = settings::home().join(".hermes");
    std::fs::create_dir_all(&hermes).map_err(|e| e.to_string())?;

    let mut backup = String::new();
    if !current.is_empty() {
        let path = backup_path();
        std::fs::copy(config_path(), &path).map_err(|e| format!("backup failed: {e}"))?;
        backup = path.to_string_lossy().to_string();
    }

    // Config first: if the plugin files then fail to land, the transport is
    // already gone and Hermes falls back to its own prompt instead of naming a
    // plugin that is not there.
    if next != current {
        atomic_write(&config_path(), next.as_bytes())?;
    }

    // .env next, with its own backup: it is the only file here that carries a
    // secret, which is why its value never appears in the preview (redact_env)
    // and why a key the user already wrote is reused rather than replaced.
    let mut env_backup = String::new();
    let mut warnings = Vec::new();
    if install {
        let token = match env_value(&env, ENV_KEY) {
            Some(existing) => existing.to_string(),
            None => generate_key()?,
        };
        let env_next = env_after(&env, &token);
        if env_next != env {
            if !env.is_empty() {
                let path = env_backup_path();
                std::fs::copy(env_path(), &path).map_err(|e| format!(".env backup failed: {e}"))?;
                env_backup = path.to_string_lossy().to_string();
            }
            atomic_write(&env_path(), env_next.as_bytes())?;
        }

        // Remember the connection — but never overwrite a URL or a key the
        // user typed into the keyring themselves.
        if !crate::secrets::present("hermes-url") {
            if let Err(err) = crate::secrets::set("hermes-url", HERMES_URL) {
                warnings.push(format!("hermes-url: {err}"));
            }
        }
        if !crate::secrets::present("hermes-token") {
            if let Err(err) = crate::secrets::set("hermes-token", &token) {
                warnings.push(format!("hermes-token: {err}"));
            }
        }
    }

    if install {
        let dir = plugin_dir();
        std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        std::fs::write(dir.join("plugin.yaml"), PLUGIN_YAML)
            .map_err(|e| format!("plugin.yaml: {e}"))?;
        std::fs::write(dir.join("__init__.py"), PLUGIN_PY)
            .map_err(|e| format!("__init__.py: {e}"))?;
    } else {
        let _ = std::fs::remove_dir_all(plugin_dir());
    }

    Ok(done_message(&backup, &env_backup, &warnings))
}

/// Temp file beside the target, then rename: a crash mid-write cannot leave
/// half a file where a config used to be.
fn atomic_write(target: &Path, bytes: &[u8]) -> Result<(), String> {
    let name = target
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| format!("{}: no file name to write", target.display()))?;
    let temp = target.with_file_name(format!("{name}.coucou-{}", std::process::id()));
    std::fs::write(&temp, bytes).map_err(|e| format!("write failed: {e}"))?;
    if let Err(err) = std::fs::rename(&temp, target) {
        let _ = std::fs::remove_file(&temp);
        return Err(format!("write failed: {err}"));
    }
    Ok(())
}

/// 32 bytes of system randomness, hex — only for a key the user does not
/// already have. A key they typed is kept exactly as they typed it.
fn generate_key() -> Result<String, String> {
    let mut buf = [0u8; 32];
    getrandom::fill(&mut buf).map_err(|e| format!("no system randomness available: {e}"))?;
    Ok(buf.iter().map(|b| format!("{b:02x}")).collect())
}

fn done_message(backup: &str, env_backup: &str, warnings: &[String]) -> String {
    let mut msg = match (backup.is_empty(), env_backup.is_empty()) {
        (true, true) => "Done. Start a new Hermes session to pick it up.".to_string(),
        (false, true) => format!(
            "Done. Previous config saved as {backup}. Start a new Hermes session to pick it up."
        ),
        (true, false) => format!(
            "Done. Previous .env saved as {env_backup}. Start a new Hermes session to pick it up."
        ),
        (false, false) => format!(
            "Done. Saved as {backup} and {env_backup}. Start a new Hermes session to pick it up."
        ),
    };
    if !warnings.is_empty() {
        msg.push(' ');
        msg.push_str(&format!(
            "Keyring: {} — put the URL and the key in Settings by hand.",
            warnings.join(", ")
        ));
    }
    msg
}

// ── Starting the API server ──────────────────────────────────────────────────

/// `hermes gateway`, detached: the server is Hermes's process, not ours, and
/// it has to outlive the settings window — and Coucou itself.
///
/// ponytail: on Windows a `hermes.cmd` shim would not be found (CreateProcess
/// only appends `.exe`); the installer writes `hermes.exe`, and if that ever
/// changes the fix is to look for the shim ourselves.
pub async fn start_gateway() -> Result<String, String> {
    let saved = crate::secrets::get("hermes-url").unwrap_or_else(|| HERMES_URL.to_string());
    let url = saved.trim().trim_end_matches('/');
    if !is_local(url) {
        return Err(format!(
            "{url} is not this machine — start it where it runs: hermes gateway"
        ));
    }
    if probe(url).await.is_some() {
        return Ok(format!("The API server is already up at {url}."));
    }

    let log = settings::local_dir().join("hermes-gateway.log");
    if let Some(dir) = log.parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    let file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log)
        .map_err(|e| format!("{}: {e}", log.display()))?;
    let err_file = file
        .try_clone()
        .map_err(|e| format!("{}: {e}", log.display()))?;

    let mut cmd = std::process::Command::new("hermes");
    cmd.arg("gateway")
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::from(file))
        .stderr(std::process::Stdio::from(err_file));
    // Its own session, so closing Coucou (or a terminal) cannot SIGHUP it.
    #[cfg(unix)]
    unsafe {
        use std::os::unix::process::CommandExt;
        cmd.pre_exec(|| {
            if libc::setsid() == -1 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x0800_0000); // CREATE_NO_WINDOW — no console flash
    }
    cmd.spawn().map_err(|err| match err.kind() {
        std::io::ErrorKind::NotFound => {
            "hermes is not on PATH — install it first (the command is at the top of this window)."
                .to_string()
        }
        other => format!("could not run `hermes gateway`: {other}"),
    })?;
    crate::log::line("hermes: started `hermes gateway`");
    Ok(format!(
        "Started `hermes gateway` — listening on {url} in a second or two. Log: {}",
        log.display()
    ))
}

/// Only a server on this machine can be started by us; anything else has to
/// be started where it runs.
fn is_local(url: &str) -> bool {
    let host = url
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .split('/')
        .next()
        .unwrap_or_default()
        .rsplit('@')
        .next()
        .unwrap_or_default()
        .split(':')
        .next()
        .unwrap_or_default()
        .trim_matches(|c| c == '[' || c == ']');
    host.is_empty() || matches!(host, "localhost" | "127.0.0.1" | "::1" | "0.0.0.0")
}

/// Any HTTP answer at all means something is already listening — even a 401
/// or a 404, which a wrong path would give. Only a connection failure means
/// we should start a server.
async fn probe(url: &str) -> Option<()> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(2))
        .build()
        .ok()?;
    let mut request = client.get(format!("{url}/health"));
    if let Some(token) = crate::secrets::get("hermes-token") {
        request = request.header("authorization", format!("Bearer {token}"));
    }
    request.send().await.ok().map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXISTING: &str = "\
# my hermes config
model: anthropic/claude-sonnet-4.6

security:
  approval:
    mode: smart

plugins:
  enabled:
    - something-else
  hook_callback_timeout: 30
";

    fn content(text: &str) -> Vec<&str> {
        text.lines()
            .filter(|line| !line.trim_start().starts_with('#'))
            .collect()
    }

    #[test]
    fn install_adds_only_our_keys() {
        let next = with_keys(EXISTING).unwrap();
        assert!(next.contains("transport: coucou"));
        assert!(next.contains("transport_fallback: builtin"));
        assert!(next.contains("- something-else"));
        assert!(next.contains("- coucou"));
        assert!(next.contains("mode: smart"), "their approval mode survives");
        assert!(next.contains("hook_callback_timeout: 30"));
        assert!(next.contains("# my hermes config"), "comments survive");
        assert!(next.contains("model: anthropic/claude-sonnet-4.6"));
    }

    #[test]
    fn install_is_idempotent() {
        let once = with_keys(EXISTING).unwrap();
        let twice = with_keys(&once).unwrap();
        assert_eq!(once, twice);
        assert_eq!(once.matches("- coucou").count(), 1);
        assert_eq!(once.matches("transport: coucou").count(), 1);
    }

    #[test]
    fn uninstall_restores_the_original() {
        let installed = with_keys(EXISTING).unwrap();
        let back = without_keys(&installed).unwrap();
        assert_eq!(content(&back), content(EXISTING));
        assert!(!back.contains("coucou"));
    }

    #[test]
    fn uninstall_keeps_somebody_elses_settings() {
        let text = "security:\n  approval:\n    transport: other\n    mode: smart\n";
        assert_eq!(without_keys(text).unwrap(), text);
        let fallback = "security:\n  approval:\n    transport_fallback: builtin\n";
        assert_eq!(without_keys(fallback).unwrap(), fallback);
    }

    #[test]
    fn an_empty_config_comes_out_with_just_ours() {
        let next = with_keys("").unwrap();
        assert!(next.contains("plugins:\n  enabled:\n    - coucou"));
        assert!(next.contains("security:\n  approval:\n    transport: coucou"));
        assert!(next.ends_with('\n'));
        assert_eq!(without_keys(&next).unwrap(), "");
        assert_eq!(without_keys("").unwrap(), "");
    }

    #[test]
    fn a_refused_shape_is_an_error_not_a_rewrite() {
        assert!(with_keys("security: nope\n").is_err());
        assert!(with_keys("plugins:\n  enabled: everything\n").is_err());
    }

    #[test]
    fn an_inline_enabled_list_is_extended_in_place() {
        let text = "plugins:\n  enabled: [a, \"b\"]\n";
        let next = with_keys(text).unwrap();
        assert!(next.contains("enabled: [a, \"b\", coucou]"));
        assert_eq!(without_keys(&next).unwrap(), text);
    }

    #[test]
    fn the_marker_holds_for_every_shape() {
        assert!(has_transport(
            "security:\n  approval:\n    transport: coucou\n"
        ));
        assert!(has_transport(
            "security:\n  approval:\n    transport: \"coucou\"\n"
        ));
        assert!(!has_transport(
            "security:\n  approval:\n    transport: other\n"
        ));
        assert!(!has_transport(
            "mcp_servers:\n  coucou:\n    url: http://x\n"
        ));
    }

    const ENV_TEXT: &str = "# my env\nFOO=bar\nexport API_SERVER_KEY='their key'\n";

    #[test]
    fn env_install_is_idempotent_and_keeps_their_lines() {
        let once = env_after(ENV_TEXT, "their key");
        assert!(once.contains("API_SERVER_ENABLED=true"));
        assert!(
            once.contains("export API_SERVER_KEY='their key'"),
            "their line survives"
        );
        assert!(once.contains("# my env"), "comments survive");
        assert_eq!(once.matches("API_SERVER_ENABLED").count(), 1);
        assert_eq!(
            env_after(&once, "their key"),
            once,
            "a second install changes nothing"
        );
    }

    #[test]
    fn env_lookalike_lines_are_never_ours() {
        assert_eq!(env_value("API_SERVER_KEYS=x\n", ENV_KEY), None);
        assert_eq!(env_value("#API_SERVER_KEY=x\n", ENV_KEY), None);
        assert_eq!(
            env_value("  export API_SERVER_KEY=\"spaced\"\n", ENV_KEY),
            Some("spaced")
        );
        assert_eq!(env_value("", ENV_KEY), None);
    }

    #[test]
    fn a_file_we_do_not_have_to_change_does_not_change() {
        let text = "API_SERVER_ENABLED=true\nAPI_SERVER_KEY=abc\n";
        assert_eq!(env_after(text, "abc"), text);
        let no_newline = "API_SERVER_KEY=abc";
        assert_eq!(set_env_value(no_newline, ENV_KEY, "abc"), no_newline);
    }

    #[test]
    fn the_key_is_masked_on_both_sides_of_the_diff() {
        let before = "FOO=1\nAPI_SERVER_KEY=super-secret\n";
        let after = env_after(before, "super-secret");
        assert!(!redact_env(before).contains("super-secret"));
        assert!(!redact_env(after).contains("super-secret"));
        assert!(redact_env(after).contains("API_SERVER_KEY=••••••••"));
        assert!(redact_env(after).contains("API_SERVER_ENABLED=true"));
    }

    #[test]
    fn generated_keys_are_32_bytes_of_hex() {
        let key = generate_key().unwrap();
        assert_eq!(key.len(), 64);
        assert!(key.chars().all(|c| c.is_ascii_hexdigit()));
        assert_ne!(key, generate_key().unwrap());
    }

    #[test]
    fn only_this_machines_url_is_ours_to_start() {
        assert!(is_local("http://127.0.0.1:8642"));
        assert!(is_local("http://localhost:8642"));
        assert!(is_local("https://[::1]:8642"));
        assert!(!is_local("https://agent.example.com"));
        assert!(!is_local("http://100.x.y.z:8642"));
    }
}
