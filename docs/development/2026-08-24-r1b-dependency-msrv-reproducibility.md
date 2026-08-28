# R1-B dependency and MSRV reproducibility

**Decision date:** 2026-08-24

**Implementation and validation:** 2026-08-25

**Status:** local review candidate — Teamlead gate

## Scope and starting identity

R1-B restores reproducible dependency resolution and allows the Rust 1.85 lane
to reach Matrical source. It begins from the verified GitHub merge of PR #3:

```text
repository Gardlok/Matrical
branch     main
commit     1c5ec09346f249496f1bb2e72095e073b348568a
tree       9677aa266b8aa403b4cdbfbe81c155c7a6a77861
version    0.1.0
```

GitHub reports `main` as the default branch. Preflight found no open pull
requests or issues and no remote branch overlap with the authorized manifest,
lockfile, toolchain, source-import, or documentation paths. Implementation used
`rehab/r1b-dependency-msrv-reproducibility`; the worktree was clean before the
branch was created.

This is working-tree evidence. Nothing is staged, committed, pushed, published,
or attributed to a final exact candidate SHA.

## Accepted owner decisions

1. Rust 1.85.0 is Matrical's current MSRV.
2. Matrical commits its root `Cargo.lock` for reproducible development and CI
   qualification.
3. Downstream library users remain free to resolve dependencies within
   Matrical's published constraints.
4. SurrealDB leaves the immediate dependency graph and remains deferred
   optional-integration research.
5. Rayon remains deferred until R6 supplies benchmark evidence.
6. Criterion remains absent until real benchmark targets exist.
7. Serde remains absent until serialization behavior exists.
8. DashMap remains absent because its compiled imports support only commented
   prototype code.
9. Crossbeam is temporarily retained because historical compiled Matrix,
   Vector, and Element types still use it. Removal requires later
   source/invariant reconstruction.
10. `ndarray` remains the intended dense-storage dependency.
11. The unfinished 0.1.0 API has no compatibility promise, while R1-B avoids
    unrelated source behavior changes.

## Package metadata and toolchain policy

The package remains `matrical` version `0.1.0`, edition 2021. `Cargo.toml` now
uses valid `authors = ["Anthony Gardner"]` metadata and declares
`rust-version = "1.85"`.

`rust-toolchain.toml` pins channel `1.85.0`, uses the minimal profile, and
installs Rustfmt and Clippy. It does not pin a host-specific target. The
secondary lane continues to require an explicit `+stable` override.

## Direct dependency surface

### Historical manifest

```text
normal: ndarray 0.15.3, rayon 1.5.1, serde 1.0.126 + derive,
        crossbeam 0.8.2, surrealdb 1.0.0-beta.9, dashmap 5.4.0
dev:    criterion 0.4
```

### R1-B manifest

```text
normal: ndarray 0.15.3, crossbeam 0.8.2
dev:    none
added:  none
```

The retained constraints were not upgraded or broadly rewritten. The new
lockfile resolves `ndarray 0.15.6` and `crossbeam 0.8.4` within those historical
constraints.

### Evidence for removals and retention

| Dependency | R1-B classification |
| --- | --- |
| SurrealDB | No compiled Matrical API use; R1-A proved a route through SurrealDB 1.5.6 into Rust-1.85-incompatible ICU dependencies. Persistence remains deferred. |
| Rayon | No Rust-source reference or benchmark evidence; parallel execution remains deferred until R6. |
| Serde | Only an unused `U32Deserializer` import existed; no serialization implementation or derive existed. |
| DashMap | Imports existed in `lib.rs` and `operations/sort.rs`, but runtime use was confined to commented prototype code. |
| Criterion | No benchmark target or reachable benchmark behavior existed. |
| Crossbeam | Compiled historical Matrix, Vector, and Element types still use `ArrayQueue`, `SegQueue`, and `AtomicCell`; removing it would combine dependency pruning with source/invariant reconstruction. |
| ndarray | Compiled Gear and Cog behavior and Gear tests use `Array2`; it remains the intended dense-storage foundation. |

Only the Serde and DashMap compiled imports made invalid by pruning were
removed. No adjacent warning cleanup, commented-code cleanup, module removal,
public API redesign, or runtime behavior change was attempted.

## Lockfile policy and identity

R1-A produced two different ignored resolutions from unchanged source:

| Resolution | Date | Lines | Packages | SHA-256 |
| --- | --- | ---: | ---: | --- |
| A | 2026-08-20 | not recorded | not recorded | `d19f46db1941a20f82cf6d8f168c08cf29c3ea7be06145f48365b23264f351ad` |
| B | 2026-08-21 | 4,650 | 471 | `927759468cc6dfe8e65a93bc2ec9109788cc9f9fb49a984eb74f6a3d26daa458` |

R1-B removed both historical lockfile ignore rules and replaced their stale
library/executable guidance with the accepted committed-development-lockfile
policy. Rust/Cargo 1.85.0 generated a fresh lockfile after manifest pruning; no
R1-A resolution was reused.

```text
Cargo.lock lines             129
Cargo.lock SHA-256           8abe052c6d793e87df19c1e6ade379caf3cad562eea693a946dc39c9e7180020
resolved packages            14 (including matrical)
retained direct resolutions  ndarray 0.15.6; crossbeam 0.8.4
```

Every subsequent Cargo command used `--locked`. The candidate lockfile line
count and SHA-256 remained unchanged throughout both toolchain lanes.

## Validation environment

