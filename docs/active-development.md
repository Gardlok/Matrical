# Matrical active development

**Last updated:** 2026-08-28

## Accepted campaign baseline

```text
repository Gardlok/Matrical
branch     main
commit     2f76a87e171a32a58a6d7244fdeb1b8794fc043a
tree       947684bc73841fb0842d5664e168e28bc8d3b05b
version    0.1.0
```

Commit `2f76a87e171a32a58a6d7244fdeb1b8794fc043a` merged PR #7 and
owner-accepted R2. PR #6 owner-accepted R1-D at
`059f148a99cfe2b5b881ada9af9acc286f584b6a`; PR #5 owner-accepted R1-C at
`16ddcc878c9cc8c8701dbc01453e08cfccd00b54`; PR #4 owner-accepted R1-B at
`1a5e4a72d7c0bb2a6ddd92b070eb853e98d6f136`; PR #3 owner-accepted R1-A at
`1c5ec09346f249496f1bb2e72095e073b348568a` with tree
`9677aa266b8aa403b4cdbfbe81c155c7a6a77861`. PR #2 closed the R0 foundation
at `dea2adb83404743558ae9da7a3d94aefdad4b903` after PR #1 established it at
`b929e48481ae7ab41c972447b1547671afe4a4d8`. The historical pre-campaign source
baseline remains `6deb812e11a519404fec90408bf95651764cd2f8` with tree
`9d643f5066c8e99ad111e5b0fe48265773a70092`.

The accepted baseline is not a claim that the public library is release-ready.

## Active campaign

**Campaign:** Matrical rehabilitation

**R0 status:** owner accepted

**R0-F status:** owner accepted

**R1-A:** OWNER ACCEPTED — MERGED IN PR #3

**R1-B:** OWNER ACCEPTED — MERGED IN PR #4

**R1-C:** OWNER ACCEPTED — MERGED IN PR #5

**R1-D:** OWNER ACCEPTED — MERGED IN PR #6

**R1:** COMPLETE — OWNER ACCEPTED

**R2:** COMPLETE — OWNER ACCEPTED — MERGED IN PR #7

**R3:** ACTIVE

R1-A was dispatched from its exact accepted commit and tree. Its reconnaissance
report is
[`development/2026-08-20-r1a-baseline-reconnaissance.md`](development/2026-08-20-r1a-baseline-reconnaissance.md).

The initial cloud environment could not execute Rust tooling. Orion completed
the executable continuation from a detached exact-SHA worktree. Consecutive
fresh resolutions drifted, and the resolved graph failed the Rust 1.85 MSRV
check before Matrical source compilation. The unchanged source compiled and all
24 discovered tests passed with the same locked graph on Rust 1.93.1. Clippy
confirmed recursive `MatricalError` debug formatting as a source correctness
defect. R1-A successfully established these blockers; later R1 slices repaired
and qualified them rather than retroactively changing the reconnaissance result.

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
8. Rayon remains deferred until R6 benchmark evidence. Crossbeam remains only
   where historical compiled non-Matrix structures still require it; R2 removes
   Crossbeam queue storage from Matrix without broad dependency cleanup.
9. Serde, DashMap, and Criterion remain absent until implemented serialization,
   map, or benchmark behavior earns them.
10. Eventual crates.io publication remains a goal, but only after R8
   qualification and explicit owner authorization.

## Baseline findings that motivate rehabilitation

- the historical `Matrix<V>` was a queue-capacity shell rather than a usable
  two-dimensional abstraction; R2 replaced that Matrix storage model with a
  checked dense `ndarray::Array2<T>` core;
- region mutation existed directly over `ndarray::Array2<f64>` in historical
  Gear code, but that code is not the accepted Matrix or R3 Lens contract;
- some public validation paths return success without executing strategies;
- `MatricalError` debug formatting was recursively defined until R1-C
  replaced the recursive formatter with derived `Debug`;
- Cog construction permitted missing context that a later strategy path
  unwrapped until R1-D converted that boundary to `InvalidContext`;
- the Vector implementation has trait bounds not implemented by Element;
- several operation modules and the top-level matrix tests are empty or
  commented placeholders;
- concurrency, parallelism, persistence, and broader optimization aspirations
  remain unsupported until later contracts and evidence justify them.

