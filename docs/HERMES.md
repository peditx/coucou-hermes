# Coucou × Hermes Agent — plan

Golden rule: **check the official docs when implementing**. Everything below was read
from the Hermes docs at plan time; re-read them before writing code. Sources:
- Hermes Agent: https://github.com/NousResearch/hermes-agent
- Architecture: https://hermes-agent.nousresearch.com/docs/developer-guide/architecture
- Security / folder access: https://hermes-agent.nousresearch.com/docs/user-guide/security
- MCP: https://hermes-agent.nousresearch.com/docs/user-guide/features/mcp
- API server: https://hermes-agent.nousresearch.com/docs/user-guide/features/api-server

> **Status — shipped:**
> - Hermes URL + key stored on all three platforms (Keychain / Credential
>   Manager / Secret Service), with a *Hermes Agent* section in Settings.
> - The tray/status menu has a **Hermes Chat** item opening a maximised
>   full-screen chat that streams over `POST /v1/chat/completions`.
> - **Mochi's island chat can be served by Hermes.** Settings → *Island chat*
>   holds two explicit choices — `Engine` (Claude / Hermes) and `If it fails`
>   (None / Claude / Hermes). Nothing switches on its own: the primary runs,
>   and only the user-picked fallback runs after it. Same prompt, same
>   multi-turn history, same file and window context as the Claude path —
>   files are inlined, because a remote agent cannot open a path on this
>   machine.
>   Code: `linux/src-tauri/src/hermes.rs`, `windows/src-tauri/src/hermes.rs`
>   (identical), `NotchBuddy/Sources/App/HermesService.swift` +
>   `ClaudeService.swift`.
> - **Live Hermes sessions and approvals in the island.** A plugin at
>   `~/.hermes/plugins/coucou/` forwards Hermes hook events to the same relay
>   Claude Code already uses, and registers `coucou` as an approval transport,
>   so Hermes permission requests arrive with Allow / Deny in the island.
>   Installed from Settings → *Hermes hooks* with the same backup → diff →
>   explicit click as the Claude hooks. See §6.
>   Code: `hermes-plugin/`, `linux/src-tauri/src/hermes_hooks.rs`,
>   `windows/src-tauri/src/hermes_hooks.rs` (identical),
>   `NotchBuddy/Sources/App/HermesHooks.swift`.
>
> **Status — still plan:** the local-file bridge (§2–§3). Coucou does *not* yet
> expose an MCP endpoint, so a remote Hermes still cannot read anything on this
> machine. Do not describe that as a shipped feature anywhere else.

---

## 1. What Hermes is

Open-source (MIT) agent from Nous Research. Install with
`curl -fsSL https://hermes-agent.nousresearch.com/install.sh | bash`, then `hermes`.
Config and data live in `~/.hermes` (`%LOCALAPPDATA%\hermes` on native Windows).

### The four commands, in order

Settings → *Hermes Agent* shows this block; it is here because every line of it
comes from the official API-server docs, and those are the ones to re-read
before changing it.

Steps 2 to 4 are also exactly what **Install plugin** does in one click — dated
backup, diff, then write — plus the URL and the key in the system keyring.
Those four steps are kept here because they are what you would run by hand, and
because they are the copy to check the docs against.

```bash
# 1  install
curl -fsSL https://hermes-agent.nousresearch.com/install.sh | bash

# 2  copy for Hermes — its key, in ~/.hermes/.env
API_SERVER_ENABLED=true
API_SERVER_KEY=<any secret you pick>

# 3  start it — http://127.0.0.1:8642 unless API_SERVER_HOST / API_SERVER_PORT say otherwise
hermes gateway
# → [API Server] listening on http://127.0.0.1:8642

# 4  copy and back up ~/.hermes/config.yaml before anything edits it
cp ~/.hermes/config.yaml ~/.hermes/config.yaml.bak-$(date +%F-%H%M%S)
```

Then the URL and that same key go into Coucou's *Hermes Agent* section — which
is the whole connection: Coucou is a client of this server and nothing else.
Install puts them there itself, so an install with nothing typed is already a
working connection.

The order is load-bearing twice. `API_SERVER_ENABLED` defaults to `false`, so
step 2 is not optional, and a key has no default — it is required even on the
loopback bind. Step 4 runs before anything writes: the installer takes its own
dated backup anyway, but one `cp` covers the edits you make by hand, which is
where a config file actually gets lost.

Three integration protocols, and only one of them fits Coucou:

| Protocol | Transport | Fits Coucou? |
|---|---|---|
| CLI / TUI | terminal | no |
| Gateway | Telegram, Discord, Slack, WhatsApp, Signal, email | no — but useful for *setup* |
| ACP adapter | stdio + JSON-RPC (VS Code, Zed, JetBrains) | editor only |
| **API Server** | **HTTP + SSE** (OpenAI-compatible) | **yes** |
| **MCP** | **remote HTTP client** | **yes — for local files** |

The API Server is what a full-screen chat window talks to; MCP is how a *remote*
Hermes reaches files that live on the *user's* machine.

---