The cloud host initially had no Rust toolchain, matching the environmental
limitation recorded in R1-A. R1-B installed isolated toolchain state outside the
repository and used an R1-B-specific workspace-backed cache because the cloud
sandbox does not expose the operator's home cache as writable:

```text
XDG_CACHE_HOME  /workspace/scratch/9fd1f7976c6f/cache
Cargo home      /workspace/scratch/9fd1f7976c6f/cache/cargo
Rustup home     /workspace/scratch/9fd1f7976c6f/cache/rustup
target root     /workspace/scratch/9fd1f7976c6f/cache/matrical/r1b/cargo-target
temporary root  /workspace/scratch/9fd1f7976c6f/cache/matrical/r1b/test-tmp
evidence root   /workspace/scratch/9fd1f7976c6f/cache/matrical/r1b/evidence
```

No repository-local `target/` was created. The workspace filesystem had 55 GiB
and 4,077,927 inodes available at preflight. No unrelated Cargo or Rust compiler
process owned the target root. The unrelated `RUST_LOG` variable was unset for
qualification; no ROSE-specific environment variable was present.

Primary toolchain:

```text
rustc 1.85.0 (4d91de4e4 2025-02-17)
cargo 1.85.0 (d73d2caf9 2024-12-31)
```

Secondary comparison toolchain:

```text
rustc 1.98.0 (88d9e12ae 2026-08-18)
cargo 1.98.0 (797e8a9bc 2026-08-05)
```

## Rust 1.85 results

| Command | Exit | Result |
| --- | ---: | --- |
| `cargo +1.85.0 generate-lockfile` | 0 | Fresh post-pruning lockfile generated; 13 dependency packages locked. |
| `cargo +1.85.0 metadata --locked --format-version 1` | 0 | Cargo accepts package metadata, `rust-version`, and the candidate locked graph. |
| `cargo +1.85.0 tree --locked` | 0 | Only Matrical, Crossbeam, ndarray, and their retained transitives resolve; SurrealDB is absent. |
| `cargo +1.85.0 check --locked --all-targets` | 0 | Rust 1.85 reaches and compiles Matrical source. |
| `cargo +1.85.0 test --locked --all-targets` | 0 | 24 matched; 24 passed; 0 failed; 0 ignored; 0 filtered. |
| `cargo +1.85.0 test --locked --doc` | 0 | 0 examples discovered; this is a coverage gap, not behavioral evidence. |
| `cargo +1.85.0 clippy --locked --all-targets` | 101 | Stops on the inherited recursive `MatricalError` `Debug` implementation. |
| `cargo +1.85.0 doc --locked --no-deps` | 0 | Documentation generated successfully without the former invalid-author metadata warning. |

The first Clippy error is unchanged in substance:

```text
src/error.rs:40
write!(f, "{:?}", self)
using `self` as `Debug` in `impl Debug` will cause infinite recursion
clippy::recursive_format_impl
```

Clippy emitted 84 library warnings and 89 test-target warnings before stopping
on that one deny-by-default error. It did not report a candidate-owned error.
R1-B records but does not repair this source defect.

## Current-stable comparison

| Command | Exit | Result |
| --- | ---: | --- |
| `cargo +stable check --locked --all-targets` | 0 | All targets compile on Rust 1.98.0. |
| `cargo +stable test --locked --all-targets` | 0 | 24 matched; 24 passed; 0 failed; 0 ignored; 0 filtered. |
| `cargo +stable test --locked --doc` | 0 | 0 examples discovered. |
| `cargo +stable clippy --locked --all-targets` | 101 | Same inherited `clippy::recursive_format_impl` blocker. |
| `cargo +stable doc --locked --no-deps` | 0 | Documentation generated successfully. |

Stable Clippy emitted 83 library warnings and 88 test-target warnings before the
same single hard error. The stable comparison supports the classification but
does not substitute for the successful Rust 1.85 compilation and test lane.

## Mechanical and documentation evidence

The strict path-scoped Rustfmt check exits 1 because the two historical source
files contain pre-existing formatting debt. The same check against the accepted
starting commit also exits 1 with the same import-order and excess-blank-line
classes. R1-B did not run automatic formatting or absorb that debt. Final diff
hygiene, whitespace, newline, relative-link, exact-path, and lockfile-stability
checks are recorded in the developer handoff after the documentation is
complete.

## Residual warnings and deferred debt

- Rust 1.85 check reports 62 library warnings and 65 test-target warnings.
- Stable check reports 61 library warnings and 64 test-target warnings.
- The warning surface remains primarily unused imports, dead fields, unused
  variables, ambiguous glob re-exports, and incomplete historical modules.
- The recursive `MatricalError` `Debug` implementation remains a confirmed
  correctness defect and the first source blocker.
- Rustdoc still discovers zero examples.
- The 24 tests still do not qualify Matrix, Lens, Cog, Tag, error formatting,
  invalid builder states, or the intended semantic flow.
- Crossbeam remains temporary inherited architecture debt.
- No CI lane exists yet; R1-B defines reproducible local inputs but does not add
  CI outside its authorized boundary.

## Recommended next slice

After R1-B is committed, qualified at its exact functional SHA, reviewed, and
owner-accepted, recommend a narrow **R1-C source-correctness slice** beginning
with the recursive `MatricalError` formatting defect and its focused regression
coverage. R1-C should classify the public error visibility/contract separately
from the minimal recursion repair rather than expanding directly into Matrix,
Lens, Gear, Cog, or Tag reconstruction.

No R1-C implementation begins from this report.

```text
R1-B STATUS: LOCAL REVIEW CANDIDATE — TEAMLEAD GATE
R1-C STATUS: BLOCKED
```
