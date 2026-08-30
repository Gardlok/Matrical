# Matrical documentation

Use this map according to what you are trying to do. New downstream users should
not need the development evidence to learn the library.

## Start here

1. [Getting started](getting-started.md) — construct a Matrix, select a Region,
   borrow through Lens/LensMut, run built-in Gears, provide Cog policy, inspect
   reports/Tags, define a Gear, and handle errors.
2. Crate rustdoc — the crate root explains the conceptual flow; `prelude` is the
   recommended import surface; `schematics` and `strategies` group deeper API
   navigation.
3. [`examples/r5_quickstart.rs`](../examples/r5_quickstart.rs) — canonical
   beginner program.
4. [`examples/r5_custom_gear.rs`](../examples/r5_custom_gear.rs) — downstream
   static Gear extension with custom typed context.

## API expectations

- [API stability and deprecation](api-stability.md) — current 0.1.0 compatibility
  position and how breaking changes are handled.
- [Architecture vision](architecture/vision.md) — Matrix/Lens/Gear/Cog/Tag
  responsibilities, authority boundaries, storage, and deferred abstractions.
- [Performance](performance.md) — R6 benchmark methodology, measured traversal
  result, allocation/copy accounting, and parallelism decision.
- [Rehabilitation roadmap](roadmap.md) — ordered slice goals and exit gates.

## Testing and contribution

- [Testing procedures](testing-procedures.md) — local and CI qualification model.
- [Active development](active-development.md) — exact accepted baseline and
  current campaign state.
- [Teamlead campaign playbook](teamlead-playbook.md) — bounded handoff and review
  workflow.
- [Contribution guide](../CONTRIBUTING.md) — repository contribution rules.

## Development evidence

The `development/` reports preserve exact implementation and qualification
history. They are review evidence, not prerequisites for ordinary API use.

- [R1-A baseline reconnaissance](development/2026-08-20-r1a-baseline-reconnaissance.md)
- [R1-B dependency/MSRV reproducibility](development/2026-08-24-r1b-dependency-msrv-reproducibility.md)
- [R1-C source correctness](development/2026-08-28-r1c-source-correctness.md)
- [R1-D runtime safety and CI closeout](development/2026-08-28-r1d-runtime-safety-ci-closeout.md)
- [R2 core invariants](development/2026-08-28-r2-core-invariants.md)
- [R3 safe Lens views](development/2026-08-28-r3-safe-lens-views.md)
- [R4 transformation composition](development/2026-08-28-r4-transform-composition.md)
- [R5 API learning surface](development/2026-08-29-r5-api-learning-surface.md)
- [R6 measure and optimize](development/2026-08-29-r6-measure-optimize.md)

Consumer design notes remain under `architecture/consumers/` and inform future
acceptance criteria without becoming Matrical's domain model.
