# R7-A — versioned dense snapshot interchange

**Date:** 2026-08-30 / qualification completed on the final PR head

## Baseline

R7-A is based exactly on the owner-accepted R6 merge:

```text
branch  main
commit  6be8b0ce910d66d784cc5e5ca2d52a59f1cd3773
tree    919f8f800f1ffa3b4750def03f803a807ff25179
version 0.1.0
MSRV    Rust 1.85.0
```

R6 accepted `Cargo.lock` SHA-256:

```text
b835d1e7d4d851e883a209e2cc41b99aeb8982f70a73821f569e7f0ef98ae62a
```

R7-A branch:

```text
rehab/r7a-versioned-snapshot
```

## Mission boundary

R7-A establishes an inert, Matrical-owned dense interchange representation. It
does not add a live storage backend, persistence engine, sparse/mapped Matrix,
backend/provider trait, GAT/HRTB provider, Strustegy dependency, or release
qualification.

The boundary is:

```text
Matrix<T>
  -> MatrixSnapshot<T>
  -> optional caller-selected Serde format/transport
  -> checked Matrix<T>
```

Matrical owns representation and semantic reconstruction. The caller owns file,
database, network, and other byte transport/storage decisions.

## Public snapshot contract

R7-A adds:

```rust
pub const DENSE_SNAPSHOT_VERSION: u32 = 1;

pub struct MatrixSnapshot<T> {
    version: u32,
    rows: u64,
    columns: u64,
    row_major: Vec<T>,
}
```

The fields are private. Public read-only accessors expose version, rows, columns,
length/emptiness, and borrowed row-major values; a consuming accessor returns the
row-major values.

`MatrixSnapshot` is exported from the crate root and its `snapshot` module, but
is intentionally excluded from `matrical::prelude::*`. Interchange is a
specialized integration concern rather than part of the everyday
Matrix/Lens/Gear import set.

The representation contains no ndarray types.

## Construction and reconstruction

Borrowed snapshot creation:

```text
Matrix::snapshot(&self) -> MatrixSnapshot<T>
T: Clone
O(n) value clone
```

Consuming creation:

```text
Matrix::into_snapshot(self) -> MatrixSnapshot<T>
no T: Clone bound
owned values transferred
```

Checked reconstruction:

```text
MatrixSnapshot::into_matrix(self) -> Result<Matrix<T>, MatricalError>
TryFrom<MatrixSnapshot<T>> for Matrix<T>
```

Reconstruction checks the schema version, converts `u64` dimensions to the
receiving platform's `usize` without truncation, and then reuses:

```text
Shape::new
Matrix::from_row_major
```

Thus shape element-count overflow and row-major length mismatch continue to use
the established Matrix invariants rather than a second independent validator.

## Error model

R7-A adds the structural variants:

```text
UnsupportedSnapshotVersion { found, supported }
SnapshotDimensionOutOfRange { rows, columns }
```

Existing `ShapeElementCountOverflow` and `RowMajorLengthMismatch` remain the
errors for those existing invariant failures. Interchange failures are not
collapsed into `Custom(String)`.

## Serde boundary

Cargo features:

```toml
[features]
default = []
serde = ["dep:serde"]
```

Optional normal dependency:

```toml
serde = { version = "=1.0.229", features = ["derive"], optional = true }
```

Development/example-only dependency:

```toml
serde_json = "=1.0.151"
```

Serde derives apply to `MatrixSnapshot<T>`, not `Matrix<T>` or ndarray storage.
Dense v1 deserialization uses deny-unknown-fields behavior. Semantic
reconstruction still validates version, dimensions, shape size, and value count
after deserialization.

`serde_json` is used to prove interoperability and maintain a deterministic
integer fixture. Matrical adds no JSON/file save/load API.

## Dense schema v1 fixture

Committed fixture:

```text
tests/fixtures/r7_dense_snapshot_v1.json
```

Logical content:

```json
{
  "version": 1,
  "rows": 2,
  "columns": 3,
  "row_major": [1, 2, 3, 4, 5, 6]
}
```

