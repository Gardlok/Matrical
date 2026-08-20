# Matrical session prompts

This directory stores Teamlead-authored implementation and review prompts that
are intended to survive beyond one chat session.

A prompt is not automatically authorized merely because it exists here. Its
header must identify one of these states:

- **DRAFT** — incomplete or waiting on a baseline/owner decision;
- **READY** — Teamlead-complete and based on an accepted exact SHA;
- **DISPATCHED** — assigned to one active development session;
- **CLOSED** — implementation accepted, superseded, or intentionally abandoned.

Before dispatch, replace every baseline placeholder with the exact accepted
`main` SHA and confirm the prompt still matches current source, issues, PRs,
roadmap, and validation procedures.

## Current prompts

- [`r1a-baseline-reconnaissance.md`](r1a-baseline-reconnaissance.md) — **DRAFT**;
  first post-R0 session to reproduce and classify the historical build without
  broadly fixing it.
