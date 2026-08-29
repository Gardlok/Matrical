# Matrical architecture vision

**Status:** accepted rehabilitation direction; R3 owner accepted; R4 transformation contract active

## Product position

Matrical is a semantic matrix-transformation library: a small Rust core for
validated matrix geometry and data, safe borrowing views, transformations,
contextual policy, and bounded metadata/provenance.

It builds on mature dense storage rather than competing with established
numerical libraries at storage layout, BLAS kernels, or general linear algebra.

## The nomenclature contract

### Matrix

The Matrix owns values and a validated two-dimensional `Shape`. It is the source
of truth for indexing and storage invariants.

R2 establishes Matrix as private `ndarray::Array2<T>` storage wrapped by
Matrical's semantic contract. Queue capacity is not shape and the historical
`ArrayQueue<Element<V>>` representation is not a compatibility requirement.

Construction, logical iteration, mutable iteration, and owned conversion use
explicit deterministic row-major semantics. Ordinary invalid index input is
fallible rather than an indexing panic. The backend is not exposed through an
unrestricted mutable reference that could reshape storage behind the validated
contract.

### Shape, Index, and Region

`Shape` proves that `rows * columns` fits in `usize`. Zero-sized shapes are valid,
including `0 x 0`, `0 x N`, and `N x 0`.

`Index` is an independently constructible row/column coordinate. Matrix access
checks `row < rows` and `column < columns` before returning a reference.

`Region` is a checked half-open rectangle:

```text
[start_row, end_row)
[start_column, end_column)
```

End boundaries may equal the Shape dimension. Reversed or out-of-bounds ranges
are typed failures. Empty regions are valid. A Region presented to a Matrix for
Lens construction is revalidated against that receiving Matrix.

### Lens

`Lens<'a, T>` is an immutable rectangular borrowing view over `Matrix<T>`;
`LensMut<'a, T>` is the mutable counterpart:

```text
Matrix<T> owns data
Lens<'a, T> borrows &'a Matrix<T>
LensMut<'a, T> borrows &'a mut Matrix<T>
```

The borrow checker prevents either view from outliving its parent borrow and
prevents a second mutable Lens through the same Matrix while the first remains
live. R3 deliberately accepts this conservative whole-Matrix mutable borrow even
for logically disjoint Regions; it adds no unsafe splitting or runtime overlap
tracking.

A Lens stores the selected Region in parent coordinates but exposes element
access in Lens-local coordinates. Rows and columns are ordinary rectangular
Lenses, including valid empty `1 x 0` or `0 x 1` selections where the parent
Shape permits them.

Lens construction, row/column selection, checked access, and iteration do not
intentionally allocate. Iteration follows logical row-major order within the
selected rectangle. `to_row_major()` is the explicit allocating conversion.

### Gear

A Gear is a transformation applied only to data visible through the Lens it is
given. R4 makes effect authority explicit with separate static traits:

```text
ReadGear<T> -> &Lens<'_, T>
MutGear<T>  -> &mut LensMut<'_, T>
```

A Gear does not normally receive `&Matrix<T>`, `&mut Matrix<T>`, ndarray storage,
or a generic provider from which it can request a broader Region. The
caller-selected Lens is the capability boundary.

Downstream crates can implement Gear traits directly with ordinary Rust. Static
dispatch is the default. No central registry, factory registration, dependency
injection container, `Any` lookup, string operation dispatch, or mandatory boxed
trait object is part of R4.

Built-in deterministic examples include read-only `SumGear` and mutating
`AddScalarGear`, `ScaleGear`, and `ClampGear`.

### Cog

A Cog supplies typed context or policy used by a Gear:

```text
Cog<C> -> Option<C>
```

The Gear's associated `Context` type identifies the required Rust type at
compile time. Central execution resolves the context and returns
`MatricalError::InvalidContext` when it is absent. `ValidateCog` provides a small
ordinary typed validation contract before the Gear runs. Invalid concrete policy
uses typed errors such as `InvalidValue`; no `Any` downcast or string lookup is
needed.

### Tag

A Tag records bounded typed metadata/provenance associated with successful Gear
execution. R4 uses a finite namespace equivalent to source label, typed stage,
and numeric sequence/batch identity.

Tags are inert data. They contain no callbacks, query objects, arbitrary command
maps, or dependency-injection payloads. Source text has one explicit role as a
provenance label and is never interpreted as executable content.

Crucially, Tags are not passed into Gear execution. They are attached to the
successful `ExecutionReport` after the Gear returns, so Tag metadata cannot act
as a hidden transformation command channel.

### Execution report

`ExecutionReport<O>` describes a successful execution without erasing the output
type. It records:

```text
Gear identity
exact selected Region
GearEffect::{ReadOnly, Mutating}
strongly typed output O
ordered provenance Tags
```

Failures remain `Err(MatricalError)` and do not produce fabricated success
reports.

## Layering

```text
matrical-core
  Shape, Index, Region, Matrix, MatricalError

matrical-view
  Lens, LensMut, validated selectors and iterators

matrical-transform
  ReadGear, MutGear, Cog, Tag, ExecutionReport

optional integrations
  parallel execution, serialization, persistence, specialized storage
```

The first rehabilitation release may remain one crate. These are contract
boundaries, not an immediate workspace-split requirement.

