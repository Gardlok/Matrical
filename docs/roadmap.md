# Matrical rehabilitation roadmap

**Historical source baseline:** `6deb812e11a519404fec90408bf95651764cd2f8`

**Accepted R0 merge:** `b929e48481ae7ab41c972447b1547671afe4a4d8`

**Accepted R1-A merge:** `1c5ec09346f249496f1bb2e72095e073b348568a`

**Accepted R1-B merge:** `1a5e4a72d7c0bb2a6ddd92b070eb853e98d6f136`

**Accepted R1-C merge:** `16ddcc878c9cc8c8701dbc01453e08cfccd00b54`

**Accepted R1-D merge:** `059f148a99cfe2b5b881ada9af9acc286f584b6a`

**Accepted R2 merge:** `2f76a87e171a32a58a6d7244fdeb1b8794fc043a`

**Accepted R3 merge:** `9fbc712084a78570e8ac2b980ff0d4474c90ee7f`

**Accepted R4 merge:** `6dc0320d1857d1c4fafd538fbf75ae80566887cc`

**Accepted R5 merge:** `acd15be9d02d27e6189aadedad3620e9558efe8f`

**Accepted R6 merge:** `6be8b0ce910d66d784cc5e5ca2d52a59f1cd3773`

**Accepted R7 merge:** `f28fc380926c8175ff9b5faeb092be5bd7426245`

**Current phase:** R8-A reviewable — ready for owner release decision

This roadmap is ordered. Later work may be researched early, but implementation
must not bypass an earlier invariant or acceptance gate.

## Campaign states

- **Proposed** — described but not owner-authorized.
- **Authorized** — bounded work may begin from the recorded baseline.
- **In development** — one focused implementation session owns the slice.
- **Reviewable** — implementation and required evidence are complete.
- **Teamlead accepted** — technical and architectural review passed.
- **Owner accepted** — the owner accepted residual risk and repository outcome.
- **Blocked** — an explicit prerequisite or unresolved result prevents progress.
- **Deferred** — intentionally outside the current campaign boundary until new
  evidence justifies reconsideration.

## R0 — Establish the base of operations

**Status:** COMPLETE — OWNER ACCEPTED

Established truthful documentation, nomenclature, rehabilitation workflow,
testing/evidence procedure, and review gates.

## R1 — Reproduce and classify the historical baseline

**Status:** COMPLETE — OWNER ACCEPTED

R1-A through R1-D established the reproducible dependency graph, Rust 1.85.0
MSRV, committed lockfile policy, source-correctness repairs, runtime-safety
repairs, and two-lane qualification CI.

## R2 — Rebuild the core invariants

**Status:** COMPLETE — OWNER ACCEPTED — MERGED IN PR #7

Delivered checked `Shape`, `Index`, `Region`, owned dense `Matrix<T>`, typed core
failures, exact row-major construction, checked access, deterministic iteration,
zero-size/overflow coverage, a downstream test, and a runnable example.

## R3 — Make Lens real

**Status:** COMPLETE — OWNER ACCEPTED — MERGED IN PR #8

Delivered immutable `Lens<'a, T>` and mutable `LensMut<'a, T>` borrowing views,
local indexing, Region revalidation, row/column selectors, logical row-major
iteration, explicit allocating conversion, and compile-fail lifetime/aliasing
evidence. A speculative GAT lending-provider abstraction was deferred.

## R4 — Reintroduce Gear, Cog, and Tag

**Status:** COMPLETE — OWNER ACCEPTED — MERGED IN PR #9

Delivered static read/mutating Gear contracts, typed Cog validation, inert Tag
provenance, ExecutionReport, deterministic built-ins, central execution paths,
and compile-fail authority evidence while preserving the rule that a Gear
receives only the caller-selected Lens/LensMut capability.

## R5 — API ergonomics and learning surface

**Status:** COMPLETE — OWNER ACCEPTED — MERGED IN PR #10

Established the recommended prelude and crate-root exports, getting-started
material, runnable quickstart/custom-Gear examples, downstream public-API smoke
coverage, API-stability policy, and documentation centered on the accepted
Matrix/Lens/Gear/Cog/Tag model.

## R6 — Measure, then optimize

**Status:** COMPLETE — OWNER ACCEPTED — MERGED IN PR #11

**Accepted merge:**

```text
commit 6be8b0ce910d66d784cc5e5ca2d52a59f1cd3773
tree   919f8f800f1ffa3b4750def03f803a807ff25179
```

