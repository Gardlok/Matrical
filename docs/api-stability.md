# Matrical API stability and deprecation policy

## Current 0.1.0 release-candidate position

Matrical R1–R7 rehabilitation is owner accepted and R8-A is qualifying version
`0.1.0` as the first rehabilitated release candidate. R8-A does **not** publish
that version, create a tag, or create a GitHub Release.

Until the owner separately authorizes a release, `0.1.0` remains a candidate and
not a published SemVer promise. If the owner publishes this candidate, the
supported Rust surface described below becomes the release-facing contract for
that version.

## Recommended, specialized, legacy, and private surface

The supported surface is intentionally curated:

- `matrical::prelude::*` is the recommended everyday Matrix/Lens/Gear API;
- named crate-root exports are supported and discoverable;
- `matrical::schematics` and `matrical::strategies` group supported concepts for
  deeper navigation;
- `matrical::snapshot` and the crate-root `MatrixSnapshot` export are a
  specialized supported interchange API and are deliberately excluded from the
  prelude;
- documentation-hidden historical operation, error-type, ElementContext,
  MatrixContext, SQL/validation, and related prototype compatibility residue is
  not recommended or supported for new downstream code;
- ndarray storage, Lens representation, internal strategies, and other
  crate-private implementation details are not public compatibility contracts.

The public learning contract centers on checked `Shape`, `Index`, `Region`, and
`Matrix`; borrowing `Lens`/`LensMut`; typed `ReadGear`/`MutGear`, `Cog`, `Tag`,
and `ExecutionReport`; and the specialized versioned snapshot boundary.

## Rust SemVer after publication

If `0.1.0` is owner-authorized and published, Cargo-compatible `0.1.x` releases
must not be used to smuggle incompatible changes to the supported Rust surface.
A deliberate breaking change to that supported surface should advance to a new
incompatible pre-1.0 line (for example `0.2.0`) unless a narrower SemVer rule
clearly applies.

Documentation-hidden historical residue does not gain a support promise merely
because it remains technically reachable. Removing or changing that residue is
still subject to deliberate review, but downstream callers should not depend on
it as released API.

## Rust API stability versus snapshot schema compatibility

Rust API compatibility and durable snapshot-schema compatibility are separate
contracts.

Dense `MatrixSnapshot` schema v1 defines these logical fields and meanings:

```text
version: u32
rows: u64
columns: u64
row_major: sequence of T in deterministic logical row-major order
```

The R8-A candidate policy is:

> Within a released line, incompatible dense snapshot semantics must not silently
> change under version 1. A future incompatible representation uses another
> explicit snapshot schema version.

This does not promise that every future Matrical release must support schema v1
forever. Any future decision to retire or migrate a released schema requires an
explicit compatibility/migration decision rather than silently reinterpreting
version 1.

A reader supporting dense snapshot v1 fails closed when presented with another
version. With the optional Serde implementation, v1 also denies unknown fields
rather than silently dropping future semantic data.

The selected serialization format remains responsible for its own element-domain
representational limits. A versioned snapshot schema does not imply every Serde
format can faithfully encode every possible `T`.

## Breaking-change discipline

A breaking change to supported Rust API or a released snapshot contract must be
deliberate and must:

1. be reviewed as bounded compatibility work rather than incidental cleanup;
2. record caller-facing migration impact;
3. update affected rustdoc, guides, examples, fixtures, and downstream smoke
   tests;
4. preserve established Matrix/Lens/Gear authority and safety contracts unless a
   separately justified architecture change is accepted;
5. for incompatible snapshot semantics, use an explicit new schema version
   rather than silently changing v1;
6. choose a Rust crate version consistent with the applicable SemVer contract.

Breaking changes are not an excuse for unrelated churn.

## Deprecation policy

A deprecation period is expected when a supported, documented Rust API has real
callers and a replacement can coexist safely long enough to aid migration.
Deprecation should identify the replacement and the reason for the transition.

Documentation-hidden historical prototype APIs may instead be removed when they
were never part of the supported learning/release contract. Snapshot-version
migration is handled separately through explicit schema versioning rather than
Rust deprecation attributes alone.

## What this document does not authorize

This policy does not authorize a version bump, release date, tag, GitHub Release,
crates.io publication, permanent schema-v1 ecosystem guarantee, new persistence
backend, or external integration. R8-A qualification and owner release approval
remain separate gates.
