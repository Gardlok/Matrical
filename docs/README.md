# Matrical documentation

This directory is the source of truth for the Matrical rehabilitation campaign,
its proposed architecture, active work, and validation procedures. Accepted
Rust source and executable tests remain authoritative for implemented behavior.

## Start here

- [`active-development.md`](active-development.md) — current baseline, active
  slice, known blockers, and owner decisions.
- [`roadmap.md`](roadmap.md) — ordered rehabilitation slices and exit gates.
- [`architecture/vision.md`](architecture/vision.md) — proposed product model,
  nomenclature, boundaries, and non-goals.
- [`architecture/consumers/longitudinal-feature-analysis.md`](architecture/consumers/longitudinal-feature-analysis.md)
  — non-binding design input from a prospective analytical consumer.
- [`testing-procedures.md`](testing-procedures.md) — local validation standard
  adapted from the disciplined ROSE procedure.
- [`teamlead-playbook.md`](teamlead-playbook.md) — fresh-session prompts, PR
  workflow, reviews, acceptance states, and owner gates.
- [`prompts/`](prompts/README.md) — Teamlead-authored prompts awaiting or cleared
  for fresh implementation sessions.

## Documentation authority

Documentation can describe:

- accepted behavior already proved by source and tests;
- proposed behavior clearly marked as proposed;
- planned work with explicit entry and exit criteria;
- immutable evidence tied to an exact functional SHA.

Documentation must not:

- present roadmap goals as implemented features;
- attach old executable evidence to a documentation-only commit;
- silently override an accepted source or test contract;
- convert Teamlead recommendation into an owner decision.

As the campaign matures, accepted architecture decisions should be recorded in
`docs/architecture/decisions/` as focused ADRs rather than buried in handoffs or
pull-request discussion.
