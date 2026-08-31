# Matrical architecture vision

**Status:** accepted rehabilitation direction through R6; R7-A dense snapshot interchange active

## Product position

Matrical is a semantic matrix-transformation library: a small Rust core for
validated matrix geometry and data, safe borrowing views, typed transformations,
contextual policy, bounded provenance, and explicit inert interchange.

It builds on mature dense storage rather than competing with established
numerical libraries at storage layout, BLAS kernels, general linear algebra, or
persistence engines.

The accepted execution flow is:

```text
Matrix
  -> Lens / LensMut
  -> Gear (+ typed Cog)
  -> ExecutionReport (+ Tags)
```

R7-A adds a separate interchange flow:

```text
live Matrix<T>
  -> MatrixSnapshot<T>
  -> caller-selected serialization/transport/storage
  -> checked Matrix<T>
```

The snapshot path carries inert data. It does not create a live storage provider
or widen Gear execution authority.

## The nomenclature contract

### Matrix

`Matrix<T>` owns values and a validated two-dimensional `Shape`. Construction,
logical iteration, mutable iteration, and owned conversion use deterministic
row-major semantics. The private first backend is `ndarray::Array2<T>`; callers
cannot reshape that storage behind Matrical's checked contract.

### Shape, Index, and Region

`Shape` proves that `rows * columns` fits in `usize`. Zero-sized shapes are valid,
including `0 x 0`, `0 x N`, and `N x 0`.

`Index` is an independently constructible row/column coordinate. Matrix and Lens
access validate it before returning a reference.

`Region` is a checked half-open rectangle. End boundaries may equal the Shape
dimension. Reversed or out-of-bounds ranges are typed failures. Empty Regions
are valid, and a Region presented for Lens construction is revalidated against
the receiving Matrix.

### Lens

`Lens<'a, T>` is an immutable rectangular borrowing view; `LensMut<'a, T>` is the
mutable counterpart. Rust lifetimes prevent a Lens from outliving its Matrix and
give a `LensMut` exclusive mutable borrow of the parent Matrix for its lifetime.

A Lens records the selected Region in parent coordinates but exposes checked
Index access in Lens-local coordinates. Logical iteration is deterministic
row-major within the selected rectangle. `to_row_major()` is the explicit
allocating `T: Clone` conversion.

R6 changed only the private representation: after Region validation,
Lens/LensMut hold an ndarray view of exactly the selected rectangle rather than
filtering parent-wide iteration. ndarray view types remain private.

### Gear, Cog, Tag, and ExecutionReport

A Gear transforms only the data visible through the Lens supplied by its caller.
Read and mutating authority are distinct static contracts. A Gear does not
normally receive Matrix storage or a generic selector/provider from which it can
request a broader Region.

`Cog<C>` carries optional typed context validated before execution. `Tag` is
bounded inert provenance and never becomes an operation selector or command
channel. `ExecutionReport<O>` records successful Gear identity, Region, effect,
typed output, and ordered Tags without erasing the output type.

### MatrixSnapshot

`MatrixSnapshot<T>` is Matrical's specialized dense interchange DTO. Its logical
v1 fields are:

```text
version: u32
rows: u64
columns: u64
row_major: Vec<T>
```

The fields are private. `DENSE_SNAPSHOT_VERSION` is `1`. Fixed-width `u64`
dimensions keep the interchange schema independent of the producer's pointer
width. Reconstruction checks conversion to the receiving platform's `usize`,
then delegates element-count and row-major length validation to `Shape::new` and
`Matrix::from_row_major`.

Snapshot creation makes ownership/copy behavior explicit:

```text
Matrix::snapshot       borrows; O(n) clones T; requires T: Clone
Matrix::into_snapshot  consumes; transfers T; no Clone bound
snapshot.into_matrix   consumes; validates; reconstructs owned dense Matrix
```

`MatrixSnapshot` contains no ndarray type, Lens internals, Gear internals,
database handle, file path, socket, mapped region, background persistence
worker, or other live authority.

## Serialization and transport boundary

Serde support is optional and belongs only to `MatrixSnapshot<T>`. `Matrix<T>`
is not directly serializable and Matrical does not derive serialization for its
private ndarray-backed storage.

The snapshot schema is format-neutral. A chosen Serde format is responsible for
its own representational limits for `T`. JSON is used only for a deterministic
integer fixture and in-memory example; it is not Matrical's storage engine and
is not evidence that arbitrary floating-point values preserve every bit through
JSON.

Matrical decides representation. The caller decides where bytes go. No
`save_json`, `load_json`, filesystem, database, network, environment/config, or
background persistence authority belongs in R7-A.

Generic deserialization is also not a complete hostile-input resource limiter.
Callers accepting untrusted documents must bound transport/format resources such
as size, nesting, and allocation. Matrical's snapshot guarantee is fail-closed
semantic reconstruction, not universal parsing-resource protection.

See [interchange.md](../interchange.md).

## Public learning surface

The supported discovery policy is:

```text
matrical::prelude::*
  recommended everyday Matrix/Lens/Gear API

matrical::{Shape, Matrix, Lens, ReadGear, ...}
  named supported and discoverable crate-root API

matrical::schematics
  core geometry/storage organization

matrical::strategies
  Lens, Gear, Cog, Tag, and reporting organization

matrical::snapshot / matrical::MatrixSnapshot
  specialized explicit interchange API
```

