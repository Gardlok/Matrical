# Matrical rehabilitation roadmap

**Historical source baseline:** `6deb812e11a519404fec90408bf95651764cd2f8`

**Accepted R0 merge:** `b929e48481ae7ab41c972447b1547671afe4a4d8`

**Accepted R1-A merge:** `1c5ec09346f249496f1bb2e72095e073b348568a`

**Accepted R1-B merge:** `1a5e4a72d7c0bb2a6ddd92b070eb853e98d6f136`

**Accepted R1-C merge:** `16ddcc878c9cc8c8701dbc01453e08cfccd00b54`

**Accepted R1-D merge:** `059f148a99cfe2b5b881ada9af9acc286f584b6a`

**Accepted R2 merge:** `2f76a87e171a32a58a6d7244fdeb1b8794fc043a`

**Accepted R3 merge:** `9fbc712084a78570e8ac2b980ff0d4474c90ee7f`

**Current phase:** R4 active

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

Delivered the current documentation foundation, architecture/nomenclature
contract, slice roadmap, testing/evidence procedure, Teamlead workflow, and exact
historical baseline record.

## R1 — Reproduce and classify the historical baseline

**Status:** COMPLETE — OWNER ACCEPTED

R1-A through R1-D established the Rust 1.85.0 MSRV, committed lockfile policy,
reproducible qualification, source-correctness repairs, runtime-safety repairs,
two-lane CI, and prototype compatibility position.

**R1 exit criteria: satisfied.**

## R2 — Rebuild the core invariants

**Status:** COMPLETE — OWNER ACCEPTED — MERGED IN PR #7

R2 delivered checked `Shape`, `Index`, half-open `Region`, owned dense
`Matrix<T>`, typed construction/access failures, deterministic row-major
iteration, zero-sized/overflow/bounds coverage, downstream integration, runnable
example, and the explicit decision that missingness is not intrinsic Matrix
storage.

```text
R2: COMPLETE — OWNER ACCEPTED — MERGED IN PR #7
```

## R3 — Make Lens real

**Status:** COMPLETE — OWNER ACCEPTED — MERGED IN PR #8

R3 delivered:

- immutable `Lens<'a, T>` borrowing `&'a Matrix<T>`;
- mutable `LensMut<'a, T>` borrowing `&'a mut Matrix<T>`;
- Region revalidation against the receiving Matrix;
- Lens-local checked indexing;
- row and column selectors using the same rectangular Lens types;
- deterministic logical row-major iteration;
- borrowing construction/access/iteration with explicit allocating
  `to_row_major()` conversion;
- compile-fail lifetime and mutable-alias examples;
- boundary-focused Lens tests;
- no project-authored unsafe;
- Rust 1.85.0 and stable locked qualification.

R3 compared concrete lifetime-generic views with a public GAT lending provider
and deferred the provider trait. R4 reassesses that decision using real Gear
composition and capability authority.

```text
R3: COMPLETE — OWNER ACCEPTED — MERGED IN PR #8
Next phase: R4 — establish transformation composition
```

## R4 — Reintroduce Gear, Cog, and Tag

**Status:** IN DEVELOPMENT — ACTIVE

**Goal:** turn the nomenclature into a coherent transformation API while keeping
the selected Lens as the least-authority boundary.

Current R4 contract:

- separate `ReadGear<T>` and `MutGear<T>` static traits;
- read Gear receives only immutable `Lens` authority;
- mutating Gear receives only `LensMut` authority;
- no normal Gear access to Matrix storage, ndarray views, or region-selection
  authority;
- direct downstream trait implementation without registry/factory/DI ceremony;
- deterministic `SumGear`, `AddScalarGear`, `ScaleGear`, and `ClampGear` examples;
- typed `Cog<C>` context with `InvalidContext` for absence;
- small `ValidateCog` policy-validation contract;
- bounded inert Tag provenance with no query/command semantics;
- generic `ExecutionReport<O>` recording Gear identity, Region, typed effect,
  typed output, and ordered Tags;
- central `execute_read` / `execute_mut` boundaries that resolve and validate Cog
  before running Gear;
- explicit full/partial/empty selection behavior;
- external downstream-defined Gear integration and runnable composition example;
- compile-fail evidence that read Gear cannot mutate through its Lens;
- project-authored R4 unsafe target of zero.

Exit gate:

- every concept has a distinct responsibility;
- read-only and mutating authority are type-level distinct;
- a Gear cannot bypass the supplied Lens Region through the public contract;
- downstream-defined Gear works without registry or `Any`;
- required Cog absence is `InvalidContext` and invalid policy is typed failure;
- Tag is bounded, deterministic, and non-executable;
- ExecutionReport preserves identity, Region, effect, typed output, and Tags;
- only selected Region mutates;
- empty selection behavior is explicit and panic-free;
- GAT/HRTB and dynamic dispatch decisions are evidence-backed;
- Rust 1.85.0 and stable exact-head CI are green with unchanged lockfile.

R5 remains blocked until R4 is reviewable and receives Teamlead/owner acceptance.

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

Planned work includes representative benchmarks, allocation/copy accounting,
comparison with direct underlying-storage operations, profiling, and optional
parallel execution only where evidence demonstrates a benefit while preserving
accepted sequential semantics.

## R7 — Optional backends and integrations

**Goal:** add extensibility only after the core demonstrates a real need.

Candidate work:

- serialization and durable representation;
- sparse or mapped storage;
- backend/lending-view traits only when real implementations justify them;
- optional persistence research;
- SurrealDB integration only with a concrete use case and isolated feature graph.

Exit gate requires at least two real implementations to justify a shared backend
abstraction and no hidden mutation or authority channel.

## R8 — Release qualification

**Goal:** prepare the first rehabilitated release candidate.

Planned work includes version/compatibility decision, changelog and migration
notes, package metadata/license audit, MSRV/stable/downstream qualification,
documentation/example audit, benchmark baseline, known limitations, and explicit
owner-controlled crates.io publication.

The likely release line remains `0.2.0`; version bumping, tagging, and publication
remain owner decisions.

## Advanced Rust policy

GATs and HRTBs are tools, not rehabilitation goals. Use them only when they
encode a real ownership, borrowing, lending, or extensibility contract more
clearly and safely than a simpler API.

R3's concrete Lens design was selected partly because Matrix was the only proven
provider. R4 supplies a stronger authority-based reassessment. A Gear should
consume an already caller-selected `Lens`/`LensMut`; giving a Gear a generic
lending provider could also give it the ability to choose a larger Region. That
is broader authority than the current transformation requires.

A public GAT `LendingView` provider would preserve static dispatch and might help
future providers, but R4 demonstrates no current provider plurality or ergonomic
benefit that outweighs the larger API and Rust 1.85 diagnostics. More
importantly, reproducing the current least-authority boundary on top of a generic
provider would add indirection just to prevent the Gear from exercising provider
selection authority. R4 therefore defers a public GAT provider again for a new,
architecture-specific reason: the transformation layer actively prefers the
narrower Lens capability.

R4 also finds no genuine callback/adapter requiring `for<'a> Fn(&Lens<'a, T>)`,
so no HRTB surface is added merely for sophistication. Static Gear dispatch is
sufficient for the demonstrated downstream extension; heterogeneous runtime
Gear registries remain deferred until a real consumer requires them.

## Cross-cutting requirements

Testing, documentation, examples, and dependency review are not final cleanup
slices. Each functional slice must carry the evidence and learning surface needed
to make its own contract reviewable.
