# Matrical rehabilitation roadmap

**Campaign baseline:** `6deb812e11a519404fec90408bf95651764cd2f8`

**Current phase:** R0 — documentation and campaign foundation

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
- owner decisions remain visibly unresolved rather than silently assumed.

## R1 — Reproduce and classify the historical baseline

**Goal:** establish what builds, what fails, and which dependencies or APIs are
historical residue.

Planned work:

- select and record an MSRV and development toolchain policy;
- establish the repository `Cargo.lock` policy;
- run compile, unit-test, rustdoc, and Clippy reconnaissance without broad
  cleanup;
- classify compiler errors, warnings, unused dependencies, empty modules,
  placeholders, panic paths, and unreachable APIs;
- establish minimal CI for the accepted toolchain and one current stable lane;
- record whether 0.1.0 is treated as an unpublished prototype API.

Exit gate:

- the baseline result is reproducible from a clean checkout;
- every failure is classified as product, dependency, toolchain, environment,
  or harness debt;
- the owner accepts the compatibility and versioning position.

## R2 — Rebuild the core invariants

**Goal:** provide a small useful Matrix with typed failures.

Planned work:

- public `MatricalError` contract;
- checked `Shape`, `Index`, and rectangular `Region` types;
- owned dense `Matrix<T>` construction and access;
- deterministic iteration and conversion behavior;
- rustdoc examples and boundary-focused tests;
- removal or quarantine of queue-based placeholder storage.

Exit gate:

- no ordinary invalid shape, index, or region causes a panic;
- zero-sized and overflow boundaries are specified and tested;
- the public core can be used by a downstream example crate;
- Miri or an equivalent deeper check is evaluated for the unsafe/aliasing scope.

## R3 — Make Lens real

**Goal:** deliver safe, borrowing views over Matrix data.

Planned work:

- immutable rectangular Lens;
- mutable rectangular Lens with exclusive borrowing;
- row and column selectors;
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
- allocation and copy accounting for Lens and Gear paths;
- comparison against direct underlying-storage operations;
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
- backend/lending-view traits, potentially using GATs;
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

## Cross-cutting requirements

Testing, documentation, examples, and dependency review are not final cleanup
slices. Each functional slice must carry the evidence and learning surface needed
to make its own contract reviewable.
