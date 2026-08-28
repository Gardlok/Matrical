# R1-C source correctness

**Implementation and validation:** 2026-08-28

**Status:** local review candidate — Teamlead gate

## Scope and starting identity

R1-C repairs the first inherited source-correctness boundary discovered during
R1-A and preserved through R1-B qualification.

```text
repository Gardlok/Matrical
branch     main
commit     1a5e4a72d7c0bb2a6ddd92b070eb853e98d6f136
tree       c4480b98b776df55f2cb974a6ef675223a7a8c99
version    0.1.0
```

Commit `1a5e4a72d7c0bb2a6ddd92b070eb853e98d6f136` is the verified merge of PR #4 and the
owner-accepted R1-B result. GitHub and local preflight agreed on the exact commit
and tree, `main` remained the default branch, no open pull request overlapped
the work, and the only diverged remote branch inspected retained the same
`src/error.rs` blob as the accepted baseline.

Implementation used `rehab/r1c-source-correctness`. The worktree was clean
before the branch was created.

R1-C changes only:

```text
src/error.rs
docs/active-development.md
docs/roadmap.md
docs/development/2026-08-28-r1c-source-correctness.md
```

No Matrix, Lens, Gear, Cog, or Tag reconstruction is included.

## Inherited defect

The accepted R1-B baseline implemented `Debug` for `MatricalError` by
formatting `self` through `{:?}` from inside its own `Debug::fmt`
implementation:

```rust
impl fmt::Debug for MatricalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
```

That recursively invokes the same formatter. Rust 1.85 and current-stable
Clippy both classified it as `clippy::recursive_format_impl`, and it was the
first hard source-correctness blocker after R1-B restored reproducible
compilation.

## Repair

R1-C removes the recursive manual formatter and derives `Debug` for both
`MatricalError` and `MatricalErrorType`.

Derived formatting is the smallest conventional implementation that represents
all existing variants without adding a second hand-maintained match over the
error enum. No variant, visibility, `Display` behavior, dependency, manifest,
MSRV, or version changes.

The final `src/error.rs` working-tree blob is:

```text
0fa3f9ac34bb9cc345de28bd791eb1b990c65cc3
```

## Focused regression coverage

Two additive unit tests live under `#[cfg(test)]` in `src/error.rs`.

The first covers:

```text
Regular(IncorrectDimensions)
Regular(IncorrectFormat)
```

The second covers:

```text
Custom(...)
InvalidValue
InvalidContext
ShouldNotOccur
IndexOutOfBounds
```

The assertions require each rendered value to identify its variant and, where
applicable, its nested error type or custom context. They deliberately avoid
requiring one exact incidental formatter string.

The historical 24 tests remain present. The full R1-C suite discovers 26 tests,
and all 26 pass on both qualification lanes.

## Rust 1.85 qualification

Primary toolchain:

```text
rustc 1.85.0 (4d91de4e4 2025-02-17)
cargo 1.85.0 (d73d2caf9 2024-12-31)
```

| Command | Exit | Result |
| --- | ---: | --- |
| `cargo +1.85.0 check --locked --all-targets` | 0 | All targets compile. |
| focused `Regular` Debug regression | 0 | 1 passed; 25 filtered out. |
| focused remaining-variant Debug regression | 0 | 1 passed; 25 filtered out. |
| `cargo +1.85.0 test --locked --all-targets` | 0 | 26 passed; 0 failed. |
| `cargo +1.85.0 test --locked --doc` | 0 | 0 doctests discovered. |
| `cargo +1.85.0 clippy --locked --all-targets` | 0 | Recursive-format blocker is gone; Clippy completes. |
| `cargo +1.85.0 doc --locked --no-deps` | 0 | Documentation generation succeeds. |

Rust 1.85 Clippy completes with inherited warning debt: 84 library warnings and
89 test-target warnings in the observed final lane. These warnings are not a
new hard Clippy boundary and are not repaired in R1-C.

## Current-stable comparison

Secondary toolchain:

```text
rustc 1.98.0 (88d9e12ae 2026-08-18)
cargo 1.98.0 (797e8a9bc 2026-08-05)
```

| Command | Exit | Result |
| --- | ---: | --- |
| `cargo +stable check --locked --all-targets` | 0 | All targets compile. |
| `cargo +stable test --locked --all-targets` | 0 | 26 passed; 0 failed. |
| `cargo +stable test --locked --doc` | 0 | 0 doctests discovered. |
| `cargo +stable clippy --locked --all-targets` | 0 | No hard Clippy blocker remains. |
| `cargo +stable doc --locked --no-deps` | 0 | Documentation generation succeeds. |

Stable Clippy reports 83 library warnings and 88 test-target warnings. The
comparison agrees with the Rust 1.85 result: repairing recursive Debug formatting
reveals no second hard Clippy failure.

## Resulting Clippy state

There is **no first remaining hard Clippy blocker** after the R1-C repair.

The remaining lint surface is warning-level inherited debt, including classes
such as unused items and variables, `result_unit_err`,
`new_without_default`, `type_complexity`, module inception, and
`len_without_is_empty`.

R1-C records that debt rather than converting this bounded correctness repair
into a warning-cleanup campaign.

## Reproducibility

The committed lockfile remained byte-identical before and after both
qualification lanes:

```text
Cargo.lock SHA-256
8abe052c6d793e87df19c1e6ade379caf3cad562eea693a946dc39c9e7180020
```

Every Cargo qualification command used `--locked`.

All build output used the home-backed external Matrical cache. A
repository-local `target/` remained absent.

## Formatting and mechanical debt

The accepted baseline and R1-C candidate both return exit 1 from the Rust 1.85
path-scoped Rustfmt check for `src/error.rs`.

The baseline already contains excess leading/interstitial blank lines and
trailing whitespace on the `InvalidContext` Display arm. R1-C does not absorb
that unrelated formatting debt. The newly added R1-C test region was manually
kept consistent with Rustfmt's requested layout without formatting the
historical file wholesale.

Final candidate validation also checks diff hygiene, authorized-path scope,
trailing whitespace, final newlines, relative Markdown links, exact lockfile
identity, and absence of a repository-local `target/`.

## Residual warnings and debt

- Rust 1.85 Clippy completes with warning-level inherited debt.
- Stable Clippy completes with the corresponding warning-level inherited debt.
- Rustdoc still discovers zero doctests.
- Historical formatting debt remains in `src/error.rs`.
- `AtomicBoolError` and the historical `Error` type remain unused.
- Existing source still contains incomplete and placeholder APIs outside R1-C.
- Crossbeam remains temporary inherited architecture debt.
- No broad warning cleanup, API reconstruction, CI work, version change, or
  repository-wide formatting was performed.

## Recommended next bounded slice

After R1-C passes Teamlead and owner acceptance, recommend a separate R1-D
source-correctness slice that investigates one runtime invalid-state or panic
boundary rather than beginning broad warning cleanup.

The previously classified Cog construction path that can permit missing context
before later unwrap behavior is a reasonable first investigation target. It
should be re-established from the accepted R1-C baseline and repaired only if
the focused evidence confirms that boundary.

Matrix/Lens/Gear/Cog/Tag reconstruction remains blocked until separately
authorized.

```text
R1-B STATUS: OWNER ACCEPTED — MERGED
R1-C STATUS: LOCAL REVIEW CANDIDATE — TEAMLEAD GATE
NEXT SOURCE SLICE: BLOCKED
```
