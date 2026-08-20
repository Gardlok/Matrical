# Matrical session prompts

This directory stores Teamlead-authored implementation and review prompts that
are intended to survive beyond one chat session.

A prompt is not automatically authorized merely because it exists here. Its
header must identify one of these states:

- **DRAFT** — incomplete or waiting on a baseline/owner decision;
- **TEMPLATE READY** — bounded and complete, but its Teamlead dispatch must
  inject the exact post-prerequisite `main` commit and tree;
- **READY** — Teamlead-complete and based on an accepted exact SHA;
- **DISPATCHED** — assigned to one active development session;
- **CLOSED** — implementation accepted, superseded, or intentionally abandoned.

Before dispatch, the Teamlead must supply the exact accepted `main` commit and
tree and confirm the prompt still matches current source, issues, PRs, roadmap,
and validation procedures. A repository template must not guess the SHA of the
merge that activates it.

## Current prompts

- [`r1a-baseline-reconnaissance.md`](r1a-baseline-reconnaissance.md) —
  **TEMPLATE READY**; first post-R0 session to reproduce and classify the
  historical build without broadly fixing it. Await an exact Teamlead dispatch
  baseline after the foundation closeout merges.
