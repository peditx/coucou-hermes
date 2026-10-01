<div align="center">

<img src="NotchBuddy/Assets.xcassets/AppIcon.appiconset/icon_256x256.png" width="96" alt="Coucou icon">

# Coucou

[🇮🇷 فارسی](README-fa.md) · [🇸🇦 العربية](README-ar.md) · [🇷🇺 Русский](README-ru.md) · [🇨🇳 中文](README-ch.md)

**Original repository:** [Louis-CFM/coucou](https://github.com/Louis-CFM/coucou) — this is a fork with CI/CD rearranged for multi-platform releases.

**A tiny friend that lives in your Mac's notch — or at the top of your screen on Windows and Linux — and keeps an eye on your Claude Code sessions.**

Approve permissions, watch your agents work, drop a file, chat with Claude — all without leaving what you're doing.

![macOS 15+](https://img.shields.io/badge/macOS-15%2B-black?logo=apple)
![Windows 10/11](https://img.shields.io/badge/Windows-10%2F11-0078D4?logo=windows&logoColor=white)
![Linux](https://img.shields.io/badge/Linux-any-16A085?logo=linux&logoColor=white)
![Swift 6](https://img.shields.io/badge/Swift-6-F05138?logo=swift&logoColor=white)
![SwiftUI](https://img.shields.io/badge/SwiftUI-native-0A84FF)
![Tauri 2](https://img.shields.io/badge/Tauri-2-FFC131?logo=tauri&logoColor=black)
![License: MIT](https://img.shields.io/badge/license-MIT-green)
![GitHub stars](https://img.shields.io/github/stars/Louis-CFM/coucou?style=social)

<img src="docs/media/demo.gif" width="760" alt="Coucou in action">

</div>

---

## Why

Some studios showed off gorgeous notch companions… and never let anyone use them.
**Coucou is the open version.** Every line of code, every animation, every sound — free to use, read, fork and remix.

Meet **Mochi**: a soft little squircle with big eyes that pops out of your notch, waves hello, follows your cursor with its eyes, gets annoyed when you poke it (and dizzy if you insist), and tells you the moment Claude Code needs you.

## Features

- 🤖 **Claude Code & Hermes Agent, live** — see every session in your notch: what it reads, edits and runs, step by step. Works with both engines. Finished? Mochi does a happy little jump.
- ✅ **Approve from the notch** — permission requests (Claude Code or Hermes) show up with **Allow / Deny**. One click, back to work.
- 🧑‍💻 **Jump to the right terminal** — open the exact terminal window of a session *(macOS)*.
- 💬 **Chat with Claude or Hermes** — built-in chat, straight from the notch. Pick the model in Settings; the list comes from your Anthropic account or point Coucou at a remote [Hermes Agent](https://github.com/NousResearch/hermes-agent).
- 🪽 **Full-screen Hermes chat** — open a desktop-style chat window with Hermes Agent. The island chat can also be served by Hermes: you pick the **Engine** (Claude / Hermes) and **Fallback** (None / Claude / Hermes) in Settings — nothing switches on its own. [Details](docs/HERMES.md).

- 📎 **Drop a file on the notch** — Mochi turns into a box and swallows it, then ask a question about it or send it by email *(email: macOS, Mail.app)*.
- 🪟 **Drag Mochi onto any window** — attach that window as context for Claude *(macOS)*.
- 🔌 **Integrations** — Stripe payments, n8n workflows, GitHub, Vercel deployments, Resend emails, Notion, Cal.com. Each one gets its own little colored Mochi.
- 🎭 **A real character** — idle breathing, blinks, eyes on a sphere that follow your mouse, emotes, 28 handcrafted sounds, a greeting on launch.
- ⌨️ **Open it with one keypress** — a global shortcut reveals the island, off by default and yours to pick in Settings (⌘⇧N on macOS, `Ctrl+Shift+N` on Windows and GNOME Linux).
- 🫥 **Invisible when idle** — hides away when nothing is running, peeks out when you hover the notch (the top edge of the screen on Windows).
- 🖥️ **Any Mac, notch or not** — on an iMac, a Mac mini, or a MacBook with its lid closed on an external display, Mochi sits in a small bar at the top of the screen.
- 🔒 **Private by design** — no telemetry, no account. Keys live in your macOS Keychain or Windows Credential Manager. The app only talks to the services you plug in.

<table>
<tr>
<td><img src="docs/media/claude-code.png" alt="Claude Code session"></td>
<td><img src="docs/media/stripe.png" alt="Stripe payments"></td>
</tr>
<tr>
<td><img src="docs/media/chat.png" alt="Chat with Claude"></td>
<td><img src="docs/media/dizzy.png" alt="Too many hits"></td>
</tr>
</table>

## Install

### Download for macOS

1. Grab the latest `Coucou.zip` from [Releases](https://github.com/Louis-CFM/coucou/releases).
2. Unzip and move **Coucou.app** to `/Applications`.
3. Launch. This build isn't notarized by Apple yet, so the first time macOS says it can't verify the developer: open **System Settings → Privacy & Security**, scroll down and click **Open Anyway** (only once).

### Windows

The Windows installer is **temporarily unavailable**. Microsoft Defender wrongly
flags the unsigned installer as malware; a false-positive report is under review
at Microsoft and the installer will come back once it is cleared and signed.
Until then you can [build it from source](#build-from-source).

There is no notch on a PC, so the island slides out of the top edge of the screen
instead of hiding inside one. See [`windows/README.md`](windows/README.md) for the
rest of the differences.

### Linux

Packages are built by CI on every push — grab `Coucou-Linux-packages` from a
workflow run, then:

```bash
sudo apt install ./Coucou-Linux-X.Y.Z-amd64.deb    # Debian / Ubuntu / Mint
sudo dnf install ./Coucou-Linux-X.Y.Z-x86_64.rpm   # Fedora / RPM-based
```

There is no notch, and no compositor lets a client pin a window to the top edge
on every desktop — so the island is an always-on-top window that positions
itself, falling back to XWayland on Wayland sessions. See
[`linux/README.md`](linux/README.md) for the differences and what is not wired
up yet.

### Build from source

**macOS** — requirements: macOS 15+, Xcode 16+, [XcodeGen](https://github.com/yonaskolb/XcodeGen).

```bash
brew install xcodegen
git clone https://github.com/Louis-CFM/coucou.git
cd coucou/NotchBuddy
xcodegen
open NotchBuddy.xcodeproj   # then ⌘R
```

**Windows** — requirements: [Rust](https://rustup.rs), Node 20+, MSVC build tools.

```powershell
git clone https://github.com/Louis-CFM/coucou.git
cd coucou/windows
npm install
npm run pack                # installer lands in windows/release/
```

**Linux** — requirements: [Rust](https://rustup.rs), Node 20+, and Tauri's build
dependencies (listed in [`linux/README.md`](linux/README.md)).

```bash
git clone https://github.com/Louis-CFM/coucou.git
cd coucou/linux
npm install
npm run pack                # .deb + .rpm land in linux/release/
```

Nothing is compiled on a development machine: every build runs in GitHub
Actions.

## Setup

Click the Coucou icon in the menu bar (macOS) or in the system tray (Windows, Linux) → **Settings…**

| What | Why | Where the key goes |
|---|---|---|
| **Claude Code hooks** | live sessions & approvals for Claude Code | **Install hooks** — Coucou backs up `~/.claude/settings.json`, merges its hooks and shows you the diff before writing anything |
| **Hermes hooks** | live sessions & approvals for Hermes Agent | **Install plugin** — Coucou backs up `~/.hermes/config.yaml`, adds its three keys and shows you the diff before writing anything |
| **Anthropic API key** | chat and questions about files | Keychain / Credential Manager / Secret Service |
| **Hermes URL + key** *(optional)* | the full-screen Hermes chat, and the island chat if you pick Hermes as its engine; for local files from the remote agent, add MCP on the Hermes side | Keychain / Credential Manager / Secret Service |
| Stripe, n8n, GitHub, Vercel, Resend, Notion, Cal.com | the integration pills | Keychain / Credential Manager / Secret Service, all optional |

If Coucou isn't running, the hook exits immediately: **Claude Code is never blocked.**

## Things to try

| Do this | Mochi does that |
|---|---|
| Hover the notch (top edge on Windows) | peeks out and says hi 👋 |
| Click it | opens |
| Hover Mochi | blinks, eyes grow |
| Click Mochi | squish + annoyed |
| Click 3 times fast | 😵‍💫 dizzy for a few seconds |
| Drag a file onto the island | turns into a box and swallows it |
| Drag Mochi onto a window *(macOS)* | attaches it as context |

## How it works

**macOS**

- **Island**: a borderless `NSPanel` hugging the notch, driven by a small state machine (`hidden → petit → home`).
- **Character**: drawn in SwiftUI `Canvas` + `TimelineView` at 60 fps — squircle body, eyes projected on a sphere, spring animations. No Rive, no Lottie, no images.
- **Claude Code & Hermes**: a tiny `nb-hook` / `coucou-hook` script receives hook events and forwards them over a Unix socket (named pipe on Windows) to the app, and the Hermes plugin at `~/.hermes/plugins/coucou/` feeds that same relay. For approvals it waits for your click, then answers the hook. Works with both engines.
- **Integrations**: lightweight pollers, paused when nothing is watching.
- **Sounds**: 28 short WAVs played through preloaded `AVAudioPlayer`s.

The macOS app is native Swift 6 / SwiftUI / AppKit with **zero third-party dependencies**.

**Windows**

- A [Tauri 2](https://tauri.app) app (Rust + TypeScript): the island is a transparent, always-on-top window that never steals focus, Mochi is drawn in Canvas 2D with the same shapes, timings and sounds as on the Mac.
- Claude Code hooks go through a tiny `coucou-hook.exe` and a named pipe, and the Hermes plugin uses that same relay; keys live in Windows Credential Manager.
- Details and differences in [`windows/README.md`](windows/README.md).

**Linux**

- The same Tauri 2 app with the platform layer swapped: the named pipe becomes a Unix socket at `$XDG_RUNTIME_DIR/coucou.sock` (checked with `SO_PEERCRED`), paths move to XDG, keys go to the Secret Service, and `.deb`/`.rpm` replace the NSIS installer.
- Details, packaging and what is still missing in [`linux/README.md`](linux/README.md).

## Contributing

Issues and PRs are very welcome — new integrations, new emotes, new sounds, bug fixes. See [CONTRIBUTING.md](CONTRIBUTING.md).

## Credits

Built by [Louis Raillé](https://louisraille.fr) with Claude Code.
Inspired by the notch-companion concepts shared by design studios — this project is independent and not affiliated with any of them.

## License

- **Code:** [MIT](LICENSE) — use it, fork it, learn from it, just keep the copyright notice.
- **Name, Mochi character, icon, sounds and media:** © Louis Raillé, all rights reserved — see [LICENSE-ASSETS.md](LICENSE-ASSETS.md). Shipping your own fork? Give it your own name and character.

<div align="center">

**If Mochi made you smile, a ⭐ helps a lot.**

[Website](https://louis-cfm.github.io/coucou/) · [Privacy](https://louis-cfm.github.io/coucou/privacy.html) · [Terms](https://louis-cfm.github.io/coucou/terms.html) · [Support](https://louis-cfm.github.io/coucou/support.html)

</div>
