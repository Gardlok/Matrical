# Matrical active development

**Last updated:** 2026-08-25

## Accepted campaign baseline

```text
repository Gardlok/Matrical
branch     main
commit     1c5ec09346f249496f1bb2e72095e073b348568a
tree       9677aa266b8aa403b4cdbfbe81c155c7a6a77861
version    0.1.0
```

Commit `1c5ec09346f249496f1bb2e72095e073b348568a` merged PR #3 and owner-accepted
R1-A. Its tree exactly matches the accepted R1-A candidate tree. PR #2 closed
the R0 foundation at `dea2adb83404743558ae9da7a3d94aefdad4b903` after PR #1
established it at `b929e48481ae7ab41c972447b1547671afe4a4d8`. The historical
pre-campaign source baseline remains `6deb812e11a519404fec90408bf95651764cd2f8`
with tree `9d643f5066c8e99ad111e5b0fe48265773a70092`.

Neither baseline is a claim that the public library is functional or
release-ready.

## Active campaign

**Campaign:** Matrical rehabilitation

**R0 status:** owner accepted

**R0-F status:** owner accepted

**R1-A:** OWNER ACCEPTED — MERGED IN PR #3

**R1-B:** LOCAL REVIEW CANDIDATE — TEAMLEAD GATE

**R1-C:** BLOCKED

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
7. The root `Cargo.lock` is committed for reproducible campaign and CI
   qualification; downstream library users remain free to resolve within the
   published dependency constraints.
8. Rayon remains deferred until R6 benchmark evidence. Crossbeam is temporarily
   retained because compiled historical Matrix, Vector, and Element types use
   it; its removal belongs to later source/invariant reconstruction.
9. Serde, DashMap, and Criterion remain absent until implemented serialization,
   map, or benchmark behavior earns them.
10. Eventual crates.io publication remains a goal, but only after R8
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
- PR #3 merged and owner-accepted R1-A at the current accepted baseline;
- each accepted candidate tree exactly matched its merged tree;
- the change was documentation-only;
- `git diff --check`, trailing-whitespace inspection, and relative-link
  verification passed before merge;
- no executable behavior, dependency graph, version, or release state changed.

## Current authorized work

R1-B restores dependency and Rust 1.85 reproducibility from the exact accepted
baseline above. The local candidate corrects Cargo metadata, pins the default
toolchain, commits a freshly generated development lockfile, removes only the
evidence-confirmed unearned dependencies, and removes the compiled imports made
invalid by that pruning. Its evidence is recorded in
[`development/2026-08-24-r1b-dependency-msrv-reproducibility.md`](development/2026-08-24-r1b-dependency-msrv-reproducibility.md).

No R1-B draft PR or merge is claimed. The recursive `MatricalError` `Debug`
defect remains the first source-correctness boundary. R1-C may not begin until
the R1-B candidate passes Teamlead and owner gates.