## 2. The file to add on the Hermes side

One file: **`~/.hermes/config.yaml`**, under the **`mcp_servers`** key.

```yaml
mcp_servers:
  coucou:
    url: "https://<your-endpoint>/mcp"     # set by the user in Coucou's menu
    headers:
      Authorization: "Bearer ${COUCOU_MCP_TOKEN}"
    enabled: true
    tools:
      include: ["read_file", "write_file", "list_dir"]   # Coucou decides the names
    timeout: 30
    connect_timeout: 10
```

Verified keys: `url`, `headers`, `enabled`, `timeout`, `connect_timeout`, `lazy`,
`idle_timeout_seconds`, `max_lifetime_seconds`, `transport` (`sse` when the endpoint
speaks SSE only), `tools.include` / `tools.exclude`, `supports_parallel_tool_calls`,
`client_cert` / `client_key`, `identity_header`, `sampling`, `elicitation`,
`auth: oauth`, `oauth.client_id` / `oauth.client_secret`.

`${VAR}` substitution happens at runtime and also inside `transport.url` and
`headers` — so the token goes in `~/.hermes/.env`, never inline in the YAML.

After editing: reload with `/reload-mcp`.

### The Telegram shortcut

The user does not have to edit YAML by hand. The gateway serves slash commands, and
the agent itself has a `terminal` tool, so from Telegram (or any other connected
platform) it can be told to add the server itself:

```
hermes mcp add coucou
hermes mcp              # interactive picker
hermes mcp install <name> [--preset <preset>]   # from the catalog
hermes mcp configure <name>
```

So the whole server-side setup reduces to: **paste the MCP URL + bearer to Hermes in
Telegram, then `/reload-mcp`.** That is the entire "integration" from the user's
point of view.

*(Unverified: the exact flags `hermes mcp add` accepts beyond `--preset`. Prefer the
YAML snippet above until the CLI reference is re-read.)*

---

## 3. Where approval lives — on Coucou's side, not Hermes's

Hermes's own folder controls are **server-side and not a local sandbox**:

