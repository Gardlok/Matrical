# R3 safe Lens views — development and qualification record

## Starting identity

```text
repository Gardlok/Matrical
branch     main
commit     2f76a87e171a32a58a6d7244fdeb1b8794fc043a
tree       947684bc73841fb0842d5664e168e28bc8d3b05b
version    0.1.0
```

Merge parents:

```text
059f148a99cfe2b5b881ada9af9acc286f584b6a
cd0154c728fa3d18904b1753a4e33354b9ff54f6
```

The second parent is the final corrected R2 candidate. R3 uses branch:

```text
rehab/r3-safe-lens-views
```

The accepted and preserved Cargo.lock SHA-256 is:

```text
8abe052c6d793e87df19c1e6ade379caf3cad562eea693a946dc39c9e7180020
```

## Historical Lens prototype disposition

The historical `src/strategies/lens.rs` validation/strategy prototype was not a
compatibility contract. R3 retires the old `Lens<V>` execution trait,
`MatrixLens<T>` wrapper, `MatrixLensTrait`, matrix-validation builders and
strategies, and their placeholder validation ceremony. Repository search found
no compiled external references requiring a compatibility shim.

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

The underlying `ndarray::Array2<T>` remains private. R3 does not expose ndarray
views publicly or through new `pub(crate)` helpers. This representation proves
the required ownership contract without broadening backend access.

No physical-contiguity promise is made. A later measured optimization may change
the private representation without changing public Lens semantics.

## Region relationship and Lens-local indexing

`Matrix::lens` and `Matrix::lens_mut` reuse the existing Matrix Region
revalidation path. A Region that was valid for another Shape is therefore not
trusted when supplied to a receiving Matrix and may return
`MatricalError::RegionOutOfBounds`.

A Lens retains its validated Region in parent coordinates, but public element
access is Lens-local. For parent Region `rows 2..4, columns 3..6`, local
`Index::new(0, 0)` addresses parent coordinate `(2, 3)`. Local bounds are
checked before coordinate translation; invalid local access returns
`MatricalError::IndexOutOfBounds`.

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

## Matrix entry points and selectors

R3 exposes:

```text
Matrix::lens
Matrix::lens_mut
Matrix::row
Matrix::row_mut
Matrix::column
Matrix::column_mut
```

Rows are rectangular `1 x columns` Lenses. Columns are rectangular `rows x 1`
Lenses. Out-of-range row or column indices return
`MatricalError::IndexOutOfBounds`.

Empty-shape behavior is explicit:

```text
Shape(0, N): no valid row; in-range columns yield 0 x 1 empty Lens
Shape(N, 0): in-range rows yield 1 x 0 empty Lens; no valid column
Shape(0, 0): neither row nor column index is valid
```

Empty Regions remain valid Lens and LensMut selections.

## Access, iteration, and allocation contract

Immutable Lens provides checked `get` and `iter`. Mutable Lens provides checked
`get`, `get_mut`, `iter`, and `iter_mut`.

Iteration is built from Matrix's deterministic logical row-major iterator plus a
non-allocating coordinate filter over the selected Region. The public order is:

```text
top selected row left-to-right
then the next selected row
...
```

The contract is logical ordering, not pointer layout or parent contiguity.

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

The owned conversions require `T: Clone`, collect into a new `Vec<T>`, and use
the same logical row-major ordering as iteration.

## Mutable aliasing contract

`LensMut<'a, T>` contains `&'a mut Matrix<T>`. Safe Rust therefore ties the
view to the mutable Matrix borrow and rejects another `matrix.lens_mut(...)`
while the first LensMut remains live and is subsequently used, even if Regions
would be disjoint.

R3 deliberately uses this conservative whole-Matrix mutable borrow. It does not
add runtime overlap tracking, raw pointers, unsafe disjoint splitting, or any
other aliasing escape hatch.

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

Design A gives direct caller ergonomics, keeps the borrow lifetime visible in
the concrete returned type, uses ordinary static dispatch, and produces simpler
ownership diagnostics. It is fully compatible with Rust 1.85.

### Design B — GAT-backed lending abstraction

