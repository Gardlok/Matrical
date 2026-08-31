# Matrical

**Status: R1–R7 owner accepted; R8-A release-candidate qualification active; version 0.1.0**

Matrical is a semantic matrix-transformation library built around validated
geometry, borrowing selections, typed transformations, contextual policy,
bounded provenance, and explicit versioned dense snapshots. It uses a private
dense `ndarray::Array2<T>` backend while exposing a Matrical-owned contract for
shape, selection, transformation authority, execution reporting, and
interchange.

The accepted library is qualified on the declared Rust 1.85.0 MSRV and current
stable through repository CI. R8-A is determining whether `0.1.0` is ready for an
owner release decision. **No crates.io publication, Git tag, GitHub Release, or
release date is authorized merely because this branch qualifies.**

## Core vocabulary

- **Matrix** — owned dense values plus a validated two-dimensional `Shape`.
- **Region** — a checked half-open rectangle used to select matrix data.
- **Lens / LensMut** — immutable or exclusive mutable borrowing views over one
  caller-selected Region, with Lens-local indexing.
- **Gear** — a typed read-only or mutating transformation that receives only the
  supplied Lens capability.
- **Cog** — typed context or policy validated before a Gear executes.
- **Tag** — bounded, inert provenance attached to successful execution reports;
  Tags never control execution.
- **ExecutionReport** — Gear identity, exact Region, effect class, typed output,
  and ordered Tags for a successful execution.
- **MatrixSnapshot** — inert, versioned dense row-major interchange data that can
  cross a boundary without exposing ndarray or acquiring live storage authority.

The normal execution flow is:

```text
Matrix
  -> Lens / LensMut
  -> Gear (+ typed Cog)
  -> ExecutionReport (+ Tags)
```

## Using the release candidate before publication

During R8-A review, use a repository/path dependency rather than implying a
registry release exists:

```toml
[dependencies]
matrical = { path = "../Matrical" }
```

For snapshot serialization through Serde:

```toml
[dependencies]
matrical = { path = "../Matrical", features = ["serde"] }
```

After a separate owner-authorized publication, downstream users may replace that
path dependency with the exact released registry version.

## Quick start

The recommended everyday import is:

```rust
use matrical::prelude::*;
```

The canonical beginner workflow is compiled as
[`examples/r5_quickstart.rs`](examples/r5_quickstart.rs):

```rust
use matrical::prelude::*;

fn main() -> Result<(), MatricalError> {
    let shape = Shape::new(3, 4)?;
    let mut matrix = Matrix::from_row_major(
        shape,
        vec![
            0.0, 1.0, 2.0, 3.0,
            4.0, 5.0, 6.0, 7.0,
            8.0, 9.0, 10.0, 11.0,
        ],
    )?;
    let region = Region::new(shape, 1..3, 1..3)?;
    let tags = vec![
        Tag::source("quickstart"),
        Tag::stage(TagStage::Transform),
        Tag::sequence(1),
    ];

    {
        let lens = matrix.lens(region)?;
        let report = execute_read(&SumGear, &lens, &Cog::new(()), tags.clone())?;
        assert_eq!(*report.output(), 30.0);
    }

    {
        let mut lens = matrix.lens_mut(region)?;
        let report = execute_mut(
            &AddScalarGear,
            &mut lens,
            &Cog::new(ScalarPolicy::new(10.0)),
            tags,
        )?;
        assert_eq!(*report.output(), 4);
    }

    assert_eq!(
        matrix.into_row_major(),
        vec![
            0.0, 1.0, 2.0, 3.0,
            4.0, 15.0, 16.0, 7.0,
            8.0, 19.0, 20.0, 11.0,
        ]
    );
    Ok(())
}
```

`Region` bounds are half-open. `Lens` and `LensMut` expose coordinates local to
the selection, and creating/reading/iterating a Lens does not intentionally
allocate. `Lens::to_row_major()` is the explicit cloning conversion.

See [Getting started](docs/getting-started.md) for construction, selection,
built-in and custom Gears, Cog validation, Tags, and error handling.

## Optional dense interchange and `serde`

Need to move dense Matrix data across a process, repository, storage, or
integration boundary? Use the crate-root `MatrixSnapshot` API. A borrowed
`snapshot()` clones values; a consuming `into_snapshot()` transfers ownership.
Checked reconstruction reuses Matrical's existing Shape/Matrix invariants.

