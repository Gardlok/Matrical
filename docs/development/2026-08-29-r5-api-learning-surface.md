# R5 API learning surface — implementation and qualification record

## Status

```text
R1: COMPLETE — OWNER ACCEPTED
R2: COMPLETE — OWNER ACCEPTED
R3: COMPLETE — OWNER ACCEPTED
R4: COMPLETE — OWNER ACCEPTED — MERGED IN PR #9
R5: COMPLETE — TEAMLEAD/OWNER ACCEPTANCE PENDING
```

Exact R5 baseline:

```text
commit 6dc0320d1857d1c4fafd538fbf75ae80566887cc
tree   c421102b113b2dc2fc78373677a956e807dee7db
version 0.1.0
```

R5 branch and review publication:

```text
branch rehab/r5-api-learning-surface
PR     #10 — R5: establish the public learning surface
state  ready for review; not draft; not merged
```

The last code-bearing candidate before this evidence-only closeout was:

```text
commit 93200ceb1b86b22ec8d60313436352268fcf775b
Qualification run #22: SUCCESS
```

The evidence-only closeout commit is requalified as the final exact PR head; its
identity and final run are recorded in the Teamlead handoff because a commit
cannot self-record its own SHA.

`Cargo.lock` is byte-identical to the accepted baseline. Its preserved SHA-256 is:

```text
8abe052c6d793e87df19c1e6ade379caf3cad562eea693a946dc39c9e7180020
```

No dependency, edition, MSRV, or package-version metadata changed.

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

Visibility narrowing initially exposed new dead-code diagnostics inside the
prototype-only data/Element/Vector modules. R5 contains only those newly induced
diagnostics at the private module boundary with `#[allow(dead_code)]`; it does
not reconstruct or clean up the prototype implementation.

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
- Lens-local coordinates and Region revalidation;
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
specific for `InvalidValue`, `InvalidContext`, and the historical
`ShouldNotOccur`, `Regular`, and `Custom` paths.

The first R5 CI run revealed that an inherited R2 integration test intentionally
asserts the exact `IndexOutOfBounds` Display text. R5 therefore restored and
preserves the accepted wording `Index out of bounds` rather than changing that
contract for stylistic consistency. The structural variant remains unchanged.
Focused R5 tests assert semantic wording for revised messages without freezing
unnecessary punctuation.

## Getting-started and runnable examples

`docs/getting-started.md` teaches Matrix creation, checked Index access, Region
selection, Lens borrowing, LensMut transformation, Cog policy, report/Tags,
custom Gear extension, errors, conversions, and next steps.

`examples/r5_quickstart.rs` is the canonical beginner workflow using only the
prelude. `examples/r5_custom_gear.rs` proves external-style custom context,
`ValidateCog`, `ReadGear`, static dispatch, and typed output without private
modules, ndarray, registry, or dynamic dispatch.

Both examples are compiled by `cargo test --locked --all-targets` in both
qualification lanes.

## Compile-fail audit

The existing high-value rustdoc cases remain:

- Lens cannot outlive Matrix;
- two LensMut borrows through one Matrix cannot coexist;
- ReadGear cannot mutate through immutable Lens authority.

R5 did not duplicate these cases merely to increase count. Fallible construction
is instead demonstrated through normal typed-error tests because misuse is a
runtime `Result` contract, not a type-system prohibition.

Qualification run #22 executed six doctests total, including all three existing
compile-fail cases, and all six passed on Rust 1.85.0 and stable.

## Downstream public API smoke gate

`tests/r5_public_api.rs` imports only `matrical::prelude::*` and proves Matrix
construction, Index access, Region selection, Lens-local read, built-in read Gear,
mutating Gear with typed Cog, report/Tags, downstream custom Gear, parent Matrix
mutation result, and structural `MatricalError` matching.

The R5 integration test passed in both qualification lanes.

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
abstraction. R5 makes no aliasing or performance implementation change.

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

