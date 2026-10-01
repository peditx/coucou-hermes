# Notch Buddy — specification

All measurements are in macOS points. The values come from `reference/notch-buddy.html` (the `NW`, `NH`, `EW`, `VIEWS`, `STATES`, `EMOTES`, `PISTES`, `AGENTS` constants and the `Bot` class). When in doubt, re-read the prototype's code.

---

## 1. Window and notch

- A borderless `NSPanel`: `styleMask [.borderless, .nonactivatingPanel]`, transparent background, no shadow, level above the menu bar (`.mainMenu + 3` or equivalent that sits above the bar and full-screen apps), `collectionBehavior [.canJoinAllSpaces, .stationary, .fullScreenAuxiliary, .ignoresCycle]`.
- Fixed size 720 × 320, anchored top-centre on the screen that has a notch. The island is drawn inside it, stuck to the top edge.
- **Click-through**: the transparent area must never block clicks. Toggle `ignoresMouseEvents` at 60 Hz depending on whether `NSEvent.mouseLocation` is inside the island's shape (plus 6 pt of margin) or not.
- The panel may become key only when a text field in the island has focus (prompt, email). Otherwise it never steals focus.
- Notch detection: `NSScreen.safeAreaInsets.top` > 0 and `auxiliaryTopLeftArea` / `auxiliaryTopRightArea`. Notch width `wN` = screen width − the two auxiliary areas; height `hN` = `safeAreaInsets.top`. The prototype uses `wN = 184`, `hN = 32`: in the app, take the real values.
- No notch (desktop Mac, external display or closed lid): show an 80 pt black bar at rest (`hidden`) on the main screen, with Mochi visible in the centre and its animation paused; 240 pt in `compact`, top-centre. Height capped at 24 pt and at the menu bar's height; character and pills adapt to that height. The greeting folds back to the real dimensions of the compact bar. The rest-state hover zone does not extend below the bar. Open views keep their 640 pt width.
- Mouse tracking: poll `NSEvent.mouseLocation` every frame. No permission needed.

### Island shape
- Black rectangle `#000`, square top corners (it blends into the top edge of the screen), rounded bottom corners: 14 pt in hidden/peek/compact, 30 pt in expanded.
- Two 14 pt concave « ears » at the top corners, on the outside, so the shape flows into the top edge of the screen (see `#island::before/::after` in the prototype).

## 2. Island modes

| Mode | Width | Height | Main character | Secondary agents |
|---|---|---|---|---|
| `hidden` | wN | hN | invisible | invisible |
| `peek` | wN + 64 | hN | Ø 18, centre x = 19 | invisible |
| `compact` | wN + 104 | hN | Ø 20, centre x = 27 | 2×2 grid in the right ear |
| `expanded` | 640 | per view (§5) | per view | per view |

