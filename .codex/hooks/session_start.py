"""Read-only learning context for Codex SessionStart; no network or file writes."""

import json
from pathlib import Path
import subprocess


ROOT = Path(__file__).resolve().parents[2]


def git(*args):
    try:
        result = subprocess.run(
            ["git", "--no-optional-locks", "-C", str(ROOT), *args],
            capture_output=True, text=True, timeout=2, check=False,
        )
        return result.stdout.strip() if result.returncode == 0 else "Unavailable"
    except (OSError, subprocess.TimeoutExpired):
        return "Unavailable"


def read(relative_path):
    try:
        return (ROOT / relative_path).read_text(encoding="utf-8")
    except OSError:
        return "Missing or unreadable; inspect before continuing."


context = [
    "Learning checkpoint hook loaded (read-only).",
    "Repository: " + str(ROOT),
    "Actual branch: " + git("rev-parse", "--abbrev-ref", "HEAD"),
    "Latest commit: " + git("log", "-1", "--oneline"),
    "Working tree:\n" + git("status", "--short", "--branch"),
    "Use this checkout's checkpoint, not conversation memory. Reconcile stale "
    "checkpoint claims with the Git evidence above. Do not switch branches. "
    "Read AGENTS.md and any active lesson before teaching. Follow the user's "
    "current request; loading this context does not authorize source edits or "
    "automatically start a lesson. Git state does not prove lesson completion.",
]
for path in ("doc/LEARNING_PLAN.md", "doc/PROGRESS.md", "Cargo.toml"):
    context.append("--- " + path + " ---\n" + read(path))

print(json.dumps({
    "hookSpecificOutput": {
        "hookEventName": "SessionStart",
        "additionalContext": "\n\n".join(context),
    }
}))
