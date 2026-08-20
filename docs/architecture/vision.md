# Matrical architecture vision

**Status:** proposed rehabilitation direction

## Product position

Matrical should become a semantic matrix-transformation library: a small Rust
core for selecting validated matrix regions, applying reusable transformations,
supplying contextual policy, and recording metadata or provenance.

It should initially build on a proven dense storage implementation rather than
compete with established numerical libraries at storage layout, BLAS kernels,
or general linear algebra.

## The nomenclature contract

### Matrix

The Matrix owns elements and a validated two-dimensional shape. It is the source
of truth for indexing and storage invariants.

A Matrix must not confuse queue capacity with shape. It must provide deliberate
construction, indexing, immutable access, mutable access, and iteration
semantics.

### Lens

A Lens selects or views part of a Matrix without taking ownership of the selected
elements. Initial Lenses should cover a validated rectangular region; row,
column, diagonal, band, triangular, and sparse selections can follow when their
semantics are justified.

Immutable and mutable Lenses must make aliasing and borrowing explicit. A Lens
must never outlive its Matrix.

### Gear

A Gear is a transformation applied to data visible through a compatible Lens.
Examples may include fill, map, scale, clamp, normalize, or copy operations.

A Gear must declare whether it reads, mutates, allocates, or requires a
particular element capability. It must not bypass Lens bounds.

### Cog

A Cog supplies typed context or policy used by a Gear. Examples may include
validation rules, operation parameters, execution policy, or domain context.

A missing required Cog should be a construction or execution error, not an
`Option::unwrap()` panic.

### Tag

A Tag records typed metadata or provenance about a Matrix, Lens, Gear execution,
or result. Tags should not become an unbounded string bag or an alternate command
channel.

## Proposed layers

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

The first rehabilitation release may remain one crate. These layers are
contract boundaries, not a commitment to an immediate workspace split.

## Foundational invariants

- Shape dimensions and total element count agree without overflow.
- Every public index and region is validated before access.
- Region ordering is explicit; reversed or empty regions have documented
  behavior.
- A mutable Lens has exclusive access to its selected storage for its lifetime.
- A Gear cannot access elements outside its Lens.
- Public fallible operations return a typed error and do not panic for ordinary
  invalid input.
- Metadata cannot mutate matrix data by an undocumented side channel.
- Parallel execution, when introduced, produces behavior equivalent to the
  accepted sequential contract unless explicitly documented otherwise.

## Storage strategy

The recommended first storage is `ndarray::Array2<T>` or a comparably mature
dense representation. A Matrical-owned wrapper should enforce the semantic
contract without exposing historical queue-based storage as a compatibility
requirement.

Backend abstraction should wait until two real storage implementations expose a
stable shared need. A premature universal storage trait would repeat the
prototype's largest problem: abstractions arriving before working behavior.

If lending views across backends later require generic associated types, GATs
may be appropriate. They should be introduced only with a concrete API example,
compile-time tests, and a simpler alternative comparison.

## Concurrency and performance

The initial contract is deterministic and sequential.

Parallelism should be optional, feature-gated, and justified by benchmark data.
Thread-safe containers do not by themselves define atomic multi-element
operations. Before concurrent mutation is introduced, Matrical must specify:

- aliasing rules;
- observation of partially completed transformations;
- cancellation and failure behavior;
- determinism requirements;
- synchronization ownership.

Zero-copy claims must identify exactly what is borrowed, what may allocate, and
how long the view remains valid.

## Error contract

`MatricalError` should be a public, non-recursive, testable error type. Errors
should distinguish at least:

- invalid or overflowing shape;
- index out of bounds;
- invalid region ordering or bounds;
- incompatible Gear or Lens;
- missing Cog or required context;
- unsupported operation;
- backend or integration failure when optional integrations exist.

## Dependency policy

Every normal dependency must serve implemented behavior. Database, concurrency,
serialization, parallelism, and benchmarking dependencies should not remain in
the default graph as architectural placeholders.

Optional capabilities should use explicit Cargo features with tests for useful
feature combinations.

## Non-goals for the rehabilitation core

- Reimplementing a complete linear-algebra ecosystem.
- Database-backed matrices in the first functional slice.
- Lock-free mutation as a default requirement.
- Runtime strategy objects where static dispatch is clearer.
- Preserving unfinished 0.1.0 behavior as a compatibility contract.
- Using advanced Rust syntax without a measurable correctness, usability, or
  performance benefit.

## Downstream consumers

Concrete consumers may inform acceptance criteria without moving their domain
model into Matrical. The first recorded input is the
[longitudinal feature-analysis consumer note](consumers/longitudinal-feature-analysis.md).
It treats application identities, capture semantics, missingness meaning, and
domain interpretation as downstream responsibilities while exercising
Matrical's numeric selection, transformation, policy, and provenance contracts.
