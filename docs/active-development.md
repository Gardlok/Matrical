# Matrical active development

**Last updated:** 2026-08-31

## Accepted campaign baseline

```text
repository Gardlok/Matrical
branch     main
commit     f28fc380926c8175ff9b5faeb092be5bd7426245
tree       5d8bac5a769ba0fc3b77dc4b107ccd90d6c0dd86
version    0.1.0
MSRV       Rust 1.85.0
```

Commit `f28fc380926c8175ff9b5faeb092be5bd7426245` merged PR #12 and the
owner-accepted R7-A versioned dense snapshot result.

The accepted baseline is a qualified rehabilitation checkpoint, not a release,
tag, GitHub Release, crates.io publication, or production-readiness claim.

## Active campaign

```text
R0:   COMPLETE — OWNER ACCEPTED
R1:   COMPLETE — OWNER ACCEPTED
R2:   COMPLETE — OWNER ACCEPTED — MERGED IN PR #7
R3:   COMPLETE — OWNER ACCEPTED — MERGED IN PR #8
R4:   COMPLETE — OWNER ACCEPTED — MERGED IN PR #9
R5:   COMPLETE — OWNER ACCEPTED — MERGED IN PR #10
R6:   COMPLETE — OWNER ACCEPTED — MERGED IN PR #11
R7-A: COMPLETE — OWNER ACCEPTED — MERGED IN PR #12
R7-B: DEFERRED — no demonstrated second-provider/integration need
R7:   COMPLETE — OWNER ACCEPTED
R8-A: AUTHORIZED — RELEASE-CANDIDATE QUALIFICATION
```

R7-B's deferral is evidence-based and temporary in meaning: sparse/mapped
storage, another live provider, or an external integration was not justified by
a concrete current need. Those possibilities were not rejected forever and may
be reconsidered when a real second-provider or integration problem exists.

R8-A starts from the exact accepted R7 baseline above on
`rehab/r8a-release-candidate`. Its bounded mission is to determine, with
mechanical evidence, whether the accepted library can be packaged, documented,
consumed from its package artifact, and versioned as a real Rust release without
repository-only assumptions.

R8-A does not authorize `cargo publish`, a Git tag, a GitHub Release, a release
date, persistence/storage expansion, or external project integration.

## Accepted owner decisions

1. Matrical is a semantic matrix-transformation library rather than a replacement
   for general linear-algebra kernels.
2. `ndarray::Array2<T>` is the private initial dense-storage foundation.
3. Rust 1.85.0 is the initial MSRV.
4. The unfinished historical prototype did not carry a compatibility promise
   into rehabilitation.
5. The first rehabilitated release is owner-gated; qualification and publication
   are separate decisions.
6. The root `Cargo.lock` remains committed for reproducible campaign and CI
   qualification.
7. Criterion 0.7.0 remains a development-only benchmark dependency with default
   features disabled and only `cargo_bench_support` enabled.
8. Rayon remains deferred because R6 measured the repaired sequential dense path
   at approximately direct-ndarray performance.
9. Serde 1.0.229 is an optional runtime feature; `serde_json` 1.0.151 is
   development/example-only.
10. `MatrixSnapshot` is inert interchange data, not a live storage backend, and
    is intentionally excluded from the everyday prelude.
11. Backend abstraction, sparse/mapped storage, persistence, and GAT/HRTB
    provider work remain deferred until real live implementations justify them.
12. Matrical core does not depend on a downstream orchestrator; future adapters
    belong at explicit integration boundaries and must preserve caller-selected
    Lens/LensMut Gear authority.

## Accepted R2–R7 result

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
built-ins. A Gear receives only the supplied Lens capability and cannot use Tags
as an execution channel.

Preserved evidence:
[`development/2026-08-28-r4-transform-composition.md`](development/2026-08-28-r4-transform-composition.md).

### Public learning surface

R5 established the recommended `matrical::prelude::*`, explicit supported
crate-root exports, task-oriented getting-started material, runnable quickstart
and custom-Gear examples, public API smoke coverage, API-stability policy, and
documentation centered on the accepted Matrix/Lens/Gear/Cog/Tag flow.

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

### Versioned dense snapshot interchange

R7-A established `MatrixSnapshot<T>` with explicit dense schema version 1,
private version/shape/value fields, fixed-width `u64` dimensions, checked
reconstruction, borrowed cloning and consuming ownership-transfer creation, an
optional Serde feature, deny-unknown-fields deserialization, a deterministic
integer JSON fixture, and explicit caller-owned transport/storage boundaries.

Accepted R7 merge:

```text
commit f28fc380926c8175ff9b5faeb092be5bd7426245
tree   5d8bac5a769ba0fc3b77dc4b107ccd90d6c0dd86
PR     #12
```

Preserved evidence:
[`development/2026-08-30-r7a-versioned-snapshot.md`](development/2026-08-30-r7a-versioned-snapshot.md)
and [interchange.md](interchange.md).

## R8-A release-candidate boundary

R8-A owns only release-readiness questions that require package metadata,
compatibility clarity, reproducible qualification, downstream package-artifact
proof, or an owner release decision. In particular it audits:

```text
package metadata and packaged contents
crate name / registry history
version recommendation
CHANGELOG
supported versus specialized versus legacy API
snapshot schema v1 release policy
direct dependency/license scope
Cargo package on Rust 1.85 and stable
independent default/serde packaged consumers
examples/docs/benchmark compile
GitHub CI and mechanical audits
```

The candidate remains version `0.1.0` unless registry/version evidence makes a
different candidate unambiguous. Owner judgment between valid alternatives is
recorded rather than guessed.

## Residual historical/future debt

Release qualification does not reconstruct historical operation, Element,
SQL/validation, or MatrixContext prototype design. Documentation-hidden
compatibility residue is not part of the recommended release surface.

Sparse/mapped storage, persistence engines, live backend traits, parallel runtime
paths, and external orchestration remain future evidence-selected work rather
than hidden R8-A scope.

## Current gate

R8-A becomes reviewable only after the final exact branch head passes package,
packaged-downstream, Rust 1.85/stable default/all-feature, examples, benchmark
compile, dependency/license, documentation, and mechanical qualification with
GitHub CI evidence.

```text
R1-R7: COMPLETE — OWNER ACCEPTED
R8-A: IN DEVELOPMENT / QUALIFICATION
```

A successful R8-A exit is either `READY FOR OWNER RELEASE DECISION` or
`NOT RELEASE READY — BLOCKERS IDENTIFIED`, depending on evidence. Neither state
itself authorizes tagging, releasing, or publishing.
