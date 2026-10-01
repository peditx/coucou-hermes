<div align="center">

<img src="src-tauri/icons/128x128.png" width="96" alt="Coucou icon">

# Coucou for Linux

**Mochi doesn't get a notch on Linux — so it lives at the top of your screen instead.**

Approve Claude Code permissions, watch your session work, drop a file, chat with Claude, keep an eye on your services — without leaving what you're doing.

![Linux](https://img.shields.io/badge/Linux-any-16A085?logo=linux&logoColor=black)
![Tauri 2](https://img.shields.io/badge/Tauri-2-FFC131?logo=tauri&logoColor=black)
![Rust](https://img.shields.io/badge/Rust-backend-000?logo=rust)
![License: MIT](https://img.shields.io/badge/license-MIT-green)

</div>

<img src="../windows/screenshots/greeting.png" width="640" alt="Mochi waving hello at launch">

---

## Install

Packages are built by CI on every push — grab `Coucou-Linux-packages` from the
workflow run, or the release once one is tagged `linux-v*`:

```bash
# Debian / Ubuntu / Mint
sudo apt install ./Coucou-Linux-X.Y.Z-amd64.deb

# Fedora / RPM-based
sudo dnf install ./Coucou-Linux-X.Y.Z-x86_64.rpm
```

## Using it

<img src="../windows/screenshots/compact.png" width="292" alt="The compact island, with the integration pills as mini Mochis">
<img src="../windows/screenshots/overview.png" width="640" alt="The overview: the focused integration on the left, the other pills on the right">
<img src="../windows/screenshots/approval.png" width="640" alt="A Claude Code permission request, with Deny and Allow">
<img src="../windows/screenshots/chat.png" width="640" alt="Chatting with Claude from the island">
<img src="../windows/screenshots/drop.png" width="640" alt="Mochi turned into a box, waiting for a file">

| What you do | What happens |
|---|---|
| Open the island from the tray | Mochi is there |
| Click Mochi | It gets annoyed. Three times in a row and it goes dizzy |
| Rest the pointer on Mochi for two seconds | Hearts |
| Drag a file onto the island | Mochi turns into a box, swallows it, then offers to answer questions about it |
| `Esc` | Closes the island |
| Tray icon | Open, Settings…, Pause, Quit |

Everything else happens on its own: a Claude Code permission request opens the
island with **Deny / Allow**, a finished session shows what it did, and your
integrations sit in the coloured pills next to Mochi.

## Claude Code

Open **Settings… → Claude Code → Install hooks…**. You get the exact diff of what
will change in `~/.claude/settings.json`, the path of the dated backup that will
be taken, and nothing is written until you click. Your own hooks are never
touched, and uninstalling removes only Coucou's entries.

The relay is a tiny binary, `coucou-hook`, copied to
`$XDG_STATE_HOME/coucou/bin/` at launch and made executable. It is given 300 ms
to reach Coucou and exits cleanly if the app is closed, slow or crashed — **a
Claude Code session is never blocked or slowed down by Coucou.** If nobody
answers a permission request in time, Coucou stays quiet and Claude Code asks in
the terminal as usual.

The relay and the app meet on `$XDG_RUNTIME_DIR/coucou.sock` (a directory logind
creates `0700`, so only your own session can reach it), and the server checks
`SO_PEERCRED` on every connection — same user, always. It works from any
terminal: GNOME Terminal, Konsole, VS Code, a plain tty.

## Chat and keys

**Settings… → Claude** takes your Anthropic API key. Keys live in the **Secret
Service** (gnome-keyring, KWallet), never on disk and never in the interface —
the island can only ask whether a key exists. Same for every integration key.

No telemetry. The only network requests Coucou makes are to the services you
configure yourself.

## Build it yourself

You need [Rust](https://rustup.rs), [Node 20+](https://nodejs.org), and Tauri's
Linux build dependencies:

```bash
sudo apt install build-essential curl wget file pkg-config \
  libssl-dev libdbus-1-dev \
  libwebkit2gtk-4.1-dev libgtk-3-dev \
  libayatana-appindicator3-dev librsvg2-dev libxdo-dev
```

```bash
cd linux
npm install
npm run tauri dev      # live-reloading development build
npm run pack           # builds .deb + .rpm into linux/release/
```

`npm run dev` alone serves the front end in an ordinary browser, which is enough
to work on the island's looks. Neither page ships in the app.

`npm run pack` leaves two files in `linux/release/`, the same names the release
workflow publishes:

```
Coucou-Linux-X.Y.Z-amd64.deb    the Debian package
Coucou-Linux-X.Y.Z-x86_64.rpm   the RPM
```

Nothing is ever compiled on a development machine for this project — every build
runs in GitHub Actions (`.github/workflows/linux.yml`), which also runs the Rust
tests on each push.

The 28 sounds are the macOS app's own files; they are never duplicated in this
folder. The path is declared once, in `SOUNDS_DIR` at the top of `vite.config.ts`
— when they move to `shared/sounds/`, change that one line.

The app icon and the tray icon are drawn in code, like Mochi itself:

```bash
npm run icons          # regenerates src-tauri/icons from scripts/gen-icons.mjs
```

### Layout

```
linux/
  src/                 island front end (TypeScript, no framework)
    mochi/             Mochi and the launch greeting, in Canvas 2D
    island/            state machine, hooks, integrations
    views/             every island view
    settings/          the settings window
  src-tauri/           Rust backend: window, Unix socket, Claude API, pollers
  hook/                coucou-hook, the Claude Code relay
  scripts/             icon generator, package packer
```

### Log

`$XDG_STATE_HOME/coucou/coucou.log` (normally `~/.local/state/coucou/coucou.log`)
— hook events, permission decisions, poller problems. It stays on your machine.

## Window placement on Linux

Layer-shell — the protocol that lets a client say "pin me to the top edge" — is
not part of core Wayland, and the compositor must implement it:

| Environment | Layer-shell |
|---|---|
| Sway, Hyprland (wlroots) | yes |
| KDE Plasma Wayland | yes |
| **GNOME Wayland** | **no** |
| X11 / Xorg | not applicable |

Coucou therefore runs its window the portable way: a borderless, always-on-top
window that positions itself at the top centre of the monitor through Tauri.
No Wayland client may choose a position at all, so on any Wayland session the
app sets `GDK_BACKEND=x11` before GTK starts and runs under **XWayland**, which
does allow positioning. Nothing to install — XWayland ships with every Wayland
desktop — and an explicit `GDK_BACKEND` in your environment is never overridden.
Layer-shell support (true top-edge placement, no XWayland) is the upgrade path;
GNOME still has no implementation of it.

## Keyboard shortcut

**Settings → General → Show island with shortcut** (off by default) registers a
global key — `Ctrl+Shift+N` out of the box, changeable to any `Ctrl` / `Alt` /
`Shift` / `Super` combination. Press it anywhere and the island opens.

Who holds the key depends on the desktop, because a Wayland client may not ask
for one itself:

| Desktop | Who holds the key |
|---|---|
| GNOME (X11 and Wayland) | a custom keybinding in `org.gnome.settings-daemon.plugins.media-keys` — the registry GNOME Settings itself writes |
| Everything else | nobody — use the tray |

Turning it off removes exactly what Coucou added; your own custom keybindings
are never touched. Either way it is logged to
`~/.local/state/coucou/coucou.log`.

## What's different from the Mac version

- No notch, so the island lives at the top centre of the screen and retracts into
  the top edge instead of hiding in a notch.
- Permission approval works from **any** terminal; the Mac build only listens to
  VS Code sessions.
- **Not yet wired:** waking the island by hovering the top edge (Linux has no
  global pointer query through Tauri — open it from the tray for now), attaching
  Mochi to a window as context, and jumping to a specific terminal window.
- **Email** keeps its feature and will use `xdg-email --attach` so any installed
  mail client handles it; with no client installed, the view says so instead of
  dropping the button.
- Cal.com shows the next bookings as a list rather than the Mac's calendar.
