# Matrical active development

**Last updated:** 2026-08-24

## Accepted campaign baseline

```text
repository Gardlok/Matrical
branch     main
commit     dea2adb83404743558ae9da7a3d94aefdad4b903
tree       b46d45e75c9337a2f28f037fea5ac8706c53098f
version    0.1.0
```

Commit `dea2adb83404743558ae9da7a3d94aefdad4b903` merged PR #2 and closed the
R0 foundation after PR #1 established it at
`b929e48481ae7ab41c972447b1547671afe4a4d8`. The historical pre-campaign source
baseline remains `6deb812e11a519404fec90408bf95651764cd2f8` with tree
`9d643f5066c8e99ad111e5b0fe48265773a70092`.

Neither baseline is a claim that the public library is functional or
release-ready.

## Active campaign

**Campaign:** Matrical rehabilitation

**R0 status:** owner accepted

**R0-F status:** owner accepted

**R1-A implementation/evidence:** COMPLETE

**R1-A documentation:** PUBLISHED IN DRAFT PR #3 — REVIEW CANDIDATE

**R1-A merge:** OWNER GATE

**R1-B:** BLOCKED UNTIL DRAFT PR #3 IS ACCEPTED AND MERGED

R1-A was dispatched from the exact accepted commit and tree above. Its
reconnaissance report is
[`development/2026-08-20-r1a-baseline-reconnaissance.md`](development/2026-08-20-r1a-baseline-reconnaissance.md).

The initial cloud environment could not execute Rust tooling. Orion completed
the executable continuation from a detached exact-SHA worktree. Consecutive
fresh resolutions drifted, and the resolved graph failed the Rust 1.85 MSRV
check before Matrical source compilation. The unchanged source compiled and all
24 discovered tests passed with the same locked graph on Rust 1.93.1. Clippy
confirmed recursive `MatricalError` debug formatting as a source correctness
defect. R1-A successfully established these blockers; it does not qualify the
library baseline as reproducible or Rust-1.85-compatible.

## Accepted owner decisions

1. Matrical will be a semantic matrix-transformation library rather than a
   replacement for general linear-algebra kernels.
2. `ndarray::Array2<T>` is the initial dense-storage foundation.
3. Rust 1.85.0 is the initial MSRV.
4. The unfinished 0.1.0 prototype has no compatibility promise.
5. The first rehabilitated release targets 0.2.0, subject to R8 qualification
   and an explicit owner release gate.
6. SurrealDB leaves the immediate dependency graph and remains deferred
   optional-integration research.
7. Execution begins sequential and deterministic. Historical concurrency and
   parallelism dependencies must not remain without an implemented purpose;
   R1-A classifies them before a later slice removes or retains them.
8. Eventual crates.io publication remains a goal, but only after R8
   qualification and explicit owner authorization.

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

These findings are inputs to R1 and later slices. R0 did not silently repair,
delete, or declare compatibility for the historical code.

## Downstream design input

The proposed analytical typing application is the first concrete downstream
consumer informing the rehabilitation campaign. Its non-binding design input is
recorded in
[`architecture/consumers/longitudinal-feature-analysis.md`](architecture/consumers/longitudinal-feature-analysis.md).

The consumer note does not make Matrical responsible for typing capture,
application identifiers, databases, cognitive-health interpretation, or
domain-specific analyzers. It supplies concrete pressures and acceptance inputs
for R2 through R6 while leaving their exact APIs open to evidence and review.

## R0 acceptance evidence

- PR #1 established the rehabilitation foundation at `b929e484...`;
- PR #2 closed R0-F at the accepted commit and tree above;
- each accepted candidate tree exactly matched its merged tree;
- the change was documentation-only;
- `git diff --check`, trailing-whitespace inspection, and relative-link
  verification passed before merge;
- no executable behavior, dependency graph, version, or release state changed.

## Current authorized work

R1-A documentation is published in draft PR
[#3](https://github.com/Gardlok/Matrical/pull/3) as a review candidate and awaits
Teamlead/owner merge acceptance. The report preserves the initial cloud
limitation, incorporates the completed Orion investigation, and recommends a
reproducibility-first R1-B boundary: explicit dependency and lockfile policy,
evidence-led dependency pruning beginning with SurrealDB, valid Cargo metadata,
and complete Rust 1.85 requalification.

R1-A does not authorize Rust-source, manifest, dependency, CI, version, or API
changes. This record does not mark PR #3 ready, authorize its merge, or authorize
R1-B work.
