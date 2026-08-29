# Matrical API stability and deprecation policy

## Current 0.1.0 position

Matrical is still in an active rehabilitation campaign. The working R2–R4 core
and its R5 learning surface represent the current design direction, but version
`0.1.0` is **not** a SemVer stability guarantee for that surface.

Before the first owner-authorized rehabilitated release:

- public APIs may change deliberately through reviewed campaign work;
- unfinished historical prototype APIs have no compatibility promise;
- Matrix, Region, Lens/LensMut, Gear, Cog, Tag, and ExecutionReport are the
  accepted conceptual direction, but their exact signatures may still change;
- no production-readiness, performance, or crates.io publication claim follows
  merely from passing the current qualification gates.

## Recommended versus legacy surface

The supported learning contract is intentionally curated:

- `matrical::prelude::*` is the recommended everyday import surface;
- named crate-root exports are supported and discoverable;
- `matrical::schematics` and `matrical::strategies` group supported concepts;
- historical operation, Element, Vector, SQL/validation, MatrixContext, and
  related prototype scaffolding is not recommended API even when some symbols
  remain reachable for temporary source compatibility.

A downstream user should not build new code around documentation-hidden legacy
surface and expect it to survive rehabilitation.

## Breaking changes before the stability gate

A pre-release breaking change may occur when it improves the accepted contract or
removes unfinished prototype exposure. Such a change must be deliberate and must:

1. be reviewed as part of a bounded campaign slice;
2. record caller-facing migration impact;
3. update affected crate rustdoc, guides, examples, and downstream smoke tests;
4. preserve the established authority and safety contracts unless a separately
   justified architecture change is accepted.

Breaking changes are not an excuse for unrelated churn.

## Deprecation policy

A deprecation period is expected when a supported, documented API has real
callers and a replacement can coexist safely long enough to aid migration.
Deprecation should identify the replacement and the reason for the transition.

During the current 0.1.0 rehabilitation, unfinished historical prototype APIs may
instead be hidden or removed directly when they were never part of the accepted
learning contract. R5 records those decisions explicitly rather than pretending
that accidental prototype exposure is stable API.

## After a release/stability gate

Once an owner-authorized stability/release gate establishes a released public
contract, SemVer governs public compatibility for that release line. Future
breaking changes must follow the versioning and migration expectations that apply
to that published contract.

This document does not authorize a version bump, release date, tag, or crates.io
publication.
