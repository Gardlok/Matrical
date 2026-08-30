# Matrical active development

**Last updated:** 2026-08-30

## Accepted campaign baseline

```text
repository Gardlok/Matrical
branch     main
commit     acd15be9d02d27e6189aadedad3620e9558efe8f
tree       bb4e2d1bb1b33254653873c9d5a4a11ca97e5add
version    0.1.0
```

Commit `acd15be9d02d27e6189aadedad3620e9558efe8f` merged PR #10 and the
owner-accepted R5 public-learning-surface result. Its second parent,
`bab1a12ae92f5024e423ac55183e4e78d756b8fd`, is the accepted final R5
candidate.

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
R6: REVIEWABLE — TEAMLEAD/OWNER ACCEPTANCE PENDING
```

R6 starts from the exact accepted R5 baseline above on
`rehab/r6-measure-optimize`. Its mission is to establish representative
performance evidence first, optimize only demonstrated waste, account for
allocation/copy behavior, decide whether parallel execution is justified, and
preserve the accepted public API and capability boundary.

The conceptual flow remains:

```text
Matrix
  -> Lens / LensMut
  -> Gear (+ typed Cog)
  -> ExecutionReport (+ Tags)
```

## Accepted owner decisions

1. Matrical is a semantic matrix-transformation library rather than a replacement
   for general linear-algebra kernels.
2. `ndarray::Array2<T>` is the private initial dense-storage foundation.
3. Rust 1.85.0 is the initial MSRV.
4. The unfinished historical 0.1.0 prototype has no compatibility promise.
5. The first rehabilitated release remains an owner-gated future milestone; no
   release, tag, or publication is authorized by R6.
6. The root `Cargo.lock` remains committed for reproducible campaign and CI
   qualification.
7. Criterion 0.7.0 is accepted in R6 as a development-only benchmark dependency
   with default features disabled and only `cargo_bench_support` enabled.
8. Rayon remains deferred: the measured R6 sequential repair brings dense
   Lens/LensMut traversal to approximately direct-ndarray performance, so R6 has
   no evidence that parallelism justifies added runtime/concurrency complexity.
9. Backend abstraction, persistence, serialization, dynamic Gear registries, DI
   containers, and speculative GAT/HRTB ceremony remain outside R6.

## Accepted R2–R5 result

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

## R6 measured result

The permanent R6 benchmark harness was established at:

```text
commit 91d1724a70c2af7ff5bd077dd8625b73302e0939
tree   988e5a5638dd6267d765c150df7a1f2a400941bc
```

That commit contains accepted R5 source plus the R6 Criterion harness. Preliminary
measurement showed fixed 4 x 4 Lens traversal scaling with the entire parent
Matrix because the inherited implementation enumerated/filter-mapped every parent
element.

The source repair at `db1c498edac854b59065cdcf1bfa5595334292aa`
replaced that scan with a checked private ndarray Region view. Regression
coverage at `9a9f4199d28da4294bdf0973cb7579e4add5d78f` preserves full/interior,
single-row/column, empty/zero-dimension, row-major mutation, local indexing, and
foreign-Region failure semantics.

The public method signatures and caller-selected Lens/LensMut authority boundary
are unchanged. No unsafe code or Rayon was added.

### Authoritative performance gate

One consolidated owner-machine run on Orion measured the baseline harness commit
and optimized candidate with identical benchmark code, lockfile, stable toolchain,
and host.

Representative result:

```text
fixed 4x4 Lens read, parent 100000x64
baseline  30.694 ms
candidate  7.242 ns
speedup    4,238,449x

candidate fixed-4x4 parent scaling:
32x24       7.341 ns
1024x64     7.329 ns
100000x64   7.242 ns
```

Every predeclared dense-Lens and Gear-over-Lens budget passed. Full details,
environment, allocation/copy accounting, profiling limitation, and the Rayon
decision are in [performance.md](performance.md).

## Residual historical debt

R6 does not reconstruct the historical operation framework, Vector/Element/SQL
prototypes, detached `MatrixContext`, broad dependency residue, or optional
backends. Those remain classified legacy/future work rather than being folded
into the performance slice.

`perf` profiling was unavailable on both evidence hosts for different reasons:
the shared GitHub runner denied hardware counters and the owner machine did not
have the `perf` executable installed. R6 did not change host security policy or
install system tooling solely for profiling. Benchmark scaling plus source
inspection independently identified the parent-wide traversal defect.

## Current gate

R6 implementation, semantic regression coverage, two-lane code qualification,
and the authoritative owner-machine before/after measurement are complete.

The detailed record is
[`development/2026-08-29-r6-measure-optimize.md`](development/2026-08-29-r6-measure-optimize.md).

```text
R6: REVIEWABLE — TEAMLEAD/OWNER ACCEPTANCE PENDING
R7: BLOCKED UNTIL R6 IS ACCEPTED AND MERGED
```

No R7 implementation, version bump, tag, release, publication, backend
abstraction, or parallel runtime path is authorized by this R6 candidate.
