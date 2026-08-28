# Matrical rehabilitation roadmap

**Historical source baseline:** `6deb812e11a519404fec90408bf95651764cd2f8`

**Accepted R0 merge:** `b929e48481ae7ab41c972447b1547671afe4a4d8`

**Accepted R1-A merge:** `1c5ec09346f249496f1bb2e72095e073b348568a`

**Accepted R1-B merge:** `1a5e4a72d7c0bb2a6ddd92b070eb853e98d6f136`

**Accepted R1-C merge:** `16ddcc878c9cc8c8701dbc01453e08cfccd00b54`

**Accepted R1-D merge:** `059f148a99cfe2b5b881ada9af9acc286f584b6a`

**Current phase:** R2 complete pending Teamlead/owner acceptance; R1 complete and owner-accepted

This roadmap is ordered. Later slices may be researched early, but implementation
should not bypass an earlier invariant or acceptance gate.

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

**Goal:** replace stale project claims with truthful documentation and a durable
rehabilitation workflow.

R0 established the architecture vision, nomenclature, roadmap, testing/evidence
workflow, exact-baseline tracking, and owner decision record.

## R1 — Reproduce and classify the historical baseline

**Status:** COMPLETE — OWNER ACCEPTED

Progression:

- R1-A reconnaissance: merged in PR #3;
- R1-B dependency/MSRV reproducibility: merged in PR #4;
- R1-C source correctness: merged in PR #5;
- R1-D runtime safety and qualification CI: merged in PR #6.

R1 established Rust 1.85.0/MSRV reproducibility, a committed lockfile policy,
source-correctness repairs, runtime-safety repairs, and two-lane qualification
CI. Its exit criteria are satisfied.

## R2 — Rebuild the core invariants

**Status:** COMPLETE — TEAMLEAD/OWNER ACCEPTANCE PENDING

**Goal:** provide a small useful Matrix with typed failures.

Delivered:

- public `MatricalError` and `std::error::Error` contract;
- checked `Shape`, `Index`, and half-open rectangular `Region` types;
- owned dense `Matrix<T>` backed privately by `ndarray::Array2<T>`;
- exact row-major construction with typed length mismatch;
- checked immutable and mutable access;
- deterministic row-major iteration, mutable iteration, and owned conversion;
- zero-sized, overflow, indexing, and region boundary coverage;
- public downstream-style integration test, runnable example, and compiled
  rustdoc example;
- explicit validity-mask decision: missingness is not intrinsic Matrix storage;
- removal of queue-backed Matrix storage while retaining detached
  `MatrixContext` only as required historical scaffolding;
- explicit unsafe/Miri evaluation for the owned core;
- successful existing-CI qualification on Rust 1.85.0 and stable.

Exit gate:

- no ordinary invalid shape, index, or region causes a panic: PASS;
- zero-sized and overflow boundaries are specified and tested: PASS;
- the public core can be consumed through intended crate-root exports: PASS;
- Matrix no longer uses queue capacity as storage or shape: PASS;
- Miri/deeper checking is evaluated against the actual unsafe/aliasing surface:
  PASS — no unsafe/aliasing machinery is introduced in the owned R2 core; Miri
  is deferred for reconsideration with mutable borrowed Lens semantics in R3.

```text
R2: COMPLETE — TEAMLEAD/OWNER ACCEPTANCE PENDING
Next phase after merge: R3 — make Lens real
```

## R3 — Make Lens real

**Status:** BLOCKED ONLY ON R2 TEAMLEAD/OWNER ACCEPTANCE

**Goal:** deliver safe, borrowing views over Matrix data.

Planned work:

- immutable rectangular Lens;
- mutable rectangular Lens with exclusive borrowing;
- row and column selectors;
- compare a GAT-backed lending-view design against a simpler lifetime-generic
  design and adopt GATs only if they provide a concrete correctness/usability
  benefit;
- iteration and conversion rules;
- compile-time lifetime examples and negative API tests where useful.

A conceptual design probe remains:

```rust
trait LendingView {
    type View<'a>
    where
        Self: 'a;

    fn view<'a>(&'a self) -> Self::View<'a>;
}
```

This is not an R2 API commitment.

Exit gate:

- Lenses cannot outlive their Matrix;
- mutable aliasing is rejected by the type system;
- selection boundaries are property-tested;
- view operations do not allocate unless documented.

## R4 — Reintroduce Gear, Cog, and Tag

**Goal:** turn the nomenclature into a coherent transformation API.

Deliver typed transformation/context/provenance contracts after Lens semantics
are proven. A Gear must not bypass Lens bounds, and missing required context must
remain a typed failure rather than a panic.

## R5 — API ergonomics and learning surface

**Goal:** make the library understandable and pleasant for a downstream user.

Expand crate/module rustdoc, runnable examples, naming/conversion consistency,
misuse tests, error-message review, and API stability/deprecation policy.

## R6 — Measure, then optimize

**Goal:** establish performance evidence before adding complexity.

Add representative benchmarks, allocation/copy accounting, comparison with
direct ndarray operations, and optional parallel execution only when measured
thresholds justify it.

## R7 — Optional backends and integrations

**Goal:** add extensibility only after the core demonstrates a real need.

Candidates include serialization, sparse/mapped storage, backend/lending-view
traits, and isolated persistence research. At least two real implementations
must justify shared backend abstractions.

## R8 — Release qualification

**Goal:** prepare the first rehabilitated release candidate.

Versioning, changelog/migration work, MSRV/stable/downstream qualification,
documentation audit, benchmark baseline, and crates.io publication remain
explicit R8/owner decisions. The likely release line is `0.2.0`.

## Advanced Rust policy

GATs, HRTBs, and related type-system tools are welcome only when they materially
encode a real ownership, borrowing, lending, callback, or extensibility
contract. They are not syntax goals.

R2 deliberately does not force them into the owned Matrix core. R3 carries the
explicit evidence-driven Lens/lending-view comparison.

## Cross-cutting requirements

Testing, documentation, examples, and dependency review are not final cleanup
slices. Each functional slice carries the evidence and learning surface needed
to make its own contract reviewable.
