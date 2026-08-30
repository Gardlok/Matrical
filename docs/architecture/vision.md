# Matrical architecture vision

**Status:** accepted rehabilitation direction through R5; R6 performance candidate reviewable

## Product position

Matrical is a semantic matrix-transformation library: a small Rust core for
validated matrix geometry and data, safe borrowing views, typed transformations,
contextual policy, and bounded metadata/provenance.

It builds on mature dense storage rather than competing with established
numerical libraries at storage layout, BLAS kernels, or general linear algebra.

The accepted conceptual flow is:

```text
Matrix
  -> Lens / LensMut
  -> Gear (+ typed Cog)
  -> ExecutionReport (+ Tags)
```

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

`Region` is a checked half-open rectangle:

```text
[start_row, end_row)
[start_column, end_column)
```

End boundaries may equal the Shape dimension. Reversed or out-of-bounds ranges
are typed failures. Empty Regions are valid. A Region presented for Lens
construction is revalidated against the receiving Matrix.

### Lens

`Lens<'a, T>` is an immutable rectangular borrowing view; `LensMut<'a, T>` is the
mutable counterpart:

```text
Matrix<T> owns data
Lens<'a, T>    borrows selected Matrix storage immutably
LensMut<'a, T> borrows selected Matrix storage mutably
```

The borrow checker prevents a Lens from outliving its Matrix and gives a
`LensMut` exclusive mutable borrow of the parent Matrix for its lifetime.
Matrical does not add unsafe disjoint splitting or runtime overlap tracking.

A Lens records the selected Region in parent coordinates but exposes checked
`Index` access in Lens-local coordinates. Local `(0, 0)` names the Region's
upper-left selected element.

R6 changed only the private representation: after Region validation, Lens/LensMut
hold an ndarray view of exactly the selected rectangle rather than holding the
whole Matrix and filtering parent-wide iteration. The public type names,
lifetimes, methods, error behavior, Region semantics, and Gear authority boundary
remain unchanged. ndarray view types are not exposed publicly.

Lens construction, row/column selection, checked access, and iteration do not
require selected-value copies or heap allocation. Logical iteration is row-major
within the selected rectangle and makes no physical-contiguity promise.
`to_row_major()` is the explicit allocating `T: Clone` conversion.

### Gear

A Gear transforms only the data visible through the Lens supplied by its caller.
Read and mutating authority are distinct static contracts:

```text
ReadGear<T> -> &Lens<'_, T>
MutGear<T>  -> &mut LensMut<'_, T>
```

A Gear does not normally receive `&Matrix<T>`, `&mut Matrix<T>`, ndarray storage,
or a generic selector/provider from which it can request a broader Region. This
is the central least-authority rule of the transformation design.

Downstream crates implement Gear traits directly with static dispatch. A central
registry, factory registration, `Any` context lookup, string operation dispatch,
DI container, and mandatory boxed trait objects are not part of the accepted
core.

Built-in deterministic examples are `SumGear`, `AddScalarGear`, `ScaleGear`, and
`ClampGear`.

### Cog

`Cog<C>` carries optional context whose concrete Rust type is selected by a
Gear's associated `Context`. `ValidateCog` runs before Gear execution. Missing
required context returns `MatricalError::InvalidContext`; invalid concrete policy
returns an appropriate typed error such as `InvalidValue`.

Cog is intentionally not a string-keyed map, `Any` container, or dependency
injection mechanism.

### Tag

`Tag` is bounded typed provenance associated with a successful execution report.
The current namespace provides a source label, typed `TagStage`, and numeric
sequence identity.

Tags are inert. Source text is never interpreted as code, a query, a command, or
a Gear selector. Tags are attached to a successful `ExecutionReport` after the
Gear returns and are not passed into Gear execution.

### ExecutionReport

`ExecutionReport<O>` records a successful execution without erasing its output:

```text
Gear identity
exact caller-selected Region
GearEffect::{ReadOnly, Mutating}
strongly typed output O
ordered provenance Tags
```

Failures remain `Err(MatricalError)`; Matrical does not fabricate success reports
around failed transformations.

## Public learning surface

R5 made the conceptual architecture the documentation map instead of exposing
prototype-era module history as the normal API.

The supported discovery policy is:

```text
matrical::prelude::*
  recommended everyday API

matrical::{Shape, Matrix, Lens, ReadGear, ...}
  named supported and discoverable crate-root API

matrical::schematics
  core geometry/storage organization

matrical::strategies
  Lens, Gear, Cog, Tag, and reporting organization
```

Historical operation scaffolding may remain source-accessible and `doc(hidden)`
during 0.1.0 rehabilitation when compatibility is useful, but it is not part of
the learning contract. Historical SQL, Element, Vector, `MatrixContext`,
`AtomicBoolError`, and raw dependency types are not prelude exports.

R6 does not alter this public learning surface.

## Layering

```text
matrical-core
  Shape, Index, Region, Matrix, MatricalError

matrical-view
  Lens, LensMut, validated selectors and iterators

matrical-transform
  ReadGear, MutGear, Cog, ValidateCog, Tag, ExecutionReport

development measurement
  Criterion benchmark harnesses; no runtime authority

optional later integrations
  measurement-driven parallelism, serialization, persistence, specialized storage
```

The first rehabilitation release may remain one crate. These are semantic
boundaries, not a requirement to split the workspace.

## Foundational invariants

- Shape dimensions and total element count agree without overflow.
- Zero-sized Shape/Matrix values are valid.
- Every public index and Region is validated before access/use.
- Region ordering is half-open and empty-selection behavior is explicit.
- Ordinary invalid public input is typed and fallible rather than panic-driven.
- Matrix and Lens logical iteration preserve deterministic row-major order.
- A Lens cannot outlive the Matrix storage it borrows.
- A mutable Lens has exclusive mutable access to its parent Matrix for its
  lifetime.
