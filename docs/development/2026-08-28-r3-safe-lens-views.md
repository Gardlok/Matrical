# R3 safe Lens views — development and qualification record

## Starting identity

```text
repository Gardlok/Matrical
branch     main
commit     2f76a87e171a32a58a6d7244fdeb1b8794fc043a
tree       947684bc73841fb0842d5664e168e28bc8d3b05b
version    0.1.0
```

The merge parents are:

```text
059f148a99cfe2b5b881ada9af9acc286f584b6a
cd0154c728fa3d18904b1753a4e33354b9ff54f6
```

The second parent is the final corrected R2 candidate. The accepted lockfile
SHA-256 is:

```text
8abe052c6d793e87df19c1e6ade379caf3cad562eea693a946dc39c9e7180020
```

R3 branch:

```text
rehab/r3-safe-lens-views
```

## Historical Lens prototype disposition

The historical `src/strategies/lens.rs` validation/strategy prototype was not a
compatibility contract. R3 removes the old `Lens<V>` execution trait,
`MatrixLens<T>` wrapper, validation-builder ceremony, validation strategies, and
associated placeholder behavior. Repository search found no compiled external
references requiring a mechanical compatibility shim.

The canonical public meanings are now borrowing views:

```rust
pub struct Lens<'a, T> { /* private immutable borrow */ }
pub struct LensMut<'a, T> { /* private exclusive mutable borrow */ }
```

## Representation and backend exposure

R3 uses a safe parent-borrow representation:

```text
Lens<'a, T>    -> &'a Matrix<T> + Region + local Shape
LensMut<'a, T> -> &'a mut Matrix<T> + Region + local Shape
```

This representation was chosen instead of directly storing
`ndarray::ArrayView2` / `ArrayViewMut2` because it satisfies the public borrowing
contract without changing Matrix backend visibility at all. No ndarray view is
made public or `pub(crate)`, and no new Matrix backend helper is required.

The public contract does not promise physical contiguity. A later measured
optimization may change the private representation without changing Lens API
semantics.

## Region relationship and local indexing

`Matrix::lens` and `Matrix::lens_mut` call the existing Matrix Region
revalidation path. A Region valid for another Shape can therefore still fail
with `MatricalError::RegionOutOfBounds` when presented to a smaller receiving
Matrix.

A Lens retains the validated Region in parent coordinates but its public
`get(Index)` / `get_mut(Index)` coordinates are local to the selected rectangle.
For a parent Region `rows 2..4, columns 3..6`, local `(0, 0)` maps to parent
`(2, 3)`. Local bounds are checked before translation, and invalid local access
returns `MatricalError::IndexOutOfBounds`.

## Geometry and empty views

Both Lens types expose:

```text
region()
shape()
rows()
columns()
len()
is_empty()
```

The Lens Shape is exactly `region.rows() x region.columns()`.

Empty Regions are valid. Row/column semantics are:

```text
Shape(0, N): no valid row; in-range columns yield 0 x 1 empty Lens
Shape(N, 0): in-range rows yield 1 x 0 empty Lens; no valid column
Shape(0, 0): neither row nor column index is valid
```

Out-of-range row/column selectors return `MatricalError::IndexOutOfBounds`.

## Iteration and allocation contract

Lens iteration is built from Matrix's deterministic row-major iterator plus a
non-allocating coordinate filter over the validated parent Region. The resulting
logical order is:

```text
top selected row, left to right
then the next selected row
...
```

The implementation intentionally promises logical ordering only, not parent
storage contiguity or pointer layout.

Must not intentionally allocate:

```text
Matrix -> Lens
Matrix -> LensMut
row / row_mut
column / column_mut
Lens/LensMut checked access
Lens/LensMut iteration
```

May allocate, explicitly:

```text
Lens::to_row_major()
LensMut::to_row_major()
```

Those conversion methods require `T: Clone`, collect selected values into a new
`Vec<T>`, and preserve the same logical row-major order.

## Mutable aliasing contract

`LensMut<'a, T>` contains `&'a mut Matrix<T>`. Safe Rust therefore ties the view
to the mutable Matrix borrow and rejects another `matrix.lens_mut(...)` while a
live first LensMut will be used. R3 intentionally does not attempt disjoint
mutable splitting. No runtime overlap table, raw pointer, or unsafe aliasing
machinery exists.

## GAT evaluation

### Design A — concrete lifetime-generic views

```rust
pub struct Lens<'a, T> { /* private borrow */ }
pub struct LensMut<'a, T> { /* private mutable borrow */ }

impl<T> Matrix<T> {
    pub fn lens(&self, region: Region) -> Result<Lens<'_, T>, MatricalError>;
    pub fn lens_mut(&mut self, region: Region) -> Result<LensMut<'_, T>, MatricalError>;
}
```

Caller ergonomics are direct; the borrow lifetime is visible in the returned
concrete type; inherent methods produce short diagnostics; static dispatch is
ordinary monomorphization; and Rust 1.85 needs no advanced associated-type
constraints for this path.

### Design B — GAT-backed lending abstraction

The evaluated shape is equivalent to:

```rust
trait LendingView<T> {
    type View<'a>
    where
        Self: 'a,
        T: 'a;

    type ViewMut<'a>
    where
        Self: 'a,
        T: 'a;

    fn view<'a>(&'a self, region: Region) -> Result<Self::View<'a>, MatricalError>;
    fn view_mut<'a>(&'a mut self, region: Region) -> Result<Self::ViewMut<'a>, MatricalError>;
}
```

