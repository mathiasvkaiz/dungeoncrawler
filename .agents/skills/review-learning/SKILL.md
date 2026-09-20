---
name: review-learning
description: Review the user's hands-on Bevy/ECS exercise when asked "Review my work" or to investigate their lesson implementation. Explain findings and record evidence without implementing fixes or advancing the curriculum.
---

# Review learning

Read root `AGENTS.md`, then `doc/LEARNING_PLAN.md`, `doc/PROGRESS.md`, and
the active lesson if present. Paths here are relative to the repository root.

1. Inspect branch, latest commit, Cargo versions, staged/unstaged/untracked changes
   and the actual exercise files. Include already committed work since the recorded
   checkpoint when relevant; an empty working diff does not mean there is no work
   to review. Establish the lesson scope from the checkpoint and user's request.
2. Compare behavior and design to that scope, including correctness, ECS ownership,
   scheduling, API usage and meaningful failure modes. Accept sound alternatives
   to lesson code. Do not turn a small exercise into a production redesign.
3. Run focused, relevant checks when practical. Distinguish source inspection,
   checks actually run, tests reported by the user, and visual validation. Reuse
   evidence for unchanged code; explain missing evidence and remaining limits.
   Use checks that do not rewrite exercise source; leave fixes to the learner.
4. Present actionable findings first, with file/line references and consequences.
   Explain corrections or provide exact replacement blocks for the user to apply.
   If there are no findings, say so and state verification limits.
5. Update `doc/PROGRESS.md`: what the user implemented, what was reviewed, checks
   and results, discussed concepts, unresolved issues and the next small action.
   Keep implementation, verification and learner understanding separate. Do not
   infer lesson completion from code or tests alone; use the user's feedback too.

Do not edit application/exercise source or dependencies, auto-format the user's
files, commit, push, or prepare the next lesson. A review may be partial and may
lead to freestyle discussion or another user edit before a later review.