The evaluated alternative is equivalent to:

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

    fn view<'a>(&'a self, region: Region)
        -> Result<Self::View<'a>, MatricalError>;

    fn view_mut<'a>(&'a mut self, region: Region)
        -> Result<Self::ViewMut<'a>, MatricalError>;
}
```

Design B preserves static dispatch and could abstract over multiple future view
providers. It does not make current Matrix-to-Lens borrowing safer, does not
simplify current callers, and adds a public trait plus associated-type lifetime
constraints and more complex diagnostics.

### Final GAT decision

R3 selects Design A and defers a public GAT lending trait. Matrix is the only
proven view provider today, and the inherent methods already express the
ownership contract clearly. GATs should be reconsidered if R4 Gear composition
needs to abstract over multiple lending providers or later backend work produces
at least two genuine implementations.

This is an evidence-backed deferral, not a rejection of GATs as a Matrical tool.

## Compile-time lifetime and alias evidence

Rustdoc `compile_fail` examples prove both required misuse cases:

1. `Lens<'a, T>` cannot escape the scope containing its Matrix;
2. a second mutable Lens cannot be created from the same Matrix while the first
   remains live and is later used.

The corrected GitHub Qualification run executed these compile-fail doctests on
both Rust 1.85.0 and stable and they passed.

## Boundary/property-style coverage

The R3 unit suite exhaustively exercises these small Shapes:

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
`dimension + 1` as Region start/end values, including ordered and reversed
combinations. Every accepted Region is converted to a Lens and checked for:

- exact selected Shape;
- iteration count equal to Region length;
- exact correspondence to parent Matrix coordinates;
- safe empty selections.

Rejected Regions must be typed `RegionReversed` or `RegionOutOfBounds`. A
separate regression proves that a Region valid for a larger foreign Shape is
revalidated and rejected by a smaller receiving Matrix.

## Unit, integration, example, and rustdoc coverage

R3 adds 10 focused Lens unit tests covering immutable and mutable rectangular
views, local first/last access, local out-of-bounds behavior, full/one/empty
views, selected-only mutation, rows, columns, zero-sized selector semantics, and
exhaustive boundary loops.

`tests/r3_lens_api.rs` adds 3 downstream-style integration tests using only the
intended public Matrical API. They cover Matrix construction, Region, immutable
Lens, local checked access, rows, columns, mutable Lens mutation reflected in the
parent, empty selectors, and typed invalid selection.

`examples/r3_lens.rs` is a Result-propagating runnable example demonstrating
immutable inspection, mutable selected-region updates, parent reflection, and
row/column selection. It compiles under the all-targets qualification gate.

Lens rustdoc adds one positive compiled example and two compile-fail borrowing
examples. The whole crate qualification reports 4 passing doctests: the existing
R2 Matrix example plus the 3 R3 Lens examples.

Observed corrected-run totals:

```text
61 unit tests
 1 R2 integration test
 3 R3 integration tests
----------------------
65 runtime tests passed
 4 doctests passed (including 2 R3 compile-fail tests)
----------------------
69 qualification tests passed
```

## Unsafe audit and Miri/deeper-check evaluation

Project-authored unsafe introduced by R3:

```text
0
```

Borrowing and alias enforcement use only safe Rust: `&Matrix<T>`,
`&mut Matrix<T>`, Matrix's checked access, and safe iterator composition.

The developer runtime for this session does not provide Rust, rustup, rustfmt,
or Miri. Installing a nightly solely for a safe wrapper would be an environment
detour without proportional additional evidence. Miri was therefore evaluated
but not made a blocking requirement. The R3 evidence instead uses:

```text
safe Rust
borrow checker
compile-fail rustdoc
exhaustive boundary tests
Rust 1.85 qualification
stable qualification
```

Miri should be reconsidered if later work introduces project-authored unsafe
code or custom disjoint mutable splitting.

## Qualification evidence

The local development runtime could not execute Cargo or rustfmt. No owner gate
was introduced for that environment limitation; the repository's existing
GitHub Qualification workflow supplied executable Rust 1.85.0 and stable
coverage.

The first PR run, GitHub Actions run `33211684131`, found the same narrow R3
compile error in both lanes: `selected_iter` carried an unused generic parameter
that Rust could not infer (`E0282`). No architectural change was required. Commit
`a17527ad6083a21a3b865502231c7b48dffbe2a7` removed that stray type parameter.

The corrected GitHub Actions run `33211889042` passed both lanes completely.

### Rust 1.85.0 lane

Observed compiler:

```text
rustc 1.85.0 (4d91de4e4 2025-02-17)
```

The workflow does not print an explicit `cargo --version`; Cargo is the toolchain
Cargo installed with the pinned Rust 1.85.0 lane.

Results:

```text
cargo check --locked --all-targets      PASS
cargo test --locked --all-targets       PASS — 65 runtime tests
cargo test --locked --doc               PASS — 4 doctests
cargo clippy --locked --all-targets     PASS
cargo doc --locked --no-deps            PASS
```

### Stable lane

Observed compiler:

```text
rustc 1.98.0 (88d9e12ae 2026-08-18)
```

The workflow likewise does not print an explicit `cargo --version`; Cargo is the
Cargo installed with that stable toolchain.

Results:

```text
cargo check --locked --all-targets      PASS
cargo test --locked --all-targets       PASS — 65 runtime tests
cargo test --locked --doc               PASS — 4 doctests
cargo clippy --locked --all-targets     PASS
cargo doc --locked --no-deps            PASS
```

The warning output remains inherited prototype debt in unrelated historical
modules. The corrected Lens implementation introduces no reported warning or
Clippy diagnostic.

## Mechanical and identity evidence

The PR changes exactly these seven authorized paths:

```text
docs/active-development.md
docs/architecture/vision.md
docs/development/2026-08-28-r3-safe-lens-views.md
docs/roadmap.md
examples/r3_lens.rs
src/strategies/lens.rs
tests/r3_lens_api.rs
```

No `src/schematics/matrix.rs` or `src/lib.rs` adjustment was required. No
mechanical out-of-primary-scope source adjustment was needed.

`Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, and the Qualification workflow
are unchanged. The Cargo.lock Git blob is identical to the accepted baseline, so
its SHA-256 remains byte-identical:

```text
before 8abe052c6d793e87df19c1e6ade379caf3cad562eea693a946dc39c9e7180020
after  8abe052c6d793e87df19c1e6ade379caf3cad562eea693a946dc39c9e7180020
```

The developer runtime has Git but no repository checkout and cannot reach
GitHub directly through the shell, so literal local `git diff --check` and
rustfmt execution were unavailable. The developer instead inspected the GitHub
PR patch and changed-path set directly; no whitespace-only or unrelated-scope
change is present. The GitHub Actions all-targets gates compile the new Rust on
both supported lanes. This limitation is recorded rather than transferred to an
owner-operated gate.

No repository-local `target/` path was added to the PR.

## Preserved constraints

```text
version      0.1.0
MSRV         Rust 1.85.0
edition      2021
dependencies unchanged
Cargo.lock   byte-identical
```

R3 adds no dependency, ndarray upgrade, backend abstraction, parallel Lens
mutation, Serde, SurrealDB, Criterion, Rayon, Gear/Cog/Tag/Vector
reconstruction, Cadenscript integration, release, tag, or publication work.

## Residual historical debt

Inherited warning and Clippy residue, `MatrixContext`, Gear/Cog/Tag/Vector
prototype debt, and unrelated operation placeholders remain outside R3.

The parent-reference Lens iterator scans the parent Matrix's logical row-major
iterator and filters to the selected Region. That is allocation-free and
semantically correct, but R6 performance measurement should decide whether an
internal ndarray-view implementation is worth adopting. Such a private
optimization need not change the public Lens contract.

## R3 exit evaluation

```text
Lens cannot outlive Matrix                              PASS
LensMut tied to mutable Matrix borrow                   PASS
simultaneous mutable borrowing rejected by Rust         PASS
invalid Region selection returns typed error            PASS
Lens-local invalid Index returns typed error             PASS
immutable rectangular Lens                              PASS
mutable rectangular Lens updates parent                 PASS
row selection                                            PASS
column selection                                         PASS
zero-sized/empty selection semantics                    PASS
logical deterministic row-major iteration               PASS
Lens creation/iteration allocation contract             PASS
explicit allocating owned conversion                    PASS
property-style boundary coverage                        PASS
external public API integration tests                   PASS
runnable example compiles                               PASS
positive rustdoc                                         PASS
lifetime compile-fail rustdoc                           PASS
mutable-alias compile-fail rustdoc                      PASS
GAT-vs-lifetime comparison                              PASS
GAT decision evidence-backed                            PASS
project-authored R3 unsafe                              0 / PASS
Miri/deeper aliasing evaluation                         PASS
Rust 1.85 qualification                                 PASS
stable qualification                                    PASS
Cargo.lock unchanged                                    PASS
existing GitHub CI                                      PASS
```

R3 is therefore a complete review candidate. The recommended next phase only
after Teamlead/owner acceptance and merge is:

```text
R4 — reintroduce Gear, Cog, and Tag
```

Do not begin R4 on the R3 branch.