This can abstract statically over multiple future view providers. It does not,
however, strengthen lifetime or alias safety relative to Design A. Matrix is the
only proven provider in R3, so the trait introduces public abstraction,
associated-type where-clauses, and more complex caller/compiler diagnostics
without simplifying present use.

### Decision

R3 selects Design A and defers a public GAT lending trait. The GAT design should
be reconsidered only if R4 Gear composition needs to abstract over multiple
borrow providers or later backend work establishes at least two real providers.
This is an evidence-based deferral, not a conclusion that GATs are unsuitable for
Matrical.

## Rust 1.85 implications

The concrete design uses ordinary lifetime parameters, inherent impls, iterator
combinators, and stable language/library features available well before Rust
1.85. It avoids adding a public GAT surface solely for feature use. The MSRV and
edition remain unchanged:

```text
rust-version 1.85
edition      2021
```

## Compile-time borrowing evidence

Rustdoc `compile_fail` examples prove both required misuse cases:

1. a `Lens<'a, T>` cannot be returned from a scope containing its local Matrix;
2. a second mutable Lens cannot be created while the first mutable Lens remains
   live and is subsequently used.

These tests require no additional compile-test dependency.

## Boundary/property-style coverage

The R3 unit test exhaustively loops over these small Shapes:

```text
0 x 0
0 x 3
3 x 0
1 x 1
1 x 4
4 x 1
2 x 3
3 x 4
```

For each dimension it combines boundary points `0`, `dimension`, and
`dimension + 1` as ordered and reversed Region starts/ends. Every accepted Region
is turned into a Lens and checked for expected Shape, iteration count, and exact
parent-coordinate values. Rejected Regions must be typed `RegionReversed` or
`RegionOutOfBounds`. Empty accepted Regions are included naturally.

A separate regression proves that a Region valid against a larger foreign Shape
is revalidated and rejected by a smaller receiving Matrix.

## Functional and downstream coverage

Unit tests cover:

- rectangular immutable selection and local first/last access;
- full-Matrix, one-element, and empty Lenses;
- local out-of-bounds errors;
- immutable deterministic iteration and owned row-major copy;
- mutable single-element and whole-Region mutation;
- selected-only parent updates and invalid mutable local access;
- empty mutable Lens safety;
- first/last/out-of-bounds rows and row mutation;
- first/last/out-of-bounds columns and column mutation;
- `N x 0`, `0 x N`, and `0 x 0` selector behavior;
- exhaustive small-domain Region boundaries.

`tests/r3_lens_api.rs` consumes Matrical only through intended public exports and
covers construction, Region, immutable Lens, local checked access, rows,
columns, mutable Lens mutation, empty selector semantics, and typed invalid
selection.

`examples/r3_lens.rs` is a Result-propagating runnable example demonstrating
immutable inspection, mutable selected-region changes reflected in the parent,
and row/column selection.

Lens rustdoc contains one positive compiled example and the two compile-fail
borrowing examples.

## Unsafe audit and Miri evaluation

Project-authored unsafe introduced by R3:

```text
0
```

Borrow/aliasing enforcement is entirely safe Rust: shared `&Matrix<T>` for Lens,
exclusive `&mut Matrix<T>` for LensMut, checked Matrix element access, and safe
iterator composition.

The developer runtime used for this session does not provide Rust/rustup, so Miri
is not available there. Installing a nightly solely for this safe wrapper would
not add evidence proportional to the environment detour. R3 instead relies on
the borrow checker, compile-fail rustdoc, exhaustive runtime boundary tests, and
the existing Rust 1.85/stable GitHub Qualification lanes. Miri remains worth
reconsidering if later work introduces project-authored unsafe code or custom
disjoint mutable splitting.

## Qualification status

The development runtime cannot execute Cargo because Rust tooling is absent.
This is an environment limitation rather than an owner-operated gate; the owner
is not asked to install or run anything. The existing GitHub Qualification
workflow is the authoritative executable environment for both Rust 1.85.0 and
stable and will run after PR publication.

Mechanical review before publication includes changed-path scope inspection,
lockfile path identity, unsafe-token inspection of new R3 source, and review of
the final compare against the accepted baseline. CI outcomes and final head/tree
identity are recorded in the closing evidence update after Qualification runs.

## Preserved constraints

```text
version      0.1.0
MSRV         Rust 1.85.0
edition      2021
dependencies unchanged
Cargo.lock   byte-identical expected SHA-256:
8abe052c6d793e87df19c1e6ade379caf3cad562eea693a946dc39c9e7180020
```

R3 does not reconstruct Gear, Cog, Tag, Vector, unrelated operations, backend
abstractions, parallel mutation, serialization, persistence, or release work.

## Residual historical debt

Inherited warnings, formatting residue, `MatrixContext`, Gear/Cog/Tag/Vector
prototype debt, and unrelated operation placeholders remain outside R3. The
parent-reference Lens representation prioritizes the proven safety/API contract;
its iterator scans the parent logical iterator and filters the selected Region.
R6 performance measurement may determine whether an internal ndarray-view
representation is worthwhile, without changing the public API.

## R3 exit evaluation and R4 recommendation

Before Teamlead handoff, R3 must still receive green Rust 1.85.0 and stable
GitHub Qualification and final mechanical identity checks. If they pass, R3 is
reviewable and the recommended next phase after Teamlead/owner acceptance is:

```text
R4 — reintroduce Gear, Cog, and Tag
```

R4 must not begin on this branch.
