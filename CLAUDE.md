# Coucou — guide for AI coding agents

Coucou has three code bases for one product: Mochi, a small animated character, shows Claude Code sessions and a few integrations, and lets the user approve, answer, chat and drop files without leaving what they are doing.

- `NotchBuddy/` — the native macOS app (Swift 6, SwiftUI, AppKit). Mochi lives in the notch.
- `windows/` — the Tauri 2 app (Rust + TypeScript). Mochi lives at the top of the screen.
- `linux/` — the same Tauri app with the platform layer swapped (Unix socket, XDG paths, Secret Service, `.deb`/`.rpm`).

## Where things are
- `NotchBuddy/Sources/App/` — all Swift code. `NotchBuddy/Resources/sounds/` — the 28 WAV sounds. `NotchBuddy/project.yml` — XcodeGen project (never edit the `.xcodeproj` by hand).
- `docs/SPEC.md`, `docs/INTEGRATIONS.md` — behaviour, views, states, integrations.
- `docs/HERMES.md` — the Hermes Agent integration: what is shipped, what is still plan.
- `design/prototype/notch-buddy.html` — original prototype, the visual source of truth. `design/captures/` — target screenshots.
- `docs/*.html` — the GitHub Pages site (privacy, terms, support, legal notice).

## Build
Nothing is compiled on a development machine. Edit, run read-only checks, push, and let the workflow do it: `build.yml` / `release.yml` (macOS), `windows.yml`, `linux.yml`. `linux.yml` also runs the Rust tests on every push.

## Rules
- Swift 6, SwiftUI + AppKit. No third-party dependencies unless truly unavoidable. The character is drawn in code (`Canvas` + `TimelineView`), no Rive/Lottie/images.
- Secrets live in the Keychain (macOS), Credential Manager (Windows) or Secret Service (Linux) — never on disk or in git.
- No telemetry. Network calls only to services the user configured.
- The island chat engine and its fallback are explicit user choices (Settings → Island chat). Never switch engines on the user's behalf.
- Never block Claude Code: if the app doesn't answer, the hook exits immediately.
- Never overwrite `~/.claude/settings.json`: dated backup, merge, show the diff, write only after the user confirms.
- Never send an email or approve a Claude Code permission without an explicit click.
- Performance: 0 % CPU when the island is hidden.
- Keep the bundle identifier `fr.louisraille.NotchBuddy` (Keychain items, preferences and permissions depend on it) and the Tauri identifier `fr.louisraille.coucou`.
- Visual changes must match the prototype and the screenshots in `design/captures/`.
