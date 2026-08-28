# Matrical rehabilitation roadmap

**Historical source baseline:** `6deb812e11a519404fec90408bf95651764cd2f8`

**Accepted R0 merge:** `b929e48481ae7ab41c972447b1547671afe4a4d8`

**Accepted R1-A merge:** `1c5ec09346f249496f1bb2e72095e073b348568a`

**Accepted R1-B merge:** `1a5e4a72d7c0bb2a6ddd92b070eb853e98d6f136`

**Accepted R1-C merge:** `16ddcc878c9cc8c8701dbc01453e08cfccd00b54`

**Accepted R1-D merge:** `059f148a99cfe2b5b881ada9af9acc286f584b6a`

**Current phase:** R2 complete pending Teamlead/owner acceptance; R3 blocked only
on R2 Teamlead/owner acceptance

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

Deliverables:

- current README and contribution guide;
- architecture vision and nomenclature contract;
- slice-based roadmap;
- local testing and evidence procedure;
- Teamlead prompt, handoff, review, and acceptance protocol;
- active-development record tied to the exact historical baseline.

Exit gate:

- documentation is internally consistent;
- links and diff hygiene pass;
- no executable claim is added;
- accepted owner decisions and remaining proposals are distinguished explicitly.

## R1 — Reproduce and classify the historical baseline

**Status:** COMPLETE — OWNER ACCEPTED

**Goal:** establish what builds, what fails, and which dependencies or APIs are
historical residue.

Completed progression:

- R1-A reconnaissance: owner-accepted and merged in PR #3;
- R1-B dependency/MSRV reproducibility: owner-accepted and merged in PR #4;
- R1-C source correctness: owner-accepted and merged in PR #5;
- R1-D runtime safety, qualification CI, and R1 closeout: owner-accepted and
  merged in PR #6.

R1 work established the Rust 1.85.0 MSRV, committed `Cargo.lock` policy,
compile/test/rustdoc/Clippy classification, source-correctness repairs,
runtime-safety repairs, two-lane qualification CI, and prototype compatibility
position without broad historical cleanup.

Exit gate:

- the baseline result is reproducible from a clean checkout: PASS;
- every blocking failure was classified and repaired or retained as explicit
  non-blocking historical debt: PASS;
- the owner accepted the compatibility and versioning position: PASS.

**R1 exit criteria: satisfied.**

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
- the public core can be used by a downstream-style integration test and
  runnable example: PASS;
- Matrix no longer uses queue capacity as storage or shape: PASS;
- Miri or an equivalent deeper check is evaluated for the unsafe/aliasing scope:
  PASS — the owned R2 core adds no unsafe or aliasing machinery, so Miri is
  low-value here and must be reconsidered with borrowed mutable Lens semantics.

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
- explicitly compare a GAT-backed lending-view design with a simpler
  lifetime-generic design and adopt GATs only if they provide a concrete
  correctness/usability benefit;
- iteration and conversion rules;
- compile-time lifetime examples and negative API tests where useful.

Exit gate:

- Lenses cannot outlive their Matrix;
- mutable aliasing is rejected by the type system;
- selection boundaries are property-tested;
- view operations do not allocate unless documented.

## R4 — Reintroduce Gear, Cog, and Tag

**Goal:** turn the nomenclature into a coherent transformation API.

Planned work:

- a minimal Gear trait or operation protocol;
- distinct read-only and mutating transformation contracts unless working Lens
  evidence justifies another effect-safe design;
- first-class downstream-defined Gears without a required runtime registry;
- several concrete transformations with deterministic behavior;
- typed Cog requirements and validation;
- bounded Tag/provenance representation;
- execution reports that identify selection, operation, and outcome;
- examples demonstrating composition without runtime-pattern ceremony.

Exit gate:

- every concept has a distinct responsibility;
- a Gear cannot bypass Lens bounds;
- required context cannot be absent at runtime without a typed error;
- examples demonstrate both static and, only if justified, dynamic dispatch.

## R5 — API ergonomics and learning surface

**Goal:** make the library understandable and pleasant for a downstream user.

Planned work:

- crate-level rustdoc and module guides;
- runnable examples from construction through transformation;
- consistent naming, builders, conversions, and prelude policy;
- compile-fail tests for important misuse;
- error messages reviewed from the caller's perspective;
- an API stability and deprecation policy.

Exit gate:

- a new user can complete representative tasks from documentation alone;
- every public item is documented or intentionally hidden;
- examples are compiled in CI;
- downstream smoke tests pass on the declared MSRV.

## R6 — Measure, then optimize

**Goal:** establish performance evidence before adding complexity.

Planned work:

- Criterion benchmarks for representative shapes and operations;
- tall, moderately narrow consumer shapes including `32 x 24`, `1,024 x 64`,
  and `100,000 x 64` where host resources permit;
- allocation and copy accounting for Lens and Gear paths;
- comparison against direct underlying-storage operations;
- a stated overhead budget relative to direct `ndarray` operations;
- profiling before layout or dispatch changes;
- optional Rayon-backed execution only where thresholds show benefit.

Exit gate:

- benchmarks are stable enough to detect regressions;
- every optimization documents its tradeoff and baseline;
- parallel results preserve the accepted sequential semantics;
- performance claims name the workload and environment.

## R7 — Optional backends and integrations

**Goal:** add extensibility only after the core demonstrates a real need.

Candidate work:

- serialization and durable representation;
- sparse or mapped storage;
- backend/lending-view traits, with GATs considered only when real
  implementations justify the borrowing abstraction;
- optional persistence research;
- SurrealDB integration only with a concrete use case and isolated feature graph.

Exit gate:

- at least two real implementations justify the abstraction;
- default users do not pay for unused integrations;
- feature combinations and compatibility are tested;
- persistence does not become a hidden mutation or authority channel.

## R8 — Release qualification

**Goal:** prepare the first rehabilitated release candidate.

Planned work:

- version and compatibility decision;
- changelog, migration notes, package metadata, and license audit;
- MSRV/current-stable/downstream qualification;
- documentation and example audit;
- benchmark baseline and known-limitations statement;
- owner-controlled crates.io publication decision.

The likely release line is `0.2.0`, but version bumping, tagging, and publication
remain explicit owner decisions.

## Advanced Rust policy

GATs and higher-ranked trait bounds (HRTBs) are tools, not rehabilitation
goals. Use them only when they encode a real ownership, borrowing, lending, or
extensibility contract more clearly and safely than a simpler API.

R2 deliberately does not force GATs into the owned Matrix core. R3 explicitly
retains the GAT evaluation for Lens and lending views. A conceptual shape for
that comparison is:

    trait LendingView {
        type View<'a>
        where
            Self: 'a;

        fn view<'a>(&'a self) -> Self::View<'a>;
    }

This remains a design probe, not an R2 implementation contract. R3 must compare
it against a simpler lifetime-generic design and choose based on evidence.

Later backend abstractions may revisit GATs or HRTBs only when concrete
implementations justify the additional type-system complexity.

## Cross-cutting requirements

Testing, documentation, examples, and dependency review are not final cleanup
slices. Each functional slice must carry the evidence and learning surface needed
to make its own contract reviewable.
