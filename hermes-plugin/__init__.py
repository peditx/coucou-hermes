"""Coucou — live session events and notch approvals for Hermes.

Lands in ~/.hermes/plugins/coucou/ (Coucou's Settings writes it) and forwards
every interesting hook to the island through the same relay Claude Code uses
(`coucou-hook` on Windows/Linux, `nb-hook` on macOS). Coucou's own protocol,
timeouts and "never block the agent" rules therefore apply unchanged — this
file adds no new channel.

Nothing here is required for Hermes to work. No relay, no island, no click:
events simply do not arrive, and an approval falls through to Hermes' own
prompt (security.approval.transport_fallback: builtin). Every path is guarded
so a failure in Coucou can never surface as a failure in Hermes.
"""

from __future__ import annotations

import json
import os
import subprocess
import sys
import uuid

# The relay's own budget for an event nobody waits on, plus a little air.
FIRE_AND_FORGET = 2.5
# coucou-hook stops listening after 110 s, nb-hook after 118 s. Asking for more
# than the relay will ever give us would only mean waiting for our own timeout.
DECISION_CAP = 120.0
# Same cap the Rust relay applies, so a big Write stays under the 1 MB payload
# limit the socket server enforces.
MAX_FIELD_LEN = 2000

# Session the island should name a card after; Hermes payloads do not all carry
# one, so the last session we saw is good enough for an approval request.
_last_session = ""


def _relay():
    """The platform relay, resolved at call time so one file serves three OSes."""
    override = os.environ.get("COUCOU_HOOK")
    if override and os.path.isfile(override):
        return override
    home = os.path.expanduser("~")

    def first(paths):
        for path in paths:
            if path and os.path.isfile(path):
                return path
        return None

    if sys.platform == "darwin":
        # App Store build writes the relay beside Claude's own config, the
        # GitHub build into Application Support. Both are constant paths.
        return first([
            os.path.join(home, ".claude", "coucou", "nb-hook"),
            os.path.join(home, "Library", "Application Support",
                         "NotchBuddy", "nb-hook"),
        ])
    if os.name == "nt":
        base = os.environ.get("LOCALAPPDATA") or ""
        return first([os.path.join(base, "Coucou", "bin", "coucou-hook.exe")])
    state = os.environ.get("XDG_STATE_HOME") or os.path.join(home, ".local", "state")
    return first([os.path.join(state, "coucou", "bin", "coucou-hook")])


def _clip(value):
    """Caps every string before it reaches the socket. Mirrors coucou-hook."""
    if isinstance(value, str):
        if len(value) <= MAX_FIELD_LEN:
            return value
        return value[:MAX_FIELD_LEN] + "…"
    if isinstance(value, dict):
        return {k: _clip(v) for k, v in value.items()}
    if isinstance(value, list):
        return [_clip(v) for v in value]
    return value


def _forward(event, payload, timeout):
    """One line to the island. Returns the relay's stdout, empty if anything
    at all went wrong — including "Coucou is not running", which is the normal
    case and must look exactly like silence."""
    relay = _relay()
    if not relay:
        return ""
    body = {k: v for k, v in payload.items() if v not in (None, "")}
    body["hook_event_name"] = event
    try:
        done = subprocess.run(
            [relay, event],
            input=(json.dumps(_clip(body), ensure_ascii=False) + "\n").encode("utf-8"),
            stdout=subprocess.PIPE,
            stderr=subprocess.DEVNULL,
            timeout=timeout,
            check=False,
        )
    except Exception:
        return ""
    return done.stdout.decode("utf-8", "replace").strip()


def _emit(event, payload, timeout=FIRE_AND_FORGET):
    """Observers never raise: a hook that exploded would be logged as a Hermes
    error, and that error would be ours."""
    try:
        _forward(event, payload, timeout)
    except Exception:
        pass


def _text(value):
    """`user_message` is a list on a multimodal turn; the island wants a line."""
    if isinstance(value, str):
        return value
    if isinstance(value, list):
        parts = []
        for part in value:
            if isinstance(part, str):
                parts.append(part)
            elif isinstance(part, dict) and isinstance(part.get("text"), str):
                parts.append(part["text"])
        return "\n".join(parts)
    return "" if value is None else str(value)


def _where(session_id=""):
    """What the island uses to name the card: the project folder."""
    return {"session_id": session_id or _last_session, "cwd": os.getcwd()}


# ── Timeline ──────────────────────────────────────────────────────────────────

def on_session_start(session_id="", **kwargs):
    global _last_session
    if session_id:
        _last_session = session_id
    _emit("SessionStart", _where(session_id))


def pre_llm_call(session_id="", user_message="", **kwargs):
    global _last_session
    if session_id:
        _last_session = session_id
    _emit("UserPromptSubmit", dict(_where(session_id), prompt=_text(user_message)))


def pre_tool_call(tool_name="", args=None, session_id="", **kwargs):
    _emit("PreToolUse", dict(
        _where(session_id),
        tool_name=tool_name,
        tool_input=args if isinstance(args, dict) else {},
    ))


def post_tool_call(tool_name="", args=None, status="", error_type="", error_message="",
                   session_id="", **kwargs):
    failed = bool(error_type) or bool(error_message) or str(status).lower() in (
        "error", "failed", "failure", "block", "blocked")
    payload = dict(_where(session_id), tool_name=tool_name)
    _emit("PostToolUseFailure" if failed else "PostToolUse", payload)


