# R2 checked dense Matrix core

**Implementation:** 2026-08-28

**Status:** R2 COMPLETE — TEAMLEAD/OWNER ACCEPTANCE PENDING

## Exact starting identity

```text
repository Gardlok/Matrical
branch     main
commit     059f148a99cfe2b5b881ada9af9acc286f584b6a
tree       0ad0e91d983912f743dc16972eb11a5a79afb286
version    0.1.0
```

This is the verified owner-accepted merge of PR #6. R1-D is merged and R1 is
complete.

Implementation branch:

```text
rehab/r2-core-invariants
```

Review PR:

```text
#7 — R2: establish checked dense Matrix core
```

## Historical problem

The prototype `Matrix<V>` stored an `ArrayQueue<Element<V>>` and treated queue
capacity as the only sizing concept. It therefore did not represent a validated
two-dimensional dense Matrix.

R2 removes queue-backed Matrix storage and makes Matrical's accepted ndarray
storage direction real. Historical `MatrixContext` remains only because an
unrelated operation trait still references it; it is detached legacy
scaffolding and is not Matrix storage or part of the new shape invariant.

## Public core contract

### MatricalError

`matrical::MatricalError` and `matrical::MatricalErrorType` are intentionally
crate-root exports. `MatricalError` implements `std::error::Error` and retains
historical variants still used by compiled prototype code.

R2 adds inspectable typed variants for:

- shape element-count overflow;
- row-major data length mismatch;
- reversed region bounds;
- out-of-bounds region geometry.

The historical unit `IndexOutOfBounds` variant is retained so existing R1-era
call sites remain source-compatible while new Matrix access uses it as the typed
bounds result.

### Shape

`Shape::new(rows, columns)` proves `rows * columns` with `checked_mul` and stores
the checked total element count. It exposes rows, columns, length, and empty
state.

Zero-sized shapes are valid:

```text
0 x 0
0 x N
N x 0
```

`usize` multiplication overflow is rejected before Matrix construction.

### Index

`Index` is a cheap copyable row/column coordinate and may be constructed
independently. Matrix access checks both dimensions and returns a typed bounds
error rather than relying on Rust indexing panics.

### Region

`Region` uses half-open bounds:

```text
[start_row, end_row)
[start_column, end_column)
```

Construction validates against a `Shape`. End boundaries may equal the shape
dimension. Reversed or out-of-bounds ranges are rejected with structured errors.
Empty regions are valid and represented explicitly. Matrix can both establish a
Region against its own Shape and revalidate an existing Region before future
Lens use.

## Matrix storage and API

`Matrix<T>` owns private `ndarray::Array2<T>` storage plus its checked `Shape`.
There is no unrestricted mutable ndarray backend reference, so callers cannot
reshape storage behind Matrical's invariant.

Construction is explicit:

```text
Matrix::from_row_major(shape, values)
```

The input length must equal `shape.len()` exactly. Short and long inputs are
both typed failures; there is no truncation or padding.

Geometry exposes shape, rows, columns, length, and empty state. Immutable and
mutable element access both use checked `Index`. `iter()` and `iter_mut()` have a
deterministic logical row-major contract. `into_row_major()` consumes the Matrix
and returns values in the same construction/iteration order.

The new Matrix core is deterministic and sequential and introduces no
concurrency machinery.

## Validity/missingness decision

A validity/missingness mask is not intrinsic Matrix storage in the first core.
`Matrix<T>` represents values and shape. Missingness belongs in an explicit
paired structure, wrapper, or downstream domain type unless a later concrete
use case proves it belongs in Matrical core.

No hidden mask is added.

## GAT/HRTB decision and R3 handoff

R2 does not force GATs or HRTBs into the owned Matrix core because there is no
demonstrated ownership/lending problem that requires them.

R3 must explicitly compare a GAT-backed Lens/lending-view design with a simpler
lifetime-generic design and choose the more meaningful correctness/usability
contract. A conceptual `type View<'a>` lending shape remains design input only;
R2 does not implement Lens or a lending-view trait.

## Coverage

R2 unit tests cover:

- ordinary Shape dimensions;
- `0 x 0`, `0 x N`, and `N x 0`;
- Shape multiplication overflow;
- exact row-major Matrix construction;
- too-short and too-long construction input;
- zero-sized Matrix construction;
- first and last element access;
- row and column out-of-bounds access;
- safe access failure on an empty Matrix;
- valid mutation and invalid mutable access;
- deterministic immutable and mutable row-major iteration;
- normal/full/end-equal/empty Regions;
- reversed row and column Regions;
- row and column region overflow;
- Matrix Region establishment/revalidation;
- owned row-major round trip.

