# R1-A historical baseline reconnaissance

**Date:** 2026-08-20

**Executable continuation:** 2026-08-21

**Status:** R1-A implementation/evidence complete; documentation is a local
review candidate; publication remains at the owner gate

## Scope and evidence boundary

This report classifies the unchanged Matrical 0.1.0 prototype at the accepted
R1-A starting identity:

```text
repository Gardlok/Matrical
branch     main
commit     dea2adb83404743558ae9da7a3d94aefdad4b903
tree       b46d45e75c9337a2f28f037fea5ac8706c53098f
version    0.1.0
```

The initial cloud commands were invoked from local branch
`docs/r1a-baseline-reconnaissance` while `HEAD` identified that exact commit.
The Orion continuation used a detached worktree at the same exact commit and
tree. No Rust source, manifest, dependency, feature, CI, version, or other
executable file was changed.

The initial cloud host had no `rustup`, `rustc`, or `cargo` executable. Those
environmental results remain recorded below. The executable investigation then
continued on Orion from a detached exact-SHA worktree using the accepted Rust
1.85.0 candidate and Rust 1.93.1 as a distinct secondary classification lane.

R1-A reconnaissance is successful because it established the actual resolution,
MSRV, source, test, lint, and documentation boundaries. It does **not** qualify
the baseline as reproducible or Rust-1.85-compatible: consecutive fresh
resolutions drifted, and the resolved dependency graph blocked Rust 1.85 before
Matrical source compilation. The same locked graph compiled and passed all 24
discovered tests on Rust 1.93.1.

## Baseline and worktree

Initial clean-checkout evidence, recorded before creating the working branch:

```text
remote         https://github.com/Gardlok/Matrical.git
default branch main
HEAD           dea2adb83404743558ae9da7a3d94aefdad4b903
HEAD tree      b46d45e75c9337a2f28f037fea5ac8706c53098f
status         ## main...origin/main
```

GitHub and remote-branch reconnaissance found:

- no open pull requests;
- no open issues;
- `docs/r0f-foundation-closeout` at `41dd29b1...`, fully merged into `main`;
- `docs/rehabilitation-foundation` at `73e755f1...`, fully merged into `main`;
- historical branch `git@github.com-Gardlok/Strustegy.git` at `db9c76de...`,
  diverged before the rehabilitation documentation and containing no R1-A
  report work. It is not an overlap, but branch retirement is an owner-controlled
  repository-hygiene decision outside R1-A.

No repository-local `AGENTS.md` or equivalent agent instruction file was
present. The governing campaign, architecture, consumer, testing, Teamlead, and
R1-A prompt documents named in the dispatch were read before editing.

## Execution environments and toolchains

### Initial cloud environment

```text
host Linux ce00d19c7323 6.18.35 #1 SMP Mon Jul 27 18:07:50 UTC 2026
     x86_64 x86_64 x86_64 GNU/Linux
```

| Command | Exit | Result |
| --- | ---: | --- |
| `uname -a` | 0 | Host string above. |
| `rustup show` | 127 | `rustup: command not found`. |
| `rustc --version --verbose` | 127 | `rustc: command not found`. |
| `cargo --version --verbose` | 127 | `cargo: command not found`. |
| `rustc +1.85.0 --version --verbose` | 127 | `rustc: command not found`. |
| `cargo +1.85.0 --version --verbose` | 127 | `cargo: command not found`. |

Rust 1.85.0 was therefore an unmet environmental prerequisite on the initial
host. No different compiler was substituted there, and the exit-127 outcomes
were not reclassified after Orion evidence became available.

### Qualified Orion continuation

Orion used a detached worktree at the exact accepted commit and tree, with build
products directed to the reusable home-backed Matrical cache rather than the
repository:

| Lane | `rustc` | `cargo` | Purpose |
| --- | --- | --- | --- |
| MSRV candidate | `1.85.0 (4d91de4e4 2025-02-17)` | `1.85.0 (d73d2caf9 2024-12-31)` | Primary dependency and compatibility classification. |
| Secondary stable | `1.93.1 (01f6ddf75 2026-02-11)` | `1.93.1 (083ac5135 2025-12-15)` | Distinct current-toolchain source, test, Clippy, and rustdoc classification. |

