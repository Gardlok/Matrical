# Matrical active development

**Last updated:** 2026-08-28

## Accepted campaign baseline

```text
repository Gardlok/Matrical
branch     main
commit     059f148a99cfe2b5b881ada9af9acc286f584b6a
tree       0ad0e91d983912f743dc16972eb11a5a79afb286
version    0.1.0
```

Commit `059f148a99cfe2b5b881ada9af9acc286f584b6a` merged PR #6 and
owner-accepted R1-D. R1 is complete.

Earlier accepted campaign merges remain:

- R1-A: PR #3;
- R1-B: PR #4;
- R1-C: PR #5;
- R1-D: PR #6.

The historical pre-campaign source baseline remains
`6deb812e11a519404fec90408bf95651764cd2f8` with tree
`9d643f5066c8e99ad111e5b0fe48265773a70092`.

The accepted baseline is not a release-readiness claim. Version remains `0.1.0`.

## Active campaign

**Campaign:** Matrical rehabilitation

**R0:** COMPLETE — OWNER ACCEPTED

**R1:** COMPLETE — OWNER ACCEPTED

**R1-D:** OWNER ACCEPTED — MERGED IN PR #6

**R2:** COMPLETE — TEAMLEAD/OWNER ACCEPTANCE PENDING

**R3:** BLOCKED ONLY ON R2 TEAMLEAD/OWNER ACCEPTANCE

R2 reconstructs the first useful public core around `Shape`, `Index`, `Region`,
`Matrix<T>`, and `MatricalError`. The dense Matrix storage direction accepted in
R0/R1 is now realized with private `ndarray::Array2<T>` storage rather than the
historical queue-capacity placeholder.

The code-complete R2 candidate passed the existing GitHub Qualification workflow
on both Rust 1.85.0 and stable after one in-scope Clippy-only test-expression
correction. The final PR remains an owner merge gate after Teamlead acceptance.

R2 evidence is recorded in
[`development/2026-08-28-r2-core-invariants.md`](development/2026-08-28-r2-core-invariants.md).

## Accepted owner decisions

1. Matrical is a semantic matrix-transformation library rather than a
   replacement for general linear-algebra kernels.
2. `ndarray::Array2<T>` is the initial dense-storage foundation.
3. Rust 1.85.0 is the initial MSRV.
4. The unfinished 0.1.0 prototype has no compatibility promise.
5. The first rehabilitated release targets 0.2.0, subject to R8 qualification
   and an explicit owner release gate.
6. SurrealDB remains deferred optional-integration research.
7. The root `Cargo.lock` stays committed for reproducible campaign and CI
   qualification.
8. Rayon remains deferred until R6 benchmark evidence. Crossbeam remains only
   where historical compiled non-Matrix structures still require it.
9. Serde, DashMap, and Criterion remain absent until implemented behavior earns
   them.
10. Eventual crates.io publication remains an R8 owner-controlled action.

## R2 core decisions

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
- GATs/HRTBs are not forced into the owned R2 core. R3 must compare a GAT-backed
  Lens/lending-view design with a simpler lifetime-generic design.

## Residual historical debt

R2 intentionally does not reconstruct Lens, Gear, Cog, Tag, Vector, the broader
operation framework, or inherited warning/formatting residue. `MatrixContext`
remains temporarily as detached legacy scaffolding because historical operation
traits still reference it; it is not Matrix storage and does not define Matrix
shape or ownership semantics.

## Current gate

R2 implementation and executable two-lane qualification are complete. PR #7 is
the Teamlead review candidate. Do not begin R3 or merge R2 until Teamlead/owner
acceptance completes the gate.