`tests/r2_core_api.rs` consumes only intended public exports and proves Shape,
Matrix construction, immutable/mutable Index access, Region use, deterministic
iteration, and `MatricalError` handling from an external-crate perspective.

`examples/r2_core_matrix.rs` is a runnable public-API example using normal
`Result` propagation and is compiled by the all-targets gate.

The public Matrix rustdoc contains a compiled construction/access/iteration
example, so R2 no longer has zero core documentation examples.

## Qualification environment

The developer execution sandbox used for this session does not contain Rust and
has no outbound network access, so local Cargo/rustfmt execution was not
available. This is an execution-environment limitation, not a repository
finding. Mechanical source/scope checks were performed directly on the
reconstructed files, and the repository's existing GitHub Qualification
workflow supplied the executable Rust gates for both required lanes.

The code-complete candidate at
`d679a045ab6eb0d8a07262c9342a789f0aafc1f3` passed workflow run
`33202151658` on both lanes.

### Rust 1.85.0

Observed toolchain:

```text
rustc 1.85.0 (4d91de4e4 2025-02-17)
```

The workflow does not emit `cargo --version`; Cargo from the installed 1.85.0
toolchain successfully executed every locked command.

Results:

```text
cargo check --locked --all-targets  PASS
cargo test --locked --all-targets   PASS
  library tests                     51 passed, 0 failed
  external integration test          1 passed, 0 failed
  example target                     compiled; 0 tests
cargo test --locked --doc           PASS — 1 passed, 0 failed
cargo clippy --locked --all-targets PASS
cargo doc --locked --no-deps        PASS
```

### Stable

Observed toolchain:

```text
rustc 1.98.0 (88d9e12ae 2026-08-18)
```

The workflow does not emit `cargo --version`; Cargo from that installed stable
toolchain successfully executed every locked command.

Results:

```text
cargo check --locked --all-targets  PASS
cargo test --locked --all-targets   PASS
cargo test --locked --doc           PASS
cargo clippy --locked --all-targets PASS
cargo doc --locked --no-deps        PASS
```

The inherited warning surface remains visible and is intentionally not converted
into an unrelated cleanup campaign. No avoidable new R2-core Clippy failure
remains. The first CI attempt exposed one Clippy-only `reversed_empty_ranges`
diagnostic in the boundary test's deliberate literal reversed range syntax; the
test now constructs those invalid bounds explicitly, preserving the same Region
contract while passing Clippy.

After this evidence-only documentation closeout, the same two-lane workflow must
remain green on the final PR head before Teamlead handoff.

## Reproducibility

`Cargo.toml`, `Cargo.lock`, and `rust-toolchain.toml` are outside R2 modification
scope and remain unchanged.

Expected and preserved lockfile SHA-256:

```text
8abe052c6d793e87df19c1e6ade379caf3cad562eea693a946dc39c9e7180020
```

Because the R2 commit trees reuse the accepted base tree and never replace the
`Cargo.lock` blob, the lockfile is byte-identical by construction.

## Unsafe and Miri evaluation

The new owned Matrix core contains no `unsafe` code and introduces no borrowing
view, aliasing machinery, raw pointer, or custom allocator behavior.

Miri was evaluated as low-value for this specific owned-storage slice. No new
toolchain ceremony is justified solely to exercise safe owned ndarray wrapping.
Miri/deeper aliasing checks should be reconsidered in R3 when borrowed mutable
Lens semantics create the first meaningful aliasing/lifetime surface.

## Residual historical debt

R2 deliberately leaves inherited warning/formatting debt and unreconstructed
Lens, Gear, Cog, Tag, Vector, validation, and operation scaffolding outside this
slice. Crossbeam remains in the dependency graph because historical compiled
non-Matrix structures still use it, but Matrix itself no longer uses Crossbeam
storage.

`MatrixContext` remains as detached compatibility scaffolding only. Its fields
are not exposed as Matrix shape or storage state.

## R2 exit evaluation

The R2 exit contract is satisfied by the code-complete candidate and its
successful two-lane executable qualification:

- ordinary invalid shape, index, region, and length input are typed failures:
  PASS;
- zero-sized and overflow boundaries are explicit and tested: PASS;
- downstream-style public API test, runnable example, and compiled rustdoc are
  present: PASS;
- queue-backed placeholder storage has been removed from Matrix: PASS;
- the new core contains no unsafe/aliasing mechanism requiring Miri in R2: PASS.

No technical R2 blocker remains. Final acceptance and merge remain Teamlead and
owner gates.

Recommended state:

```text
R2: COMPLETE — TEAMLEAD/OWNER ACCEPTANCE PENDING
Next phase after merge: R3 — make Lens real
```