`MatrixSnapshot` is intentionally excluded from the prelude so serialization and
integration concerns do not become part of every caller's everyday import set.
Historical operation scaffolding may remain source-accessible and `doc(hidden)`
during 0.1.0 rehabilitation but is not part of the learning contract.

## Layering

```text
matrical-core
  Shape, Index, Region, Matrix, MatricalError

matrical-view
  Lens, LensMut, validated selectors and iterators

matrical-transform
  ReadGear, MutGear, Cog, ValidateCog, Tag, ExecutionReport

matrical-interchange
  MatrixSnapshot, dense schema version, checked reconstruction

development measurement
  Criterion benchmark harnesses; no runtime authority

optional later integrations
  real consumer adapters, sparse/mapped storage, measured parallelism
```

These are semantic boundaries within the current crate, not a requirement to
split the workspace.

## Foundational invariants

- Shape dimensions and total element count agree without overflow.
- Zero-sized Shape/Matrix values are valid.
- Every public index and Region is validated before access/use.
- Matrix and Lens logical iteration preserve deterministic row-major order.
- A Lens cannot outlive the Matrix storage it borrows.
- A mutable Lens has exclusive mutable access to its parent Matrix for its
  lifetime.
- A Gear cannot escape the exact caller-selected Lens/LensMut capability.
- Tag/provenance cannot mutate Matrix data or select execution by side channel.
- Dense snapshot v1 has an explicit version and fixed field semantics.
- Snapshot dimensions are converted without truncation.
- Snapshot reconstruction reuses Matrix/Shape invariants and fails closed on
  unsupported versions or malformed shape/value relationships.
- A snapshot is inert representation, not live storage/execution authority.
- Performance optimization must not expose the private ndarray backend or weaken
  checked semantics.

## Backend abstraction and advanced Rust

Backend abstraction waits until multiple real live implementations expose a
stable shared need. A snapshot is not such an implementation: it is an inert DTO.
R7-A therefore does not add `MatrixBackend`, `StorageBackend`, lending providers,
GAT/HRTB factories, generic Lens providers, sparse storage, or mapped storage.

GATs and HRTBs remain tools rather than design goals. R3 found concrete
lifetime-generic Lenses clearer with one provider; R4 added a least-authority
reason not to give Gears a generic selector/provider. A future abstraction must
solve a concrete second-provider composability problem without weakening that
boundary.

## Future Strustegy integration

Matrical core must remain independently useful. Strustegy may eventually
orchestrate Matrical through public APIs, and `MatrixSnapshot` provides inert data
an adapter may transport or store. Such an adapter should live in Strustegy, a
dedicated integration crate, or another explicit opt-in boundary.

Matrical does not depend on Strustegy in R7-A and does not invent placeholder
Strustegy traits. Any future orchestration still leaves Gear execution bounded by
the caller-selected Lens/LensMut.

## Concurrency and performance

The accepted execution contract remains deterministic and sequential. R6's
private checked-view repair removed parent-wide Lens scans and measured dense
traversal approximately at direct-ndarray speed on the owner evidence host.
Rayon remains deferred because no measured R6 workload justified the extra
runtime/concurrency surface.

Snapshot creation/reconstruction is O(Matrix elements) when values must be
cloned or transferred. The borrowed snapshot path is explicitly not zero-copy.
No new R7-A Criterion benchmark is required because the slice does not change R6
traversal mechanics.

See [performance.md](../performance.md) for R6 methodology and measurements.

## Error contract

`MatricalError` is public, structural, non-recursive, and implements
`std::error::Error`. R7-A adds structural unsupported-snapshot-version and
snapshot-dimension-range failures while reusing the existing shape-overflow and
row-major-length variants. New interchange failures are not collapsed into
free-form `Custom(String)`.

## Dependency policy

ndarray remains the private dense storage backend. Crossbeam remains historical
non-Matrix residue. Criterion remains development-only benchmark infrastructure.

R7-A adds exact Serde 1.0.229 as an optional normal dependency and exact
serde_json 1.0.151 as a development/example-only dependency. Serde/serde_json
may already be present in `Cargo.lock` through Criterion; lockfile presence is
not the same as activation in Matrical's default normal runtime dependency graph.
Default versus `serde`-enabled normal dependency trees are qualified separately.

## Non-goals for the rehabilitation core

- Reimplementing a complete linear-algebra ecosystem.
- Persistence engines or hidden I/O authority in the snapshot API.
- Sparse/mapped Matrix storage in R7-A.
- A backend/provider trait with only one live backend.
- Unsafe disjoint mutable Lens splitting.
- Giving a Gear arbitrary Region-selection authority.
- Runtime Gear registries or DI containers without a concrete consumer need.
- Direct Strustegy dependency inside Matrical core.
- Treating one serialization format as universally faithful for every `T`.
- Treating generic deserialization as universal resource-exhaustion protection.
- Using advanced Rust syntax or concurrency machinery without concrete benefit.

## Downstream consumers

Concrete consumers may inform acceptance criteria without moving their domain
model into Matrical. Application identity, capture semantics, missingness,
persistence, transport, and domain interpretation remain downstream
responsibilities unless a later bounded slice explicitly accepts them.
