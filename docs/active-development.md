# Matrical active development

**Last updated:** 2026-08-20

## Accepted historical baseline

```text
repository Gardlok/Matrical
branch     main
commit     6deb812e11a519404fec90408bf95651764cd2f8
tree       9d643f5066c8e99ad111e5b0fe48265773a70092
version    0.1.0
```

The baseline is historical provenance, not a claim that the public library is
functional or release-ready.

## Active campaign

**Campaign:** Matrical rehabilitation

**Active slice:** R0 — establish the base of operations

**Candidate branch:** `docs/rehabilitation-foundation`

**Candidate PR:** draft documentation-foundation PR for this branch

R0 establishes truthful project documentation, the Matrix/Lens/Gear/Cog/Tag
architecture direction, a staged roadmap, MVECv1 local validation, and the
Teamlead session/review protocol.

## Baseline findings that motivate rehabilitation

- `Matrix<V>` is a queue-capacity shell rather than a usable two-dimensional
  abstraction.
- region mutation exists directly over `ndarray::Array2<f64>` in Gear, but the
  behavior is not integrated with Matrix or Lens.
- some public validation paths return success without executing strategies.
- `MatricalError` debug formatting is recursively defined.
- Cog construction permits missing context that later code unwraps.
- the Vector implementation has trait bounds not implemented by Element.
- several operation modules and the top-level matrix tests are empty or
  commented placeholders.
- concurrency, parallelism, persistence, and zero-copy aspirations are not yet
  supported by defined public contracts or evidence.
- the README references a contribution guide that did not exist and described
  the project as actively developing long after the last source update.

These findings are inputs to R1 and later slices. This documentation change does
not silently repair, delete, or declare compatibility for the historical code.

## Current recommendation

Build Matrical as a semantic transformation layer over mature matrix storage:

```text
Matrix -> Lens -> Gear (+ Cog) -> result (+ Tags)
```

Start sequential and deterministic. Introduce advanced type-system features,
parallelism, concurrent mutation, persistence, and alternate storage only when a
working contract demonstrates their benefit.

## Pending owner decisions

1. Accept or revise the semantic transformation product position.
2. Confirm whether `ndarray::Array2<T>` should be the first dense storage core.
3. Select an MSRV policy; Rust 1.85.0 is a candidate for alignment with nearby
   projects, but is not yet accepted here.
4. Decide whether the unfinished 0.1.0 surface has any compatibility obligation.
5. Confirm whether the first rehabilitated release should target 0.2.0.
6. Decide whether SurrealDB is removed from the immediate dependency graph and
   retained only as deferred optional-integration research.
7. Confirm whether crates.io publication is an eventual campaign goal.

## R0 exit evidence

Required before Teamlead acceptance:

- documentation-only diff confirmed;
- `git diff --check` passes;
- internal documentation links resolve;
- no executable behavior or dependency graph changed;
- candidate SHA and PR recorded after publication authorization.

## Next recommended slice

R1 should perform reproducible compile and dependency reconnaissance. It should
classify the historical source before editing it broadly, establish the toolchain
and lockfile policy, and produce the first bounded implementation plan from
actual compiler and test evidence.
