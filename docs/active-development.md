# Matrical active development

**Last updated:** 2026-08-29

## Accepted campaign baseline

```text
repository Gardlok/Matrical
branch     main
commit     6dc0320d1857d1c4fafd538fbf75ae80566887cc
tree       c421102b113b2dc2fc78373677a956e807dee7db
version    0.1.0
```

Commit `6dc0320d1857d1c4fafd538fbf75ae80566887cc` merged PR #9 and the
owner-accepted R4 transformation result. Its second parent,
`1d84483e5063845b0d04c2ced5b7e6335381d951`, is the accepted final R4
candidate. Earlier accepted rehabilitation merges remain recorded in the
[roadmap](roadmap.md) and development evidence.

The accepted baseline is a qualified rehabilitation checkpoint, not a claim of
release or production readiness.

## Active campaign

```text
R0: COMPLETE — OWNER ACCEPTED
R1: COMPLETE — OWNER ACCEPTED
R2: COMPLETE — OWNER ACCEPTED — MERGED IN PR #7
R3: COMPLETE — OWNER ACCEPTED — MERGED IN PR #8
R4: COMPLETE — OWNER ACCEPTED — MERGED IN PR #9
R5: ACTIVE
```

R5 starts from the exact baseline above on
`rehab/r5-api-learning-surface`. Its mission is to make the accepted R2–R4 API
understandable and pleasant for a new downstream Rust developer without making
them read rehabilitation history or implementation source.

The conceptual flow remains:

```text
Matrix
  -> Lens / LensMut
  -> Gear (+ typed Cog)
  -> ExecutionReport (+ Tags)
```

R5 may curate imports, module visibility, documentation, examples, caller-facing
error wording, and pre-release prototype exposure. It must not weaken the Lens
capability boundary, add new transformation authority, begin R6 performance
work, or create a release claim.

## Accepted owner decisions

1. Matrical is a semantic matrix-transformation library rather than a replacement
   for general linear-algebra kernels.
2. `ndarray::Array2<T>` is the private initial dense-storage foundation.
3. Rust 1.85.0 is the initial MSRV.
4. The unfinished historical 0.1.0 prototype has no compatibility promise.
5. The first rehabilitated release remains an owner-gated future milestone; no
   release, tag, or publication is authorized by R5.
6. The root `Cargo.lock` remains committed for reproducible campaign and CI
   qualification.
7. Rayon, Criterion, backend abstraction, persistence, and broad optimization
   remain deferred to later evidence-driven phases.
8. Serde, SurrealDB, dynamic Gear registries, DI containers, and speculative
   GAT/HRTB ceremony remain outside R5.

## Accepted R2–R4 result

### Core geometry and storage

R2 established checked `Shape`, `Index`, half-open `Region`, owned `Matrix<T>`,
and structural `MatricalError` boundaries. Zero-sized shapes are valid, row-major
construction is exact, ordinary indexing is checked, and `ndarray` remains
private.

Preserved evidence:
[`development/2026-08-28-r2-core-invariants.md`](development/2026-08-28-r2-core-invariants.md).

### Borrowing views

R3 established `Lens<'a, T>` and `LensMut<'a, T>`. The caller chooses a Region,
Lens indexing is local to that selection, and Rust lifetimes enforce parent
lifetime and exclusive mutable borrowing. Construction/access/iteration do not
intentionally allocate; `to_row_major()` is the explicit cloning conversion.

Preserved evidence:
[`development/2026-08-28-r3-safe-lens-views.md`](development/2026-08-28-r3-safe-lens-views.md).

### Typed transformation composition

R4 established separate `ReadGear<T>` and `MutGear<T>` contracts, typed
`Cog<C>` context validated through `ValidateCog`, inert `Tag` provenance,
`ExecutionReport<O>`, central `execute_read` / `execute_mut`, and deterministic
built-ins (`SumGear`, `AddScalarGear`, `ScaleGear`, `ClampGear`). A Gear receives
only the supplied Lens capability and cannot use Tags as an execution channel.

Preserved evidence:
[`development/2026-08-28-r4-transform-composition.md`](development/2026-08-28-r4-transform-composition.md).

## R5 learning-surface direction

R5 establishes three intentional discovery layers:

```text
matrical::prelude::*
  recommended everyday imports

matrical::{...}
  named supported crate-root exports

matrical::schematics / matrical::strategies
  deeper conceptual organization
```

Historical SQL/Element/Vector and operation scaffolding is not a parallel public
API. Compatibility residue may remain hidden while the crate is still 0.1.0,
but normal users should not encounter it in the primary rustdoc path.

The task-oriented entry points are:

- repository [README](../README.md);
- [getting-started guide](getting-started.md);
- generated crate rustdoc;
- runnable `examples/r5_quickstart.rs`;
- runnable `examples/r5_custom_gear.rs`;
- [API stability policy](api-stability.md).

## Residual historical debt

R5 does not reconstruct the historical operation framework, Vector/Element/SQL
prototypes, detached `MatrixContext`, or broad dependency residue. Those paths
remain historical scaffolding and are not evidence that the recommended API
requires them.

## Current gate

R5 is active from baseline
`6dc0320d1857d1c4fafd538fbf75ae80566887cc`, tree
`c421102b113b2dc2fc78373677a956e807dee7db`.

The detailed implementation and exact-head qualification record is
[`development/2026-08-29-r5-api-learning-surface.md`](development/2026-08-29-r5-api-learning-surface.md).

R6 remains blocked until the R5 candidate receives Teamlead/owner acceptance and
is merged.
