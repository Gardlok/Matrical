# Changelog

All notable changes to Matrical's supported release-facing surface are recorded
here.

## [0.1.1] - Unreleased

### Changed

- Reorganized the public README so installation and a minimal checked
  Matrix/Region/Lens example appear before advanced transformation terminology.
- Corrected the recorded publication date of version `0.1.0` to 2026-09-04.

This patch release changes documentation and release metadata only. It does not
change Matrical's API, behavior, dependencies, MSRV, or snapshot schema.

## [0.1.0] - 2026-09-04

First public release of Matrical.

### Added

- Checked `Shape`, `Index`, half-open `Region`, and owned dense `Matrix<T>`
  construction/access with structural errors and deterministic row-major
  semantics.
- Borrowing `Lens<'a, T>` and `LensMut<'a, T>` views that preserve the exact
  caller-selected Region and make immutable versus mutable authority explicit in
  Rust types.
- Static `ReadGear` / `MutGear` transformation contracts, typed `Cog` context and
  validation, inert `Tag` provenance, and typed `ExecutionReport` results.
- A curated public learning surface through `matrical::prelude::*`, named
  crate-root exports, rustdoc, getting-started material, and runnable examples.
- Versioned dense `MatrixSnapshot<T>` interchange with checked reconstruction,
  fixed-width dimensions, explicit schema version 1, and optional Serde support
  behind the non-default `serde` feature.
- Declared MSRV of Rust 1.85.0 with qualification on both Rust 1.85 and current
  stable.

### Changed

- Improved Lens/LensMut traversal so selected iteration uses a checked private
  ndarray Region view rather than scanning unrelated parent cells. Measurements
  showed the dense path at approximately direct-ndarray traversal performance
  while preserving the public authority contract.

### Behavioral and compatibility boundaries

- `ndarray::Array2<T>` remains a private dense backend, not a public Matrical
  storage contract.
- A Gear receives only the `Lens`/`LensMut` selected by its caller; it receives no
  arbitrary Matrix or Region-selection authority.
- Matrical provides no persistence engine and owns no file, database, network,
  mapped-storage, or background transport authority.
- Matrical makes no parallel-execution promise. The accepted execution path is
  deterministic and sequential; Rayon remains deferred pending measured need.
- Dense snapshot schema v1 is versioned separately from the Rust crate version.
  Within a released line, incompatible dense snapshot semantics must not silently
  change under schema version 1. A future incompatible representation requires a
  different explicit snapshot schema version.
- Documentation-hidden historical compatibility residue is not part of the
  recommended supported surface and should not be used for new downstream code.
