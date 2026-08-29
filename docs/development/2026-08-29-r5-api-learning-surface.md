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

Review publication:

```text
branch rehab/r5-api-learning-surface
PR     #10 — R5: establish the public learning surface
state  ready for review; not draft; not merged
```

The last code-bearing R5 candidate before evidence-only closeout was:

```text
commit 93200ceb1b86b22ec8d60313436352268fcf775b
Qualification run #22: SUCCESS
```

The evidence-only closeout head is requalified after this report is committed;
its exact commit/tree and final CI run are returned in the Teamlead handoff
because a commit cannot self-record its own SHA.

`Cargo.lock` remains byte-identical to the accepted baseline. Preserved SHA-256:

```text
8abe052c6d793e87df19c1e6ade379caf3cad562eea693a946dc39c9e7180020
```

No dependency, edition, MSRV, package-version, tag, or publication change is part
of R5.

## README and learning surface

The README now describes the working R2–R4 library instead of the pre-R2
prototype. Stale claims that Matrix is unusable, core tests/examples are absent,
or Gear/Cog/Tag are future work were removed. It teaches:

```text
Matrix
  -> Lens / LensMut
  -> Gear (+ typed Cog)
  -> ExecutionReport (+ Tags)
```

The README quickstart corresponds to the compiled canonical R5 example, and the
status wording explicitly keeps Matrical at 0.1.0 without a production or release
readiness claim.

Crate-level `//!` rustdoc now explains purpose, Matrix ownership, Lens authority,
Gear/Cog/Tag composition, typed errors, public navigation, stability, and an
end-to-end compiled example.

`docs/getting-started.md` provides the task-oriented progression from Matrix
construction through custom Gear extension and error handling.
`docs/api-stability.md` records the current 0.1.0 breaking-change/deprecation
position. `docs/README.md` now prioritizes learning material and keeps campaign
evidence secondary.

## Public-surface audit

### Recommended everyday API

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

These names are explicitly exported at the crate root and through one
`matrical::prelude::*`.

### Specialized supported organization

`matrical::schematics` groups checked geometry/storage and
`matrical::strategies` groups Lens/Gear/Cog/Tag/reporting concepts. These module
paths are navigation aids rather than required ceremony.

### Hidden/legacy compatibility surface

The historical `operations` namespace remains source-accessible but
`#[doc(hidden)]` during 0.1.0 rehabilitation. `MatricalErrorType`,
`MatrixContext`, and `ElementContext` likewise remain documentation-hidden only
where historical signatures still require reachability.

### Removed prototype exposure

Prototype SQL/data-validation, Element internals, Vector internals, file-layout
modules, `AtomicBoolError`, the prototype `Error` container, and raw dependency
types are no longer normal downstream discovery/re-export surface.

Broad prototype-era crate-root glob re-exports were replaced with explicit named
supported exports. R2–R4 accepted concepts remain public.

Visibility narrowing initially exposed new dead-code diagnostics inside
prototype-only data/Element/Vector modules. R5 contains only those newly induced
diagnostics at the private module boundary with `#[allow(dead_code)]`; it does
not reconstruct or clean up those prototype implementations.

## Public documentation coverage

The recommended R2–R4 learning contract now documents:

- zero-sized Shape semantics;
- Region half-open bounds and valid empty Regions;
- Lens-local coordinates and Region revalidation;
- non-allocating Lens borrowing operations versus allocating `to_row_major`;
- LensMut exclusive parent-Matrix borrowing;
- ReadGear versus MutGear authority;
- typed Cog absence and validation;
- inert ordered Tags;
- ExecutionReport success semantics;
- deterministic built-in Gear behavior;
- typed caller-facing errors and structural matching.

Existing Matrix/Lens/Gear rustdoc compile-fail examples remain the primary misuse
evidence.

## Naming, builder, and conversion audit

No accepted R2–R4 public concept or method was renamed. These pairs remain
intentional:

```text
from_row_major / into_row_major / to_row_major
lens / lens_mut
row / row_mut
column / column_mut
execute_read / execute_mut
```