def subagent_start(**kwargs):
    _emit("SubagentStart", _where(kwargs.get("parent_session_id", "")))


def subagent_stop(**kwargs):
    _emit("SubagentStop", _where(kwargs.get("parent_session_id", "")))


def on_session_end(failed=False, session_id="", **kwargs):
    # Fires at every turn finalisation, which is exactly when Claude Code says
    # Stop / StopFailure — so a Hermes turn ends the same way a Claude one does.
    _emit("StopFailure" if failed else "Stop", _where(session_id))


def on_session_finalize(**kwargs):
    _emit("SessionEnd", _where(kwargs.get("session_id", "")))


def on_session_reset(**kwargs):
    _emit("SessionEnd", _where(kwargs.get("session_id", "")))


# ── Approval transport ────────────────────────────────────────────────────────

def _behaviour(stdout):
    """The relay's answer, or None when there is not one.

    None means "Coucou could not answer" — closed, paused, or nobody clicked in
    time. It is the one case we deliberately do not turn into a decision."""
    if not stdout:
        return None
    try:
        decision = json.loads(stdout)["hookSpecificOutput"]["decision"]
        behaviour = decision.get("behavior")
        if behaviour in ("allow", "deny"):
            return behaviour, bool(decision.get("updatedPermissions"))
    except Exception:
        pass
    word = stdout.strip().lower()
    if word in ("allow", "deny"):
        return word, False
    return None


def present(request):
    """Show Hermes' approval in the notch and wait for the click.

    Raising here is not a failure to report: it is how we hand the question
    back to Hermes' own prompt, which is what
    `security.approval.transport_fallback: builtin` asks us to do whenever
    Coucou cannot be the one to answer.
    """
    timeout = max(5.0, min(float(getattr(request, "timeout_seconds", 300) or 300),
                           DECISION_CAP))
    stdout = _forward("PermissionRequest", {
        "session_id": _last_session,
        "cwd": os.getcwd(),
        "tool_name": "Hermes",
        "tool_input": {
            "command": request.command,
            "description": request.description,
        },
    }, timeout)
    answer = _behaviour(stdout)
    if answer is None:
        raise RuntimeError("coucou did not answer")
    behaviour, persisted = answer
    if behaviour == "deny":
        return request.respond("deny")
    # "always" only when the island offered it and Hermes allows this scope;
    # anything we are unsure about stays a one-shot approval.
    if persisted and "always" in request.allowed_choices:
        return request.respond("always")
    return request.respond("once")


def register(ctx):
    ctx.register_hook("on_session_start", on_session_start)
    ctx.register_hook("pre_llm_call", pre_llm_call)
    ctx.register_hook("pre_tool_call", pre_tool_call)
    ctx.register_hook("post_tool_call", post_tool_call)
    ctx.register_hook("subagent_start", subagent_start)
    ctx.register_hook("subagent_stop", subagent_stop)
    ctx.register_hook("on_session_end", on_session_end)
    ctx.register_hook("on_session_finalize", on_session_finalize)
    ctx.register_hook("on_session_reset", on_session_reset)
    # Selected only if security.approval.transport says `coucou`; registration
    # on its own changes nothing.
    ctx.register_approval_transport("coucou", present)


# ── Self-check ────────────────────────────────────────────────────────────────
# `python3 hermes-plugin/__init__.py`. Nothing here runs inside Hermes.

if __name__ == "__main__":
    allow = ('{"hookSpecificOutput":{"hookEventName":"PermissionRequest",'
             '"decision":{"behavior":"allow","updatedPermissions":["x"]}}}')
    deny = ('{"hookSpecificOutput":{"hookEventName":"PermissionRequest",'
            '"decision":{"behavior":"deny","message":"no"}}}')
    assert _behaviour(allow) == ("allow", True)
    assert _behaviour(deny) == ("deny", False)
    assert _behaviour('{"permissionDecision":"allow"}') is None  # not our shape
    assert _behaviour("") is None
    assert _behaviour("not json") is None
    assert _behaviour("allow") == ("allow", False)
    assert _text(["a", {"type": "text", "text": "b"}]) == "a\nb"
    assert _text(None) == ""
    assert len(_clip("x" * 5000)) == MAX_FIELD_LEN + 1
    assert _clip({"a": ["y" * 5000]})["a"][0].endswith("…")

    # A relay answering on stdout is the whole protocol; prove the round trip.
    import stat
    import tempfile

    with tempfile.TemporaryDirectory() as tmp:
        fake = os.path.join(tmp, "relay")
        with open(fake, "w") as fh:
            fh.write("#!/bin/sh\n")
            fh.write("cat >/dev/null\n")
            fh.write("echo '{\"hookSpecificOutput\":{\"hookEventName\":"
                     "\"PermissionRequest\",\"decision\":{\"behavior\":\"allow\"}}}'\n")
        os.chmod(fake, os.stat(fake).st_mode | stat.S_IXUSR)
        os.environ["COUCOU_HOOK"] = fake
        assert _forward("PermissionRequest", {"tool_name": "Hermes"}, 5) != ""
        # Nothing listening must read as silence, never as an error.
        os.environ["COUCOU_HOOK"] = os.path.join(tmp, "missing")
        assert _forward("PermissionRequest", {"tool_name": "Hermes"}, 5) == ""
        del os.environ["COUCOU_HOOK"]
    print("coucou plugin self-check ok")
