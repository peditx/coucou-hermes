# Notch Buddy — integrations

Golden rule: **check the official docs when implementing**. The formats below are the plan, not a guarantee. Sources to re-read:
- Claude Code hooks: https://code.claude.com/docs/en/hooks
- Claude API (Messages, web search tool, models): https://docs.claude.com/en/api/overview
- n8n public API: `{instance URL}/api/v1/docs` (playground of Louis's instance)

---

## 1. Claude Code (Louis's sessions)

### Architecture
```
claude (terminal, VS Code, Claude app)
  └─ "command" hook ─► nb-hook (small Swift executable, shipped with the app)
                         └─ Unix socket ─► Notch Buddy.app
                         ◄─ decision (for PermissionRequest)
```
- `nb-hook`: a separate target in the project, copied to `~/Library/Application Support/NotchBuddy/bin/nb-hook` on first launch.
- Socket: `~/Library/Application Support/NotchBuddy/nb.sock` (GitHub version) or `~/Library/Containers/fr.louisraille.Coucou/Data/nb.sock` (App Store version). Directory in 0700, socket in 0600. Same-user connections only (verified with `getpeereid`). 1 MB and 5 s maximum per message, 32 simultaneous connections.
- `nb-hook <Event>` reads the hook JSON on stdin, adds the terminal context (`TERM_PROGRAM`, `ITERM_SESSION_ID`, `TERM_SESSION_ID`, `__CFBundleIdentifier`, the tty found by walking up the parent processes, `cwd`), and sends it to the app.
- **If the app does not answer within 300 ms, `nb-hook` exits with code 0 without writing anything**: Claude Code carries on as normal. Never blocks.

### Events to wire up and character state
| Hook | Effect in the app |
|---|---|
| `SessionStart` | creates the task (name = folder), state `idle` |
| `UserPromptSubmit` | state `thinking`, ticker line = start of the prompt |
| `PreToolUse` | state `working`, line = tool + target (« Edit Invoice.swift », « Bash npm test ») |
| `PostToolUse` / `PostToolUseFailure` | updates the line; a failure stays `working` |
| `PermissionRequest` | `approval` alert (see below) |
| `Notification` | depending on the type: waiting for input → `question` if a question is being asked, otherwise nothing; usage limit → `ratelimit` |
| `Stop` | state `finished` → `finished` view for 5.2 s, summary = last useful sentence of the response if available |
| `StopFailure` (if present in the docs) | `error` alert |
| `SubagentStart` / `SubagentStop` | show « + sub-agent » in the ticker |
| `SessionEnd` | removes the task |

Check the exact list of events and their fields in the docs.

### Approving from the notch
- On `PermissionRequest`, `nb-hook` **waits** for the app's decision (110 s default, configurable) then writes the hook's decision JSON to stdout (per the current docs: `hookSpecificOutput` with `decision.behavior` = `allow` or `deny`). Hook timeout in settings.json: decision + 10 s.
- No answer before the deadline, or app closed → no output at all, the terminal shows its usual prompt. If Louis answers in the terminal, the app drops the alert on the next session event.
- A bug has been reported where `deny` was ignored on `PermissionRequest` (GitHub issue anthropics/claude-code #19298). **Test both allow and deny**; if deny does not work, move the decision to `PreToolUse` (`permissionDecision`) for the tools involved.
- « Always allow »: if the docs let us return a persistent permission rule, use it. Otherwise the app keeps its own list (project + tool + command pattern) and answers `allow` automatically afterwards. The list is visible and deletable in Settings.
- Y / N shortcuts while the `approval` view is open.

### Answering questions
- If Claude uses the question tool (`AskUserQuestion`), intercept it in `PreToolUse` and show the options in the `question` view.
- Check in the docs whether a hook can supply the answer. If so: clicking an option = the answer. **If not**: the view shows the question and a « Answer in the terminal » button that jumps to the session. Do not hack simulated keyboard typing.

### Jumping to the terminal
| Captured context | Action |
|---|---|
| `TERM_PROGRAM=Apple_Terminal` + tty | Terminal AppleScript: select the tab whose `tty` matches, activate |
| `TERM_PROGRAM=iTerm.app` + `ITERM_SESSION_ID` | iTerm AppleScript: select the session, activate |
| `TERM_PROGRAM=vscode` | open the `cwd` folder in VS Code or Cursor (depending on `__CFBundleIdentifier`) |
| Ghostty, Warp, other | activate the app |
| nothing (Claude app) | activate the Claude app |
Requests Automation permission the first time (normal).

### Installing the hooks: mandatory procedure
1. Read `~/.claude/settings.json` (create it if it does not exist).
2. Copy it to `~/.claude/settings.json.bak-AAAAMMJJ-HHMM`.
3. **Merge**: add the Notch Buddy hooks without touching the existing ones. The `nb-hook` path is quoted (it contains a space).
4. Show the diff to Louis, wait for his OK, write.
5. An « Uninstall the hooks » button in Settings that removes only the Notch Buddy entries.

---

## 2. n8n (Louis's workflows)

- Settings: instance URL (probably `https://n8nlouis.dcsys.tech`, **to confirm with Louis**) and n8n API key (Keychain). The key is created in n8n: Settings → n8n API.
- The Mac polls n8n, not the other way round: **poll** the public API every 5 s:
  - workflow names: `GET /api/v1/workflows` (10 min cache);
  - recent executions: `GET /api/v1/executions` with status filters and `limit`.
- Mapping:
  - running execution → `working` task (if the API exposes running executions; otherwise n8n only shows up on errors and successes, which is acceptable);
  - new failed execution → `error` alert, detail = failing node + message (`GET /api/v1/executions/{id}?includeData=true`);
  - success → mini-character `finished` for 3 s in compact, **without** opening the island (otherwise too much noise), unless a setting says otherwise.
- Buttons:
  - « Retry » → the public API's retry endpoint (check that it exists and its path in the instance playground). If it does not: open the execution in n8n.
  - « Open in n8n » → open `{URL}/workflow/{workflowId}/executions/{executionId}` in the default browser.
- « Tracked workflows » setting: all by default, a checklist.

---

## 3. Dropped files

- Native drag and drop onto the panel (types `fileURL`). Copy the files into `~/Library/Application Support/NotchBuddy/inbox/` (that is the `uploading` phase).
- `choose` view:
  - **Ask a question about it** → `prompt` view with a file pill. Sent to the Claude API (§5): PDF as a `document` block, images as an `image` block, text and code (≤ 200 KB) as text. Other types: message « I can't read this format, but I can email it. »
  - **Send by email** → `mail` view (§6).
- Clean the inbox after 7 days.

---

## 4. Attaching the character to a window

1. On drop, find the window under the point: `CGWindowListCopyWindowInfo(.optionOnScreenOnly)`, first layer-0 window that is not ours and contains the point. Get the app, title, frame.
2. Show the **halo**: a transparent, non-clickable panel laid over the window frame. A 3 pt conic rainbow border that rotates in 3 s (`#FF6B5B → #F7B32B → #2DD4A7 → #38BDF8 → #A78BFA → #F472B6`), a multicolour veil in multiply mode that breathes (see `.attach` in the prototype), 600 ms fade-in. `attach` sound, Wink emote.
3. Context sent to Claude:
   - capture the window with ScreenCaptureKit (`SCScreenshotManager`), resized to a max width of 1568 px;
   - if it is Safari, Chrome, Arc or Brave: the active tab's URL and title via AppleScript.
4. `prompt` view with the « Safari, escale.fr » pill (app + domain), focus on the field.
5. The halo stays during `searching`, disappears when the result shows or when the island closes.

Permissions: Screen Recording (capture) and Automation (browser). If denied: carry on without the capture or without the URL, and say so in one line in the view.

---

## 5. Claude API (search)

- `POST https://api.anthropic.com/v1/messages`, headers `x-api-key`, `anthropic-version`, `content-type: application/json` (versions to check in the docs).
- Default model: `claude-sonnet-5`, configurable in Settings. Check the list of available models in the docs.
- Server-side web search tool on the API: the up-to-date type id is in the docs (as of writing, `web_search_20250305`); `max_uses` 5.
- System prompt (English, for the notch UI): answer briefly, for display in the notch, in strict JSON format:
  ```json
  { "title": "…", "items": [ { "label": "…", "detail": "…", "url": "…" } ], "note": "…" }
  ```
  Maximum 3 items. If the JSON is invalid: show the raw text (max 3 lines) in the `result` view.
- User message content: capture (image block) + « URL: … / Title: … / Request: … », or file (§3) + request, or request alone (Ask tab).
- During the call: state `searching`, `searching` view, shimmering text. Response: state `finished`, `result` view, Proud emote, `finish` sound.
- Result buttons: « Open » (first link, only if it is http or https; otherwise the button is disabled), « Copy » (text), « Close ».
- Network error or invalid key: state `error`, `note` view with the reason in one sentence and « Open Settings to check the key ».
- Microphone (field button): `SFSpeechRecognizer` dictation in `en-US`, on-device if possible. Optional (M9). If the permission is denied, hide the button.

---

## 6. Mail (the Mac's Mail app)

- `mail` view: To (required, address validation), Subject (pre-filled: the file name), Message (optional, one line).
- Sending only on clicking « Send », via AppleScript (`NSAppleScript`) on Mail:
  ```applescript
  tell application "Mail"
    set m to make new outgoing message with properties {subject:"…", content:"…", visible:false}
    tell m
      make new to recipient at end of to recipients with properties {address:"…"}
      make new attachment with properties {file name:(POSIX file "…")} at after the last paragraph of content
    end tell
    delay 1
    send m
  end tell
  ```
  The `delay` gives Mail time to pick the attachment up (known Mail behaviour). `Info.plist`: `NSAppleEventsUsageDescription`.
- Success: `note` view « Mail sent to … », Wink emote, `send` sound. Failure: `error` state with the reason.

---

## 7. macOS permissions requested (recap for Louis)

| Permission | Why | When |
|---|---|---|
| Automation → Mail | send emails | first send |
| Automation → Terminal / iTerm / browser | jump to the right tab, read the URL | first use |
| Screen Recording | capture the attached window | first attach |
| Microphone + Speech Recognition (optional) | dictation | first click on the mic |

No Accessibility permission needed.