The optional `serde` feature adds `Serialize` / `Deserialize` participation only
for `MatrixSnapshot<T>`. It does **not** make `Matrix<T>` serializable, expose
ndarray storage, or add a persistence engine. `serde_json` is development and
example-only.

Dense snapshot schema v1 uses:

```text
version: u32
rows: u64
columns: u64
row_major: sequence of T in deterministic logical row-major order
```

Within a released line, incompatible dense snapshot semantics must not silently
change under schema version 1. A future incompatible representation requires a
different explicit snapshot schema version. Rust SemVer and snapshot schema
versions remain separate contracts.

See [Interchange](docs/interchange.md) for schema, copy/ownership behavior,
format limitations, and the caller-owned transport boundary.

## Public API policy

- `matrical::prelude::*` is the recommended everyday API.
- Named crate-root exports are supported and discoverable.
- `matrical::schematics` and `matrical::strategies` group the same supported
  concepts for deeper navigation.
- `matrical::snapshot` is the specialized supported interchange namespace and is
  excluded from the everyday prelude deliberately.
- Documentation-hidden operation/error/context prototype compatibility residue
  is not recommended or supported for new downstream code.
- ndarray and private Lens/Gear internals are implementation details.

Matrical does not give a Gear direct Matrix or arbitrary Region-selection
authority. Tags are provenance rather than a command channel.

See [API stability](docs/api-stability.md) for the candidate SemVer and snapshot
compatibility policy.

## Performance posture

R6 repaired inherited parent-wide Lens traversal by holding a checked private
ndarray Region view. Same-host measurements showed fixed-size selected traversal
no longer scaled with unrelated parent cells and large dense Lens/Gear paths at
approximately direct-ndarray performance. Those measurements are evidence, not a
universal throughput promise.

The accepted execution contract remains deterministic and sequential; Rayon is
not part of the release candidate because R6 did not demonstrate a measured need
for parallel runtime machinery.

The detailed benchmark evidence remains in the repository's
[performance report](https://github.com/Gardlok/Matrical/blob/main/docs/performance.md).

## Shipped examples

The package ships runnable examples for the accepted progression:

- `r2_core_matrix`
- `r3_lens`
- `r4_transform`
- `r5_quickstart`
- `r5_custom_gear`
- `r7_snapshot` (`serde` feature required)

R8-A qualifies every applicable example on Rust 1.85.0 and stable.

## Release posture

`CHANGELOG.md` describes the `0.1.0` unreleased release candidate. The repository
maintainer procedure is in
[`docs/release.md`](https://github.com/Gardlok/Matrical/blob/main/docs/release.md).

R8-A may conclude either `READY FOR OWNER RELEASE DECISION` or
`NOT RELEASE READY — BLOCKERS IDENTIFIED`. Neither result automatically performs
or authorizes publication.

Repository-only campaign material remains available through the
[documentation map](https://github.com/Gardlok/Matrical/blob/main/docs/README.md),
[roadmap](https://github.com/Gardlok/Matrical/blob/main/docs/roadmap.md), and
[active development record](https://github.com/Gardlok/Matrical/blob/main/docs/active-development.md).
Those files are intentionally not required for ordinary packaged-crate use.

## Design principles

- Correctness before cleverness.
- Ordinary invalid shape, index, Region, required-context, and snapshot
  reconstruction input is fallible rather than panic-driven.
- Borrowing and mutation authority stay explicit in Rust types.
- Convenience must not let a Gear escape its caller-selected Lens.
- Tags are provenance, never a command channel.
- Interchange data is inert; callers own transport and persistence authority.
- Advanced Rust features and performance work are added only when concrete
  evidence justifies them.
- Documentation, examples, and downstream-style package tests are part of the
  release contract.

## Contributing

Repository development remains evidence-gated. See the
[contribution guide](https://github.com/Gardlok/Matrical/blob/main/CONTRIBUTING.md)
and [testing procedures](https://github.com/Gardlok/Matrical/blob/main/docs/testing-procedures.md)
before selecting campaign work.

## License

Matrical is licensed under the [MIT License](LICENSE).
