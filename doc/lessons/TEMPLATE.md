# NN — Lesson title

Status: draft / ready / in progress / reviewed.
Prerequisite checkpoint: specify lesson/substep and actual starting files.
Target: specify learning branch and file paths; never assume an empty project.
Version context: record Bevy version and Rust edition checked.

## Outcome

One observable result and its completion criteria. Keep this introduction brief:
the learner wants to implement first and read the fuller explanations afterward.

## Apply it yourself

For each file, give its exact path, whether to create/replace/edit it, and complete
copyable code or an exact replacement block. Include all needed imports and
dependencies. No ellipses inside code intended to compile. Distinguish required
code from illustrative snippets. The tutor does not apply these exercise edits.
Keep notes between code blocks short and necessary for implementation. Put the
full conceptual explanation after the run/check step.
Include concise `///` comments above functions describing purpose, caller and
timing, and `//` comments for non-obvious implementation details. Explain intent
rather than narrating every statement. These comments belong in the copyable code.

## Verify

Provide commands, expected results and common failure symptoms. Explain meaningful
tests and what they prove. Use visual acceptance criteria where more appropriate.
Record what the tutor actually checked; label code not yet compiled as unverified.

## Design and concepts

Explain the Bevy/ECS behavior, ownership and scheduling decisions. Introduce Rust
details in context. Include an alternative, its tradeoff, and any intentional
shortcut. Use a small diagram only when it clarifies a real relationship.
Focus on connections across functions and relevant Bevy/Rust concepts; do not
repeat explanations already given in function or inline comments.

Use visible notes such as `> ℹ️ **Bevy — Who calls this?**` and
`> ℹ️ **Rust — What does mut allow here?**` followed by focused paragraphs.
For new Bevy mechanisms, explain the caller, timing/frequency, parameter supply,
and registration versus execution. Include a short concrete execution sequence
when it helps. Explain relevant Rust syntax using the actual lesson code,
including the distinction between mutable bindings and mutable references.
Keep essential content visible in terminal Markdown; optional HTML folding must
not be required to read the lesson. Keep these explanations after code and checks.

## Experiment and discuss

One small learner modification or prediction exercise. Optional hints before a
solution; avoid giving away the exercise answer immediately.

## Checkpoint

- [ ] Lesson prepared and validation status recorded.
- [ ] User applied the code.
- [ ] Result verified with evidence.
- [ ] Experiment/questions discussed with user.

Update `doc/PROGRESS.md` with exact remaining work and next action. The user can
request "Review my work" for review and "Close session" to save the handoff,
commit session changes and push. Do not invoke closing merely because checks pass.
Record any enduring design decision briefly here with its rationale and revisit
condition.
