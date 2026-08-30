# Matrical performance

Matrical treats performance claims as measured engineering evidence rather than
API promises. R6 established a repeatable Criterion harness, compared the
accepted R5 implementation with the R6 candidate on the same owner machine, and
optimized only the measured Lens traversal defect.

## Scope

The R6 harness measures the dense path that currently matters to the supported
API:

```text
Matrix
  -> Lens / LensMut
  -> direct traversal or Gear execution
```

The benchmark shapes are:

```text
32 x 24
1024 x 64
100000 x 64
```

Each shape exercises full, large-interior, single-row, single-column, and fixed
4 x 4 selections. Copy benchmarks cover `Lens::to_row_major()` and
`LensMut::to_row_major()` for full and fixed 4 x 4 selections.

Read traversal compares:

- a direct `ndarray` selected view;
- `Lens::iter()`;
- `execute_read(SumGear, ...)`.

Mutation compares:

- direct mutable `ndarray` selected traversal;
- `LensMut::iter_mut()`;
- `execute_mut(AddScalarGear, ...)`.

Lens/LensMut construction is measured separately so construction overhead cannot
be confused with traversal cost.

## Benchmark dependency and settings

Criterion is a development-only dependency:

```toml
criterion = { version = "=0.7.0", default-features = false, features = ["cargo_bench_support"] }
```

The two harnesses are `benches/r6_selection.rs` and
`benches/r6_transform.rs`, both with `harness = false`. They use
`std::hint::black_box`, sample size 10, a 500 ms warm-up, and a 2 s measurement
window.

Criterion is not a runtime dependency. Rayon is not present in the R6 dependency
graph.

Run the harness with:

```bash
cargo +stable bench --locked --bench r6_selection
cargo +stable bench --locked --bench r6_transform
```

Normal CI compiles benchmark targets but does not enforce wall-clock timing
thresholds. Performance timing from shared CI runners is diagnostic only.

## What R6 found

The accepted R5 Lens implementation validated a Region and then implemented
selected traversal by enumerating the entire parent Matrix, deriving each
element's row/column, and filtering for Region membership.

That made traversal proportional to parent Matrix size even when the selection
was tiny.

A preliminary shared-runner baseline made the defect unmistakable: a fixed 4 x 4
Lens over increasingly large parents grew with parent size while a direct
`ndarray` 4 x 4 view remained approximately constant. That preliminary run was
used to justify optimization, not as an authoritative performance claim.

R6 replaces that parent-wide scan with a private checked `ndarray` view of the
already validated Region. `Lens` and `LensMut` still expose the same public
Matrical API and authority boundary; `ndarray` remains private.

## Predeclared performance budget

The budget was fixed before the traversal source was changed:

1. fixed-size selection cost must not materially scale with unrelated parent
   cells;
2. for medium/large dense selections, Lens traversal should be no more than
   3.00x equivalent direct `ndarray` traversal;
3. for selections of at least 65,536 elements, Gear execution should be no more
   than 1.25x direct Lens/LensMut traversal;
4. tiny selections are judged with absolute cost as well as ratios because a
   few nanoseconds of framework work can make tiny ratios misleading.

## Authoritative owner-machine result

The authoritative before/after run was captured on 2026-08-30 from identical
benchmark code and lockfiles.

Environment:

```text
host             orion
OS/kernel        Debian Linux, 6.1.0-49-amd64
CPU              AMD Ryzen 7 3800XT, 8 cores / 16 threads
memory           31 GiB
virtualization   none
CPU governor     schedutil
rustc stable     1.98.0 (88d9e12ae 2026-08-18)
cargo stable     1.98.0 (797e8a9bc 2026-08-05)
```

Exact identities:

```text
accepted R5 merge
  commit acd15be9d02d27e6189aadedad3620e9558efe8f
  tree   bb4e2d1bb1b33254653873c9d5a4a11ca97e5add

baseline harness commit
  commit 91d1724a70c2af7ff5bd077dd8625b73302e0939
  tree   988e5a5638dd6267d765c150df7a1f2a400941bc

optimized source/test candidate
  commit 9a9f4199d28da4294bdf0973cb7579e4add5d78f
  tree   8220cc67a9e2a56df30ee71fdcf53854b0c4e43c

Cargo.lock SHA-256
  b835d1e7d4d851e883a209e2cc41b99aeb8982f70a73821f569e7f0ef98ae62a
```

The baseline harness commit contains accepted R5 source plus only the R6
benchmark/development dependency. The benchmark configuration, `Cargo.toml`,
`Cargo.lock`, and `benches/` contents were byte-identical between the baseline
and candidate measurement worktrees.

Representative Criterion medians:

