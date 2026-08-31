# Matrical active development

**Last updated:** 2026-08-31

## Accepted campaign baseline

```text
repository Gardlok/Matrical
branch     main
commit     6be8b0ce910d66d784cc5e5ca2d52a59f1cd3773
tree       919f8f800f1ffa3b4750def03f803a807ff25179
version    0.1.0
```

Commit `6be8b0ce910d66d784cc5e5ca2d52a59f1cd3773` merged PR #11 and the
owner-accepted R6 dense-traversal performance result.

The accepted baseline is a qualified rehabilitation checkpoint, not a release or
production-readiness claim.

## Active campaign

```text
R0: COMPLETE — OWNER ACCEPTED
R1: COMPLETE — OWNER ACCEPTED
R2: COMPLETE — OWNER ACCEPTED — MERGED IN PR #7
R3: COMPLETE — OWNER ACCEPTED — MERGED IN PR #8
R4: COMPLETE — OWNER ACCEPTED — MERGED IN PR #9
R5: COMPLETE — OWNER ACCEPTED — MERGED IN PR #10
R6: COMPLETE — OWNER ACCEPTED — MERGED IN PR #11
R7: ACTIVE
R7-A: AUTHORIZED — VERSIONED DENSE SNAPSHOT INTERCHANGE
R7-B: BLOCKED ON R7-A ACCEPTANCE/MERGE
R8: BLOCKED ON R7 COMPLETION
```

R7-A starts from the exact accepted R6 baseline above on
`rehab/r7a-versioned-snapshot`. Its bounded mission is to add a Matrical-owned,
versioned dense snapshot DTO, checked reconstruction, an optional Serde feature,
a committed integer JSON fixture/example, dual default/all-feature CI coverage,
and documentation of the caller-owned transport boundary.

R7-A does not authorize sparse/mapped storage, persistence engines, filesystem or
network APIs, backend/provider traits, GAT/HRTB provider abstractions, Strustegy
dependencies, release qualification, or publication.

The execution flow remains:

```text
Matrix
  -> Lens / LensMut
  -> Gear (+ typed Cog)
  -> ExecutionReport (+ Tags)
```

R7-A adds a separate inert interchange path:

```text
Matrix<T>
  -> MatrixSnapshot<T>
  -> optional caller-selected Serde format/transport
  -> checked Matrix<T> reconstruction
```

## Accepted owner decisions

1. Matrical is a semantic matrix-transformation library rather than a replacement
   for general linear-algebra kernels.
2. `ndarray::Array2<T>` is the private initial dense-storage foundation.
3. Rust 1.85.0 is the initial MSRV.
4. The unfinished historical 0.1.0 prototype has no compatibility promise.
5. The first rehabilitated release remains an owner-gated future milestone; no
   release, tag, or publication is authorized by R7-A.
6. The root `Cargo.lock` remains committed for reproducible campaign and CI
   qualification.
7. Criterion 0.7.0 remains a development-only benchmark dependency with default
   features disabled and only `cargo_bench_support` enabled.
8. Rayon remains deferred because R6 measured the repaired sequential dense path
   at approximately direct-ndarray performance.
9. R7-A introduces Serde 1.0.229 only as an optional Matrical runtime feature;
   `serde_json` 1.0.151 is development/example-only.
10. `MatrixSnapshot` is inert interchange data, not a live storage backend, and
    is intentionally excluded from the everyday prelude.
11. Backend abstraction, sparse/mapped storage, persistence, and GAT/HRTB
    provider work remain deferred until real live implementations justify them.
12. Matrical core does not depend on Strustegy; a future adapter belongs in an
    integration boundary and must preserve caller-selected Lens/LensMut Gear
    authority.

## Accepted R2–R6 result

### Core geometry and storage

R2 established checked `Shape`, `Index`, half-open `Region`, owned `Matrix<T>`,
and structural `MatricalError` boundaries. Zero-sized shapes are valid,
row-major construction is exact, ordinary indexing is checked, and `ndarray`
remains private.

Preserved evidence:
[`development/2026-08-28-r2-core-invariants.md`](development/2026-08-28-r2-core-invariants.md).

### Borrowing views

R3 established `Lens<'a, T>` and `LensMut<'a, T>`. The caller chooses a Region,
Lens indexing is local to that selection, and Rust lifetimes enforce parent
lifetime and exclusive mutable borrowing. `to_row_major()` is the explicit
cloning conversion.

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

### Public learning surface

R5 established the recommended `matrical::prelude::*`, explicit supported
crate-root exports, task-oriented getting-started material, runnable quickstart
and custom-Gear examples, public API smoke coverage, API-stability policy, and
documentation that teaches the accepted Matrix/Lens/Gear/Cog/Tag flow without
requiring rehabilitation history.

Preserved evidence:
[`development/2026-08-29-r5-api-learning-surface.md`](development/2026-08-29-r5-api-learning-surface.md).

### Measured dense traversal

R6 established Criterion evidence, repaired inherited parent-wide Lens traversal
with checked private ndarray Region views, preserved public semantics/authority,
accounted for allocation/copy behavior, and explicitly deferred Rayon because the
repaired sequential path was already approximately direct-ndarray speed.

Accepted R6 merge:

```text
commit 6be8b0ce910d66d784cc5e5ca2d52a59f1cd3773
tree   919f8f800f1ffa3b4750def03f803a807ff25179
PR     #11
```

Preserved evidence:
[`development/2026-08-29-r6-measure-optimize.md`](development/2026-08-29-r6-measure-optimize.md)
and [performance.md](performance.md).

## R7-A implementation boundary

`MatrixSnapshot<T>` owns:

```text
version: u32
rows: u64
columns: u64
row_major: Vec<T>
```

Fields are private. `DENSE_SNAPSHOT_VERSION` is `1`. Borrowed snapshot creation
clones values and requires `T: Clone`; consuming snapshot creation transfers
owned values without a `Clone` bound. Reconstruction checks the version,
converts dimensions without truncation, and delegates shape/length validation to
`Shape::new` and `Matrix::from_row_major`.

The optional `serde` feature applies only to `MatrixSnapshot<T>`. Matrix itself
remains non-serializable and ndarray remains outside the interchange schema.
The committed JSON fixture uses integer values and validates schema semantics;
JSON is not Matrical's persistence engine or a universal element-fidelity claim.

See [interchange.md](interchange.md) and
[`development/2026-08-30-r7a-versioned-snapshot.md`](development/2026-08-30-r7a-versioned-snapshot.md).

## Residual historical/future debt

R7-A does not reconstruct the historical operation framework, Vector/Element/SQL
prototypes, detached `MatrixContext`, or broad dependency residue. It also does
not add sparse/mapped storage, persistence engines, live backend traits,
parallel runtime paths, or direct application orchestration.

## Current gate

R7-A is reviewable only after the final exact branch head passes default and
all-feature Rust 1.85/stable qualification, benchmark compilation, mechanical
scope/whitespace/link checks, and GitHub CI.

```text
R1-R6: COMPLETE — OWNER ACCEPTED
R7-A: IN DEVELOPMENT / QUALIFICATION
R7-B: BLOCKED ON R7-A ACCEPTANCE/MERGE
R8: BLOCKED ON R7 COMPLETION
```

No R7-B implementation, version bump, tag, release, publication, persistence
backend, sparse/mapped backend, or Strustegy integration is authorized in this
branch.
