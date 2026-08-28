# R1-D runtime safety, qualification CI, and R1 closeout

**Implementation and validation:** 2026-08-28

**Status:** ready for Teamlead final review

## Scope and starting identity

R1-D starts from the exact owner-accepted R1-C merge:

    repository Gardlok/Matrical
    branch     main
    commit     16ddcc878c9cc8c8701dbc01453e08cfccd00b54
    tree       b5dc20f2bb85840e7f2c08ecf2c4c6ca346bb1bc
    version    0.1.0

Implementation branch:

    rehab/r1d-runtime-safety-ci-closeout

R1-D changes exactly six authorized paths:

    src/strategies/cog.rs
    src/operations/mechanics.rs
    .github/workflows/qualification.yml
    docs/active-development.md
    docs/roadmap.md
    docs/development/2026-08-28-r1d-runtime-safety-ci-closeout.md

`Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, Matrix, Lens, Gear, Tag,
dependency versions, error variants, and package version remain unchanged.

## Runtime hazards repaired

### Missing Cog context

`CogBuilder` can construct a `Cog` without context. The inherited
`CogStrategyImpl::execute()` repeatedly unwrapped that optional context.

R1-D resolves the context once and returns `MatricalError::InvalidContext`
when it is absent. The strategy no longer panics at that boundary.

### Cog coordinate bounds

The inherited Cog operation and strategy bounds checks represented ordinary
out-of-range coordinates as `MatricalError::ShouldNotOccur`.

R1-D changes both ordinary bounds failures to
`MatricalError::IndexOutOfBounds`. No new error variant is added.

### Nested validation type erasure

`NestedValidationStrategy::as_any()` was a compiled `todo!()` and therefore
panicked whenever invoked.

R1-D implements the existing trait contract by returning `self` as `&dyn Any`.

## Regression coverage

Five focused tests cover the repaired boundaries and valid Cog behavior:

    strategies::cog::tests::cog_strategy_without_context_returns_invalid_context
    strategies::cog::tests::cog_operation_out_of_bounds_returns_index_out_of_bounds
    strategies::cog::tests::cog_strategy_out_of_bounds_returns_index_out_of_bounds
    strategies::cog::tests::valid_cog_paths_remain_ok
    operations::mechanics::tests::test_nested_validation_strategy_as_any_identifies_strategy

Each focused regression was executed individually on Rust 1.85.0 and passed.
The full suite contains 31 tests, and all 31 pass on both qualification lanes.

## Panic-path source audit

The required Rust-source audit searched for active and historical uses of
`unwrap(`, `expect(`, `todo!(`, `unimplemented!(`, and `panic!(`.

Before R1-D, two publicly reachable hard panic boundaries were confirmed:

- `CogStrategyImpl::execute()` unwrapped optional Cog context;
- `NestedValidationStrategy::as_any()` invoked `todo!()`.

Both are repaired in R1-D.

One compiled `unwrap()` remains structurally safe in
`ValidatorFactory::create_validator()`: it calls `last_mut().unwrap()`
immediately after pushing a validator into the same vector.

The historical module-private `fn main()` in `src/strategies/cog.rs` also
contains unwraps. That function is dead library-module residue rather than a
public execution path and remains outside this bounded repair.

Other matches are test-only or occur in commented historical/placeholder code.
No additional active `expect`, `panic!`, or `unimplemented!` boundary was found,
and the audit identified no additional hard R1 blocker.

## Rust 1.85 qualification

Toolchain:

    rustc 1.85.0 (4d91de4e4 2025-02-17)
    cargo 1.85.0 (d73d2caf9 2024-12-31)

- `cargo +1.85.0 check --locked --all-targets`: PASS
- five focused R1-D regressions: PASS, one test each
- `cargo +1.85.0 test --locked --all-targets`: PASS, 31 passed; 0 failed
- `cargo +1.85.0 test --locked --doc`: PASS, 0 doctests
- `cargo +1.85.0 clippy --locked --all-targets`: PASS
- `cargo +1.85.0 doc --locked --no-deps`: PASS

Rust 1.85 Clippy completes with the same inherited warning boundary recorded
at accepted R1-C: 84 library warnings and 89 test-target warnings.

An initially introduced `clippy::items_after_test_module` warning from local
test placement was corrected before final qualification. The final candidate
does not introduce that warning.

## Current-stable qualification

Toolchain:

    rustc 1.98.0 (88d9e12ae 2026-08-18)
    cargo 1.98.0 (797e8a9bc 2026-08-05)

- `cargo +stable check --locked --all-targets`: PASS
- `cargo +stable test --locked --all-targets`: PASS, 31 passed; 0 failed
- `cargo +stable test --locked --doc`: PASS, 0 doctests
- `cargo +stable clippy --locked --all-targets`: PASS
- `cargo +stable doc --locked --no-deps`: PASS

Stable Clippy completes with the same inherited warning boundary recorded at
accepted R1-C: 83 library warnings and 88 test-target warnings.

All five R1-D regression names were explicitly confirmed as passing in the
stable full-suite log.

## Qualification CI

R1-D adds `.github/workflows/qualification.yml`.

The workflow runs on pull requests and pushes to `main`, with two lanes:

- Rust 1.85.0;
- current stable.

Each lane installs its toolchain explicitly, selects it with `rustup override`,
installs the matching Clippy component, and runs:

- `cargo check --locked --all-targets`;
- `cargo test --locked --all-targets`;
- `cargo test --locked --doc`;
- `cargo clippy --locked --all-targets`;
- `cargo doc --locked --no-deps`.

Rustfmt is deliberately not a mandatory CI gate because accepted historical
formatting debt remains outside R1-D scope.

## Reproducibility

The committed lockfile remained byte-identical throughout R1-D:

    Cargo.lock SHA-256
    8abe052c6d793e87df19c1e6ade379caf3cad562eea693a946dc39c9e7180020

Every qualification Cargo command used `--locked`.

Build output was directed beneath:

    ${XDG_CACHE_HOME:-$HOME/.cache}/matrical/r1d-runtime-safety-ci-closeout/

A repository-local `target/` remained absent.

## Advanced Rust policy

R1-D introduces no GAT or HRTB API.

GATs and higher-ranked trait bounds remain evidence-led tools rather than
rehabilitation goals. They should be adopted only when they encode a real
ownership, borrowing, lending, callback, or extensibility contract more clearly
and safely than a simpler design.

R3 explicitly carries forward evaluation of a GAT-backed Lens/lending-view
shape. Later backend abstractions may revisit GATs or HRTBs only when concrete
implementations justify the additional type-system complexity.

## Residual debt

R1-D deliberately does not broaden into general warning cleanup, formatting
cleanup, dependency changes, API redesign, or Matrix/Lens/Gear/Tag reconstruction.

Remaining known debt includes inherited Clippy warnings, historical formatting
debt, private/dead and commented source residue, unused or incomplete abstractions,
and the larger invariant reconstruction scheduled for R2 and later phases.

Zero doctests remain a documented coverage gap. R1-D does not broaden into
inventing documentation examples solely to change that count.

The source audit found no additional hard runtime-safety blocker requiring an
R1-D scope expansion.

## R1 closeout recommendation

    R1-A: OWNER ACCEPTED — MERGED
    R1-B: OWNER ACCEPTED — MERGED
    R1-C: OWNER ACCEPTED — MERGED
    R1-D: READY FOR TEAMLEAD FINAL REVIEW

    R1 exit criteria: satisfied
    Recommended next phase: R2 — rebuild core invariants
    R2: BLOCKED ONLY ON R1-D MERGE ACCEPTANCE

R1-D should proceed to Teamlead final review. R2 must not begin until R1-D is
accepted and merged.