These findings remain historical inputs to rehabilitation. R3 replaces only the
unfinished historical Lens meaning and does not silently declare the remaining
prototype scaffolding complete.

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
- PR #2 closed R0-F at `dea2adb83404743558ae9da7a3d94aefdad4b903`;
- each accepted candidate tree exactly matched its merged tree;
- the change was documentation-only;
- `git diff --check`, trailing-whitespace inspection, and relative-link
  verification passed before merge;
- no executable behavior, dependency graph, version, or release state changed.

## R1 closeout evidence

R1-C source-correctness evidence remains preserved in
[`development/2026-08-28-r1c-source-correctness.md`](development/2026-08-28-r1c-source-correctness.md).

R1-D repaired the confirmed Cog and nested-validation runtime-safety boundaries,
added focused regressions, and established two-lane qualification CI for Rust
1.85.0 and current stable. Its evidence remains preserved in
[`development/2026-08-28-r1d-runtime-safety-ci-closeout.md`](development/2026-08-28-r1d-runtime-safety-ci-closeout.md).

PR #6 merged the owner-accepted R1-D result. R1 exit criteria are satisfied and
R1 is complete.

## R2 core result

R2 established the first checked public core around `Shape`, `Index`, `Region`,
`Matrix<T>`, and `MatricalError`:

- zero-sized shapes (`0 x 0`, `0 x N`, `N x 0`) are valid;
- shape element-count overflow is rejected during `Shape` construction;
- public element access is checked through `Index` and returns typed errors;
- `Region` uses checked half-open row and column bounds and permits empty
  regions;
- Matrix construction, iteration, mutable iteration, and owned conversion use a
  deterministic logical row-major contract;
- Matrix storage is private `ndarray::Array2<T>` with no unrestricted mutable
  backend escape hatch;
- a validity/missingness mask is not intrinsic Matrix storage; it belongs in an
  explicit paired structure, wrapper, or downstream domain type unless later
  evidence proves otherwise;
- GATs/HRTBs were not forced into the owned R2 core. R3 compares a GAT-backed
  lending-view design with a concrete lifetime-generic design.

PR #7 merged the owner-accepted R2 result at the exact current baseline above.
The preserved R2 evidence remains in
[`development/2026-08-28-r2-core-invariants.md`](development/2026-08-28-r2-core-invariants.md).

## R3 borrowing-view candidate

R3 replaces the historical validation/strategy Lens prototype with concrete
`Lens<'a, T>` and `LensMut<'a, T>` borrowing views. The current implementation
keeps ndarray private by borrowing the checked parent `Matrix<T>` directly:
immutable Lens stores `&Matrix<T>` and mutable Lens stores `&mut Matrix<T>`.
This makes Matrix lifetime and mutable exclusivity visible to the Rust borrow
checker without project-authored unsafe code or runtime overlap tracking.

The selected `Region` remains expressed in parent coordinates. Public `Index`
access through either Lens is Lens-local, so local `(0, 0)` maps to the Region's
top-left parent coordinate. Row/column selectors return the same rectangular
Lens types, including `1 x 0` rows for in-range rows of `N x 0` matrices and
`0 x 1` columns for in-range columns of `0 x N` matrices.

Creation, checked access, row/column selection, and iteration are borrowing
operations and do not intentionally allocate. `to_row_major()` is the explicit
allocating `T: Clone` conversion. Logical iteration is row-major within the
selected rectangle and does not promise physical contiguity.

The detailed R3 design and qualification record is
[`development/2026-08-28-r3-safe-lens-views.md`](development/2026-08-28-r3-safe-lens-views.md).

## Residual historical debt

R3 intentionally does not reconstruct Gear, Cog, Tag, Vector, the broader
operation framework, or inherited warning/formatting residue. `MatrixContext`
remains detached legacy scaffolding because historical operation traits still
reference it; it is not Matrix storage and does not define Matrix shape,
ownership, or Lens semantics.

## Current gate

R3 development is active on `rehab/r3-safe-lens-views` from accepted baseline
`2f76a87e171a32a58a6d7244fdeb1b8794fc043a`. R4 remains blocked until the R3
candidate completes qualification and receives Teamlead/owner acceptance.