R6 established reproducible Criterion evidence, found inherited parent-wide Lens
traversal, repaired it with checked private ndarray Region views, preserved the
public API and Gear authority boundary, documented allocation/copy behavior, and
deferred Rayon because the repaired sequential path measured approximately at
direct-ndarray speed.

See [performance.md](performance.md) and
[`development/2026-08-29-r6-measure-optimize.md`](development/2026-08-29-r6-measure-optimize.md).

## R7 — Optional interchange and integrations

**Status:** COMPLETE — OWNER ACCEPTED

R7 established a stable inert interchange boundary before considering live
storage or concrete external integrations.

### R7-A — Versioned dense snapshot interchange

**Status:** COMPLETE — OWNER ACCEPTED — MERGED IN PR #12

**Accepted merge:**

```text
commit f28fc380926c8175ff9b5faeb092be5bd7426245
tree   5d8bac5a769ba0fc3b77dc4b107ccd90d6c0dd86
```

R7-A delivered `MatrixSnapshot<T>` with private schema fields, fixed-width `u64`
dimensions, `DENSE_SNAPSHOT_VERSION = 1`, checked reconstruction, borrowed
cloning and consuming ownership-transfer paths, optional exact Serde support,
deny-unknown-fields deserialization, a deterministic integer fixture, and
explicit format/transport/resource caveats.

It did not add a persistence engine or live backend abstraction. ndarray remains
private and Gear authority remains bounded to caller-selected Lens/LensMut.

See [interchange.md](interchange.md) and
[`development/2026-08-30-r7a-versioned-snapshot.md`](development/2026-08-30-r7a-versioned-snapshot.md).

### R7-B — Evidence-selected next storage/integration slice

**Status:** DEFERRED — NO DEMONSTRATED SECOND-PROVIDER/INTEGRATION NEED

R7-A acceptance did not itself demonstrate a need for another live Matrix
provider, sparse/mapped storage, persistence, or a concrete external adapter.
R7-B therefore does not invent an abstraction merely to advance the roadmap.

This is not a permanent rejection of sparse/mapped storage or integration work.
If a real second provider or external consumer later demonstrates a concrete
composability problem, a future bounded slice may reconsider the appropriate
boundary. Matrical core must remain independently useful, and any adapter should
preserve caller-selected Lens/LensMut Gear authority.

## R8 — Release qualification

**Status:** R8-A REVIEWABLE

### R8-A — First rehabilitated release candidate

**Status:** REVIEWABLE — READY FOR OWNER RELEASE DECISION

**Baseline:**

```text
commit f28fc380926c8175ff9b5faeb092be5bd7426245
tree   5d8bac5a769ba0fc3b77dc4b107ccd90d6c0dd86
version 0.1.0
MSRV    Rust 1.85.0
```

**Goal:** answer with mechanical evidence whether the accepted Matrical library
can be packaged, documented, consumed, and versioned as a real Rust release
without repository-only assumptions.

R8-A owns package metadata/contents, registry and version reconnaissance,
changelog, supported API classification, dense snapshot-v1 release policy,
direct dependency/license audit, optional historical Crossbeam cleanup when it
has no supported behavior, package verification on Rust 1.85/stable, independent
packaged-artifact consumers for default and serde configurations, example/docs
qualification, benchmark compile, release checklist, and an explicit readiness
recommendation.

Valid R8-A exits are:

```text
READY FOR OWNER RELEASE DECISION
NOT RELEASE READY — BLOCKERS IDENTIFIED
```

Both are evidence-valid outcomes. R8-A does not publish to crates.io, create a
tag, create a GitHub Release, announce a release date, or make an irreversible
public release decision.

## Advanced Rust policy

GATs and HRTBs are tools, not rehabilitation goals. R3 found the concrete
lifetime-generic Lens API clearer with one proven provider, and R4 established a
least-authority reason not to give a Gear a general selector/provider.

`MatrixSnapshot` does not reopen that decision: it is inert data, not a live
Matrix provider. A future GAT/HRTB abstraction still requires a concrete second
provider and a composability problem that cannot be expressed as clearly with
the existing capability boundary.

## Cross-cutting requirements

Testing, documentation, examples, dependency review, performance evidence,
interchange compatibility, and authority analysis are part of every functional
slice. They are not deferred cleanup. Release publication remains an explicit
owner-controlled action after qualification.
