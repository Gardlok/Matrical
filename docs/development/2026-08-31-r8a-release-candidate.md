# R8-A release-candidate qualification evidence

**Date:** 2026-08-31

**Qualified:** 2026-09-01

**Status:** REVIEWABLE — READY FOR OWNER RELEASE DECISION

The release-facing candidate was qualified at commit
`c91674d3c524bd50550c00bcc28c8945ca53324f`, tree
`3afe0338303f39ede6fec196e516adb2e8cc2d7b`, by GitHub Actions Qualification
run 36.

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

The stable R8-A lane queried the exact crates.io API endpoint with an identified
qualification client. It returned HTTP 404 for `matrical`, establishing that no
current crate record or registry version exists. `cargo publish --dry-run
--locked` then packaged, verified, and reached the simulated upload step before
Cargo aborted the upload because it was a dry run.

Repository-side release history at the starting baseline:

```text
GitHub Releases: none
Git tag refs:    none
```

Version before: `0.1.0`.

Final recommendation: keep `0.1.0`. Exact registry evidence shows no existing
`matrical` crate record or version conflict, and repository reconnaissance found
no prior tag or GitHub Release. R8-A does not invent a version bump solely because
it is a release gate.

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

No serious release-blocking public-symbol error was identified in the recommended
or specialized supported surface. Both toolchain lanes passed default/all-feature
check, test, doctest, Clippy, rustdoc, shipped examples, and independent
packaged-artifact consumers.

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

Crossbeam is historical in character but is still real compiled compatibility
usage. `schematics::element` uses `SegQueue`, `ArrayQueue`, and `AtomicCell`, and
the documentation-hidden historical `ElementOperation` contract names
`ElementContext`. The crate-internal historical Vector also uses `SegQueue`.

An exploratory R8-A removal proved that deleting only the obvious Vector residue
was insufficient: the ordinary locked build then failed because
`ElementContext` still requires Crossbeam. Removing Crossbeam completely would
therefore require deleting or redesigning retained compatibility residue rather
than merely dropping an unused normal dependency.

Decision:

```text
current usage: retained historical ElementContext/ElementOperation and Vector residue
keep/remove:   KEEP
authority:     no supported Matrix/Lens/Gear/snapshot behavior depends on it
rationale:     removal would broaden R8-A into compatibility/API cleanup
```

R8-A keeps Crossbeam and leaves any later removal to a separately reviewed
prototype-compatibility decision.

## Direct dependency and license audit

Lock-resolved versions and roles:

| Dependency | Version | Scope | Purpose | Direct license |
| --- | --- | --- | --- | --- |
| ndarray | 0.15.6 | normal | private dense Matrix backend | MIT OR Apache-2.0 |
| crossbeam | 0.8.4 | normal | retained historical compatibility residue | MIT OR Apache-2.0 |
| serde | 1.0.229 | optional normal | MatrixSnapshot Serialize/Deserialize | MIT OR Apache-2.0 |
| criterion | 0.7.0 | dev-only | benchmark harness | Apache-2.0 OR MIT |
| serde_json | 1.0.151 | dev-only | snapshot tests/example | MIT OR Apache-2.0 |

No direct dependency license identified above conflicts with Matrical's MIT
distribution. CI records the default and serde-enabled normal dependency trees.

The final dependency graph is unchanged from the accepted R7 baseline. CI
recorded the same `Cargo.lock` SHA-256 for base and candidate:

```text
5975a977e470eb8ed55e14a9b6d9cdb4c711f3931f53accab7e6da78710119f1
```

## Package, downstream, examples, and full qualification

Qualification run 36 passed on:

```text
MSRV:   rustc 1.85.0 / cargo 1.85.0
stable: rustc 1.98.0 / cargo 1.98.0
```

Both lanes passed:

```text
check/test/doctest/Clippy/rustdoc — default features
check/test/doctest/Clippy/rustdoc — all features
all six shipped examples
cargo bench --locked --no-run
cargo package --locked --list and package verification
unexpected packaged-file audit
default packaged-artifact downstream consumer
Serde packaged-artifact downstream consumer
git diff --check
Markdown relative-link audit
packaged Markdown-link audit
final-newline audit
unsafe audit
tracked artifact audit
```

The package contains exactly 42 files: Cargo metadata, license, root README and
changelog, library source, six examples, two benchmarks, and the three selected
release-facing documents. CI/editor configuration, development evidence, prompt
archives, tests, and generated artifacts are absent.

Package measurements:

| Toolchain | Archive bytes | Unpacked bytes |
| --- | ---: | ---: |
| Rust 1.85.0 | 41,516 | 176,234 |
| stable 1.98.0 | 41,598 | 176,234 |

The archive-byte difference is toolchain-generated package metadata; the unpacked
payload size is identical. The independent consumers depend on an unpacked
generated `.crate`, never on the repository checkout, and use only public
Matrical API.

The stable lane additionally passed exact crates.io reconnaissance and
`cargo publish --dry-run --locked`. No credentialed or non-dry-run publication
occurred.

## Performance baseline

R8-A does not alter accepted Lens traversal mechanics. The candidate compiles the
existing benchmark harness on both qualification toolchains and preserves the R6
owner-host measurement record instead of reopening the expensive benchmark
campaign without a performance-sensitive code change.

## Release blockers and recommendation

No release-readiness blocker remains in the qualified R8-A scope.

```text
R8-A result: READY FOR OWNER RELEASE DECISION
Recommended owner action: Teamlead review, then an explicit owner decision
```

Qualification does not itself authorize a tag, GitHub Release, release date, or
crates.io publication. Those remain separate owner-controlled actions.
