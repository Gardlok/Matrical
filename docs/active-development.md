# Matrical active development

**Last updated:** 2026-08-20

## Accepted campaign baseline

```text
repository Gardlok/Matrical
branch     main
commit     b929e48481ae7ab41c972447b1547671afe4a4d8
tree       70d63b16f8d38da6de26d18c15b71c773e2b8f53
version    0.1.0
```

Commit `b929e48481ae7ab41c972447b1547671afe4a4d8` merged PR #1 and
established the rehabilitation foundation. The historical pre-campaign source
baseline remains `6deb812e11a519404fec90408bf95651764cd2f8` with tree
`9d643f5066c8e99ad111e5b0fe48265773a70092`.

Neither baseline is a claim that the public library is functional or
release-ready.

## Active campaign

**Campaign:** Matrical rehabilitation

**R0 status:** owner accepted

**Next slice:** R1-A — historical baseline reconnaissance

R1-A becomes dispatchable when this foundation-closeout record is merged. The
Teamlead dispatch must supply the exact resulting `main` commit and tree; a
repository prompt cannot safely predict the SHA of the merge that activates it.

## Accepted owner decisions

1. Matrical will be a semantic matrix-transformation library rather than a
   replacement for general linear-algebra kernels.
2. `ndarray::Array2<T>` is the initial dense-storage foundation.
3. Rust 1.85.0 is the initial MSRV.
4. The unfinished 0.1.0 prototype has no compatibility promise.
5. The first rehabilitated release targets 0.2.0, subject to R8 qualification
   and an explicit owner release gate.
6. SurrealDB leaves the immediate dependency graph and remains deferred
   optional-integration research.
7. Execution begins sequential and deterministic. Historical concurrency and
   parallelism dependencies must not remain without an implemented purpose;
   R1-A will classify them before a later slice removes or retains them.
8. Eventual crates.io publication remains a goal, but only after R8
   qualification and explicit owner authorization.

## Baseline findings that motivate rehabilitation

- `Matrix<V>` is a queue-capacity shell rather than a usable two-dimensional
  abstraction.
- region mutation exists directly over `ndarray::Array2<f64>` in Gear, but the
  behavior is not integrated with Matrix or Lens.
- some public validation paths return success without executing strategies.
- `MatricalError` debug formatting is recursively defined.
- Cog construction permits missing context that later code unwraps.
- the Vector implementation has trait bounds not implemented by Element.
- several operation modules and the top-level matrix tests are empty or
  commented placeholders.
- concurrency, parallelism, persistence, and zero-copy aspirations are not yet
  supported by defined public contracts or evidence.

These findings are inputs to R1 and later slices. R0 did not silently repair,
delete, or declare compatibility for the historical code.

## Downstream design input

The proposed analytical typing application is the first concrete downstream
consumer informing the rehabilitation campaign. Its non-binding design input is
recorded in
[`architecture/consumers/longitudinal-feature-analysis.md`](architecture/consumers/longitudinal-feature-analysis.md).

The consumer note does not make Matrical responsible for typing capture,
application identifiers, databases, cognitive-health interpretation, or
domain-specific analyzers. It supplies concrete pressures and acceptance inputs
for R2 through R6 while leaving their exact APIs open to evidence and review.

## R0 acceptance evidence

- PR #1 merged into `main` at the accepted commit and tree above.
- the accepted candidate tree exactly matches the merged tree;
- the change was documentation-only;
- `git diff --check`, trailing-whitespace inspection, and relative-link
  verification passed before merge;
- no executable behavior, dependency graph, version, or release state changed.

## Next authorized work

R1-A performs reproducible compile, dependency, test, rustdoc, and Clippy
reconnaissance. It classifies the historical source before broad editing,
establishes evidence for lockfile and dependency decisions, and recommends the
smallest justified R1-B repair boundary.

R1-A does not authorize Rust-source, manifest, dependency, CI, version, or API
changes.
