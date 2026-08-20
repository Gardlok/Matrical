# Contributing to Matrical

Matrical is in a controlled rehabilitation campaign. Contributions are welcome,
but the current priority is converging on a small coherent library rather than
expanding the historical prototype in every planned direction at once.

## Before starting

Read:

1. [`docs/active-development.md`](docs/active-development.md);
2. [`docs/roadmap.md`](docs/roadmap.md);
3. [`docs/architecture/vision.md`](docs/architecture/vision.md);
4. [`docs/testing-procedures.md`](docs/testing-procedures.md);
5. [`docs/teamlead-playbook.md`](docs/teamlead-playbook.md).

Confirm the current `main` SHA, open pull requests, open issues, and the active
slice before relying on a prior handoff or conversation.

## Scope

Each change should have one bounded purpose and explicit exit criteria. Do not
combine an API redesign, dependency cleanup, formatting campaign, benchmark
rewrite, and documentation overhaul in one implementation PR.

Unrelated warnings and debt should be recorded rather than silently absorbed.

## Branches and pull requests

Use a focused branch name such as:

```text
docs/rehabilitation-foundation
fix/error-contract
feat/core-region
test/lens-properties
```

Open a draft pull request unless the owner or Teamlead explicitly requests
otherwise. A development session must not merge, mark ready, enable auto-merge,
tag, publish, or change the release version without explicit owner authority.

The pull request should state:

- accepted starting SHA;
- intended scope and exclusions;
- changed files and public contracts;
- exact validation commands and exit statuses;
- tests added or changed;
- known residual risks or deferred work.

## Implementation expectations

- Preserve the Matrix/Lens/Gear/Cog/Tag vocabulary unless an accepted
  architecture decision changes it.
- Encode shape, indexing, and region invariants in types and constructors.
- Do not introduce a panic path into a public fallible API.
- Do not claim thread safety, zero-copy behavior, or performance without a
  precise contract and evidence.
- Keep dependencies minimal and feature-gated where appropriate.
- Add rustdoc and examples for public behavior.
- Test failure boundaries as well as happy paths.

## Validation

Follow the [Matrical testing procedures](docs/testing-procedures.md). Select the
smallest validation ladder that proves the changed boundary, capture explicit
exit statuses, and attribute executable evidence to the exact functional SHA.

Documentation-only follow-up commits do not inherit or reattribute executable
test evidence.

## Review and acceptance

Passing tests makes a change reviewable; it does not make it accepted. The
Teamlead reviews scope, architecture, API behavior, tests, documentation, and
evidence. The repository owner retains final authority over merge, version,
release, publication, and risk acceptance decisions.