No builders were added. `Shape`, `Region`, `Cog`, `ScalarPolicy`, `ClampPolicy`,
and `Tag` have small typed constructors where builders would add ceremony without
solving a demonstrated usability problem.

No new `From`/`Into` conversion hides fallibility or allocation:

```text
Matrix::from_row_major  -> fallibly constructs owned Matrix
Matrix::into_row_major  -> consumes Matrix and returns owned values
Lens::to_row_major      -> clones the borrowed selection into new owned storage
```

## Error Display review

All `MatricalError` variants reachable through the accepted surface were reviewed.
Structural variants were preserved. Caller-facing Display wording is more
specific for `InvalidValue`, `InvalidContext`, and historical
`ShouldNotOccur`/`Regular`/`Custom` paths.

Qualification run #20 revealed that an inherited R2 integration test intentionally
asserts the exact `IndexOutOfBounds` text. R5 therefore restored and preserves:

```text
Index out of bounds
```

rather than changing that accepted wording for stylistic consistency. Focused R5
unit tests cover semantic wording for revised messages without freezing every
punctuation detail.

## Runnable examples and downstream smoke test

`examples/r5_quickstart.rs` uses only the prelude and demonstrates Shape, Matrix,
Region, Lens, SumGear, LensMut, typed `Cog<ScalarPolicy>`, AddScalarGear,
ExecutionReport, Tags, and the resulting parent Matrix.

`examples/r5_custom_gear.rs` demonstrates downstream-defined context,
`ValidateCog`, `ReadGear`, static dispatch, and typed output without private
modules, ndarray, a registry, or dynamic dispatch.

`tests/r5_public_api.rs` imports only `matrical::prelude::*` and exercises Matrix,
Index, Region, Lens, built-in read/mutating Gears, typed Cog, reports/Tags, a
custom downstream Gear, parent Matrix mutation, and structural `MatricalError`
matching.

Both examples compile through `cargo test --locked --all-targets`, and the R5
integration test passes in both qualification lanes.

## Compile-fail audit

The existing high-value cases are retained without duplication:

- Lens cannot outlive Matrix;
- two LensMut borrows from one Matrix cannot coexist;
- ReadGear cannot mutate through immutable Lens authority.

Qualification run #22 executed six doctests total, including all three
compile-fail cases; all passed on Rust 1.85.0 and stable.

## API stability and deprecation

`docs/api-stability.md` states that rehabilitation remains active at 0.1.0,
accepted concepts are the design direction rather than a SemVer guarantee, and
unfinished prototype APIs have no compatibility promise. Deliberate pre-release
breaks must be reviewed, documented, and reflected in guides/examples/tests. A
deprecation period is expected for real supported callers when safe coexistence
is useful; unfinished pre-release prototype APIs may be hidden or removed
directly. SemVer governs after an owner-authorized stability/release gate.

R5 authorizes no version, date, release, tag, or publication.

## Authority preservation and GAT/HRTB decision

R5 adds no convenience path equivalent to Gear receiving `&mut Matrix`, Gear
choosing arbitrary Regions, Tag-driven execution, `Any`/string Cog lookup,
mutable backend exposure, unchecked indexing, a dynamic Gear registry, or DI.
The caller-selected Lens/LensMut remains the transformation authority boundary.

R5 documents rather than reopens the accepted GAT/HRTB decision. No provider
abstraction or additional Region-selection authority was added.

## Unsafe audit

No R5 Rust change introduces an `unsafe` block, `unsafe fn`, or project-authored
unsafe abstraction. R5 makes no aliasing or performance implementation change.

## Documentation-only usability walkthrough

Using only README, crate rustdoc, `docs/getting-started.md`, and public examples:

```text
What problem does Matrical solve?        ANSWERED
How do I create a Matrix?                ANSWERED
How do I select data?                    ANSWERED
How do I mutate only a selection?        ANSWERED
What is a Gear?                          ANSWERED
Why do I need Cog?                       ANSWERED
What are Tags?                           ANSWERED
How do I write my own Gear?              ANSWERED
How do errors work?                      ANSWERED
What API stability should I expect?      ANSWERED
```

