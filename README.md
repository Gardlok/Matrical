# Matrical

**Status: rehabilitation campaign — architecture experiment, not production-ready**

Matrical is an experimental Rust library for expressing matrix work as a
combination of selection, transformation, context, and metadata.

The original prototype explored generic elements, concurrent containers,
runtime strategies, validation, database-backed data, and zero-copy views. The
rehabilitation campaign is retaining the most distinctive part of that work—the
nomenclature and semantic model—while rebuilding the implementation around
small, testable invariants.

Matrical is not currently ready for crates.io consumers. The public API may
change substantially while the core is reconstructed.

## The model

Matrical's working vocabulary is:

- **Matrix** — the owned data and its validated shape.
- **Lens** — a bounded view or selection over matrix data.
- **Gear** — a transformation applied through a Lens.
- **Cog** — context or policy that influences a Gear.
- **Tag** — metadata or provenance attached to data or an operation.

The intended flow is:

```text
Matrix -> Lens -> Gear (+ Cog) -> result (+ Tags)
```

These names are not decorative aliases. Each concept must own a distinct,
documented responsibility and must preserve the Matrix invariants.

## Direction

The recommended direction is a semantic matrix-transformation library built on
proven storage and numerical foundations. Matrical should differentiate itself
through validated regions, composable views, contextual transformations, and
provenance—not by reimplementing every linear-algebra kernel.

The campaign begins with a deliberately narrow sequence:

1. establish a reproducible build and truthful project baseline;
2. rebuild `Matrix`, shape, index, region, and error invariants;
3. introduce safe immutable and mutable Lenses;
4. introduce testable Gears, Cogs, and Tags;
5. add examples, property tests, benchmarks, and measured optimization;
6. evaluate optional parallel and persistent backends only after the sequential
   contract is sound.

See the [rehabilitation roadmap](docs/roadmap.md) for slice boundaries and exit
criteria.

## Design principles

- Correctness before cleverness.
- Invalid shape and region states should be difficult or impossible to create.
- Public fallible operations return typed errors rather than panic.
- Zero-copy views borrow from their source and make that relationship explicit.
- Concurrency and parallelism require defined semantics and measured benefit.
- Advanced Rust features are used when they strengthen the contract, not merely
  to demonstrate sophistication.
- Dependencies must have an implemented purpose and a bounded feature surface.
- Documentation, examples, and tests are part of the API.

## Repository guide

- [Documentation map](docs/README.md)
- [Architecture vision](docs/architecture/vision.md)
- [Rehabilitation roadmap](docs/roadmap.md)
- [Active development](docs/active-development.md)
- [Testing procedures](docs/testing-procedures.md)
- [Teamlead campaign playbook](docs/teamlead-playbook.md)
- [Contributing](CONTRIBUTING.md)

## Current limitations

The historical source contains incomplete and placeholder APIs. In particular,
the current `Matrix` abstraction is not yet a usable two-dimensional container,
some validation paths do not execute their configured strategies, and core
matrix tests and examples are absent. The first implementation slices will
replace or remove those paths rather than claim compatibility with unfinished
behavior.

No stability, performance, thread-safety, zero-copy, database-integration, or
release-readiness claim should be inferred until its roadmap gate has passed.

## Contributing

Matrical is being rehabilitated through small, focused development sessions and
reviewable pull requests. Start with [CONTRIBUTING.md](CONTRIBUTING.md) and the
[active campaign record](docs/active-development.md) before selecting work.

## License

Matrical is licensed under the [MIT License](LICENSE).
