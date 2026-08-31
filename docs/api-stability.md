# Matrical API stability and deprecation policy

## Current 0.1.0 position

Matrical is still in an active rehabilitation campaign. The working R2–R6 core,
R5 learning surface, and R7-A snapshot interchange direction represent the
current design, but version `0.1.0` is **not** a SemVer stability guarantee for
that Rust API.

Before the first owner-authorized rehabilitated release:

- public Rust APIs may change deliberately through reviewed campaign work;
- unfinished historical prototype APIs have no compatibility promise;
- Matrix, Region, Lens/LensMut, Gear, Cog, Tag, ExecutionReport, and
  MatrixSnapshot are accepted conceptual directions, but exact Rust signatures
  may still change;
- no production-readiness, performance, schema-ecosystem, or crates.io
  publication claim follows merely from passing the current qualification gates.

## Rust API stability versus snapshot schema compatibility

Rust API compatibility and durable snapshot-schema compatibility are separate
contracts.

`MatrixSnapshot` dense schema v1 currently defines these logical fields and
meanings:

```text
version: u32
rows: u64
columns: u64
row_major: sequence of T in logical row-major order
```

R7-A does **not** promise that schema v1 has a permanent ecosystem compatibility
guarantee before the R8 owner-controlled release/stability gate. However, once a
versioned schema is introduced, incompatible field or semantic changes must not
be silently applied to the same version number. Deliberate incompatible schema
changes should introduce a new snapshot version and explicit migration/reader
behavior.

A reader supporting dense snapshot v1 fails closed when presented with another
version. With the optional Serde implementation, v1 also denies unknown fields
rather than silently dropping future semantic data.

The selected serialization format remains responsible for its own element-domain
representational limits. The snapshot schema being versioned does not imply that
every Serde format can faithfully encode every possible `T`.

## Recommended versus specialized versus legacy surface

The supported learning contract is intentionally curated:

- `matrical::prelude::*` is the recommended everyday Matrix/Lens/Gear API;
- named crate-root exports are supported and discoverable;
- `matrical::schematics` and `matrical::strategies` group everyday supported
  concepts;
- `matrical::snapshot` and the crate-root `MatrixSnapshot` export are a
  specialized interchange API and are deliberately excluded from the prelude;
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
3. update affected crate rustdoc, guides, examples, fixtures, and downstream
   smoke tests;
4. preserve established authority and safety contracts unless a separately
   justified architecture change is accepted;
5. for snapshot schemas, use an explicit new version rather than silently
   changing incompatible v1 semantics.

Breaking changes are not an excuse for unrelated churn.

## Deprecation policy

A deprecation period is expected when a supported, documented Rust API has real
callers and a replacement can coexist safely long enough to aid migration.
Deprecation should identify the replacement and the reason for the transition.

During the current 0.1.0 rehabilitation, unfinished historical prototype APIs may
instead be hidden or removed directly when they were never part of the accepted
learning contract. Snapshot-version migration is handled separately through
explicit versioning rather than Rust deprecation attributes alone.

## After a release/stability gate

Once an owner-authorized stability/release gate establishes a released public
contract, SemVer governs Rust API compatibility for that release line and the
release documentation must state the supported snapshot-schema compatibility
policy. Future breaking changes must follow the versioning and migration
expectations that apply to that published contract.

This document does not authorize a version bump, release date, tag, crates.io
publication, or permanent schema-v1 ecosystem guarantee.
