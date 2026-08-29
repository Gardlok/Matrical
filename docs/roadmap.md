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

**Current phase:** R5 active

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
- **Deferred** — intentionally outside the current campaign boundary.

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

Delivered:

- distinct `ReadGear<T>` and `MutGear<T>` static traits;
- the caller-selected Lens as the transformation authority boundary;
- downstream-defined Gears without registry, `Any`, or string dispatch;
- built-ins `SumGear`, `AddScalarGear`, `ScaleGear`, and `ClampGear`;
- typed `Cog<C>` and `ValidateCog` policy validation;
- bounded inert `Tag` provenance;
- `ExecutionReport<O>` with Gear identity, Region, effect, typed output, and
  ordered Tags;
- `execute_read` / `execute_mut` central execution paths;
- compile-fail evidence that read Gear authority cannot mutate;
- unchanged lockfile and passing Rust 1.85.0/stable qualification.

R4 preserved the authority rule:

```text
caller chooses Region
-> caller creates Lens / LensMut
-> Gear receives only that bounded capability
```

A Gear does not receive Matrix storage or a provider from which it can request a
broader Region. GAT/HRTB and dynamic-registry abstractions remain deferred until
a concrete consumer demonstrates a need.

## R5 — API ergonomics and learning surface

**Status:** ACTIVE

**Baseline:**

```text
commit 6dc0320d1857d1c4fafd538fbf75ae80566887cc
tree   c421102b113b2dc2fc78373677a956e807dee7db
```

**Goal:** make the working R2–R4 library understandable and pleasant for a new
downstream Rust developer without requiring rehabilitation history or source
inspection.

Authorized work:

- rewrite the README around the working library;
- add crate-level rustdoc and a compiled end-to-end example;
- establish one curated `prelude` and explicit crate-root/module policy;
- classify and hide unfinished prototype exposure where appropriate for 0.1.0;
- document supported public contracts rather than restating names;
- add a task-oriented getting-started guide;
- add runnable quickstart and custom-Gear examples;
- add a downstream-style public API smoke test;
- audit naming, builder, conversion, and caller-facing error behavior;
- document pre-release stability and deprecation policy;
- preserve and clarify high-value compile-fail misuse examples;
- perform a documentation-only new-user walkthrough;
- qualify Rust 1.85.0 and stable with an unchanged lockfile.

Exit gate:

- README truthfully describes the working R2–R4 library and has a usable quick
  start;
- crate rustdoc compiles and teaches the conceptual flow;
- the recommended API is deliberately curated and the prelude contains no
  prototype junk;
- supported public items are documented and legacy exposure is classified;
- getting-started and both R5 examples are mechanically checked;
- the public API smoke test uses only recommended downstream imports;
- naming/builders/conversions/errors have explicit decisions;
- stability/deprecation policy exists;
- important compile-fail examples teach the authority/lifetime constraints;
- R3/R4 authority boundaries are unchanged;
- documentation alone answers representative new-user questions;
- Rust 1.85.0 and stable qualification pass with byte-identical `Cargo.lock`.

R5 does not add performance claims, Criterion, Rayon, new dependencies, backend
abstractions, a GAT provider, dynamic Gear registry, persistence, release/tagging,
or downstream application integration.

## R6 — Measure, then optimize

**Status:** BLOCKED ON R5 TEAMLEAD/OWNER ACCEPTANCE

**Goal:** establish performance evidence before adding complexity.

Planned work includes representative benchmarks, allocation/copy accounting,
direct-backend comparison, profiling, an explicit overhead budget, and optional
parallel execution only when measurement justifies it. R6 owns performance
claims and optimization decisions; R5 must not begin them.

## R7 — Optional backends and integrations

**Status:** BLOCKED ON EARLIER GATES

Candidate work includes serialization, durable representation, sparse/mapped
storage, and backend/lending traits only after real implementations justify the
abstraction. Optional persistence must not become a hidden mutation or authority
channel.

## R8 — Release qualification

**Status:** BLOCKED ON EARLIER GATES

Prepare the first rehabilitated release candidate: version/compatibility decision,
changelog and migration notes, package/license audit, MSRV/stable/downstream
qualification, documentation/example audit, benchmark baseline, and explicit
owner-controlled publication decision.

No version bump, tag, release date, or publication is authorized by R5.

## Advanced Rust policy

GATs and HRTBs are tools, not rehabilitation goals. R3 found the concrete
lifetime-generic Lens API clearer with one proven provider. R4 added an authority
reason to keep that decision: a generic provider passed to a Gear could grant
Region-selection authority broader than the caller-selected Lens.

R5 therefore documents the accepted choice rather than reopening it. A future
GAT/HRTB abstraction requires a concrete composability problem that cannot be
expressed as clearly with the existing capability boundary.

## Cross-cutting requirements

Testing, documentation, examples, dependency review, and authority analysis are
part of every functional slice. They are not deferred cleanup.
