# R1-A implementation prompt — historical baseline reconnaissance

**State:** TEMPLATE READY — requires an exact Teamlead dispatch baseline

**Repository:** `Gardlok/Matrical`

**Accepted R0 predecessor:**
`b929e48481ae7ab41c972447b1547671afe4a4d8`

**Required starting commit and tree:** supplied by the Teamlead at dispatch

Do not execute this repository template directly. The Teamlead dispatch must
name the exact current `main` commit and tree after all prerequisite
documentation merges. Stop if either value is absent or does not match the clean
checkout.

## Accepted campaign decisions

- semantic matrix-transformation library over mature numerical storage;
- initial dense storage based on `ndarray::Array2<T>`;
- Rust 1.85.0 initial MSRV;
- no compatibility promise for unfinished 0.1.0 prototype APIs;
- 0.2.0 first rehabilitated release target, subject to R8 qualification;
- SurrealDB deferred outside the immediate dependency graph;
- sequential and deterministic execution first;
- crates.io publication remains release-gated.

These decisions guide classification. They do not authorize implementation
changes in R1-A.

## Mission

Reproduce and classify the current Matrical build, test, documentation, and
dependency state from a clean checkout. Produce durable evidence and a narrowly
scoped reconnaissance report that enables the Teamlead to design R1-B without
guessing.

This is an evidence and classification session. It is not authorization to
rewrite the Matrix core, modernize every dependency, remove historical modules,
or make the warning output clean.

## Fresh reconnaissance

Before changing any file:

1. verify the repository, default branch, exact `main` SHA, and clean status;
2. verify the required starting SHA matches accepted `main`;
3. inspect open PRs, issues, and non-default branches for overlapping work;
4. read `README.md`, `CONTRIBUTING.md`, `docs/active-development.md`,
   `docs/roadmap.md`, `docs/architecture/vision.md`,
   `docs/architecture/consumers/longitudinal-feature-analysis.md`,
   `docs/testing-procedures.md`, and `docs/teamlead-playbook.md`;
5. inspect `Cargo.toml`, presence or absence of `Cargo.lock`, all Rust modules,
   tests, examples, benches, and CI configuration;
6. record installed Rust toolchains and host information relevant to dependency
   resolution;
7. stop if the baseline or campaign state conflicts with this prompt.

Do not rely on this prompt's historical observations when current repository
evidence can answer the question.

## Required investigation

### Manifest and dependency resolution

- Record Cargo package metadata and manifest warnings.
- Resolve dependencies without upgrading or editing them.
- Record whether resolution succeeds with the historical manifest.
- Identify direct dependencies unused by compiled behavior.
- Identify dependency, MSRV, feature, or yanked-version blockers.
- Do not run an unbounded dependency-update campaign.

### Compile and test surface

Attempt the MVECv1 ladder appropriate to the unchanged historical source:

- `cargo check --all-targets`;
- focused tests only if a compile boundary requires diagnosis;
- `cargo test --all-targets` when compilation permits;
- `cargo test --doc`;
- `cargo clippy --all-targets` as classification evidence, not as authorization
  to fix every warning;
- `cargo doc --no-deps` when compilation permits;
- examples only where those targets exist.

Capture explicit exit statuses and the first complete root-cause failure for each
blocked phase. Do not repeatedly rerun unexplained failures until one passes.

### Source classification

Classify, with file and line references:

- compile blockers;
- unconditional panic or recursion paths;
- public APIs that are unreachable, unsound in contract, or silent no-ops;
- empty and commented-out modules;
- tests that exercise the actual Matrix/Lens/Gear/Cog/Tag model versus unrelated
  utilities;
- duplicate validation abstractions;
- concurrency primitives without defined composite semantics;
- dependencies present only as architectural placeholders;
- documentation claims unsupported by executable evidence.

For each direct dependency, distinguish implemented use from historical intent.
In particular, classify SurrealDB, Crossbeam, DashMap, Rayon, Serde, and
benchmarking dependencies without removing or upgrading them in this slice.

Do not turn this report into a full redesign proposal. Relate findings to the
accepted architecture vision and identify the smallest likely R1-B repair
boundary.

## Allowed changes

- one dated reconnaissance report under `docs/development/`;
- a minimal update to `docs/active-development.md` that links the report and
  accurately records R1-A state;
- generated `Cargo.lock` only if the session separately explains whether it is
  evidence or an intended repository policy change. Do not commit it by default.

No Rust source, manifest, dependency, CI, version, README, roadmap, or
architecture change is authorized in R1-A.

## Required report

The report must include:

- starting SHA and tree state;
- host and Rust/Cargo versions;
- dependency-resolution result;
- exact commands and exit statuses;
- test counts where tests run;
- warning and failure classification;
- current target inventory;
- public-contract risk summary;
- recommended R1-B scope and explicit exclusions;
- evidence paths and cleanup/preservation status.

Do not include credentials, private paths unrelated to MVECv1, or enormous raw
build logs in Git. Summarize and include bounded diagnostic excerpts.

## Validation

Re-evaluate the final validation scope from the actual changed files. At minimum:

- `git diff --check`;
- relative Markdown link verification;
- confirmation that the final diff contains only the authorized documentation
  paths;
- exact-SHA attribution for every executable result.

## Pull-request boundary

Create one focused branch and one draft PR only after explicit owner authorization
for the required Git actions. Do not merge, mark ready, enable auto-merge, bump
the version, tag, publish, or manually dispatch workflows.

## Handoff

Return:

```text
Starting SHA:
Starting tree:
Final SHA:
Branch:
Draft PR:
Changed files:
Dependency-resolution result:
Compile/test/doc/Clippy results with exit statuses:
Root blockers:
Inherited debt:
Recommended R1-B boundary:
Residual risks:
```