| Benchmark | R5-source baseline | R6 candidate | Change |
| --- | ---: | ---: | ---: |
| 32x24 parent, fixed 4x4 Lens read | 2.659 us | 7.341 ns | 362x faster |
| 1024x64 parent, fixed 4x4 Lens read | 262.380 us | 7.329 ns | 35,800x faster |
| 100000x64 parent, fixed 4x4 Lens read | 30.694 ms | 7.242 ns | 4,238,449x faster |
| 100000x64 parent, fixed 4x4 Gear read | 31.111 ms | 9.004 ns | 3,455,319x faster |
| 100000x64 full Lens read | 30.066 ms | 4.374 ms | 6.87x faster |
| 100000x64 full Gear read | 29.772 ms | 4.501 ms | 6.61x faster |
| 100000x64 single-column Lens read | 30.842 ms | 491.520 us | 62.75x faster |
| 100000x64 fixed 4x4 LensMut transform | 30.269 ms | 17.367 ns | 1,742,903x faster |
| 100000x64 fixed 4x4 Gear transform | 29.124 ms | 35.778 ns | 814,020x faster |
| 100000x64 full LensMut transform | 30.318 ms | 6.384 ms | 4.75x faster |
| 100000x64 full Gear transform | 30.028 ms | 6.782 ms | 4.43x faster |
| 100000x64 fixed 4x4 `Lens::to_row_major()` | 30.047 ms | 35.753 ns | 840,405x faster |
| 100000x64 full `Lens::to_row_major()` | 42.526 ms | 11.036 ms | 3.85x faster |

The fixed 4 x 4 candidate Lens read was 7.341 ns, 7.329 ns, and 7.242 ns across
the three parent shapes. The largest/smallest timing ratio was 0.986x: the cost
no longer tracks unrelated parent cells.

The dense-path budget also passed:

| Candidate comparison | Ratio | Budget |
| --- | ---: | ---: |
| 100000x64 full Lens read / direct ndarray | 0.990x | <= 3.00x |
| 100000x64 interior Lens read / direct ndarray | 0.921x | <= 3.00x |
| 100000x64 full LensMut transform / direct ndarray | 1.000x | <= 3.00x |
| 100000x64 interior LensMut transform / direct ndarray | 0.892x | <= 3.00x |
| 1024x64 full Gear read / Lens | 0.997x | <= 1.25x |
| 100000x64 full Gear read / Lens | 1.029x | <= 1.25x |
| 1024x64 full Gear mutation / LensMut | 1.196x | <= 1.25x |
| 100000x64 full Gear mutation / LensMut | 1.062x | <= 1.25x |

These are measurements from one machine and toolchain, not universal throughput
guarantees.

## Construction tradeoff

Creating a fixed 4 x 4 Lens now constructs private ndarray view metadata. On the
largest parent the measured Lens construction median moved from about 1.782 ns
to 35.664 ns; LensMut moved from about 1.550 ns to 36.202 ns.

The candidate construction cost remains effectively constant across all three
parent shapes (roughly 35–36 ns) and performs no parent-wide traversal. R6 accepts
this small fixed cost because it removes microseconds-to-milliseconds of
selection-dependent waste from traversal and copying.

## Allocation and copy accounting

The supported R6 paths have these structural costs:

| Operation | Required heap/copy behavior |
| --- | --- |
| Lens/LensMut creation | no selected-value copy; no required heap allocation |
| Lens/LensMut iteration | no selected-value copy; no required heap allocation |
| `SumGear` body | no Gear-owned selected-value allocation |
| `AddScalarGear` body | in-place mutation; no selected-value allocation |
| empty execution Tags (`Vec::new()`) | no tag payload allocation |
| `Tag::source(&str)` | may allocate owned `String` storage for non-empty borrowed input |
| `to_row_major()` | allocates an output `Vec` and clones selected values |

R6 did not install a custom allocator or heap profiler, so this table is source
accounting rather than a claim about an exact allocator-call count. In
particular, `to_row_major()` is intentionally allocating, but R6 does not claim
that every allocator implementation performs exactly one allocation.

## Profiling limitation

The preliminary GitHub runner denied hardware profiling
(`perf_event_paranoid=4`). The owner machine had `perf_event_paranoid=3`, but
`perf` itself was not installed. R6 did not change host security policy or install
system tooling solely to obtain a profile.

The optimization decision therefore rests on two converging signals:

1. the pre-change benchmark scaled almost linearly with unrelated parent element
   count for fixed-size selections; and
2. source inspection located the parent-wide enumerate/filter traversal that
   exactly explains that scaling.

The same-machine before/after run then demonstrated that replacing that scan with
a checked private ndarray view removed the pathological scaling.

## Parallelism decision

R6 does **not** add Rayon.

After the targeted sequential repair, large dense Lens/LensMut traversal is
already approximately direct-ndarray speed and Gear overhead remains within the
predeclared budget. Rayon would therefore add a runtime dependency, thread-pool
scheduling, crossover thresholds, and a larger concurrency/authority test
surface without a measured R6 bottleneck that requires it.

Parallel execution remains a legitimate future experiment when a real workload
shows enough independent per-element computation to amortize scheduling cost.
It is not part of the current default contract.

## Performance contract

R6's durable contract is intentionally narrow:

- a Lens traverses only its selected Region rather than scanning unrelated parent
  cells;
- logical iteration remains deterministic row-major order;
- Lens/LensMut preserve caller-selected Region authority;
- ndarray storage/views remain private implementation details;
- no unsafe disjoint mutation or hidden parallel execution is introduced;
- `to_row_major()` remains the explicit allocating/cloning conversion;
- benchmark evidence is reproducible but machine-specific.

Detailed R6 chronology and qualification evidence are preserved in
[`development/2026-08-29-r6-measure-optimize.md`](development/2026-08-29-r6-measure-optimize.md).