The stable lane does not substitute for or establish the accepted Rust 1.85.0
MSRV.

## Manifest and target inventory

Static manifest metadata:

```text
package name    matrical
package version 0.1.0
edition         2021
author          Anthony Gardner
features        none declared
```

The manifest has no explicit `[lib]`, binary, example, benchmark, build-script,
workspace, profile, or feature declarations. Filesystem and tracked-file
inventory establishes the following surface:

| Surface | Evidence | Classification |
| --- | --- | --- |
| Library | `src/lib.rs` | One implicit library target. |
| Binaries | No `src/main.rs` or `src/bin/` | None. The private `fn main()` in `src/strategies/cog.rs:171-181` is an ordinary module function, not a binary target. |
| Unit tests | 24 active `#[test]` declarations | 19 in `src/operations/mechanics.rs`; 5 in `src/strategies/gear.rs`. All 24 passed on Rust 1.93.1. |
| Top-level test module | `src/tests.rs` | Included under `cfg(test)`, but all intended Matrix tests are commented out and it declares no active test. |
| Integration tests | No `tests/` directory | None. |
| Doctests | No actual `//!` or `///` rustdoc | Rustdoc test discovery passed but found zero examples. Slash separators in `lens.rs` are not documentation. |
| Examples | No `examples/` directory | None. |
| Benchmarks | `src/bench.rs` only | Orphaned/commented prototype source; not referenced from `lib.rs` and not a Cargo benchmark target. |
| Build scripts | No `build.rs` | None. |
| Features | No `[features]` table | Only the default dependency graph can be described statically. |
| CI | No `.github/workflows/` | None. |

The initial cloud worktree contained no `Cargo.lock`; Cargo was unavailable and
created neither a lockfile nor `target/`. Orion confirmed that lockfiles are
ignored, including the root rule at `.gitignore:20` (`/Cargo.lock`). Orion's
generated lockfile remained ignored and uncommitted, and its target directory
remained outside the repository.

## Dependency resolution and MVECv1 result

### Initial cloud limitation

The required ordered ladder was first invoked in the cloud environment. Every
command stopped before execution because Cargo was absent:

| Exact command | Exit | Classification |
| --- | ---: | --- |
| `cargo +1.85.0 metadata --format-version 1` | 127 | Environmental prerequisite; Cargo absent. |
| `cargo +1.85.0 check --all-targets` | 127 | Environmental prerequisite; compilation did not start. |
| `cargo +1.85.0 test --all-targets` | 127 | Environmental prerequisite; zero tests executed. |
| `cargo +1.85.0 test --doc` | 127 | Environmental prerequisite; doctest discovery did not run. |
| `cargo +1.85.0 clippy --all-targets` | 127 | Environmental prerequisite; no lint result. |
| `cargo +1.85.0 doc --no-deps` | 127 | Environmental prerequisite; rustdoc did not run. |

The complete initial diagnostic was:

```text
cargo: command not found
```

These are preserved environmental results, not product failures and not the
final R1-A executable conclusion.

### Orion resolution and lockfile drift

Rust 1.85 metadata resolution completed on Orion:

```text
cargo +1.85.0 metadata --format-version 1
exit 0
```

The unchanged source tree produced different ignored lockfiles on consecutive
dates:

| Resolution | Date | SHA-256 | Additional evidence |
| --- | --- | --- | --- |
| A | 2026-08-20 | `d19f46db1941a20f82cf6d8f168c08cf29c3ea7be06145f48365b23264f351ad` | First captured resolution. |
| B | 2026-08-21 | `927759468cc6dfe8e65a93bc2ec9109788cc9f9fb49a984eb74f6a3d26daa458` | 4,650 lines; 471 packages locked. |