### Initial repair loop

Qualification run #20 on initial commit
`06e50cd6158059be084e291b57aeca1e5ec44f90` compiled successfully in both lanes
but failed the test step because R5 had changed the accepted exact
`IndexOutOfBounds` Display wording. The in-scope repair commit
`64a0ccc375fec256c3db549bb9951b09e063225e` restored the accepted wording.
Qualification run #21 then passed both lanes completely.

A follow-up visibility-diagnostic containment commit produced code-bearing
candidate `93200ceb1b86b22ec8d60313436352268fcf775b`. Qualification run #22 passed
both lanes completely.

### Rust 1.85.0

Observed compiler:

```text
rustc 1.85.0 (4d91de4e4 2025-02-17)
```

The workflow did not separately print `cargo --version`; all Cargo commands ran
under the installed Rust 1.85.0 toolchain.

Qualification run #22:

```text
cargo check --locked --all-targets      PASS
cargo test --locked --all-targets       PASS
cargo test --locked --doc               PASS
cargo clippy --locked --all-targets     PASS
cargo doc --locked --no-deps            PASS
```

Observed test evidence includes 62 library unit tests, the R2/R3/R4/R5 integration
tests, all-target example compilation, and six passing doctests including the
three compile-fail authority/lifetime cases.

### Stable

Observed compiler:

```text
rustc 1.98.0 (88d9e12ae 2026-08-18)
```

The workflow did not separately print `cargo --version`; all Cargo commands ran
under the installed stable toolchain.

Qualification run #22:

```text
cargo check --locked --all-targets      PASS
cargo test --locked --all-targets       PASS
cargo test --locked --doc               PASS
cargo clippy --locked --all-targets     PASS
cargo doc --locked --no-deps            PASS
```

### Diagnostic disposition

The accepted R4 baseline's stable qualification produced 37 library warnings at
`cargo check` and 56 library warnings at `cargo clippy`. The R5 code-bearing
candidate produced 20 library warnings at `cargo check` and 32 library warnings
at `cargo clippy` on stable; Rust 1.85.0 reports the same 20/32 library warning
counts. Remaining diagnostics are inherited prototype/operations/test debt and
are not caused by the recommended R5 public API.

R5 therefore adds no avoidable warning/Clippy debt to the recommended surface and
does not run a repo-wide historical cleanup.

### Mechanical and scope evidence

The R5 PR changes exactly 18 authorized paths. `Cargo.toml` and `Cargo.lock` are
not changed. No repository-local `target/` path is added or modified. All authored
text files retain final newlines. New and changed relative documentation links
were audited against repository paths, and the README quickstart corresponds to
the compiled canonical example. The PR patch was inspected for accidental scope,
generated artifacts, dependency changes, and whitespace damage before closeout.

The final Teamlead handoff records the final exact evidence-only head/tree,
`git diff --check` result from the available clean checkout audit, final PR scope,
and the final exact-head CI run.

## Residual historical debt

R5 intentionally does not reconstruct the doc-hidden operation framework,
Element/Vector/SQL prototypes, detached compatibility contexts, dependency graph,
or inherited prototype warnings. Historical operation and test diagnostics remain
visible in CI but are non-failing and pre-existing in substance. R6 owns
measurement and optimization; R7 owns optional backends/integrations; R8 owns
release qualification.

## New R5 debt

None identified. The accepted API-learning surface compiles and is exercised from
downstream-style imports. Prototype compatibility residue remains deliberately
classified rather than silently promoted into the supported API.

## R5 exit result

Subject to Teamlead/owner acceptance of PR #10:

```text
R5: COMPLETE — TEAMLEAD/OWNER ACCEPTANCE PENDING
R5 PR: READY — NOT DRAFT
R5 MERGE: NOT AUTHORIZED
Next phase after merge: R6 — measure, then optimize
```

R6 must not begin on the R5 branch.
