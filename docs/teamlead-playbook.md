# Matrical Teamlead campaign playbook

**Campaign:** Matrical rehabilitation

**Owner:** Anthony Gardner

**Teamlead responsibility:** architecture stewardship, slice sequencing,
fresh-session prompt generation, implementation handoff review, PR review,
validation assessment, and campaign-state tracking.

## Authority boundaries

The Teamlead may:

- assess repository state and recommend architecture;
- define bounded slices and exit criteria;
- generate implementation and peer-review prompts;
- review code, tests, documentation, and evidence;
- accept technical work at the Teamlead gate;
- identify blocks, residual risk, and the next recommended slice.

The owner retains authority over:

- merge and final repository acceptance;
- version changes and compatibility promises;
- tags, releases, and crates.io publication;
- major product-position changes;
- acceptance of unresolved risk or intentionally deferred correctness debt;
- dismissal of the normal Teamlead review requirement.

## Session workflow

### 1. Fresh reconnaissance

Every new implementation or review session begins from repository evidence, not
from conversational memory alone.

Record:

- repository and default branch;
- current `main` SHA and tree state;
- package version and toolchain policy;
- open PRs, issues, and active branches;
- active-development record and roadmap slice;
- relevant manifests, docs, tests, examples, and nearby implementation patterns;
- overlap with other workstreams;
- validation scope implied by the actual changed files.

If repository evidence conflicts with a handoff, stop and reconcile the
baseline before implementing.

### 2. Bounded implementation prompt

The Teamlead prompt must contain:

- mission and user value;
- accepted starting SHA and tree;
- owned files or modules;
- required behavior and invariants;
- explicit non-goals;
- compatibility and dependency constraints;
- required tests, docs, examples, and validation;
- PR and owner-control boundaries;
- exact handoff format.

Prompts should ask the developer to reassess the validation ladder from the
actual final diff rather than blindly execute a stale command list.

A durable repository template may require the Teamlead dispatch to inject its
exact starting commit and tree. This is necessary when the merge containing the
template or its prerequisite closeout determines the final baseline. The
dispatch—not the template's prediction—becomes authoritative, and the developer
must verify both values before implementation.

### 3. Focused development branch and draft PR

One session should normally create one focused branch and at most one draft PR.
It must not merge, mark ready, enable auto-merge, bump versions, tag, publish, or
dispatch unrelated workflows without explicit owner authority.

### 4. Developer handoff

The handoff must report:

- starting and final SHAs;
- concise implementation summary;
- changed files and public contracts;
- tests and examples added;
- exact commands with exit statuses;
- warnings, failures, environmental limits, and inherited debt;
- API or architecture decisions made;
- residual risks and deferred work;
- PR URL and current PR state.

### 5. Teamlead review

Review in this order:

1. baseline and scope integrity;
2. correctness and invariant preservation;
3. panic, aliasing, overflow, and boundary behavior;
4. public API ergonomics and compatibility;
5. tests, examples, and documentation;
6. dependency and feature impact;
7. validation evidence and exact-SHA attribution;
8. roadmap fit and residual risk.

The review outcome is one of:

- **ACCEPT** — technically ready for the next owner-controlled action;
- **ACCEPT WITH FOLLOW-UP** — accepted with separately bounded non-blocking debt;
- **HOLD** — correctable findings block acceptance;
- **STOP** — baseline, authority, scope, or architecture is invalid and must be
  reconciled before continuing.

### 6. Owner gate

Teamlead acceptance does not merge or release the work. The owner chooses the
authorized next action. For local qualification, use small gates:

```text
assessment -> narrow command block -> PASS/STOP -> review results
```

Do not issue a long chain of mutations before examining the preceding result.

### 7. Campaign update

After owner acceptance, update the active-development record with:

- accepted merge SHA;
- completed slice or sub-slice;
- evidence attribution;
- residual debt;
- newly authorized or blocked work;
- next recommended session prompt.

## PR review standard

A PR is not accepted merely because it compiles. The Teamlead must determine
whether it:

- solves the assigned problem without hidden scope expansion;
- makes invalid states harder to represent;
- has failure-boundary tests, not only happy-path tests;
- keeps the nomenclature coherent;
- avoids unearned safety or performance claims;
- leaves documentation truthful;
- preserves owner-controlled version and release boundaries.

Review comments should distinguish blocking findings from optional improvement.
Unrelated cleanup should become a separate issue or slice rather than expanding
the active PR.

## Base-of-operations record

The rehabilitation campaign begins from historical `main`:

```text
commit 6deb812e11a519404fec90408bf95651764cd2f8
version 0.1.0
```

The initial documentation foundation does not accept the historical source as a
working API. It establishes the process by which that source will be classified,
replaced, tested, and eventually released.

PR #1 established that foundation on accepted `main` commit
`b929e48481ae7ab41c972447b1547671afe4a4d8`, tree
`70d63b16f8d38da6de26d18c15b71c773e2b8f53`. Later session dispatches must use
the current accepted `main` identity rather than treating this R0 provenance as
a permanently executable baseline.