Tests deserialize the fixture, reconstruct the checked Matrix, verify shape and
row-major values, serialize a matching snapshot, and compare JSON values
semantically rather than depending on whitespace or object-key formatting.

## Malformed-input and ownership coverage

R7-A tests prove:

- version 2 deserializes as data but fails reconstruction with
  `UnsupportedSnapshotVersion` when this reader supports v1;
- a 2 x 3 snapshot carrying two values fails with
  `RowMajorLengthMismatch { expected: 6, actual: 2 }`;
- oversized dimensions are checked without truncation: 32-bit targets exercise
  `SnapshotDimensionOutOfRange`; 64-bit targets exercise the existing
  element-count-overflow path for `u64::MAX x 2`;
- unknown v1 fields are rejected by Serde;
- `0 x 0`, `0 x N`, and `N x 0` with zero values roundtrip;
- `Matrix<NonClone> -> into_snapshot() -> into_matrix()` works, proving the
  consuming path transfers ownership without accidentally requiring `Clone`.

## Format and untrusted-input caveats

`MatrixSnapshot<T>` is format-neutral. Serde participation does not imply every
selected format can faithfully represent every possible `T`. The canonical JSON
fixture deliberately uses integers and does not establish arbitrary
floating-point bit-preservation claims.

Generic deserialization is not a transport-level resource limiter. Callers
accepting untrusted documents must bound size, nesting, allocation, and related
resource use at the transport/format layer. R7-A's fail-closed guarantee is
semantic Matrix reconstruction, not universal parsing-resource protection.

## Future storage and integration boundary

Dense snapshot v1 is not a sparse or mapped backend. R7-A does not add
`MatrixBackend`, `StorageBackend`, a lending provider, or another generalized
live-storage abstraction because Matrical still has one accepted live Matrix
provider.

Matrical core remains independently useful and has no Strustegy dependency. A
future Strustegy adapter may use public Matrical APIs and transport/store
`MatrixSnapshot` as inert data, but should live in Strustegy, a dedicated
integration crate, or another explicit opt-in boundary. Gear authority remains
bounded to caller-selected Lens/LensMut capabilities.

## CI contract

The qualification workflow continues to test both Rust 1.85.0 and stable. Each
lane retains the default-feature contract and adds the all-feature contract:

```text
cargo check --locked --all-targets
cargo test --locked --all-targets
cargo test --locked --doc
cargo clippy --locked --all-targets
cargo doc --locked --no-deps

cargo check --locked --all-targets --all-features
cargo test --locked --all-targets --all-features
cargo test --locked --doc --all-features
cargo clippy --locked --all-targets --all-features
cargo doc --locked --no-deps --all-features

cargo bench --locked --no-run
```

The workflow also records exact Rust/Cargo versions, `Cargo.lock` SHA-256,
default and serde-enabled normal dependency trees, and runs the R7-A mechanical
scope/whitespace/link/final-newline/unsafe/tracked-target audit.

## Lockfile behavior

The accepted R6 lock already contained exact Serde 1.0.229 and serde_json 1.0.151
packages transitively through Criterion. R7-A therefore does not perform a broad
dependency update or add unrelated package versions. The Matrical package entry
changes to record its new optional Serde and development serde_json dependency
edges.

This distinction is deliberate:

```text
present in Cargo.lock
!=
activated in Matrical's default normal runtime dependency graph
```

The default and `serde`-enabled normal Cargo trees are recorded in CI evidence.

## Performance

R7-A does not change R6 Lens/LensMut traversal implementation. Existing R6
benchmarks remain part of compilation qualification.

Snapshot complexity is explicit:

```text
snapshot(&Matrix)     O(n) clone
into_snapshot(Matrix) ownership transfer; no Clone requirement
into_matrix(snapshot) checked owned dense reconstruction
```

No new Criterion benchmark is introduced because R7-A does not present a new
traversal optimization question.

## Qualification evidence

Exact final-head GitHub CI supplies the authoritative Rust 1.85.0/stable default,
all-feature, benchmark-compile, dependency-tree, lock-hash, and mechanical
evidence for the review candidate. The PR handoff records those exact results.

R7-B remains blocked until R7-A is accepted and merged. R8 remains blocked until
R7 completes.
