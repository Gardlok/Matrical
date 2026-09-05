# Matrical

Matrical is a Rust library for working with dense matrices through checked
regions, borrowing views, and typed transformations.

It makes matrix operations explicit: what part of a matrix is being accessed,
whether it can be mutated, and what transformation is being performed.

## Install

```toml
[dependencies]
matrical = "0.1"
```

Matrical supports Rust 1.85.0 and newer.

## Quick start

The recommended everyday import is `matrical::prelude::*`.

```rust
use matrical::prelude::*;

fn main() -> Result<(), MatricalError> {
    let shape = Shape::new(2, 3)?;
    let matrix = Matrix::from_row_major(
        shape,
        vec![1, 2, 3, 4, 5, 6],
    )?;

    let region = Region::new(shape, 0..2, 1..3)?;
    let lens = matrix.lens(region)?;

    println!("{:?}", lens.to_row_major());
    Ok(())
}
```

This prints `[2, 3, 5, 6]`. The `Region` selects both rows and the last two
columns. Region ranges are half-open, just like ordinary Rust ranges.

## Core concepts

- **`Shape`** validates the dimensions of a matrix.
- **`Matrix<T>`** owns dense values in deterministic row-major order.
- **`Region`** describes a checked, half-open rectangular selection.
- **`Lens<'a, T>`** borrows an immutable view of a Region.
- **`LensMut<'a, T>`** borrows an exclusive mutable view of a Region.

Lens coordinates are local to the selected Region. Creating, indexing, and
iterating a Lens does not intentionally allocate. `to_row_major()` is the
explicit cloning conversion when an owned vector is needed.

The underlying `ndarray::Array2<T>` storage is private. Callers work through
Matrical's checked `Shape`, `Matrix`, `Region`, and Lens APIs rather than relying
on backend layout details.

See [Getting started](docs/getting-started.md) for checked construction,
indexing, mutable views, error handling, and larger examples.

## Typed transformations

Matrical can run reusable transformations over exactly the Lens selected by the
caller:

- **`ReadGear`** inspects an immutable `Lens`.
- **`MutGear`** modifies a `LensMut`.
- **`Cog<C>`** carries typed context or policy validated before execution.
- **`ExecutionReport<O>`** records the Gear, selected Region, effect, and typed
  output of a successful transformation.
- **`Tag`** values add inert provenance to a report. Tags never control
  execution.

Built-in Gears include `SumGear`, `AddScalarGear`, `ScaleGear`, and `ClampGear`.
Execution is deterministic and sequential. A Gear receives only the Lens or
LensMut supplied by its caller, not the full Matrix or permission to select a
different Region.

The [`r4_transform`](examples/r4_transform.rs) and
[`r5_custom_gear`](examples/r5_custom_gear.rs) examples show built-in and custom
transformations.

## Snapshots and Serde

`MatrixSnapshot<T>` is a versioned, dense row-major representation for moving
matrix data across an application boundary without exposing the private ndarray
backend. Reconstruction is checked against Matrical's Shape and Matrix
invariants.

Serde support is optional:

```toml
[dependencies]
matrical = { version = "0.1", features = ["serde"] }
```

The feature adds `Serialize` and `Deserialize` implementations for
`MatrixSnapshot<T>` only. It does not make `Matrix<T>` serializable or add file,
database, network, or persistence behavior.

See [Interchange](docs/interchange.md) for snapshot schema versioning, ownership
and copy behavior, and untrusted-input considerations.

## Performance

Lens and Gear traversal operate over the selected ndarray Region rather than
scanning unrelated matrix cells. The accepted API remains deterministic and
sequential; Matrical does not currently promise parallel execution.

See the [performance report](https://github.com/Gardlok/Matrical/blob/main/docs/performance.md)
for benchmark methodology and results.

## Supported Rust and API stability

The minimum supported Rust version is 1.85.0. `matrical::prelude::*` is the
recommended everyday API; `matrical::snapshot` is a specialized interchange
namespace and is deliberately not included in the prelude.

Matrical is in the `0.1.x` release line. See [API stability](docs/api-stability.md)
for the supported public surface, pre-1.0 compatibility policy, and the separate
snapshot-schema version contract.

## Examples

The repository and crate package include:

- [`r2_core_matrix`](examples/r2_core_matrix.rs) — checked matrix construction
  and access;
- [`r3_lens`](examples/r3_lens.rs) — immutable and mutable borrowing views;
- [`r4_transform`](examples/r4_transform.rs) — built-in typed transformations;
- [`r5_quickstart`](examples/r5_quickstart.rs) — an end-to-end workflow;
- [`r5_custom_gear`](examples/r5_custom_gear.rs) — a custom Gear and Cog policy;
- [`r7_snapshot`](examples/r7_snapshot.rs) — Serde snapshot roundtrip (requires
  the `serde` feature).

## Contributing

See the [contribution guide](https://github.com/Gardlok/Matrical/blob/main/CONTRIBUTING.md)
and [testing procedures](https://github.com/Gardlok/Matrical/blob/main/docs/testing-procedures.md).

## License

Matrical is licensed under the [MIT License](LICENSE).
