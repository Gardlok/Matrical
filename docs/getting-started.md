# Getting started with Matrical

This guide assumes ordinary Rust familiarity and no knowledge of Matrical's
rehabilitation history. The recommended import style is:

```rust
use matrical::prelude::*;
```

## 1. Create a Matrix

A `Shape` validates the two-dimensional geometry, including element-count
overflow. Zero-sized dimensions are valid. `Matrix::from_row_major` then consumes
exactly `Shape::len()` values in logical row-major order and returns a typed error
if the value count does not match.

The complete workflow in this guide is compiled as
[`examples/r5_quickstart.rs`](../examples/r5_quickstart.rs).

```rust
let shape = Shape::new(3, 4)?;
let mut matrix = Matrix::from_row_major(
    shape,
    vec![
        0.0, 1.0, 2.0, 3.0,
        4.0, 5.0, 6.0, 7.0,
        8.0, 9.0, 10.0, 11.0,
    ],
)?;
```

`Matrix::into_row_major()` later consumes the Matrix and returns values in the
same logical order. Matrical does not expose the private `ndarray` storage as the
public ownership contract.

## 2. Read safely

Use `Index::new(row, column)` with `Matrix::get` or `get_mut`. An `Index` is just
a coordinate; access validates it against the Matrix or Lens-local shape and
returns `MatricalError::IndexOutOfBounds` when it does not fit.

```rust
assert_eq!(*matrix.get(Index::new(1, 2))?, 6.0);
```

## 3. Select a Region

A Region is a checked half-open rectangle. `1..3` means rows 1 and 2; the end
boundary is excluded and may equal the Shape dimension. Empty Regions are valid;
reversed or out-of-bounds Regions return typed errors.

```rust
let region = Region::new(shape, 1..3, 1..3)?;
```

A Region is revalidated when a Matrix turns it into a Lens, so a Region created
for one Shape cannot silently grant invalid access to another Matrix.

## 4. Borrow through Lens

`matrix.lens(region)?` returns an immutable zero-copy borrowing view. Lens
coordinates are **local** to the selected rectangle: local `Index::new(0, 0)` is
the Region's top-left parent element.

Lens construction, checked access, and iteration do not intentionally allocate.
Iteration is deterministic logical row-major order over the selection; it makes
no promise about physical contiguity. `to_row_major()` is the explicit `T: Clone`
conversion that allocates an owned vector.

```rust
let lens = matrix.lens(region)?;
assert_eq!(lens.get(Index::new(0, 0))?, &5.0);
```

The Lens cannot outlive its Matrix borrow. Existing rustdoc `compile_fail`
examples also demonstrate that two `LensMut` values cannot coexist through the
same Matrix borrow.

## 5. Transform through LensMut

A mutating Gear receives only a `LensMut`, not `&mut Matrix<T>`. The caller
therefore chooses the exact Region of mutable authority before the Gear runs.

The built-in mutating examples are `AddScalarGear`, `ScaleGear`, and `ClampGear`.
The built-in read Gear is `SumGear`.

```rust
let mut lens = matrix.lens_mut(region)?;
let report = execute_mut(
    &AddScalarGear,
    &mut lens,
    &Cog::new(ScalarPolicy::new(10.0)),
    vec![],
)?;
assert_eq!(*report.output(), 4);
```

Dropping or ending the LensMut borrow returns ordinary access to the parent
Matrix. Matrical does not add unchecked or arbitrary-region mutation as a
convenience shortcut.

## 6. Supply Cog policy

`Cog<C>` carries one concrete context type. A Gear declares that type through its
associated `Context`. `execute_read` / `execute_mut` resolve the Cog and call
`ValidateCog::validate` before the Gear executes.

`Cog::<C>::empty()` produces `MatricalError::InvalidContext` when context is
required. Built-in `ScalarPolicy` and `ClampPolicy` use `InvalidValue` when their
contents fail validation. There is no `Any` downcast, string-key lookup, registry,
or DI container in this path.

No builders were added in R5: `Shape`, `Region`, `Cog`, `ScalarPolicy`,
`ClampPolicy`, and `Tag` already have small typed constructors whose required
arguments are clearer at the call site than builder ceremony.

## 7. Inspect ExecutionReport and Tags

Successful execution returns `ExecutionReport<O>` with:

- the Gear's static name;
- the exact caller-selected Region;
- `GearEffect::ReadOnly` or `GearEffect::Mutating`;
- the strongly typed output `O`;
- ordered caller-supplied Tags.

Tags are inert provenance. `Tag::source`, `Tag::stage`, and `Tag::sequence` record
bounded metadata, but Tags are not passed into `Gear::apply` and cannot select or
modify an operation.

```rust
let tags = vec![
    Tag::source("example"),
    Tag::stage(TagStage::Transform),
    Tag::sequence(1),
];
let lens = matrix.lens(region)?;
let report = execute_read(&SumGear, &lens, &Cog::new(()), tags)?;
assert_eq!(report.effect(), GearEffect::ReadOnly);
```

## 8. Define your own Gear

Downstream extensibility is ordinary static Rust: implement `ValidateCog` for a
context type and `ReadGear<T>` or `MutGear<T>` for the transformation. No central
registry, dynamic dispatch, internal module import, or `ndarray` dependency is
required.

The complete program below is compiled as
[`examples/r5_custom_gear.rs`](../examples/r5_custom_gear.rs):

```rust
use matrical::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Threshold {
    minimum: f64,
}

impl ValidateCog for Threshold {
    fn validate(&self) -> Result<(), MatricalError> {
        if self.minimum.is_finite() {
            Ok(())
        } else {
            Err(MatricalError::InvalidValue)
        }
    }
}

struct CountAtLeast;

impl ReadGear<f64> for CountAtLeast {
    type Context = Threshold;
    type Output = usize;

    fn name(&self) -> &'static str {
        "count_at_least"
    }

    fn apply(
        &self,
        lens: &Lens<'_, f64>,
        context: &Self::Context,
    ) -> Result<Self::Output, MatricalError> {
        Ok(lens.iter().filter(|value| **value >= context.minimum).count())
    }
}
```

The Gear receives the already-selected Lens. It cannot request a larger Region
through this contract.

## 9. Handle common errors

`MatricalError` implements `std::error::Error`, `Debug`, `Display`, and structural
variants for the accepted public boundaries. Match the variant when program logic
depends on the failure class; use `Display` for concise caller-facing context.

```rust
let bad = Matrix::from_row_major(Shape::new(1, 2)?, vec![1.0]);
assert!(matches!(
    bad,
    Err(MatricalError::RowMajorLengthMismatch {
        expected: 2,
        actual: 1,
    })
));
```

Other high-frequency variants include `ShapeElementCountOverflow`,
`RegionReversed`, `RegionOutOfBounds`, `IndexOutOfBounds`, `InvalidContext`, and
`InvalidValue`. Historical `Regular`, `Custom`, and `ShouldNotOccur` variants are
not the preferred basis for new API design.

## 10. Conversions and where to go next

The current conversion names intentionally encode ownership and copy behavior:

- `Matrix::from_row_major` — fallibly constructs owned Matrix storage;
- `Matrix::into_row_major` — consumes the Matrix and returns owned values;
- `Lens::to_row_major` / `LensMut::to_row_major` — clone the borrowed selection
  into a new owned vector.

R5 adds no `From`/`Into` implementation that would hide fallibility or allocation.

Next read the [architecture vision](architecture/vision.md) for responsibility and
authority boundaries, [API stability policy](api-stability.md) for the current
0.1.0 compatibility position, and the [roadmap](roadmap.md) for later measurement
and release gates.