- `HERMES_WRITE_SAFE_ROOT` limits *writes* only, and only for the process that has it.
- The always-on denylist (`~/.ssh/`, `~/.aws/`, `~/.kube/`, `/etc/sudoers`,
  `~/.netrc`, plus Hermes's own `.env` / `vault/` / token caches) applies to
  `write_file` and `patch` — **reads are mostly unguarded**.
- The docs state plainly that the `terminal` tool runs as the same OS user and can
  still `cat` a denied path. It is defence in depth, not a boundary.

None of that protects files on the user's machine. So the gate must be enforced
where the bytes are: **inside Coucou's MCP endpoint.**

```
remote Hermes ──MCP──▶ Coucou (local) ──▶ "read ~/project/x.ts?" ──allow──▶ file
                                              ▲
                                   explicit click, here
```

Rule: **no read or write leaves Coucou without an explicit click**, reusing the
existing `PermissionRequest` allow/deny pattern. Consequence worth stating plainly —
even if the remote Hermes instance is compromised, it still cannot read a local file
until the user clicks Allow.

*(Unverified: whether Hermes gates a plain MCP tool call per-call. Do not rely on it;
our side gates anyway.)*

---

## 4. The network reality

A remote server cannot reach local files unless something dials out or listens in.
There is no third option.

| Option | Direction | Cost |
|---|---|---|
| **A. MCP over HTTP** (above) | server → Coucou | Coucou's endpoint must be reachable: public TLS endpoint, or loopback + reverse tunnel |
| **B. Register/WS reverse channel** | Coucou → server | outbound-only, no open port — but see caveat |
| **C. Snapshot sync** | Coucou → server only | no inbound at all; the agent works on a copy, not the live file |

- **A** is standard and works today with zero changes to Hermes.
- **B** is modelled on Hermes's browser-control flow: `POST /v1/browser-control/register`
  returns a single-use ticket with a 30 s TTL, then `GET /v1/browser-control/ws` with
  subprotocols `hermes-browser-control-v1` + `hermes-browser-control-ticket.<ticket>`
  (never in the query string), then `browser.controller.command` / `.result` frames.
  The transport is generic but the capability vocabulary is browser-specific and
  filtered through a fixed allowlist in `/v1/capabilities`, so this needs an upstream
  PR before it can carry `file.*`.
- **C** is the fallback: no new exposure, no dependency on Hermes, but not live access.

**Recommendation: ship C first, then A.** Both use code Coucou already has
(`reqwest` + `PermissionRequest` + `KNOWN_KEYS`). Treat B as a separate upstream
contribution.

### Auth on the endpoint

Hermes's own API server binds `127.0.0.1:8642` by default
(`API_SERVER_HOST` / `API_SERVER_PORT`, `API_SERVER_ENABLED=true`) and requires
`Authorization: Bearer <API_SERVER_KEY>` on every request, because it exposes terminal
access. The mirror rule applies to Coucou's endpoint: bearer required, TLS required,
and the token lives in the Secret Service on the Coucou side (`KNOWN_KEYS`:
`hermes-url`, `hermes-token`) and in `~/.hermes/.env` on the Hermes side. Never in
`settings.json`, never in git.

---

## 5. What Coucou has to gain

1. **Menu entries** — endpoint URL + bearer in `KNOWN_KEYS` (three lines in
   `secrets.rs`), shown as "present / absent" only.
2. **Full-screen chat window** — reuse the existing hidden second window
   (`create_settings_window`) plus a message list and composer. Stream over SSE.
   **All network I/O stays in Rust** (`reqwest` + `rustls` are already dependencies)
   so the token never reaches the WebView and `connect-src` stays closed.
3. **Consent view** — allow/deny for each file read/write, same pattern as
   `PermissionRequest`.
4. **Trusted folder list** — explicit, per-folder, stored in `settings.json` (paths
   are not secrets). Anything outside the list is refused by Coucou regardless of
   what the remote side asks for.

Watch-outs: `DECISION_TIMEOUT` is 108 s while Hermes's approval timeout defaults to
300 s and fails closed — align them, or Coucou gives up first. And a remote endpoint
is slow and flaky by nature, so the chat view must degrade to a visible
"server unreachable" state instead of hanging.

---

## 6. Hermes hooks — shipped

Hermes loads third-party plugins only when the name appears in `plugins.enabled`,
and an approval transport only when `security.approval.transport` names it. Both
are consent steps, so **both are written by the user clicking Install in
Settings**, never by a background task. The installer touches exactly three
keys and nothing else:

```yaml
plugins:
  enabled:
    - coucou

security:
  approval:
    transport: coucou
    transport_fallback: builtin   # default is `deny`
```

`transport_fallback: builtin` is not decoration. Hermes fails **closed**: a
transport that errors, times out, is not registered, or answers with a stale
request denies the approval outright, and `deny` is the default. The one-line
opt-in is what makes a Coucou that isn't running fall back to Hermes' own
prompt instead of silently denying — so Claude Code is never blocked, and
Hermes is never blocked either.

### What the plugin does

- `~/.hermes/plugins/coucou/{plugin.yaml, __init__.py}` — one file that maps
  Hermes hook events onto the Claude vocabulary the island already understands
  (`SessionStart`, `UserPromptSubmit`, `PreToolUse`, `PostToolUse`, `Stop`,
  `SessionEnd`, `SubagentStart`, `SubagentStop`), so `hooks.ts` and the island
  views need no Hermes-specific branch and no second integration card.
- Every event goes out through the **existing relay** — `coucou-hook` on
  Windows/Linux, `nb-hook` on macOS — as a single `subprocess.run`. The plugin
  reuses the relay's timeouts, payload caps and "never block Claude Code"
  rule instead of re-implementing them. If nothing is listening it writes
  nothing and says nothing.
- `present(request)` forwards the command and description as a
  `PermissionRequest` and waits for your click. No answer, an unreadable
  answer, or a refusal to answer raises — which `transport_fallback: builtin`
  turns into Hermes' own prompt. There is no path where the plugin approves by
  itself.

### The installer

`linux/src-tauri/src/hermes_hooks.rs` and `windows/src-tauri/src/hermes_hooks.rs`
are identical, and `NotchBuddy/Sources/App/HermesHooks.swift` is the same logic
in Swift for `config.yaml` and the plugin. All three follow the Claude hooks
discipline: read → dated backup → merge only our keys → unified diff → write
only after an explicit click, and only while the files still match the bytes
the diff was computed from — for Linux and Windows that fingerprint now covers
`config.yaml` **and** `.env` together.

On Linux and Windows Install also writes `~/.hermes/.env`: `API_SERVER_ENABLED`
set to `true`, and an `API_SERVER_KEY` — theirs if they have one already,
otherwise 32 random bytes generated at write time and stored in the keyring.
The value never appears in the diff (it is masked) or in a log. Uninstall
leaves `.env` alone: `API_SERVER_ENABLED` is Hermes's own setting and a key the
user wrote is theirs. The keyring is only ever *filled* — a URL or key the user
typed into it is not overwritten.

The YAML is edited **line by line and never round-tripped through a parser**:
`config.yaml` carries comments, anchors and ordering that belong to the user
and to Hermes, and a parse/serialise cycle would rewrite all of it. Uninstall
removes `transport`, `transport_fallback`, our entry in `plugins.enabled`, and
any mapping that our removal left empty — and nothing else. If you changed
`transport` by hand, your value is left alone.

The macOS app ships the plugin in its bundle (`hermes-plugin/`, a folder
resource in `project.yml`) so install needs no network. The App Store build
cannot reach `~/.hermes` from inside its sandbox, so the install buttons are
absent there.

`Start API server` runs `hermes gateway` detached (own session, no console
window) with stdout and stderr appended to
`~/.local/state/coucou/hermes-gateway.log`, after probing the saved URL so an
up server is reported instead of started twice. It refuses a URL that is not
this machine's own — that one is started where it runs.

*(Re-read the official plugin docs before changing any of this — the config
keys and the fail-closed default are theirs, not ours.)*
