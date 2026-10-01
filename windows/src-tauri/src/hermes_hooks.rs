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

use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::settings;

/// Directory name under ~/.hermes/plugins and the transport name it registers.
pub const PLUGIN_NAME: &str = "coucou";

/// The plugin ships inside the binary so install needs no network and no
/// second copy of the code. Both files live in the repo at `hermes-plugin/`.
const PLUGIN_YAML: &str = include_str!("../../../hermes-plugin/plugin.yaml");
const PLUGIN_PY: &str = include_str!("../../../hermes-plugin/__init__.py");

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

fn current_fingerprint() -> String {
    match std::fs::read(config_path()) {
        Ok(bytes) => crate::hooks::fingerprint(&bytes),
        Err(_) => crate::hooks::fingerprint(b""),
    }
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
        None => return,
    };

    // Inline form: `enabled: [a, b]` — extended where it stands.
    if value.starts_with('[') {
        if inline_has(value, item) {
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
        if !inline_has(value, item) {
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
fn parent_keys_of(path: &[&str]) -> Vec<&str> {
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
                drop_empty_mapping(&lines, vec!["security"]);
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
    Ok(HermesHookPreview {
        diff: crate::hooks::unified_diff(&current, &next),
        backup: backup_path().to_string_lossy().to_string(),
        config_path: config_path().to_string_lossy().to_string(),
        plugin_path: plugin_dir().to_string_lossy().to_string(),
        fingerprint: current_fingerprint(),
    })
}

/// Writes the merged (or cleaned) config and the plugin files, after a backup.
pub fn write(install: bool, fingerprint: &str) -> Result<String, String> {
    let current = read_config()?;
    if current_fingerprint() != fingerprint {
        return Err(format!(
            "{} changed since the preview. Nothing was written — review the new diff.",
            config_path().display()
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
        let path = config_path();
        let dir = path.parent().unwrap_or(Path::new("."));
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
        let temp = config_path().with_extension(format!("yaml.coucou-{}", std::process::id()));
        std::fs::write(&temp, next.as_bytes()).map_err(|e| format!("write failed: {e}"))?;
        if let Err(err) = std::fs::rename(&temp, config_path()) {
            let _ = std::fs::remove_file(&temp);
            return Err(format!("write failed: {err}"));
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

    Ok(backup)
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
}
