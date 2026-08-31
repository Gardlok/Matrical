# R8-A release-candidate qualification evidence

**Date:** 2026-08-31

**Status:** qualification in progress; this document must be updated from the
final exact candidate and GitHub Actions evidence before R8-A closeout.

## Exact starting identity

```text
repository Gardlok/Matrical
branch     main
commit     f28fc380926c8175ff9b5faeb092be5bd7426245
tree       5d8bac5a769ba0fc3b77dc4b107ccd90d6c0dd86
version    0.1.0
MSRV       Rust 1.85.0
```

The starting commit is the verified merge of PR #12. Baseline GitHub
reconnaissance found no GitHub Releases and no tag refs.

R7 closeout for R8-A is:

```text
R7-A: COMPLETE — OWNER ACCEPTED — MERGED IN PR #12
R7-B: DEFERRED — no demonstrated second-provider/integration need
R7:   COMPLETE — OWNER ACCEPTED
```

The R7-B disposition does not permanently reject sparse/mapped storage or future
integrations.

## Package metadata

Before R8-A, `[package]` declared only name, version, edition, rust-version, and
author. The candidate adds release-facing facts:

```text
description Checked matrix selection and transformation with typed borrowing views and versioned snapshots.
license     MIT
repository  https://github.com/Gardlok/Matrical
readme      README.md
documentation omitted intentionally; use normal crates.io/docs.rs behavior after publication
homepage    omitted; no independent homepage is established
keywords    matrix, selection, transformation
categories  data-structures
```

The committed `LICENSE` is the authority for MIT metadata. The candidate also
uses an explicit package allowlist so release-facing source/docs/examples/benches
are shipped while repository campaign evidence, CI/editor settings, and generated
artifacts are not.

## Registry/name and version reconnaissance

Package name: `matrical`.

Exact crates.io API reconnaissance and `cargo publish --dry-run --locked` are
performed by the stable R8-A CI lane. Until that result is captured, registry
occupancy/history remains **PENDING** rather than inferred from search absence.

Repository-side release history at the starting baseline:

```text
GitHub Releases: none
Git tag refs:    none
```

Version before: `0.1.0`.

Current recommendation: keep `0.1.0` unchanged unless exact registry evidence
shows a conflict or historical external release. R8-A does not invent a version
bump solely because it is a release gate. Final recommendation: **PENDING exact
registry/qualification evidence**.

## Public API audit

Recommended supported surface:

```text
matrical::prelude::*
supported named crate-root exports
matrical::schematics
matrical::strategies
```

Specialized supported surface:

```text
matrical::snapshot
matrical::MatrixSnapshot
DENSE_SNAPSHOT_VERSION
```

Documentation-hidden historical operation/error/context compatibility residue is
not recommended for new downstream callers. ndarray storage and internal Lens /
Gear representation remain private implementation.

No serious release-blocking public-symbol error has been identified in the
recommended or specialized supported surface. Final compile/rustdoc/downstream
proof: **PENDING CI**.

## Dense snapshot schema v1 policy

The v1 logical schema remains:

```text
version: u32
rows: u64
columns: u64
row_major: sequence of T in deterministic logical row-major order
```

Candidate release policy:

> Within a released line, incompatible dense snapshot semantics must not silently
> change under version 1. A future incompatible representation uses another
> explicit snapshot schema version.

Rust SemVer and snapshot schema versions are separate concepts. This policy does
not promise universal indefinite v1 support; any later migration/retirement must
be explicit rather than silently reinterpreting version 1.

## Crossbeam decision

Baseline source search found Crossbeam only in crate-internal historical
`schematics::vector` prototype residue through `crossbeam::queue::SegQueue`.
`Vector` is not part of the supported public surface and no current Matrix,
Lens, Gear, snapshot, example, test, or benchmark contract requires it.

R8-A therefore removes the historical Vector module and Crossbeam normal runtime
dependency. This is the one dependency cleanup explicitly authorized by the
mission and does not broaden into general prototype cleanup.

## Direct dependency and license audit

Lock-resolved baseline/candidate versions and roles:

| Dependency | Version | Scope | Purpose | Direct license |
| --- | --- | --- | --- | --- |
| ndarray | 0.15.6 | normal | private dense Matrix backend | MIT OR Apache-2.0 |
| serde | 1.0.229 | optional normal | MatrixSnapshot Serialize/Deserialize | MIT OR Apache-2.0 |
| criterion | 0.7.0 | dev-only | benchmark harness | Apache-2.0 OR MIT |
| serde_json | 1.0.151 | dev-only | snapshot tests/example | MIT OR Apache-2.0 |
| crossbeam | 0.8.4 baseline only | removed | historical internal Vector queue | MIT OR Apache-2.0 |

No direct dependency license identified above conflicts with Matrical's MIT
distribution. CI records the default and serde-enabled normal dependency trees.

## Package, downstream, examples, and full qualification

The final exact evidence is intentionally not predicted. The following are
**PENDING GitHub Actions qualification** on both Rust 1.85.0 and stable:

```text
Cargo.lock SHA-256 before/after
cargo package --locked --list
unexpected packaged-file audit
.crate archive size
unpacked package size
cargo package --locked
default packaged-artifact downstream consumer
serde packaged-artifact downstream consumer
all shipped examples
check/test/doctest/clippy/doc — default features
check/test/doctest/clippy/doc — all features
cargo bench --locked --no-run
cargo publish --dry-run --locked (stable only)
git diff --check
Markdown relative-link audit
final-newline audit
unsafe audit
tracked artifact audit
```

The independent consumers depend on an unpacked generated `.crate`, never on the
repository checkout, and use only public Matrical API.

## Performance baseline

R8-A does not alter accepted Lens traversal mechanics. The candidate compiles the
existing benchmark harness on both qualification toolchains and preserves the R6
owner-host measurement record instead of reopening the expensive benchmark
campaign without a performance-sensitive code change.

## Release blockers and recommendation

Current blocker before closeout: final exact CI/package/registry evidence has not
yet been recorded.

```text
R8-A result: PENDING QUALIFICATION
Recommended owner action: no tag, release, or publish action yet
```

This file will be corrected to one of the authorized R8-A exit states after the
final exact candidate completes qualification.
