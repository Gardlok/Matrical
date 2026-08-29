# R5 API learning surface — implementation and qualification record

## Status

```text
R1: COMPLETE — OWNER ACCEPTED
R2: COMPLETE — OWNER ACCEPTED
R3: COMPLETE — OWNER ACCEPTED
R4: COMPLETE — OWNER ACCEPTED — MERGED IN PR #9
R5: ACTIVE — implementation candidate
```

Exact R5 baseline:

```text
commit 6dc0320d1857d1c4fafd538fbf75ae80566887cc
tree   c421102b113b2dc2fc78373677a956e807dee7db
version 0.1.0
```

Expected and preserved `Cargo.lock` SHA-256:

```text
8abe052c6d793e87df19c1e6ade379caf3cad562eea693a946dc39c9e7180020
```

## README disposition

The README now describes the working R2–R4 library rather than the pre-R2
prototype. Stale claims that Matrix is unusable, core tests/examples are absent,
or Gear/Cog/Tag are future work were removed. It teaches the accepted
Matrix -> Lens/LensMut -> Gear(+Cog) -> ExecutionReport(+Tags) flow, includes a
quickstart corresponding to the compiled R5 example, and states the 0.1.0
pre-release position without claiming production/release readiness.

## Crate-root and public-surface audit

Classification used for R5:

### A. Recommended everyday API

```text
MatricalError
Shape, Index, Region, Matrix
Lens, LensMut
ReadGear, MutGear
Cog, ValidateCog, ScalarPolicy, ClampPolicy
execute_read, execute_mut
ExecutionReport, GearEffect
SumGear, AddScalarGear, ScaleGear, ClampGear
Tag, TagStage
```

These names are exported at the crate root and through one `prelude`.

### B. Specialized but legitimate organization

`matrical::schematics` groups validated geometry/storage and
`matrical::strategies` groups views/transformation/context/provenance. They are
navigation aids, not ceremony required by normal callers.

### C. Historical/prototype compatibility surface

The historical `operations` namespace remains public but `#[doc(hidden)]` so
existing 0.1.0 source can still reach it without making it part of the learning
contract. `MatricalErrorType`, `MatrixContext`, and `ElementContext` remain
reachable only as documentation-hidden compatibility types where historical
public signatures still require them.

### D. Implementation/prototype detail removed from downstream exposure

Prototype SQL/data validation, Element internals, Vector internals, private
Matrix implementation module layout, private Gear/Lens/Cog/Tag file modules,
`AtomicBoolError`, and the prototype `Error` container are crate-internal. Raw
Crossbeam and ndarray types are not re-exported.

## Prelude policy

`matrical::prelude::*` is the recommended everyday import. It contains only the
high-frequency accepted R2–R4 surface listed above. There is no second prelude.
Crate-root named exports remain supported and discoverable; grouped module paths
remain available for deeper navigation.

## Module visibility/re-export decision

Broad prototype-era glob re-exports were removed from the crate root. The root
now explicitly exports supported names. `schematics` and `strategies` explicitly
re-export supported concepts while their file-layout modules are crate-private.
This prevents accidental historical source organization from defining the public
API without physically rewriting unrelated implementation.

## Public documentation coverage

Crate-level rustdoc now explains purpose, ownership, Lens authority, Gear/Cog/Tag
composition, errors, navigation, stability, and an end-to-end compiled example.
Existing Matrix/Lens/Gear docs already describe the core contracts and compile-fail
misuse. R5 strengthens Cog/Tag/error documentation and module-level navigation.

Key contracts documented for users:

- zero-sized Shape semantics;
- Region half-open bounds and valid empty Regions;
- Lens-local coordinates and revalidation;
- borrowing Lens operations versus allocating `to_row_major`;
- LensMut whole-Matrix exclusive borrow;
- ReadGear versus MutGear authority;
- typed Cog absence and validation;
- inert ordered Tags;
- typed ExecutionReport success semantics;
- deterministic built-in Gear behavior.

## Naming audit

No accepted R2–R4 concept or method was renamed. The existing pairs remain
coherent:

```text
from_row_major / into_row_major / to_row_major
lens / lens_mut
row / row_mut
column / column_mut
execute_read / execute_mut
```

The ownership/copy distinction in the row-major conversion names is useful and
was preserved.

## Builder decision

No builders were added. `Shape`, `Region`, `Cog`, `ScalarPolicy`, `ClampPolicy`,
and `Tag` have small typed constructors with few required arguments. A builder
would add ceremony without solving ambiguity and would risk resurrecting the
prototype's factory/builder style.

## Conversion decision

No new `From`/`Into` conversions were added. Fallible Matrix construction remains
explicit through `from_row_major`; consuming ownership is visible in
`into_row_major`; cloning a borrowed selection is visible in `to_row_major`.
Matrical does not hide fallibility or allocation behind infallible conversion
traits.

