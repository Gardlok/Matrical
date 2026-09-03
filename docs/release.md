# Matrical release qualification

This is the durable maintainer procedure for qualifying a Matrical release. It
does not itself authorize a version, tag, GitHub Release, or crates.io publish.
Those actions remain explicit owner gates.

## 1. Establish the candidate identity

1. Start from a clean `main` synchronized with `origin/main`.
2. Record the exact starting commit and tree.
3. Record the package version and declared MSRV.
4. Work on a focused release branch and keep unrelated source cleanup out of the
   release diff.

## 2. Verify the compatibility decision

Before packaging, review:

- `matrical::prelude::*` as the recommended everyday surface;
- supported named crate-root exports;
- `matrical::schematics` and `matrical::strategies` as supported grouped
  namespaces;
- `matrical::snapshot` / `MatrixSnapshot` as the specialized interchange
  surface;
- documentation-hidden compatibility residue as unsupported for new callers;
- the current dense snapshot schema policy in `docs/api-stability.md` and
  `docs/interchange.md`.

Rust SemVer and snapshot schema versions are separate decisions. An incompatible
snapshot semantic change must not silently retain the same schema version.

## 3. Audit version and registry state

Record:

- the package version in `Cargo.toml`;
- existing repository tags and GitHub Releases;
- the exact crates.io package-name/version state;
- whether the changelog entry matches the candidate version.

Do not invent a bump merely because a release gate is running. If multiple
version choices are valid and owner judgment is required, leave the version
unchanged and record the recommendation.

## 4. Qualify Rust 1.85 and stable

Run all commands with the committed lockfile on Rust 1.85.0 and current stable:

```bash
cargo check --locked --all-targets
cargo test --locked --all-targets
cargo test --locked --doc
cargo clippy --locked --all-targets
cargo doc --locked --no-deps

cargo check --locked --all-targets --all-features
cargo test --locked --all-targets --all-features
cargo test --locked --doc --all-features
cargo clippy --locked --all-targets --all-features
cargo doc --locked --no-deps --all-features

cargo bench --locked --no-run
```

Compile/run every shipped example under the applicable feature configuration.
Do not infer example health from library tests alone.

## 5. Audit package contents

Run:

```bash
cargo package --locked --list
```

Inspect the exact list. A release package should contain the license, README,
changelog, library source, intentionally shipped examples/benchmarks, and the
release-facing documentation selected by `Cargo.toml`. It should not carry CI,
editor configuration, rehabilitation evidence, prompt archives, cache output,
or generated build artifacts.

Run both:

```bash
cargo +1.85.0 package --locked
cargo +stable package --locked
```

Use separate home-backed evidence directories when sequential Cargo package
verification would otherwise collide. Do not commit generated `.crate` files or
`target/` output.

Record archive and unpacked sizes as a sanity check rather than an arbitrary size
budget.

## 6. Prove packaged-artifact consumption

For each qualification toolchain:

1. locate the generated `.crate` archive;
2. unpack it under `${XDG_CACHE_HOME:-$HOME/.cache}/matrical/`;
3. create an independent tiny Cargo project outside the repository;
4. depend on the unpacked package path, never the source checkout;
5. run a default-feature consumer proving Matrix creation, Region/Lens, one
   `ReadGear`, one `MutGear`, and a `MatrixSnapshot` roundtrip;
6. run a `features = ["serde"]` consumer proving `MatrixSnapshot` participates in
   serialization/deserialization and reconstructs successfully.

The smoke consumer must use public API only.

## 7. Audit dependencies and licenses

Record the lock-resolved versions, role, runtime/dev-only scope, and direct
license of every direct dependency. Confirm no direct license conflicts with
Matrical's MIT distribution. Avoid adding permanent release tooling merely to
produce this evidence.

Record default and serde-enabled normal dependency trees and the exact
`Cargo.lock` SHA-256.

## 8. Preserve performance evidence

Compile the benchmark harness on both supported qualification toolchains.
Re-run timing only when candidate source changes plausibly affect an accepted
performance-sensitive path or another concrete regression signal exists.
Metadata/documentation-only release work is not a reason to reopen the full R6
benchmark campaign.

## 9. Mechanical repository checks

At minimum run:

```text
git diff --check
Markdown relative-link validation
final-newline validation
unsafe audit
tracked target/.crate/generated-artifact audit
Cargo.lock SHA-256
package-content audit
dependency-graph audit
```

Store exact evidence beneath a home-backed cache location such as
`${XDG_CACHE_HOME:-$HOME/.cache}/matrical/`.

## 10. Optional non-publishing registry dry-run

When the environment can safely contact crates.io without requesting or exposing
credentials, `cargo publish --dry-run --locked` may be used as packaging/registry
reconnaissance. Record whether success/failure is caused by metadata, registry
name/version state, network availability, or package verification.

Never substitute a non-dry-run publish.

## 11. Owner-controlled release gate

Only after qualification is reviewable should the owner decide whether to:

1. accept the candidate version;
2. authorize a Git tag;
3. authorize a GitHub Release;
4. optionally authorize crates.io publication.

Those are separate decisions. Do not perform any of them merely because the
qualification branch passed CI.

## 12. Post-release verification

If and only if the owner separately authorizes and performs a release, verify the
exact released tag/artifact/version, registry package metadata when applicable,
documentation availability, and an ordinary downstream dependency from the
released source. Record discrepancies rather than silently repairing or
republishing them.