- A read Gear receives no mutable Lens capability.
- A mutating Gear cannot access values outside its supplied LensMut Region.
- Missing required Cog context is a typed failure.
- Invalid contextual policy is rejected before Gear execution.
- Tag/provenance cannot mutate Matrix data or select an operation by side
  channel.
- Convenience APIs must not silently broaden authority.
- Performance optimization must not expose the private ndarray backend or weaken
  checked Region semantics.

## Constructors, builders, and conversions

The accepted simple types use explicit typed constructors rather than builders.
`Shape`, `Region`, `Cog`, `ScalarPolicy`, `ClampPolicy`, and `Tag` do not
currently have enough optional configuration or ordering complexity to justify
builder ceremony.

Conversion names intentionally communicate ownership/allocation:

```text
Matrix::from_row_major  constructs owned checked Matrix; fallible
Matrix::into_row_major  consumes Matrix and returns owned values
Lens::to_row_major      borrows Lens, clones selected T values, allocates
```

Matrical does not add `From`/`Into` implementations that conceal fallibility.

## Storage and missingness

The accepted first dense storage is private `ndarray::Array2<T>`. R6 also uses
private ndarray view types internally to represent an already validated Lens
selection. This is an implementation detail, not a new public backend contract.

A validity/missingness mask is not intrinsic Matrix storage. Missingness belongs
in an explicit paired structure, wrapper, or downstream domain type unless a
future concrete use case proves that core Matrical should own that semantic.

Backend abstraction waits until multiple real implementations expose a stable
shared need.

## Advanced Rust and authority

GATs and HRTBs are tools, not design goals. R3 evaluated a lending-provider GAT
and retained concrete lifetime-generic Lenses because Matrix was the only proven
provider. R4 added a stronger reason: passing a selector/provider to a Gear could
give it authority to choose a broader Region than the caller intended.

The accepted architecture therefore consumes an already-selected capability:

```text
caller: Matrix -> Lens / LensMut
Gear:   receives that exact Lens / LensMut
```

R6 improves the private traversal representation without reopening that public
abstraction. A future GAT/HRTB abstraction still requires a concrete
composability failure that justifies the extra public complexity without
weakening least authority.

## Dynamic dispatch

Static Gear dispatch is the default. The public extension model allows a
downstream crate to define a context type, implement `ValidateCog`, implement a
Gear trait, and call `execute_read`/`execute_mut` without registration or boxing.
Heterogeneous runtime pipelines remain deferred until a real consumer requires
them.

## Concurrency and performance

The accepted execution contract remains deterministic and sequential. Thread-safe
containers do not by themselves define safe matrix-level concurrency.

R6 established a Criterion benchmark harness and found one dominant sequential
defect: Lens/LensMut traversed the full parent Matrix and filtered for selected
Region membership. The private checked-view repair removes that parent-wide scan.

On the authoritative owner-machine run, candidate dense traversal is already
approximately equivalent to direct ndarray traversal:

```text
100000x64 full Lens read / direct ndarray       0.990x
100000x64 full LensMut transform / direct       1.000x
100000x64 full Gear read / Lens                 1.029x
100000x64 full Gear mutation / LensMut          1.062x
```

A fixed 4 x 4 Lens read stays near 7.3 ns across parents ranging from `32 x 24`
to `100000 x 64`; its cost no longer scales with unrelated parent cells.

R6 therefore does not add Rayon. Parallel execution remains optional future work
only when a concrete workload demonstrates enough per-element computation to
justify scheduling, threshold policy, and added concurrency surface.

See [performance.md](../performance.md) for methodology, exact measurements,
allocation/copy accounting, and limitations.

## Error contract

`MatricalError` is public, structural, non-recursive, and implements
`std::error::Error`. Geometry/construction failures carry available structural
context. `InvalidContext` means required Cog context is absent; `InvalidValue`
means a value or typed policy failed validation. Legacy variants remain visibly
classified rather than being presented as richer errors than they are.

## Dependency policy

ndarray remains the private dense storage backend; Crossbeam remains historical
non-Matrix residue outside R6.

R6 adds exact Criterion 0.7.0 only under `[dev-dependencies]`, with default
features disabled and `cargo_bench_support` enabled. Criterion does not grant
runtime authority and is not a normal Matrical dependency.

Rayon is not added. Optional database, serialization, parallelism, and
specialized-backend capabilities must earn explicit design/feature/test work
later.

## Non-goals for the rehabilitation core

- Reimplementing a complete linear-algebra ecosystem.
- Database-backed matrices in the first functional slices.
- A hidden validity/missingness mask in `Matrix<T>`.
- Lock-free or parallel Matrix/Lens mutation as a default requirement.
- Unsafe disjoint mutable Lens splitting.
- Giving a Gear arbitrary Region-selection authority.
- Runtime Gear registries or DI containers without a concrete consumer need.
- Tags as SQL/query/command envelopes.
- Preserving unfinished 0.1.0 prototype APIs as a compatibility contract.
- Using advanced Rust syntax or concurrency machinery without a correctness,
  usability, or measured performance benefit.
- Treating benchmark numbers from one host as universal throughput guarantees.

## Downstream consumers

Concrete consumers may inform acceptance criteria without moving their domain
model into Matrical. The first recorded input remains the
[longitudinal feature-analysis consumer note](consumers/longitudinal-feature-analysis.md).
Application identity, capture semantics, missingness meaning, persistence, and
domain interpretation remain downstream responsibilities.