(Ø = body diameter. The character's canvas is Ø / 0,6 on a side: the body takes up 60 % of the canvas, the rest is for particles, hands and the badge.)

Compact grid: pills Ø 9,5 around the point (width − 27, hN/2), offset ±6. 1 agent: centred. 2: side by side. 3: two on top, one below. 4: a square.

## 3. Behaviour rules (validated by Louis)

1. **Nothing running** → `hidden`. Completely invisible.
2. **Mouse over the notch** while `hidden` → `peek` immediately, the character comes out doing a coucou (hands + `peek` sound + `greet` sound). If the mouse stays 650 ms → `expanded` (the `overview` view, or `empty` if there is no task at all). If it leaves during the peek → back to `hidden` after 600 ms.
3. **Tasks running and Louis is active** → `compact`: very thin, character visible, it follows the mouse with its eyes anywhere on the screen.
4. **Hover in compact** → `expanded` after 200 ms. Clicking the character in compact → `expanded` straight away.
5. **Auto close**: once open, the island folds after **60 s without activity** (mouse movement over the island, click, typing). Leaving the island does not close it. During the last 10 seconds, a 2 pt line at the bottom centre (160 pt → 0, white 35 %) shows the countdown. `Esc` closes.
6. **Louis absent** (no mouse movement for 3 min, configurable) → `hidden`, even with tasks running. On the first movement → back to `compact` if tasks are running.
7. **Alerts** (permission, question, error): the island opens itself on the alert view, **even if Louis is absent**, and stays open (no auto close) until he answers.
8. **Finished**: the island opens on the `finished` view for 5.2 s, then removes the task and folds away.
9. Several alerts at once: a queue, one at a time, in arrival order.
10. **Focus**: the big character represents the focused task (the last alert, otherwise the first one that is working). The other tasks are the mini-characters. Clicking a mini-character focuses it.

## 4. Island animations

- Opening / enlarging: 520 ms, spring with slight overshoot, equivalent to `cubic-bezier(.32,1.22,.42,1)`. In SwiftUI, start from `.spring(response: 0.5, dampingFraction: 0.72)` and tune by eye against the prototype.
- Closing / shrinking: 340 ms, `cubic-bezier(.45,0,.2,1)`, no overshoot.
- Width, height, radius, position and size of the character, position and size of the mini-characters animate **together** (the « shared element » effect: the mini-characters move from grid to pills to column without disappearing).
- View content: exit 160 ms (opacity 0, blur 8, scale 0,97); entrance 300 ms with a 160 ms delay (after the container has started growing). The header appears with a 300 ms delay.
- Mini-characters: 35 ms stagger per index.
- Pill labels appear 220 ms after the movement starts.
- On entering `expanded`, the character blinks.
- Sounds: `open` on open, `close` on close.

## 5. Views (expanded mode, width 640)

Common structure: 34 pt header (tabs on the left: Overview, Ask, Drop; on the right: “N running” + sound button). Content inset 36 top, 10 left, right, bottom. Cards: radius 20, background `#141518`, 3.5 % white border. In views other than `overview`, the mini-characters move into a **column** on the right (Ø 16, x = width − 31, y = 50 + i × 24) and the card leaves 42 pt on the right.

Card colour veil: radial gradient from the bottom (120 % × 90 %, centre 50 % / 130 %), in the state's colour:
red `rgba(244,80,94,.55)`, green `rgba(52,211,153,.5)`, pink `rgba(244,114,182,.55)`, amber `rgba(245,165,36,.42)`, cyan `rgba(34,211,238,.38)`, indigo `rgba(99,102,241,.5)`, neutral `rgba(255,255,255,.08)`.

| View | Height | Character (x, Ø) | Content | Capture |
|---|---|---|---|---|
| `overview` | 196 | 64, 70 | left card 322 wide: agent line + task ticker; right card: pills | 03 |
| `empty` | 150 | 70, 62 | “Nothing running right now.” + “Ask Claude” button | 16 |
| `approval` | 206 | 62, 56 | agent + “Claude Code wants to run a command”, code block, Deny (N), Always allow, Allow (Y) | 04 |
| `question` | 196 | 62, 56 | agent + question + options as buttons | 05 |
| `error` | 190 | 62, 58 | agent + tool, title, detail in red `#FF8D97`, Retry, Open in n8n | 06 |
| `finished` | 170 | 62, 58 | agent + summary, View terminal, OK | 07 |
| `confused` | 160 | 76, 66 | “Too many slaps at once.” | 08 |
| `upload` | 176 | 140, 62 | dashed zone, “Drop your files here”, labels | 09 |
| `uploading` | 150 | on the bar, Ø 28 | “Sending file” + %, green bar, the character is the bar's cursor | 10 |
| `choose` | 170 | 60, 52 | “file is ready.”, Ask a question about it, Send by email | 11 |
| `mail` | 210 | 56, 46 | To, Subject (+ optional Message), Send, Cancel fields | 12 |
| `prompt` | 156 | 52, 44 | context pill + field + mic + send | 13 |
| `searching` | 156 | 52, 44 | context + shimmering “Claude is reading the page and searching the web…” | 14 |
| `result` | 262 (adapts to content, max 320) | 52, 44 | title, 3 lines of result, buttons | 15 |
| `note` | 136 | 60, 50 | short message (mail sent, copied, opening n8n…), closes itself after 2 s | — |

Character vertical centre: 36 + (height − 46) / 2, except `result` (y = 86).

### Task ticker (overview)
- Position x = 112 in the card, a 96 pt window with a top/bottom gradient mask, 4 lines of 30 pt (previous, current, next, next+1).
- Current line: 14 pt medium, shimmering text (a grey → white → grey gradient sweeping over 2.2 s). Others: 13 pt `#5F646D`, 14 pt icon.
- Every 2.8 s, if the focused task is working: everything moves up 30 pt over 450 ms `cubic-bezier(.3,.9,.3,1)`.
- In the real app, the lines = the session's latest actions (tool + target: “Edit Invoice.swift”, “Bash npm test”) or the n8n nodes.

### Pills (overview)
- 132 × 34, radius 17, agent colour at 13 % background, 32 % border, mini-character Ø 24 centred 17 pt from the left edge, 12 pt label in the agent's colour lightened by 25 %. Two columns, 8 apart, vertically centred in the right card (which starts at x = 342).

### Buttons
- Pill, 12.5 pt medium, white 9 % background (hover 15 %), primary: `#F5F6F8` background, `#0B0C0E` text. Press: scale 0,94. Shortcuts shown in a small bordered pill (Y, N).

## 6. Agent colours (fixed)

| Agent | Colour |
|---|---|
| Korus | `#FF6B5B` |
| SBE Hub | `#2DD4A7` |
| Morning AI Brief (n8n) | `#F7B32B` |
| Publication IG (n8n) | `#A78BFA` |
| louisraille.fr | `#38BDF8` |
| Others | pick in this order: `#F472B6`, `#34D399`, `#FB923C`, `#60A5FA`, `#E879F9`, then loop |

A Claude Code session's name = the working folder's name (`cwd`), with a configurable alias table (e.g. `sbe-hub` → “SBE Hub”). An n8n workflow's name = the workflow's name.

## 7. The character: Mochi

Port the prototype's `Bot` class **as is** into Swift (`Canvas` inside `TimelineView(.animation(paused:))`). Mochi constants (`PISTES.mochi`):

- R = 0,3 × canvas side. Body: superellipse of exponent 2,7, radii rx = 1,14 R, ry = 0,88 R, offset +0,06 R downwards.
- Body gradient: `#FFFAF5` (top right) → `#DDCCBF` (bottom left). State tint: linear gradient from bottom to top, state colour at 92 % × tint fading to transparent at −0,25 ry. Radial shading (20 % black at the edge) and a 55 % white radial highlight top right.
- Cheeks: two ellipses pink `rgba(255,120,150,.5 × blush)`, blush minimum 0,35 for Mochi, they follow the gaze.
- Eyes: ink `#1A1412`, width 0,25 R, height 0,27 R, angular spacing ±0,37 rad, vertical tilt −0,12 rad. Projected on a sphere (yaw, pitch, roll) with a perspective shortcut and clipped by the silhouette: that is what produces the roll-overs (the eyes come out through the top and come back through the bottom).
- Gaze: follows the mouse with a delay (`tanh(dx/260)`, `tanh(dy/200)`, exponential smoothing). Random blinks every 2,2 to 5,4 s, double blink 22 % of the time.
- Mini-characters: same engine, body tinted with the agent's colour, badges reduced.
- Big character's canvas: 230 pt × 2 (Retina); mini: 76 pt × 2.

### States (`STATES`)

| Key | Label | Colour | Tint | Eyes | Badge | Particularity |
|---|---|---|---|---|---|---|
| `idle` | At rest | `#E6E9EE` | 0 | pill | none | |
| `working` | Working | `#3B9EFF` | 0.72 | pill | animated “•••” pill | |
| `thinking` | Thinking | `#8B5CF6` | 0.72 | pill | “•••” | looks top right |
| `searching` | Searching | `#6366F1` | 0.72 | pill | “•••” | eyes sweeping left to right |
| `approval` | Waiting for your go-ahead | `#F5A524` | 0.78 | big | “!” | small hops on a loop |
| `question` | Asking a question | `#22D3EE` | 0.75 | pill | “?” | head tilted 0,17 rad |
| `error` | Error | `#F4505E` | 0.78 | flat | red dot | horizontal shake on entry |
| `finished` | Finished | `#34D399` | 0.35 | happy (arc) | green dot | full roll-over 950 ms + sparks |
| `ratelimit` | Limit reached | `#FB923C` | 0.72 | tired | orange dot | sweat drops |
| `sleeping` | Asleep | `#94A3B8` | 0.32 | closed | none | breathing, rising “z” |
| `dizzy` | Dizzy | `#F472B6` | 0.7 | spirals | none | double roll-over 1.3 s |

Halo behind the character: radial gradient in the state's colour, opacity 0.2 to 0.6 depending on the state (`glow`, `go`), blur 6.

Mapping to real events: see `INTEGRATIONS.md`. `sleeping` = no task for 10 min and the island opened manually; `ratelimit` = usage limit reported by Claude Code.

### Emotes (`EMOTES`) and their real triggers

| Emote | Eyes | Extra | Sound | Trigger |
|---|---|---|---|---|
| Love | hearts `#FF4D6D` | cheeks at full, rising hearts | `love` | mouse still for 1.9 s over the character |
| Surprised | small dots | jump + enlarged eyes | `pop` | when you grab it |
| Proud | stars `#F7B32B` | stars, head tilted back | `proud` | search result displayed |
| Wink | one eye closed | head tilted | `wink` | mail sent, window grabbed |
| Yawn | tired then closed | vertical stretch, “z” | `yawn` | just before moving to `sleeping` |
| Happy | arcs | cheeks | — | after a decision, a file swallowed |
| Annoyed | slanted slits | purple halo `#A855F7` | `annoyed` | a slap |

## 8. Interactions with the character

- **Hover** (expanded): blink, eyes ×1,08, `hover` sound. Still for 1.9 s → Love.
- **Click** in compact/peek → opens. **Click** in expanded → slap: squash (70/130/170 ms), Annoyed for 800 ms, purple halo, `slap` + `annoyed` sounds.
- **3 clicks within 1.7 s** → `dizzy` state for 3.3 s, `confused` view, `dizzy` sound, then back to the previous view and state.
- **Drag** the character (> 7 pt): a floating Ø 54 character follows the cursor (Surprised + `pop`), the notch one disappears. Dropped on another app's window → **attaches** (see INTEGRATIONS §4). Dropped elsewhere → returns to the notch in 420 ms while shrinking.
- **Drag a file** from Finder towards the notch zone (±220 pt around the centre, up to 26 pt below the island) → `upload` view, the character morphs into a “tray” (380 ms morph with bounce) and looks at the file. Green outline and green veil while the file is over it.
- **Drop**: the file falls into the character (360 ms), `gulp` at 330 ms, squash + Happy, back to the round shape at 950 ms, `uploading` view (1.2 to 2.1 s, `tick` every 10 %, matches copying into the app's working folder), `approve` sound, then the `choose` view.
- Several files: same flow, label “3 files”.

## 9. Sounds

Files `assets/sounds/*.wav` (48 kHz stereo), rendered from the prototype's engine with a ×6 gain. **Player default volume: 0,12** to match the prototype's level; the Settings volume slider goes from 0 to 0,2. Played with preloaded `AVAudioPlayer`s (zero latency), several sounds can overlap. Disableable in the island header and in Settings (persisted).

| Event | Sound |
|---|---|
| peek / coucou | `peek` + `greet` |
| open / close | `open` / `close` |
| character hover / small UI click | `hover` / `blip` |
| slap / annoyed / dizzy | `slap` / `annoyed` / `dizzy` |
| working / thinking / searching | `work` / `think` / `search` |
| permission / question / error / limit | `approval` / `question` / `error` / `rate` |
| finished | `finish` |
| decision accepted, upload done | `approve` |
| file swallowed / progress | `gulp` / `tick` |
| sending (prompt, mail) / window attach | `send` / `attach` |
| emotes | `love`, `pop`, `proud`, `wink`, `yawn`, `sleep` |

No sound for silent updates (task ticker, mini-characters changing state except alerts).

## 10. Menu bar and settings

A small item in the menu bar (icon: Mochi's silhouette, monochrome). Menu: Open the notch, Run the demo (⌃⌥⌘D), Settings…, Debug ▸ (force each view, each state, each emote, add dummy tasks), Quit.

Settings window (SwiftUI, simple):
- Anthropic API key (Keychain), model (default see INTEGRATIONS §5).
- n8n: instance URL, API key (Keychain), polling interval, tracked workflows (all by default).
- Claude Code: hooks status (installed or not), Install / Uninstall button, decision wait time (110 s default).
- Sound on/off, volume. Auto close (60 s default). Absence delay (3 min default).
- Launch at login (`SMAppService.mainApp`).
- Project name aliases and colours.

## 11. Milestones

Each milestone ends with build + screenshot + comparison against the references + commit (see CLAUDE.md).

- **M0 Base**: check Xcode (`xcodebuild -version`), XcodeGen, `git init`, `project.yml`, agent app that launches and shows dummy content. Debug menu.
- **M1 Island**: panel, notch detection, 4 modes, §3 rules, click-through, §4 animations, dummy data.
- **M2 Character**: port of `Bot` (Mochi), all states and emotes, mini-characters, halo, badges, particles, hands. Paused while hidden.
- **M3 Views**: all §5 views, ticker, pills, column, shared element, veils. Compare against the 16 captures.
- **M4 Sounds**: wiring §9, sound settings.
- **M5 Claude Code**: hooks, approvals, questions, jump to terminal (INTEGRATIONS §1).
- **M6 n8n**: polling, errors, retry, open (INTEGRATIONS §2).
- **M7 Files**: drag and drop, prompt on file, email via Mail (INTEGRATIONS §3 and §6).
- **M8 Windows + search**: attach, capture, URL, Claude API with web search, result view (INTEGRATIONS §4 and §5).
- **M9 Finish**: demo mode (DEMO.md), full settings, launch at login, no-notch screen, CPU/RAM measurement, final visual comparison pass.

## 12. Acceptance criteria

- Side by side with the prototype, Louis sees no difference in the character, colours, timings or sounds.
- No click lost because of the transparent window.
- A Claude Code session is never blocked by the app (app closed, crashed or slow → the terminal takes over).
- Hidden = 0 % CPU; compact < 3 %; memory < 100 MB.
- The demo (⌃⌥⌘D) films in one take with no interaction.