At minimum, Resolution B advanced `icu_provider` from `2.3.0` to `2.3.1`.
Fresh `cc 1.4.4` and `zerovec-derive 0.11.6` artifacts were also observed.
Because the repository ignores and does not commit `Cargo.lock`, exact
repository qualification is date-dependent even when the source commit and tree
are unchanged. Resolution B remained ignored and uncommitted throughout R1-A.

### Rust 1.85 compatibility result

The locked MSRV check failed before Matrical source compilation:

```text
cargo +1.85.0 check --locked --all-targets
exit    101
elapsed less than one second
```

Resolved transitive dependencies requiring Rust 1.86 through 1.88 included:

- `ar_archive_writer 0.5.3`;
- the `darling 0.23.0` family;
- the ICU `2.3.x` family;
- `idna_adapter 1.2.2`;
- `psm 0.1.32`;
- the `serde_with 3.22.0` family;
- the `time 0.3.55` family.

The locked graph did not change. A confirmed inverse route into the incompatible
ICU graph is:

```text
matrical 0.1.0
`-- surrealdb 1.5.6
    `-- surrealdb-core 1.5.6
        `-- object_store 0.8.0 / url 2.5.8
            `-- idna 1.1.0
                `-- idna_adapter 1.2.2
                    |-- icu_normalizer 2.3.0
                    `-- icu_properties 2.3.0
                        `-- icu_provider 2.3.1
```

Rust 1.85 support is therefore **not established**. The first confirmed MSRV
blocker is dependency resolution, and SurrealDB supplies a demonstrated path
into one incompatible family. The remaining Rust 1.85 test, Clippy, and rustdoc
phases cannot qualify source behavior while this check boundary is blocked.

### Rust 1.93.1 secondary classification

Resolution B compiled successfully on the distinct stable lane:

```text
cargo +stable check --locked --all-targets
exit             0
elapsed          132 seconds
library warnings 65
test warnings    68
```

This proves that the unchanged Matrical source compiles with Resolution B on
Rust 1.93.1. It does not establish Rust 1.85 compatibility.

All discovered unit tests executed successfully:

```text
cargo +stable test --locked --all-targets
exit    0
elapsed 81 seconds

24 passed
0 failed
0 ignored
```

The 24 tests remain concentrated in generic validation mechanics and Gear.
Matrix, Lens, Cog, Tag, recursive error formatting, invalid builder states, and
the accepted semantic flow still lack meaningful exercised contracts.

Rustdoc test discovery also succeeded, but found no examples:

```text
cargo +stable test --locked --doc
exit 0

0 passed
0 failed
0 documentation examples discovered
```

This is a successful command and a documentation-coverage gap, not a doctest
qualification of public behavior.

### Clippy correctness boundary

```text
cargo +stable clippy --locked --all-targets
exit                 101
warning headers      97
Clippy lint mentions 7
```

Clippy reported 86 library warnings and 91 test-target warnings before stopping
on a deny-by-default correctness lint at `src/error.rs:40`:

```text
write!(f, "{:?}", self)

