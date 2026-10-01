# Changelog

## 0.1.4

- Global shortcut that reveals the island (`Ctrl+Shift+N` by default) — Windows and Linux, off until you switch it on in Settings
- Linux: the panel no longer swallows clicks — when the island is folded, clicks fall through to the window underneath (X11)
- Settings puts Hermes first and spells out that the Hermes plugin is a file copied to `~/.hermes/plugins/coucou/`, not part of the app
- The island's Settings view shows the Hermes plugin's install state beside Claude Code's

## 0.1.3

- Maintainer metadata is PeDitX now — `authors` in both `Cargo.toml` files (shows up as the Debian `Maintainer` field) and `copyright` in both `tauri.conf.json` files (Windows file properties)
- Releases no longer claim macOS: this fork ships Linux and Windows, macOS comes from the original publisher

## 0.1.2

- Hermes Agent — live sessions and approvals in the island on macOS, Windows and Linux: a plugin at `~/.hermes/plugins/coucou/` feeds the same relay Claude Code uses, installed from Settings with the same backup → diff → confirm flow
- Settings spells out the Hermes setup on all three platforms — where the API server is reachable from, where the key goes, how local file access is wired
- README in English, Farsi, Arabic, Russian and Chinese with a language selector, pointing at the original publisher

## Unreleased

- Linux app — the same Tauri build with X11/Wayland and `.deb`/`.rpm` packages
- Hermes Agent — a full-screen chat window on macOS, Windows and Linux, plus an explicit choice of engine (Claude or Hermes) for the island chat with an explicit fallback; nothing switches on its own
- Compact island on screens without a notch (#22) — thanks @Kamasoutra
- Only web links (http/https) open from the notch; other kinds of links from Claude or integrations are ignored (#16) — thanks @Cris1670
- Hook socket limited to your own user account, with size and time limits; logs no longer keep commands, n8n data or full URLs, and stay under 1 MB (#16) — thanks @Cris1670 and @Vignesh-Thangamariappan
- The island always reopens after folding, and Settings opens below it, resizable — thanks @rouderz
- Choose the Claude model for the chat in Settings; the list comes from your Anthropic account, and Claude Sonnet 4.6 stays the default — thanks @rouderz
- Windows build artifacts are now downloadable from a manual CI run — thanks @MysJofR