No answer requires rehabilitation evidence or implementation-source reading.

## Qualification history

### Repair loop

Initial commit `06e50cd6158059be084e291b57aeca1e5ec44f90` reached compilation in
both CI lanes, but Qualification run #20 failed the test step because R5 changed
the accepted exact `IndexOutOfBounds` Display wording. Repair commit
`64a0ccc375fec256c3db549bb9951b09e063225e` restored that contract; run #21 then
passed both lanes completely.

Visibility-diagnostic containment produced code-bearing candidate
`93200ceb1b86b22ec8d60313436352268fcf775b`; Qualification run #22 passed both
lanes completely.

### Rust 1.85.0

Observed compiler:

```text
rustc 1.85.0 (4d91de4e4 2025-02-17)
```

The workflow did not print `cargo --version` separately; all Cargo commands ran
under the installed 1.85.0 toolchain.

Run #22:

```text
cargo check --locked --all-targets      PASS
cargo test --locked --all-targets       PASS
cargo test --locked --doc               PASS
cargo clippy --locked --all-targets     PASS
cargo doc --locked --no-deps            PASS
```

Observed test evidence includes 62 library unit tests, R2/R3/R4/R5 integration
tests, all-target example compilation, and six passing doctests.

### Stable

Observed compiler:

```text
rustc 1.98.0 (88d9e12ae 2026-08-18)
```

The workflow did not print `cargo --version` separately; all Cargo commands ran
under the installed stable toolchain.

Run #22:

```text
cargo check --locked --all-targets      PASS
cargo test --locked --all-targets       PASS
cargo test --locked --doc               PASS
cargo clippy --locked --all-targets     PASS
cargo doc --locked --no-deps            PASS
```

### Diagnostic disposition

The accepted R4 baseline stable run produced 37 library warnings during check and
56 library warnings during Clippy. The R5 code-bearing candidate produces 20
library warnings during check and 32 during Clippy on stable; Rust 1.85.0 reports
the same 20/32 library warning counts.

Remaining diagnostics are historical prototype/operations/test debt. R5 therefore
adds no avoidable warning/Clippy debt to the recommended public surface and does
not perform a repo-wide cleanup.

## Mechanical and scope evidence

PR #10 changes exactly 18 authorized paths before this evidence-only report edit;
this report remains one of those same 18 paths. `Cargo.toml` and `Cargo.lock` are
unchanged. No `target/` path or generated build artifact is added to the PR.
Authored R5 text files retain final newlines. New relative documentation links
were reviewed against repository paths, and the README quickstart corresponds to
the canonical example that CI compiles.

The GitHub connector exposes the PR patch and exact repository identities but not
a shell-backed Git checkout. An attempted independent checkout in the execution
container was unavailable because that environment cannot resolve GitHub. Thus
`git diff --check` cannot honestly be reported as locally executed in this
session; the complete PR patch was instead inspected for whitespace damage and no
trailing-whitespace/patch-format issue was found. This is an evidence limitation,
not a Cargo or CI blocker.

The final Teamlead handoff records the final evidence-only head/tree, exact changed
paths, lockfile identity, final-newline/link/unsafe/target findings, and the final
exact-head two-lane CI result.

## Residual historical debt

R5 intentionally does not reconstruct the doc-hidden operation framework,
Element/Vector/SQL prototypes, detached compatibility contexts, historical
operation/test warnings, or dependency residue. R6 owns measurement and
optimization; R7 owns optional backends/integrations; R8 owns release
qualification.

## New R5 debt

None identified. Prototype compatibility residue remains deliberately classified
instead of being silently promoted into supported API.

## R5 exit result

Subject to Teamlead/owner acceptance of PR #10:

```text
R5: COMPLETE — TEAMLEAD/OWNER ACCEPTANCE PENDING
R5 PR: READY — NOT DRAFT
R5 MERGE: NOT AUTHORIZED
Next phase after merge: R6 — measure, then optimize
```

R6 must not begin on the R5 branch.
