# Matrical architecture vision

**Status:** accepted rehabilitation direction; R2 core accepted; R3 borrowing-view contract active

## Product position

Matrical is a semantic matrix-transformation library: a small Rust core for
validated matrix geometry and data, followed by safe borrowing views,
transformations, contextual policy, and bounded metadata/provenance.

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
Lens construction is revalidated against that receiving Matrix, even if the
Region was valid for another Shape.

### Lens

`Lens<'a, T>` is an immutable rectangular borrowing view over `Matrix<T>`;
`LensMut<'a, T>` is the mutable counterpart. R3 represents them by borrowing the
checked parent Matrix directly rather than exposing ndarray views publicly:

```text
Matrix<T> owns data
Lens<'a, T> borrows &'a Matrix<T>
LensMut<'a, T> borrows &'a mut Matrix<T>
```

The borrow checker prevents either view from outliving its parent borrow and
prevents a second mutable Lens through the same Matrix while the first remains
live. R3 deliberately accepts this conservative whole-Matrix mutable borrow even
for logically disjoint Regions; it does not add unsafe splitting or runtime
overlap tracking.

A Lens stores the selected Region in parent coordinates but exposes element
access in Lens-local coordinates. If a Region selects rows `2..4` and columns
`3..6`, Lens-local `Index::new(0, 0)` refers to parent coordinate `(2, 3)`.
Local out-of-bounds access returns `MatricalError::IndexOutOfBounds`.

Rows and columns are ordinary rectangular Lenses. An in-range row of an `N x 0`
Matrix is a valid empty `1 x 0` Lens; an in-range column of a `0 x N` Matrix is a
valid empty `0 x 1` Lens. `0 x 0` has neither a valid row nor a valid column.

Lens construction, row/column selection, checked access, and iteration do not
intentionally allocate. Iteration follows logical row-major order within the
selected rectangle and makes no physical-contiguity promise. `to_row_major()` is
the explicit allocating conversion and requires `T: Clone`.

### Gear

A Gear is a transformation applied to data visible through a compatible Lens.
It must declare relevant effects/capabilities and must not bypass Lens bounds.

### Cog

A Cog supplies typed context or policy used by a Gear. Missing required context
is a typed failure, not an unwrap panic.

### Tag

A Tag records typed metadata or provenance about a Matrix, Lens, Gear execution,
or result. Tags must not become an unbounded string bag or alternate command
channel.

## Layering

```text
matrical-core
  Shape, Index, Region, Matrix, MatricalError

matrical-view
  Lens, LensMut, validated selectors and iterators

matrical-transform
  Gear, Cog, Tag, execution reports

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
- A Gear cannot access values outside its Lens.
- Metadata cannot mutate matrix data by an undocumented side channel.
- Parallel execution, if introduced, preserves accepted sequential semantics
  unless explicitly documented otherwise.

## Storage and missingness

The accepted first dense storage is `ndarray::Array2<T>`, kept as a private
implementation detail behind Matrical-owned invariants.

R3 does not broaden backend visibility. Its current Lens representation borrows
the checked `Matrix<T>` wrapper and uses Matrix's checked access and logical
iterators. This keeps the backend private while preserving zero-copy borrowing.
A later performance phase may compare an internal ndarray-view representation if
measurement demonstrates a need, without changing the public Lens contract.

A validity/missingness mask is **not intrinsic Matrix storage**. `Matrix<T>`
represents values and shape. Missingness belongs in an explicit paired
structure, wrapper, or downstream domain type unless a later concrete use case
proves that Matrical core should own that semantic.

Backend abstraction waits until at least two real storage implementations expose
a stable shared need. A premature universal storage trait would repeat the
prototype's largest problem: abstractions arriving before working behavior.

## Advanced Rust and borrowing

GATs and HRTBs are tools, not design goals. R3 evaluated both concrete
lifetime-generic borrowing views and a GAT-backed lending abstraction.

The concrete design keeps ownership visible at the call site:

```rust
impl<T> Matrix<T> {
    pub fn lens(&self, region: Region) -> Result<Lens<'_, T>, MatricalError>;
    pub fn lens_mut(&mut self, region: Region) -> Result<LensMut<'_, T>, MatricalError>;
}
```

A GAT-backed `LendingView` trait could associate `View<'a>` and `ViewMut<'a>`
with future providers while retaining static dispatch. Today Matrix is the only
proven provider. The trait would not strengthen the borrow checker guarantees,
would not simplify current callers, and would make Rust 1.85 diagnostics and API
surface more complex. R3 therefore keeps the concrete design and defers a public
GAT abstraction until multiple implementations or R4 Gear composition provides
a real need.

Compile-fail rustdoc examples cover Lens escape and simultaneous mutable Lens
misuse, so the decision is grounded in the actual ownership surface rather than
syntax preference.

## Concurrency and performance

The initial contract is deterministic and sequential. Thread-safe containers do
not by themselves define safe matrix-level concurrency.

Parallelism remains optional and evidence-driven. Before concurrent mutation is
introduced, Matrical must specify aliasing, partial-observation, cancellation,
failure, determinism, and synchronization ownership.

Zero-copy claims identify only borrowing operations. `to_row_major()` explicitly
allocates and clones selected values; it is not described as zero-copy.

## Error contract

`MatricalError` is a public, non-recursive, inspectable error type implementing
`std::error::Error`. R2 distinguishes shape element-count overflow, exact
row-major length mismatch, index bounds, reversed regions, and out-of-bounds
regions while retaining historical variants still used by compiled prototype
modules. R3 reuses `RegionReversed`, `RegionOutOfBounds`, and `IndexOutOfBounds`
rather than adding Lens-specific error variants.

## Dependency policy

Every normal dependency must serve implemented behavior. R3 changes no dependency
metadata or lockfile. ndarray remains the private dense-storage backend and
Crossbeam remains historical non-Matrix residue outside this slice.

Optional database, serialization, parallelism, and benchmarking capabilities
must earn explicit features and tests later.

## Non-goals for the rehabilitation core

- Reimplementing a complete linear-algebra ecosystem.
- Database-backed matrices in the first functional slices.
- A hidden validity/missingness mask in `Matrix<T>`.
- Lock-free or parallel Matrix/Lens mutation as a default requirement.
- Unsafe disjoint mutable Lens splitting during R3.
- Preserving unfinished 0.1.0 behavior as a compatibility contract.
- Using advanced Rust syntax without a measurable correctness, usability, or
  performance benefit.

## Downstream consumers

Concrete consumers may inform acceptance criteria without moving their domain
model into Matrical. The first recorded input remains the
[longitudinal feature-analysis consumer note](consumers/longitudinal-feature-analysis.md).
Application identities, capture semantics, missingness meaning, and domain
interpretation remain downstream responsibilities.
