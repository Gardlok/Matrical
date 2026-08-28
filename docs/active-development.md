# Matrical active development

**Last updated:** 2026-08-28

## Accepted campaign baseline

```text
repository Gardlok/Matrical
branch     main
commit     9fbc712084a78570e8ac2b980ff0d4474c90ee7f
tree       4db71daeb50553edc6cdc69a2986f93087be4f35
version    0.1.0
```

Commit `9fbc712084a78570e8ac2b980ff0d4474c90ee7f` merged PR #8 and
owner-accepted R3. PR #7 owner-accepted R2 at
`2f76a87e171a32a58a6d7244fdeb1b8794fc043a`; PR #6 owner-accepted R1-D at
`059f148a99cfe2b5b881ada9af9acc286f584b6a`; PR #5 owner-accepted R1-C at
`16ddcc878c9cc8c8701dbc01453e08cfccd00b54`; PR #4 owner-accepted R1-B at
`1a5e4a72d7c0bb2a6ddd92b070eb853e98d6f136`; PR #3 owner-accepted R1-A at
`1c5ec09346f249496f1bb2e72095e073b348568a`. PR #2 closed the R0 foundation at
`dea2adb83404743558ae9da7a3d94aefdad4b903` after PR #1 established it at
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

**R3:** COMPLETE — OWNER ACCEPTED — MERGED IN PR #8

**R4:** ACTIVE

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
   where historical compiled non-Matrix structures still require it.
9. Serde, DashMap, and Criterion remain absent until implemented serialization,
   map, or benchmark behavior earns them.
10. Eventual crates.io publication remains a goal, but only after R8
   qualification and explicit owner authorization.

## Historical findings that motivate rehabilitation

- the historical `Matrix<V>` was a queue-capacity shell rather than a usable
  two-dimensional abstraction; R2 replaced it with checked dense
  `ndarray::Array2<T>` storage behind Matrical invariants;
- historical Gear owned independent `ndarray::Array2<f64>` data and performed
  region mutation outside the accepted Matrix/Lens architecture;
- historical Cog construction mixed callbacks, strategy objects, optional
  context, and independent ndarray data;
- historical Tag was a bare string name alongside unused parameterized-query/DI
  residue;
- `MatricalError` debug formatting was recursively defined until R1-C;
- Cog construction permitted missing context that a later strategy path
  unwrapped until R1-D converted that boundary to `InvalidContext`;
- the Vector implementation and several operation modules remain prototype debt;
- concurrency, parallelism, persistence, and broader optimization aspirations
  remain unsupported until later contracts and evidence justify them.

These findings remain historical inputs. R4 reconstructs only the transformation
layer and does not silently declare unrelated prototype scaffolding complete.

## Downstream design input

The proposed analytical typing application remains the first concrete downstream
consumer informing the rehabilitation campaign. Its non-binding design input is
recorded in
[`architecture/consumers/longitudinal-feature-analysis.md`](architecture/consumers/longitudinal-feature-analysis.md).

The consumer note does not make Matrical responsible for typing capture,
application identifiers, databases, cognitive-health interpretation, or
domain-specific analyzers.

## R1 closeout evidence

R1-C source-correctness evidence remains preserved in
[`development/2026-08-28-r1c-source-correctness.md`](development/2026-08-28-r1c-source-correctness.md).

R1-D repaired the confirmed Cog and nested-validation runtime-safety boundaries,
added focused regressions, and established two-lane qualification CI for Rust
1.85.0 and current stable. Its evidence remains preserved in
[`development/2026-08-28-r1d-runtime-safety-ci-closeout.md`](development/2026-08-28-r1d-runtime-safety-ci-closeout.md).

PR #6 merged the owner-accepted R1-D result. R1 exit criteria are satisfied.

## R2 core result

R2 established the checked public core around `Shape`, `Index`, `Region`,
`Matrix<T>`, and `MatricalError`:

- zero-sized shapes are valid;
- shape element-count overflow is rejected during construction;
- public element access is checked through `Index` and returns typed errors;
- `Region` uses checked half-open row and column bounds and permits empty
  regions;
- Matrix construction, iteration, mutable iteration, and owned conversion use a
  deterministic logical row-major contract;
- Matrix storage is private `ndarray::Array2<T>` with no unrestricted mutable
  backend escape hatch;
- a validity/missingness mask is not intrinsic Matrix storage;
- GATs/HRTBs were not forced into the owned R2 core.

PR #7 merged the owner-accepted R2 result. Preserved evidence remains in
[`development/2026-08-28-r2-core-invariants.md`](development/2026-08-28-r2-core-invariants.md).

## R3 borrowing-view result

R3 replaced the historical Lens prototype with concrete `Lens<'a, T>` and
`LensMut<'a, T>` borrowing views over the checked parent Matrix. Immutable Lens
stores `&Matrix<T>` and mutable Lens stores `&mut Matrix<T>`, making lifetime and
mutable exclusivity visible to the borrow checker without project-authored
unsafe code or runtime overlap tracking.

The selected `Region` remains in parent coordinates while public `Index` access
is Lens-local. Construction, checked access, row/column selection, and iteration
are borrowing operations; `to_row_major()` is the explicit allocating
conversion. Iteration is deterministic logical row-major order.

R3 compared concrete lifetime-generic views with a public GAT lending provider
and deferred the GAT surface because the concrete design was simpler and Matrix
was the only proven provider. R4 must reassess that decision using real Gear
composition and authority boundaries rather than copying the R3 rationale.

PR #8 merged the owner-accepted R3 result. Detailed evidence remains in
[`development/2026-08-28-r3-safe-lens-views.md`](development/2026-08-28-r3-safe-lens-views.md).

## R4 transformation candidate

R4 is rebuilding Gear, Cog, and Tag around the accepted Lens capability:

```text
Matrix<T>
    -> Lens / LensMut
    -> read-only or mutating Gear
    +  typed Cog context/policy
    -> ExecutionReport<O>
    +  bounded Tag provenance
```

The central authority rule is that a Gear operates only on data exposed through
the Lens it receives. Read and mutating Gear traits are distinct. Typed Cog
context is resolved and validated centrally before execution. Missing required
context returns `InvalidContext`. Tags are bounded inert provenance and are not
passed into Gear execution. Reports record Gear identity, exact Region, typed
effect, typed output, and ordered Tags.

Built-in R4 examples are deterministic `SumGear`, `AddScalarGear`, `ScaleGear`,
and `ClampGear`. Downstream crates can implement the public Gear traits directly
with static dispatch and no registry.

The detailed implementation and decision record is
[`development/2026-08-28-r4-transform-composition.md`](development/2026-08-28-r4-transform-composition.md).

## Residual historical debt

R4 does not reconstruct Vector, unrelated operation modules, detached
`MatrixContext`, dependency cleanup, or inherited warning/formatting residue.
Those remain outside the transformation-layer exit gate.

## Current gate

R4 development is active on `rehab/r4-transform-composition` from accepted
baseline `9fbc712084a78570e8ac2b980ff0d4474c90ee7f`, tree
`4db71daeb50553edc6cdc69a2986f93087be4f35`. R5 remains blocked until the R4
candidate completes exact-head qualification and receives Teamlead/owner
acceptance.
