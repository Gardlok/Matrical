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

**Current phase:** R6 reviewable; Teamlead/owner acceptance pending

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
- passing Rust 1.85.0/stable qualification.

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

**Status:** COMPLETE — OWNER ACCEPTED — MERGED IN PR #10

**Accepted merge:**

```text
commit acd15be9d02d27e6189aadedad3620e9558efe8f
tree   bb4e2d1bb1b33254653873c9d5a4a11ca97e5add
```

R5 made the working R2–R4 library understandable without requiring
rehabilitation history or source inspection. It established the README/crate
rustdoc learning path, curated prelude and crate-root exports, task-oriented
getting-started material, runnable quickstart and custom-Gear examples,
downstream-style public API smoke coverage, explicit API-stability policy, and
caller-facing documentation of the accepted authority/lifetime rules.

R5 added no performance claims, Criterion, Rayon, backend abstraction,
persistence, release/tagging, or downstream application integration.

## R6 — Measure, then optimize

**Status:** REVIEWABLE — TEAMLEAD/OWNER ACCEPTANCE PENDING

**Baseline:**

```text
commit acd15be9d02d27e6189aadedad3620e9558efe8f
tree   bb4e2d1bb1b33254653873c9d5a4a11ca97e5add
```

**Goal:** establish performance evidence before adding complexity, optimize only
demonstrated waste, preserve the public API/authority boundary, and make an
evidence-driven parallelism decision.

Delivered candidate work:

- exact Criterion 0.7.0 development-only dependency with default features
  disabled and `cargo_bench_support` enabled;
- `r6_selection` and `r6_transform` benchmark harnesses for three Matrix sizes,
  five selection patterns, direct ndarray, Lens/LensMut, Gear execution, Lens
  construction, and explicit copy paths;
- a pre-optimization baseline showing that fixed-size Lens traversal incorrectly
  scaled with unrelated parent Matrix cells;
- an overhead budget declared before source optimization;
- a targeted private implementation repair that creates checked ndarray Region
  views and traverses only the selected data;
- semantic regression coverage for row-major ordering, single-row/column,
  empty/zero-dimensional selections, selected-only mutation, local access, and
  foreign-Region rejection;
- same-machine authoritative before/after Criterion evidence on the owner host;
- structural allocation/copy accounting;
- a documented profiling limitation rather than a host-policy workaround;
- an explicit decision not to add Rayon because the repaired sequential path is
  already approximately direct-ndarray speed.

Predeclared candidate budgets all pass:

```text
100000x64 full Lens read / direct ndarray          0.990x <= 3.00x
100000x64 interior Lens read / direct ndarray      0.921x <= 3.00x
100000x64 full LensMut / direct ndarray             1.000x <= 3.00x
100000x64 interior LensMut / direct ndarray         0.892x <= 3.00x
1024x64 full Gear read / Lens                      0.997x <= 1.25x
100000x64 full Gear read / Lens                    1.029x <= 1.25x
1024x64 full Gear mutation / LensMut               1.196x <= 1.25x
100000x64 full Gear mutation / LensMut             1.062x <= 1.25x
```

A fixed 4 x 4 Lens read on the largest `100000 x 64` parent moved from
30.694 ms to 7.242 ns in the authoritative owner-machine run, and candidate
fixed-selection time remained essentially constant across all three parent
shapes.

Exit gate:

- benchmark harness is reproducible and development-only;
- accepted R5 behavior is measured before source optimization;
- optimization is tied to an observed bottleneck rather than speculation;
- fixed-size selection no longer scans unrelated parent cells;
- dense Lens/LensMut and Gear overhead budgets pass;
- allocation/copy behavior and profiling limitations are explicit;
- no unsafe/public-backend/authority broadening is introduced;
- Rayon or another parallel path is added only if measurement justifies it;
- Rust 1.85.0 and stable code qualification pass;
- final review head passes ordinary PR CI;
- performance claims remain machine-specific and reproducible rather than
  universal promises.

See [performance.md](performance.md) and
[`development/2026-08-29-r6-measure-optimize.md`](development/2026-08-29-r6-measure-optimize.md).

## R7 — Optional backends and integrations

**Status:** BLOCKED ON R6 TEAMLEAD/OWNER ACCEPTANCE AND MERGE

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

No version bump, tag, release date, or publication is authorized by R6.

## Advanced Rust policy

GATs and HRTBs are tools, not rehabilitation goals. R3 found the concrete
lifetime-generic Lens API clearer with one proven provider. R4 added an authority
reason to keep that decision: a generic provider passed to a Gear could grant
Region-selection authority broader than the caller-selected Lens.

R6 does not reopen that public abstraction. Its optimization is private:
Lens/LensMut retain the same lifetime-generic public surface while their internal
borrow now points at the already selected ndarray view.

A future GAT/HRTB abstraction still requires a concrete composability problem
that cannot be expressed as clearly with the existing capability boundary.

## Cross-cutting requirements

Testing, documentation, examples, dependency review, performance evidence, and
authority analysis are part of every functional slice. They are not deferred
cleanup.
