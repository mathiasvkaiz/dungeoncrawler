---
name: continue-learning
description: Resume this repository's Bevy/ECS learning path when the user says "Continue learning" or asks to resume the curriculum. Prepare or continue one small lesson; ordinary conceptual questions do not trigger lesson advancement.
---

# Continue learning

Read root `AGENTS.md`, then `doc/LEARNING_PLAN.md`, `doc/PROGRESS.md`, and
the active lesson if one exists. Paths here are relative to the repository root.
Follow their teaching contract; this skill does not authorize exercise source edits.

1. Inspect the actual branch, latest commit, staged/unstaged/untracked changes,
   and Cargo.toml/Cargo.lock versions. Reconcile stale checkpoint claims without
   treating implemented code or passing tests as proof of learning completion.
2. Briefly state the checkpoint and the next small action. Resume unfinished
   discussion or exercises before preparing another lesson. If the checkpoint
   lacks essential evidence, inspect the files and ask only what remains unknown.
3. When the learner is ready for the next increment, prepare one Markdown lesson
   using `doc/lessons/TEMPLATE.md`. Target `examples/learning/main.rs` and modules
   alongside it; preserve the showcase in `src/`. Give exact paths and complete
   copyable files or replacement blocks, intent, ECS ownership, relevant Rust
   explanations, tradeoffs, failure modes, and a small experiment/check.
   Follow the template's code-first order and visible Bevy/Rust notes. Explain
   who calls new hooks/systems, when they run and how parameters are supplied;
   distinguish registration from execution and explain relevant Rust syntax
   using the lesson's code. Put function-specific explanations in concise doc/
   inline comments within the copyable code; keep the later concepts section
   focused on connections without repeating those comments.
4. Match APIs to the installed Bevy version. Validate copyable code in isolation
   when practical, never by filling in the user's exercise files. Label exactly
   what was checked and what remains unverified.
5. Update `doc/PROGRESS.md` with lesson/substep, preparation status, verification,
   pending user exercise and next action. Stop for the user to implement or discuss.

Do not commit, push, switch branches, or prepare subsequent lessons as part of
this skill. Freestyle questions are normal conversation; answer them within the
current learning context without requiring another workflow command.
