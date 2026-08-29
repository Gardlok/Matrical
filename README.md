# Matrical

**Status: rehabilitated core working; API-learning surface active; version 0.1.0**

Matrical is a semantic matrix-transformation library built around validated
geometry, borrowing selections, typed transformations, contextual policy, and
provenance. It uses a private dense `ndarray::Array2<T>` backend while exposing a
Matrical-owned contract for shape, selection, transformation authority, and
execution reporting.

The rehabilitated R2–R4 core is working and qualified on the declared Rust 1.85
MSRV and current stable through the repository qualification lanes. Matrical is
still `0.1.0`: the public API may change before the first rehabilitated release,
and this repository is **not** yet claiming production or release readiness.

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

The normal flow is:

```text
Matrix
  -> Lens / LensMut
  -> Gear (+ typed Cog)
  -> ExecutionReport (+ Tags)
```

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
            0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0,
        ],
    )?;
    let region = Region::new(shape, 1..3, 1..3)?;
    let tags = vec![
        Tag::source("r5-quickstart"),
        Tag::stage(TagStage::Transform),
        Tag::sequence(1),
    ];

    {
        let lens = matrix.lens(region)?;
        let report = execute_read(&SumGear, &lens, &Cog::new(()), tags.clone())?;
        println!(
            "{} {:?} {:?} -> {}",
            report.gear(),
            report.effect(),
            report.region(),
            report.output()
        );
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
        println!(
            "{} {:?} affected {} values; tags={:?}",
            report.gear(),
            report.effect(),
            report.output(),
            report.tags()
        );
        assert_eq!(*report.output(), 4);
    }

    assert_eq!(
        matrix.into_row_major(),
        vec![
            0.0, 1.0, 2.0, 3.0, 4.0, 15.0, 16.0, 7.0, 8.0, 19.0, 20.0, 11.0,
        ]
    );
    Ok(())
}
```

`Region` bounds are half-open. `Lens` and `LensMut` expose coordinates local to
the selection, and creating/reading/iterating a Lens does not intentionally
allocate. `Lens::to_row_major()` is the explicit cloning conversion.

## Public API policy

- `matrical::prelude::*` is the recommended everyday API.
- named crate-root exports are the supported discoverable API.
- `matrical::schematics` and `matrical::strategies` group the same supported
  concepts for deeper navigation.
- prototype operation/Element/Vector/SQL scaffolding is not part of the learning
  contract; some compatibility residue remains intentionally hidden while 0.1.0
  rehabilitation continues.

Matrical does not expose `ndarray` as part of the Matrix/Lens contract and does
not give a Gear direct Matrix or arbitrary Region-selection authority.

## Where next

- [Getting started](docs/getting-started.md) — task-oriented walkthrough from
  construction through custom Gears and errors.
- Crate rustdoc — start at `matrical`, then follow `prelude`, `schematics`, and
  `strategies`.
- [Runnable examples](examples/) — including the quickstart and a downstream
  custom Gear.
- [Architecture vision](docs/architecture/vision.md) — responsibilities and
  authority boundaries.
- [API stability policy](docs/api-stability.md) — what `0.1.0` does and does not
  promise.
- [Roadmap](docs/roadmap.md) — campaign gates and future work.
- [Documentation map](docs/README.md) — testing and development evidence.

Rehabilitation history remains available under `docs/development/`, but normal
library usage should not require reading it.

## Design principles

- Correctness before cleverness.
- Ordinary invalid shape, index, Region, and required-context input is fallible
  rather than panic-driven.
- Borrowing and mutation authority stay explicit in Rust types.
- Convenience must not let a Gear escape its caller-selected Lens.
- Tags are provenance, never a command channel.
- Advanced Rust features and performance work are added only when concrete
  evidence justifies them.
- Documentation, examples, and downstream-style tests are part of the API.

## Contributing

Matrical is still being rehabilitated through focused, reviewable slices. Start
with [CONTRIBUTING.md](CONTRIBUTING.md), the [active campaign record](docs/active-development.md),
and the [testing procedures](docs/testing-procedures.md) before selecting work.

## License

Matrical is licensed under the [MIT License](LICENSE).