## Foundational invariants

- Shape dimensions and total element count agree without overflow.
- Zero-sized Shape/Matrix values are valid and do not require special panic
  paths.
- Every public index and region is validated before access/use.
- Region ordering is half-open and empty-region behavior is explicit.
- Public construction/access failures are typed and do not panic for ordinary
  invalid input.
- Matrix and Lens logical iteration preserve deterministic row-major order.
- A Lens cannot outlive the Matrix borrow it contains.
- A mutable Lens has exclusive access to its parent Matrix for its lifetime.
- A read Gear receives no mutable Lens capability.
- A mutating Gear cannot access values outside its supplied LensMut Region.
- Missing required Cog context is a typed failure.
- Invalid contextual policy is rejected before Gear execution.
- Tag/provenance cannot mutate Matrix data or select an operation by side
  channel.
- Parallel execution, if introduced, preserves accepted sequential semantics
  unless explicitly documented otherwise.

## Storage and missingness

The accepted first dense storage is `ndarray::Array2<T>`, kept as a private
implementation detail behind Matrical-owned invariants.

Lens borrows the checked `Matrix<T>` wrapper and uses Matrix's checked access and
logical iterators. Gear adds no independent ndarray storage and no direct ndarray
slicing contract.

A validity/missingness mask is **not intrinsic Matrix storage**. `Matrix<T>`
represents values and shape. Missingness belongs in an explicit paired
structure, wrapper, or downstream domain type unless a later concrete use case
proves that Matrical core should own that semantic.

Backend abstraction waits until real storage implementations expose a stable
shared need. A premature universal storage trait would repeat the prototype's
largest problem: abstractions arriving before working behavior.

## Advanced Rust and authority

GATs and HRTBs are tools, not design goals. R3 evaluated a GAT-backed lending
provider and deferred it partly because Matrix was the only proven provider.
R4 reassesses the choice using the now-real transformation architecture.

### Design A: consume an already-selected capability

```text
caller: Matrix -> Lens / LensMut
Gear:   receives that Lens / LensMut
```

This directly enforces least authority: the Gear cannot choose a larger Region
than the caller granted.

### Design B: give Gear a lending provider

A public GAT provider could associate `View<'a>` / `ViewMut<'a>` with future
providers and preserve static dispatch. But a provider that can create arbitrary
Regions also gives the Gear selection authority it does not need. Restricting
that provider enough to restore least authority would recreate the already
working Lens boundary through a more complex public abstraction.

R4 therefore defers a public GAT lending-provider trait for an authority-specific
reason: Gear composition actively benefits from receiving the narrower
caller-selected Lens rather than a provider. The current architecture gains no
meaningful downstream reuse, ergonomics, or diagnostic benefit from the broader
surface.

R4 likewise finds no genuine adapter requiring `for<'a> Fn(&Lens<'a, T>)` or an
equivalent HRTB. Gear methods naturally operate over the borrow they receive.
HRTBs remain available when a future concrete adapter needs lifetime-universal
callback behavior.

## Dynamic dispatch

R4 uses static dispatch. An external integration test defines a downstream Gear
and context type without registry or boxing, demonstrating the current extension
requirement. Heterogeneous runtime pipelines can be reconsidered when a real
consumer requires them; R4 does not add `Vec<Box<dyn Gear>>`, a runtime registry,
or string lookup speculatively.

## Concurrency and performance

The initial contract is deterministic and sequential. Thread-safe containers do
not by themselves define safe matrix-level concurrency.

Parallelism remains optional and evidence-driven. Before concurrent mutation is
introduced, Matrical must specify aliasing, partial observation, cancellation,
failure, determinism, and synchronization ownership.

Zero-copy claims identify only borrowing operations. `to_row_major()` explicitly
allocates and clones selected values.

## Error contract

`MatricalError` is a public, non-recursive, inspectable error type implementing
`std::error::Error`. R2 distinguishes shape, construction, index, and region
failures. R4 reuses `InvalidContext` for absent required Cog context and
`InvalidValue` for invalid contextual policy rather than creating an elaborate
new hierarchy.

## Dependency policy

R4 changes no dependency metadata or lockfile. ndarray remains the private dense
storage backend and Crossbeam remains historical non-Matrix residue outside this
slice.

Optional database, serialization, parallelism, and benchmarking capabilities
must earn explicit features and tests later.

## Non-goals for the rehabilitation core

- Reimplementing a complete linear-algebra ecosystem.
- Database-backed matrices in the first functional slices.
- A hidden validity/missingness mask in `Matrix<T>`.
- Lock-free or parallel Matrix/Lens mutation as a default requirement.
- Unsafe disjoint mutable Lens splitting.
- Runtime Gear registries or dependency-injection containers without a concrete
  consumer need.
- Tags as SQL/query/command envelopes.
- Preserving unfinished 0.1.0 behavior as a compatibility contract.
- Using advanced Rust syntax without a measurable correctness, usability, or
  performance benefit.

## Downstream consumers

Concrete consumers may inform acceptance criteria without moving their domain
model into Matrical. The first recorded input remains the
[longitudinal feature-analysis consumer note](consumers/longitudinal-feature-analysis.md).
Application identities, capture semantics, missingness meaning, and domain
interpretation remain downstream responsibilities.
