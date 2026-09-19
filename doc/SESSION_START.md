# Learning checkpoint hook

The repository's `.codex/hooks.json` runs `.codex/hooks/session_start.py` on
startup, resume, clear and compaction. It reads the current checkout's Git state,
learning plan, progress and Cargo manifest into the assistant's context. It does
not write files, call the network, change branches or start lessons automatically.
The assistant must still read AGENTS.md and any active lesson, and inspect the
locked Cargo versions before teaching.

Prerequisites: Git, `/usr/bin/python3`, and a Codex client supporting SessionStart
hooks. The installed CLI was verified as 0.155.1 with `hooks` enabled.

## Activate and test in a fresh session

1. Open Codex from this repository. In the CLI, run `/hooks`, review the
   repository's **Loading learning checkpoint** hook and mark it trusted. The
   project must also be trusted for its local configuration to load.
2. Start a fresh session in this repository after trusting the hook.
3. Send: **"Verify the learning checkpoint hook loaded. Tell me the actual
   branch, current checkpoint and next small action. Do not edit files yet."**
4. A successful check identifies the injected **Learning checkpoint hook loaded
   (read-only).** marker, reports the actual branch and checkpoint, and does not
   confuse prepared code or a commit with a completed lesson. Reading the files
   manually is useful fallback behavior, but is not proof that the hook ran.
5. Once verified, ask to resume the learning path. Record that live verification
   in `doc/PROGRESS.md` and proceed to lesson 01.

If the hook is missing from `/hooks`, check project trust and whether the client
loaded this repository's configuration. Desktop-client activation has not been
verified; the steps above use the CLI. New or changed hook definitions need review
again. Do not bypass trust to perform this test.

Verification so far: the exact configured command was run from the repository
root and `src/`. Both produced valid SessionStart JSON with the actual branch and
checkpoint. This verifies the command and output, not automatic client execution.

Source: [Official OpenAI hook documentation](https://developers.openai.com/es-419/docs/hooks).