using `self` as `Debug` in `impl Debug` will cause infinite recursion
clippy::recursive_format_impl
```

This confirms the static finding as a source-level defect: debug-formatting a
`MatricalError` can recurse until stack overflow. The 24 passing tests do not
exercise that path. R1-A records but does not repair it.

### Documentation generation

```text
cargo +stable doc --locked --no-deps
exit    0
elapsed 24 seconds
```

The generated crate index at `target/doc/matrical/index.html` in the external
target was 4,811 bytes with SHA-256
`d5391a978deedb629a7594b36cf1fa4c776becffcc40cf2af6da201d258877ec`.
The only documentation-build warning header was:

```text
unused manifest key: package.author
```

Documentation generation therefore succeeds on Rust 1.93.1, while the manifest
uses the invalid singular key `author` instead of Cargo's `authors` metadata.
All locked Orion commands left Resolution B unchanged.

## Direct-dependency purpose classification

This table classifies direct manifest entries against reachable source. It does
not substitute for a resolved dependency graph.

| Direct dependency | Static evidence | Classification |
| --- | --- | --- |
| `ndarray = "0.15.3"` | `Array2<f64>` storage in `cog.rs` and `gear.rs`; region mutation with `s![]`; five Gear tests construct arrays. | Implemented prototype runtime and test use, but operations bypass `Matrix` and do not establish the accepted semantic model. This is the intended future dense foundation. |
| `crossbeam = "0.8.2"` | `ArrayQueue`, `SegQueue`, and `AtomicCell` occur in `Matrix`, `Vector`, `Element`, and imports. | Implemented in historical prototype types, but primarily as queue/concurrency machinery without defined two-dimensional or composite-operation semantics. Requires later retain/remove evidence. |
| `serde = "1.0.126"` with `derive` | Only `U32Deserializer` is imported in `src/lib.rs:8`; no serialization implementation or derive is present. | Historical placeholder/import-only; no implemented product behavior found. |
| `dashmap = "5.4.0"` | Imported at `src/lib.rs:19` and `src/operations/sort.rs:2`; the proposed sort implementation is commented out. | Historical placeholder/import-only; no implemented runtime behavior found. |
| `rayon = "1.5.1"` | No Rust-source reference. | Historical parallelism placeholder; no implemented runtime or test use found. |
| `surrealdb = "1.0.0-beta.9"` | No compiled API reference; only placeholder comments in `src/schematics/data.rs:177-191`. Resolution B selected `surrealdb 1.5.6`, which provides a confirmed route into the Rust-1.85-incompatible ICU graph. | Unearned historical persistence dependency and a demonstrated MSRV blocker route. Accepted architecture already defers it outside the immediate graph. |
| `criterion = "0.4"` | No compiled reference and no `[[bench]]`/`benches/` target. | Historical benchmark placeholder; `src/bench.rs` is unreachable and mostly commented. |

No direct dependency can be classified as merely transitive because each is a
top-level manifest entry. Resolution B establishes transitive graph behavior,
but only SurrealDB's incompatible route was traced deeply enough in R1-A to
support an immediate removal recommendation.

## Static source classification

### Core architecture

- `Matrix<V>` stores an `ArrayQueue<Element<V>>`, not two-dimensional dense
  storage (`src/schematics/matrix.rs:29-32`). Its only constructor accepts one
  capacity, leaves dimensions as `None`, and exposes no insertion, indexing,
  access, iteration, shape, or region API (`matrix.rs:34-51`). It is not a
  meaningful Matrix under the accepted contract.
- Gear and Cog own `ndarray::Array2<f64>` directly
  (`src/strategies/gear.rs:25-47`, `src/strategies/cog.rs:119-133`). Gear mutates
  an ndarray slice without going through Matrix or Lens
  (`gear.rs:132-149`). Matrix, Lens, Gear, Cog, and Tag are disconnected.
- `Lens<V>` is a recursive execution trait rather than a bounded borrowed view,
  and `MatrixLens<T>` is an opaque value wrapper without selection or borrowing
  semantics (`src/strategies/lens.rs:26-49`). No implemented Lens borrows from a
  Matrix.
- `Tag` and `ParameterizedQuery` expose neither constructors nor accessors, and
  have no connection to a transformation result
  (`src/strategies/tag.rs:5-21`).
- The code contains no GAT or HRTB implementation. Those remain correctly
  deferred. Runtime trait objects and strategy containers are already numerous,
  but no matrix-semantic need or dispatch comparison supports them.

### Correctness and failure boundaries

- `MatricalError` has an unconditionally recursive `Debug` implementation:
  formatting it with `{:?}` calls the same implementation again
  (`src/error.rs:38-42`). This is a concrete panic/stack-overflow risk whenever
  a caller formats an error. `test_basics` contains such an error-formatting
  branch at `src/strategies/gear.rs:241-244`, although its present test data uses
  valid bounds.
- The error type is declared in a private module and is not publicly re-exported
  (`src/lib.rs:21-23`), while public operations return it. Downstream callers
  cannot conveniently name or match the advertised error contract.
- `CogBuilder::build` accepts absent data and constructs a Cog with `context:
  None` (`src/strategies/cog.rs:137-168`). `CogStrategyImpl::execute` then
  unconditionally unwraps that optional context four times
  (`cog.rs:96-110`). An allowed builder state can therefore panic inside a
  `Result`-returning operation.
- `GearOperationImpl::apply` checks endpoint bounds but not region ordering
  before creating an inclusive ndarray slice (`src/strategies/gear.rs:132-149`).
  A reversed region is not converted into a typed error; its exact runtime
  outcome remains unexecuted in this session.
- Gear and Cog constructors accept coordinates unrelated to data shape and do
  not validate ordering. `GearFactoryImpl` also records constructor coordinates
  but ignores them when `create` receives a second pair
  (`src/strategies/gear.rs:88-116`). These are invalid or misleading builder
  states.
- `NestedValidationStrategy::as_any` is an unconditional `todo!()` panic
  (`src/operations/mechanics.rs:105-116`). The type has no public constructor,
  so this path is currently difficult for a downstream caller to reach, but it
  remains an incomplete implementation.
- `Matrix::new_by_size` has no zero-capacity or allocation-boundary validation
  (`src/schematics/matrix.rs:40-51`). Those boundaries are not exercised by the
  passing tests and must not be inferred as safe.
- `Vector<V>` requires `Element<V>: PartialEq + Eq + Clone`, but Matrical
  implements none of those traits for `Element<V>`
  (`src/schematics/vector.rs:12-18`, `src/schematics/element.rs:15-18`). Its
  public constructor and methods cannot be used with the crate's declared
  Element type as provided.

### Silent validation and duplicate abstractions

- `MatrixValidation::is_valid` iterates configured strategies without invoking
  them and always returns `Ok(())` (`src/strategies/lens.rs:159-183`).
- `MatrixValidationBuilder::build` discards every configured strategy and
  returns an empty vector (`src/strategies/lens.rs:187-208`).
- `SqlValidation::is_valid` likewise ignores its strategies and always returns
  an empty successful string (`src/schematics/data.rs:102-128`).
- Both `IsValid` blanket implementations return `true` for every `T`
  (`src/strategies/lens.rs:51-59`, `src/schematics/data.rs:26-35`).
- Validation abstractions are duplicated across `operations/mechanics.rs`,
  `strategies/lens.rs`, and `schematics/data.rs`, with incompatible return types
  and behavior. Only the generic mechanics helpers have active tests; none prove
  Matrix/Lens/Gear/Cog/Tag invariants.

### Empty, commented, and unreachable surfaces

- `arithmetic.rs`, `bitwise.rs`, `boolean.rs`, and `relational.rs` are empty;
  `relational.rs` is not even declared by `operations/mod.rs`.
- `aggregate.rs`, `filter.rs`, and `sort.rs` contain only imports or commented
  prototype implementations.
- `src/tests.rs` contains only commented historical tests.
- `src/bench.rs` is not a Cargo target and its benchmark body is commented.
- The source has no actual rustdoc, runnable examples, integration tests,
  failure-boundary tests, shape/index/region tests, property tests, or downstream
  smoke surface.

### Compilation and warning boundary

Rust 1.85 did not reach Matrical source compilation because the locked
dependency graph failed its compiler-version prerequisites. Rust 1.93.1 did
compile all targets, proving the unchanged source is accepted by that newer
compiler while emitting 65 library and 68 test-target warnings.

The warning surface includes the statically observed unused imports and
parameters, unread private fields, dead code, ambiguous glob re-exports, and
comment-only modules. Clippy then confirmed recursive error formatting as a
deny-by-default correctness defect rather than merely a warning. The manifest's
singular `package.author` key is independently confirmed as unused.

## Public-contract risk summary

The 0.1.0 prototype does not provide a usable semantic matrix-transformation
contract:

1. Matrix has no two-dimensional storage or access contract.
2. Lens does not borrow or select Matrix storage.
3. Gear bypasses both Matrix and Lens.
4. Cog permits missing required context and can panic.
5. Tag has no usable construction or provenance behavior.
6. public fallible operations use an inaccessible, recursively formatted error
   type and do not consistently convert invalid states to errors.
7. validation can silently succeed without running configured policy.
8. the active tests primarily prove generic validation helpers and happy-path
   direct ndarray Gear mutation, not the accepted semantic flow.

These are inherited prototype risks, not regressions introduced by R1-A. The
accepted no-compatibility position for unfinished 0.1.0 APIs remains justified.

## Recommended R1-B boundary

Recommend **R1-B: restore dependency and MSRV reproducibility**, before
repairing source behavior.

The bounded objective should be:

1. establish an explicit dependency and lockfile policy;
2. remove SurrealDB from Matrical's immediate graph, consistent with the
   accepted architecture and its demonstrated incompatible route;
3. remove other unearned persistence or concurrency dependencies only where the
   R1-A source inventory supports removal;
4. correct `package.author` to valid Cargo package metadata;
5. encode `rust-version = "1.85"` only alongside evidence that the resulting
   graph actually supports it;
6. regenerate a clean post-pruning resolution rather than committing Resolution
   B;
7. rerun the complete Rust 1.85 qualification ladder on the pruned graph;
8. retain the recursive `MatricalError` `Debug` defect as the first confirmed
   source-correctness repair after dependency/MSRV restoration.

Whether to commit a development lockfile is a Teamlead recommendation that
requires an explicit owner decision. R1-B must not silently change the current
ignored-lockfile policy.

This boundary is smaller and safer than Matrix reconstruction or broad cleanup:
R1-A proved that dependency selection is date-dependent, that the current graph
cannot reach source compilation on Rust 1.85, and that SurrealDB is both unused
by compiled Matrical behavior and a demonstrated route into incompatible
transitives. Pruning and re-resolving that boundary is therefore evidence-led.

### Explicit R1-B exclusions

- no Matrix, Shape, Index, Region, Lens, Gear, Cog, or Tag redesign;
- no broad warning cleanup or automatic `cargo fix`;
- no dependency modernization unrelated to MSRV restoration or evidence-led
  removal;
- no CI creation;
- no examples, benchmark campaign, performance claim, or parallel execution;
- no package-version change, compatibility promise, tag, release, or publish;
- no advanced backend, GAT, HRTB, persistence, or consumer-domain API.

## Evidence artifacts and cleanup status

```text
repository report       docs/development/2026-08-20-r1a-baseline-reconnaissance.md
campaign link           docs/active-development.md
cloud Cargo.lock        absent before and after the exit-127 attempts
cloud target/           absent
Orion ordinary status   clean
Orion Cargo.lock        present, ignored, uncommitted, Resolution B unchanged
Orion tracked changes   none
Orion staged changes    none
Orion target/           absent from the repository
external target size    2.0G in the home-backed Matrical cache
evidence files          22, retained outside the repository
final-summary.txt       SHA-256 febb491288acb76271a612a3e2d233020091d7d701a488a7e906cfb16054c58b
evidence-sha256.txt     SHA-256 1c90189c35c354afe53847a98d54c76ce98d1a81b0ac4bd7558bce9bc768ca53
```

The Orion cache paths, raw logs, generated lockfile, and build products remain
outside the repository. No credentials or unrelated private paths are included.
The documentation worktree retains only the two authorized paths for review.

## Unresolved questions and residual risks

- Should Matrical commit a lockfile for campaign/CI reproducibility or keep the
  current ignored library policy with another reproducibility mechanism?
- Does the evidence-led pruned graph resolve reproducibly and compile throughout
  the complete Rust 1.85 ladder?
- Which additional unearned dependencies can be removed without combining R1-B
  with semantic redesign?
- The confirmed recursive `MatricalError` defect remains unrepaired and untested.
- The large warning surface remains inherited debt; R1-B must not absorb it as a
  broad cleanup campaign.
- Passing 24 tests still supplies no meaningful Matrix, Lens, Cog, Tag, error,
  builder-invariant, or downstream semantic coverage.

R1-A implementation and evidence are complete. The documentation is a local
review candidate, publication remains at the owner gate, and R1-B remains
blocked until this documentation is accepted and merged. The library baseline
itself is not fully qualified.
