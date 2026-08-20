# Matrical local testing and validation procedures

**Standard:** Matrical Validation Environment Contract v1 (`MVECv1`)

**Status:** proposed rehabilitation testing standard

## Purpose

MVECv1 adapts the repository discipline used by ROSE to Matrical's smaller
library boundary. It exists to produce focused, reproducible evidence without
turning every change into a repository-wide cleanup or attributing results to
the wrong source state.

## Mandatory principles

### Validate the affected boundary

Identify the changed files, modules, public contracts, features, examples, and
toolchain surfaces. Run the smallest validation ladder that proves them. Broaden
only when a shared manifest, public contract, feature graph, or release gate
requires it.

Do not use repository-wide formatting or linting to absorb unrelated historical
debt into a focused change.

### Preserve diagnostics

Validation command blocks must not use `set -euo pipefail` or an equivalent mode
that exits before statuses and diagnostic context are recorded.

Capture each command's result explicitly:

```bash
run_step() {
  local name="$1"
  shift

  printf '\n==> %s\n' "$name"
  "$@"
  local status=$?
  printf '<== %s: exit %s\n' "$name" "$status"
  return "$status"
}
```

A caller must decide whether failure blocks later work or permits read-only
diagnosis. Do not hide the first root-cause error beneath follow-on failures.

### Record exact provenance

Before validation, record:

```bash
git branch --show-current
git rev-parse HEAD
git status --short
```

Executable evidence belongs to that exact functional SHA. A documentation-only
follow-up may reference earlier evidence but must not claim that the executable
checks ran against the documentation commit.

### Keep build products private and reusable

On Orion and similar hosts, prefer home-backed cache storage over a constrained
or memory-backed `/tmp`:

```bash
MATRICAL_CACHE_ROOT="${XDG_CACHE_HOME:-$HOME/.cache}/matrical"
MATRICAL_TEST_TMP="$MATRICAL_CACHE_ROOT/test-tmp"
MATRICAL_TARGET_DIR="$MATRICAL_CACHE_ROOT/cargo-target"

install -d -m 0700 \
  "$MATRICAL_CACHE_ROOT" \
  "$MATRICAL_TEST_TMP" \
  "$MATRICAL_TARGET_DIR"
```

For storage-heavy or subprocess tests, bind conventional temporary variables:

```bash
TMPDIR="$MATRICAL_TEST_TMP" \
TMP="$MATRICAL_TEST_TMP" \
TEMP="$MATRICAL_TEST_TMP" \
CARGO_TARGET_DIR="$MATRICAL_TARGET_DIR" \
cargo test
```

Do not use broad `cargo clean` as a routine prerequisite. Inspect and remove only
owned, understood build or fixture roots when cleanup is actually required.

## Preflight

Confirm:

- expected branch and starting SHA;
- no unintended tracked or untracked work;
- exact changed-file scope;
- selected Rust toolchain;
- sufficient bytes and inodes for build roots;
- no unrelated Cargo process owns the same target or fixture root.

The R1 baseline slice will establish the accepted MSRV, current-stable lane, and
`Cargo.lock` policy. Until then, every result must state the toolchain and whether
dependency resolution changed.

## Validation ladder

### Step 1: Diff hygiene

```bash
git diff --check
git diff --check <starting-sha>...HEAD
```

Documentation-only changes should also verify internal Markdown links.

### Step 2: Path-scoped formatting

Run `rustfmt --check` only for changed Rust files. If no Rust file changed, do
not run Rust formatting merely to manufacture evidence.

### Step 3: Compile the affected surface

For the current single-crate layout:

```bash
cargo check --all-targets
```

Once an accepted lockfile policy exists, use `--locked` for reproducible
validation. Feature-specific work must name the relevant feature set.

### Step 4: Focused tests

Run the smallest named unit, integration, doctest, or property-test selection
that directly exercises the change. Confirm the filter matched tests; zero tests
is not evidence.

### Step 5: Full affected-crate tests

```bash
cargo test --all-targets
cargo test --doc
```

Run examples when public behavior or documentation changes:

```bash
cargo run --example <name>
```

### Step 6: Scope-appropriate Clippy

```bash
cargo clippy --all-targets -- -D warnings
```

During historical-debt slices, the Teamlead may accept a classified inherited
warning baseline instead of forcing unrelated cleanup. New warnings in changed
code remain blocking unless explicitly justified.

### Step 7: Deeper checks when justified

Depending on the changed boundary, add:

- property tests for shape, index, and region invariants;
- compile-fail tests for borrowing and type-state contracts;
- Miri for unsafe or aliasing-sensitive code;
- feature-matrix checks;
- criterion benchmarks for performance claims;
- downstream smoke crates for public API and MSRV claims.

These are targeted evidence, not ceremonial steps for every documentation or
value-only change.

## Validation classes

| Change class | Minimum evidence |
|---|---|
| Documentation only | Diff hygiene, link inspection, exact baseline reference |
| Pure value or error logic | Compile, focused unit tests, full crate tests, scoped Clippy |
| Public API or borrowing contract | Above plus rustdoc, examples, downstream or compile-fail tests |
| Manifest or feature graph | Dependency resolution record, all affected feature combinations |
| Performance claim | Functional ladder plus reproducible benchmark and environment record |
| Release candidate | MSRV, current stable, docs, examples, features, package, and downstream qualification |

## Result classification

Classify a non-zero result before changing code:

- **product regression** — accepted behavior is broken;
- **implementation defect** — candidate logic fails its intended contract;
- **dependency/toolchain failure** — resolution or compiler compatibility blocks;
- **environment failure** — storage, process, permissions, or host resources block;
- **harness failure** — the command, filter, fixture, or evidence collector is wrong;
- **inherited debt** — pre-existing behavior outside the bounded change;
- **interruption/ambiguous** — execution ended without a proven final outcome.

Do not repeatedly rerun an unexplained failure until it passes. Preserve the
first complete diagnostic and reconcile ambiguous state before retrying a test
that can mutate durable fixtures.

## Evidence handoff

Every development-session handoff should include:

```text
Starting SHA:
Functional SHA:
Toolchain:
Scope:
Commands and exit statuses:
Tests matched/passed/failed/ignored:
Warnings or inherited debt:
Environment or fixture roots:
Residual risks:
```

Passing validation makes a change reviewable. Teamlead acceptance, owner
acceptance, versioning, and publication are separate gates.
