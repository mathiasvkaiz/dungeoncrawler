---
name: close-session
description: Close a learning session when the user requests "Close session" or invokes this skill to save the handoff, commit session changes and push the current branch. Mere discussion of closing or a documentation-only wrap-up does not authorize Git delivery.
---

# Close session

Read root `AGENTS.md`, then `doc/LEARNING_PLAN.md`, `doc/PROGRESS.md`, and
the active lesson if present. Paths here are relative to the repository root.

An actual request to close the session (including `$close-session`) authorizes
updating learning docs, committing the relevant session changes including the
user's implementation, and pushing the current branch. Do not ask again for
routine confirmation. Honor narrower instructions such as "do not push".
Discussing or creating this skill is not invocation. Source edits, branch changes,
history rewriting and force pushes are outside this authorization.

1. Inspect the branch, HEAD, status, staged and unstaged diffs, untracked files,
   upstream and push destination. Review all intended session changes, including
   existing local commits that would be pushed. Preserve user work and existing
   staging. If scope or destination is ambiguous, finish the unambiguous handoff
   work and clarify before committing/pushing the affected work. Do not include
   unrelated changes, generated artifacts or suspected secrets silently.
2. Reconcile the latest review and tests with the final code. Run only needed,
   relevant checks; do not repeat unchanged checks without reason. Record failures
   and gaps. A partial or failing exercise can be saved as an explicitly incomplete
   checkpoint; do not fix it automatically or describe it as completed.
3. Update `doc/PROGRESS.md` with the active lesson/substep, prepared vs implemented
   vs reviewed/discussed state, evidence (including who ran each check), unresolved
   questions, pending exercise and one exact next action. Update the plan or lesson
   only for lasting decisions or necessary corrections. Keep the handoff concise.
   Record HEAD as the pre-close commit, not as the hash of the forthcoming commit.
   Do not claim a push succeeded before it has happened.
4. Inspect the final diff, run `git diff --check`, and stage explicit session paths.
   Inspect the entire staged diff before committing so previously staged work is
   not included accidentally. Commit with a concise message matching the actual
   changes (identify an incomplete checkpoint when applicable). If there is no
   change, do not manufacture a commit. Existing unpushed session commits may
   still need delivery.
5. Push only the current branch to its verified destination, normally its upstream.
   If no upstream exists, use a clearly established remote and matching branch;
   ask if the destination is unclear. Follow environment approval requirements.
   On rejection, authentication failure or divergence, preserve the local commit
   and report the exact remaining action. Do not automatically merge, rebase,
   reset, bypass hooks or force-push. Retry only an understood transient failure or
   a required sandbox escalation; stop on repeated failure.
6. Verify the resulting status and push outcome. Report the saved checkpoint,
   commit hash, branch/destination, checks, any remaining changes or delivery
   failure, and the next action for a fresh session. Do not begin another lesson.
