# Matrical architecture vision

**Status:** accepted rehabilitation direction; R2 core contract active

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
are typed failures. Empty regions are valid. R3 Lens will consume this contract.

### Lens

A Lens selects or views part of a Matrix without taking ownership of selected
values. Initial Lenses should cover a validated rectangular `Region`; row,
column, diagonal, band, triangular, and sparse selections can follow only when
semantics are justified.

Immutable and mutable Lenses must make aliasing and borrowing explicit. A Lens
must never outlive its Matrix.

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
- Matrix iteration and owned conversion preserve deterministic logical row-major
  order.
- A mutable Lens has exclusive access to its selected storage for its lifetime.
- A Gear cannot access values outside its Lens.
- Metadata cannot mutate matrix data by an undocumented side channel.
- Parallel execution, if introduced, preserves accepted sequential semantics
  unless explicitly documented otherwise.

## Storage and missingness

The accepted first dense storage is `ndarray::Array2<T>`, kept as a private
implementation detail behind Matrical-owned invariants.

A validity/missingness mask is **not intrinsic Matrix storage**. `Matrix<T>`
represents values and shape. Missingness belongs in an explicit paired
structure, wrapper, or downstream domain type unless a later concrete use case
proves that Matrical core should own that semantic.

Backend abstraction waits until at least two real storage implementations expose
a stable shared need. A premature universal storage trait would repeat the
prototype's largest problem: abstractions arriving before working behavior.

## Advanced Rust and borrowing

GATs and HRTBs are tools, not design goals. R2 has no demonstrated need for them
in owned dense storage and therefore does not force them into Matrix.

R3 must compare a GAT-backed lending-view API with a simpler lifetime-generic
Lens design. A conceptual probe is:

```rust
trait LendingView {
    type View<'a>
    where
        Self: 'a;

    fn view<'a>(&'a self) -> Self::View<'a>;
}
```

The design should be adopted only if it materially improves correctness or
usability and is supported by compile-time examples/tests.

## Concurrency and performance

The initial contract is deterministic and sequential. Thread-safe containers do
not by themselves define safe matrix-level concurrency.

Parallelism remains optional and evidence-driven. Before concurrent mutation is
introduced, Matrical must specify aliasing, partial-observation, cancellation,
failure, determinism, and synchronization ownership.

Zero-copy claims must identify exactly what is borrowed, what may allocate, and
how long the view remains valid.

## Error contract

`MatricalError` is a public, non-recursive, inspectable error type implementing
`std::error::Error`. R2 distinguishes shape element-count overflow, exact
row-major length mismatch, index bounds, reversed regions, and out-of-bounds
regions while retaining historical variants still used by compiled prototype
modules.

## Dependency policy

Every normal dependency must serve implemented behavior. R2 uses the already
accepted ndarray dependency and does not change dependency metadata or the
committed lockfile. Crossbeam may remain for historical non-Matrix code but is
not Matrix storage after R2.

Optional database, serialization, parallelism, and benchmarking capabilities
must earn explicit features and tests later.

## Non-goals for the rehabilitation core

- Reimplementing a complete linear-algebra ecosystem.
- Database-backed matrices in the first functional slice.
- A hidden validity/missingness mask in `Matrix<T>`.
- Lock-free or parallel Matrix mutation as a default requirement.
- Lens implementation during R2.
- Preserving unfinished 0.1.0 behavior as a compatibility contract.
- Using advanced Rust syntax without a measurable correctness, usability, or
  performance benefit.

## Downstream consumers

Concrete consumers may inform acceptance criteria without moving their domain
model into Matrical. The first recorded input remains the
[longitudinal feature-analysis consumer note](consumers/longitudinal-feature-analysis.md).
Application identities, capture semantics, missingness meaning, and domain
interpretation remain downstream responsibilities.
