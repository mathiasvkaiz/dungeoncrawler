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
5. Once verified, say **"Continue learning"** to resume the checkpoint. Record live
   verification in `doc/PROGRESS.md` when edits are authorized. Do not restart at
   lesson 01 if the checkpoint has advanced.

If the hook is missing from `/hooks`, check project trust and whether the client
loaded this repository's configuration. Desktop-client activation has not been
verified; the steps above use the CLI. New or changed hook definitions need review
again. Do not bypass trust to perform this test.

Verification: the injected marker and checkpoint were observed in the session on
2026-09-20. This verifies loading in that session, not every client or lifecycle
event. Direct command checks from the repository root and `src/` also passed.

## Everyday learning flow

- **"Continue learning"**: prepare or resume the next small step.
- Work through the lesson and test your implementation. Ask freestyle questions
  whenever useful; they do not require a workflow command.
- **"Review my work"**: inspect your implementation and discuss findings.
- **"Close session"**: update the handoff, commit relevant session code/docs and
  push the current branch. A partial lesson stays partial in the next session.

The corresponding skills are in `.agents/skills/`; root `AGENTS.md` routes these
requests and holds shared teaching rules. `doc/PROGRESS.md` is the current handoff,
and `doc/LEARNING_PLAN.md` holds the durable roadmap. The hook only loads context;
it never invokes a skill, commits, pushes or marks a lesson complete. Session
renaming is not a required step. "Wrap up" alone updates documentation only.

Source: [Official OpenAI hook documentation](https://developers.openai.com/es-419/docs/hooks).