## Error-message review

All `MatricalError` variants reachable through the accepted surface were reviewed.
Structural variants were preserved. Caller-facing Display wording is now more
specific for `InvalidValue`, `InvalidContext`, `IndexOutOfBounds`, and the
historical `ShouldNotOccur` path. Historical `Regular` and `Custom` variants are
identified as legacy rather than presented as the modern error design. Focused
tests assert semantic wording without freezing complete punctuation.

## Getting-started and runnable examples

`docs/getting-started.md` teaches Matrix creation, checked Index access, Region
selection, Lens borrowing, LensMut transformation, Cog policy, report/Tags,
custom Gear extension, errors, conversions, and next steps.

`examples/r5_quickstart.rs` is the canonical beginner workflow using only the
prelude. `examples/r5_custom_gear.rs` proves external-style custom context,
`ValidateCog`, `ReadGear`, static dispatch, and typed output without private
modules, ndarray, registry, or dynamic dispatch.

## Compile-fail audit

The existing high-value rustdoc cases remain:

- Lens cannot outlive Matrix;
- two LensMut borrows through one Matrix cannot coexist;
- ReadGear cannot mutate through immutable Lens authority.

R5 did not duplicate these cases merely to increase count. Fallible construction
is instead demonstrated through normal typed-error tests because misuse is a
runtime `Result` contract, not a type-system prohibition.

## Downstream public API smoke gate

`tests/r5_public_api.rs` imports only `matrical::prelude::*` and proves Matrix
construction, Index access, Region selection, Lens-local read, built-in read Gear,
mutating Gear with typed Cog, report/Tags, downstream custom Gear, parent Matrix
mutation result, and structural `MatricalError` matching.

## API stability and deprecation

`docs/api-stability.md` records that rehabilitation is active at 0.1.0, accepted
concepts are design direction rather than a SemVer guarantee, prototype APIs have
no compatibility promise, deliberate pre-release breaks must update docs/tests/
examples and record migration impact, and a deprecation period is expected for
real supported callers where coexistence is practical. No release, version, tag,
or publication is authorized by R5.

## Authority preservation

R5 adds no API that gives Gear `&mut Matrix`, arbitrary Region selection,
mutable backend access, Tag-driven execution, `Any`/string Cog lookup, unchecked
indexing, dynamic Gear registration, GAT provider authority, or HRTB ceremony.
The R3/R4 least-authority design is unchanged.

## Unsafe audit

R5 introduces no `unsafe` block, `unsafe fn`, or project-authored unsafe
abstraction. Existing R2–R4 code likewise preserves the safe Matrix/Lens
boundary; R5 makes no aliasing/performance change.

## Documentation-only usability walkthrough

Using only README, crate rustdoc, `docs/getting-started.md`, and public examples:

- What problem does Matrical solve? **Answered** — semantic checked matrix
  selection/transformation with typed context and provenance.
- How do I create a Matrix? **Answered** — Shape + from_row_major.
- How do I select data? **Answered** — half-open Region + Lens.
- How do I mutate only a selection? **Answered** — LensMut + MutGear.
- What is a Gear? **Answered** — static transformation constrained to supplied
  Lens authority.
- Why do I need Cog? **Answered** — typed context/policy validated centrally.
- What are Tags? **Answered** — inert ordered provenance on successful reports.
- How do I write my own Gear? **Answered** — public custom Gear example.
- How do errors work? **Answered** — typed variants + Display.
- What API stability should I expect? **Answered** — explicit 0.1.0 policy.

No answer requires the development-history reports or implementation source.

## Qualification record

This section is completed against the exact final PR head. Required lanes are:

```text
Rust 1.85.0
  cargo check --locked --all-targets
  cargo test --locked --all-targets
  cargo test --locked --doc
  cargo clippy --locked --all-targets
  cargo doc --locked --no-deps

stable
  same five gates
```

The repository Qualification workflow executes both lanes. R5 records exact
observed versions, CI run result, final commit/tree, changed paths, lockfile
identity, diff/link/newline checks, and repository-local target inspection in the
final Teamlead handoff.

## Residual historical debt

R5 intentionally does not reconstruct the doc-hidden operation framework,
Element/Vector/SQL prototypes, detached compatibility contexts, dependency graph,
or inherited prototype warnings. R6 owns measurement and optimization; R7 owns
optional backends/integrations; R8 owns release qualification.

## R5 exit result

Pending exact-head qualification and Teamlead/owner review, the intended exit is:

```text
R5: COMPLETE — TEAMLEAD/OWNER ACCEPTANCE PENDING
Next phase after merge: R6 — measure, then optimize
```

R6 must not begin on the R5 branch.
