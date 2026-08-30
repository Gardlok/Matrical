# R6 measure and optimize — implementation and qualification record

## Status

```text
R1: COMPLETE — OWNER ACCEPTED
R2: COMPLETE — OWNER ACCEPTED
R3: COMPLETE — OWNER ACCEPTED
R4: COMPLETE — OWNER ACCEPTED
R5: COMPLETE — OWNER ACCEPTED — MERGED IN PR #10
R6: COMPLETE — TEAMLEAD/OWNER ACCEPTANCE PENDING
```

Exact accepted R6 baseline:

```text
commit  acd15be9d02d27e6189aadedad3620e9558efe8f
tree    bb4e2d1bb1b33254653873c9d5a4a11ca97e5add
version 0.1.0
```

This is the verified merge of PR #10 and the owner-accepted R5 result.

R6 branch:

```text
rehab/r6-measure-optimize
```

R6 changes no package version, edition, MSRV, release/tag state, public ndarray
exposure, Gear authority, or publication state.

## Mission

R6 was required to:

1. measure the accepted R5 behavior before changing performance-sensitive source;
2. establish a representative, reproducible benchmark harness;
3. compare Matrical Lens/LensMut/Gear paths with direct private-backend behavior;
4. identify allocation/copy costs and a concrete hotspot;
5. declare an overhead budget before optimizing;
6. change source only when the evidence justified it;
7. preserve public API, deterministic semantics, and the caller-selected
   Lens/LensMut authority boundary;
8. evaluate Rayon only after the sequential evidence was known;
9. keep performance timing out of ordinary CI pass/fail policy;
10. preserve exact qualification/evidence for Teamlead review.

R6 does not begin optional backends, persistence, release qualification, or R7.

## Benchmark harness identity

The permanent benchmark harness was established first:

```text
commit 91d1724a70c2af7ff5bd077dd8625b73302e0939
tree   988e5a5638dd6267d765c150df7a1f2a400941bc
parent acd15be9d02d27e6189aadedad3620e9558efe8f
subject bench: establish R6 performance harness
```

The source implementation at this commit is still the accepted R5 Lens
implementation. The commit adds only the measurement dependency/lockfile and
benchmark targets needed to observe it.

Criterion configuration:

```toml
[dev-dependencies]
criterion = { version = "=0.7.0", default-features = false, features = ["cargo_bench_support"] }

[[bench]]
name = "r6_selection"
harness = false

[[bench]]
name = "r6_transform"
harness = false
```

R6 Cargo.lock identity:

```text
SHA-256 b835d1e7d4d851e883a209e2cc41b99aeb8982f70a73821f569e7f0ef98ae62a
```

The lockfile contains Criterion 0.7.0 and its required benchmark dependencies.
Rayon is not present.

The harness uses `std::hint::black_box` and:

```text
sample size       10
warm-up           500 ms
measurement time  2 s
```

Matrix shapes:

```text
32 x 24
1024 x 64
100000 x 64
```

Selections:

```text
full
large interior (quarter margins)
single row
single column
fixed 4 x 4
```

Read levels:

```text
direct ndarray selected-view sum
prebuilt Lens iter().sum()
execute_read(SumGear) over the same Lens
```

Mutation levels:

```text
direct ndarray selected-view add-scalar
direct LensMut iter_mut() add-scalar
execute_mut(AddScalarGear) over the same LensMut
```

Selection creation is benchmarked separately. `to_row_major()` is measured
separately for full and fixed 4 x 4 selections and records byte throughput.

## Measure first: accepted R5 source

The accepted R5 Lens implementation stored a reference to the full Matrix and
implemented selected iteration with a helper equivalent to:

```text
matrix.iter()
-> enumerate every parent element
-> derive parent row/column
-> filter_map elements whose coordinates are inside Region
```

That design suggested a concrete hypothesis: selection traversal cost could scale
with total parent element count instead of selected element count.

A preliminary shared GitHub runner was used only to test that hypothesis. It is
not treated as an authoritative wall-clock performance source.

Representative preliminary fixed 4 x 4 Lens read medians were:

```text
parent 32x24       ~1.47 us
parent 1024x64     ~143.50 us
parent 100000x64   ~14.00 ms
```

The equivalent direct ndarray 4 x 4 view remained about 8.4 ns across those
parents. Lens creation itself remained near 2 ns across parent sizes.

The same pattern appeared for fixed 4 x 4 mutation and
`Lens::to_row_major()`. This isolated the dominant problem to traversal rather
than Region validation or Lens construction.

These shared-runner numbers justified optimization; they were not used as the
final performance claim.

## Predeclared budget

Before the traversal source was changed, R6 fixed this budget:

```text
complexity:
  fixed-size selection must not materially scale with unrelated parent cells

dense Lens/LensMut:
  <= 3.00x equivalent direct ndarray traversal for medium/large selections

Gear:
  <= 1.25x Lens/LensMut for selections >= 65,536 elements

tiny selections:
  report absolute cost as well as ratio; ratio alone is not pass/fail
```

The budget was not changed after candidate measurements were available.

## Targeted optimization

The measured repair is:

```text
commit db1c498edac854b59065cdcf1bfa5595334292aa
subject perf: remove measured Lens traversal waste
```

`Matrix<T>` gained crate-private checked helpers that create immutable/mutable
ndarray views of an already validated Region.

`Lens<'a, T>` now stores a private `ArrayView2<'a, T>`.
`LensMut<'a, T>` now stores a private `ArrayViewMut2<'a, T>`.

Consequences:

- `iter()` traverses only the selected ndarray view;
- `iter_mut()` mutates only the selected ndarray view;
- `get()` / `get_mut()` use Lens-local coordinates against that view;
- the parent-wide enumerate/filter-map path is removed;
- `to_row_major()` collects only selected values;
- `Region` is still stored/reported in parent coordinates;
- public Matrix/Lens/LensMut/Gear signatures are unchanged;
- ndarray remains private;
- no unsafe code is introduced;
- no new transformation authority is introduced.

This is deliberately a representation/traversal repair, not a new API.

## Semantic regression candidate

Regression coverage was committed separately:

```text
commit 9a9f4199d28da4294bdf0973cb7579e4add5d78f
tree   8220cc67a9e2a56df30ee71fdcf53854b0c4e43c
parent db1c498edac854b59065cdcf1bfa5595334292aa
subject test: preserve optimized selection semantics
```

`tests/r6_performance_contract.rs` covers:

- full Region row-major traversal;
- interior Region row-major traversal;
- single-row ordering;
- single-column ordering;
- empty row selections;
- empty column selections;
- zero-width Matrix behavior;
- zero-height Matrix behavior;
- `0 x 0` Matrix behavior;
- selected-only mutable traversal;
- row-major mutable replacement order;
- Lens-local access after mutation;
- rejection of a Region created for a foreign Shape.

The optimization does not weaken R3/R4 lifetime or capability semantics.

## Candidate code qualification

The execution environment available to the developer session did not provide a
local Rust toolchain. A temporary branch-only workflow was therefore used to
qualify the exact code-bearing candidate on Rust 1.85.0 and stable before owner
performance measurement.

Temporary validation identity:

```text
workflow R6 Candidate Validation
run      33255678427
run no.  1
head     d6d06317ac9cb2262b09a4c794bf18a4f81186d9
result   SUCCESS
```

That temporary head added only the validation workflow on top of
`9a9f4199d28da4294bdf0973cb7579e4add5d78f`. Both toolchain jobs passed:

```text
cargo check --locked --all-targets      PASS
cargo test --locked --all-targets       PASS
cargo test --locked --doc               PASS
cargo clippy --locked --all-targets     PASS
cargo doc --locked --no-deps            PASS
cargo bench --locked --no-run           PASS
```

The benchmark job also completed successfully.

After its evidence was captured, the R6 branch was force-restored to the clean
candidate:

```text
rehab/r6-measure-optimize
-> 9a9f4199d28da4294bdf0973cb7579e4add5d78f
```

The temporary workflow/commit is not part of the final R6 PR history.

## Authoritative owner-machine before/after measurement

Because shared GitHub runners are unsuitable for stable wall-clock performance
claims, R6 used one consolidated owner-machine run for the authoritative
comparison.

Captured:

```text
UTC 2026-08-30T20:16:30Z
```

Host:

```text
hostname           orion
kernel             Linux 6.1.0-49-amd64
CPU                AMD Ryzen 7 3800XT 8-Core Processor
physical cores     8
logical CPUs       16
memory             31 GiB
virtualization     none
CPU governor       schedutil
rustc +stable      1.98.0 (88d9e12ae 2026-08-18)
cargo +stable      1.98.0 (797e8a9bc 2026-08-05)
```

Measurement identities:

```text
baseline
  commit 91d1724a70c2af7ff5bd077dd8625b73302e0939
  tree   988e5a5638dd6267d765c150df7a1f2a400941bc

candidate
  commit 9a9f4199d28da4294bdf0973cb7579e4add5d78f
  tree   8220cc67a9e2a56df30ee71fdcf53854b0c4e43c

baseline Cargo.lock SHA-256
  b835d1e7d4d851e883a209e2cc41b99aeb8982f70a73821f569e7f0ef98ae62a

candidate Cargo.lock SHA-256
  b835d1e7d4d851e883a209e2cc41b99aeb8982f70a73821f569e7f0ef98ae62a
```

The owner script mechanically verified that `Cargo.toml`, `Cargo.lock`, and
`benches/` were identical between baseline and candidate before running the
comparison.

Raw Criterion output and parsed medians were preserved outside the repository.
No `target/`, Criterion report, profiler output, or generated benchmark artifact
is added to the PR.

### Representative before/after medians

```text
32x24/small_4x4/lens_sum
  baseline   2.659 us
  candidate  7.341 ns
  speedup    362.133x

1024x64/small_4x4/lens_sum
  baseline   262.380 us
  candidate  7.329 ns
  speedup    35,799.757x

100000x64/small_4x4/lens_sum
  baseline   30.694 ms
  candidate  7.242 ns
  speedup    4,238,449.004x

100000x64/small_4x4/gear_sum
  baseline   31.111 ms
  candidate  9.004 ns
  speedup    3,455,318.865x

100000x64/full/direct_ndarray_sum
  baseline   4.488 ms
  candidate  4.420 ms

100000x64/full/lens_sum
  baseline   30.066 ms
  candidate  4.374 ms
  speedup    6.874x

100000x64/full/gear_sum
  baseline   29.772 ms
  candidate  4.501 ms
  speedup    6.614x

100000x64/single_column/lens_sum
  baseline   30.842 ms
  candidate  491.520 us
  speedup    62.748x

100000x64/small_4x4/direct_ndarray_add_scalar
  baseline   20.939 ns
  candidate  20.450 ns

100000x64/small_4x4/lens_mut_add_scalar
  baseline   30.269 ms
  candidate  17.367 ns
  speedup    1,742,903.207x

100000x64/small_4x4/gear_add_scalar
  baseline   29.124 ms
  candidate  35.778 ns
  speedup    814,019.789x

100000x64/full/direct_ndarray_add_scalar
  baseline   6.543 ms
  candidate  6.387 ms

100000x64/full/lens_mut_add_scalar
  baseline   30.318 ms
  candidate  6.384 ms
  speedup    4.749x

100000x64/full/gear_add_scalar
  baseline   30.028 ms
  candidate  6.782 ms
  speedup    4.427x

copy/100000x64/small_4x4/lens_to_row_major
  baseline   30.047 ms
  candidate  35.753 ns
  speedup    840,405.001x

copy/100000x64/full/lens_to_row_major
  baseline   42.526 ms
  candidate  11.036 ms
  speedup    3.853x
```

### Complexity gate

Candidate fixed 4 x 4 Lens read:

```text
32x24 parent       7.341 ns
1024x64 parent     7.329 ns
100000x64 parent   7.242 ns
largest/smallest   0.986x
```

The fixed selection no longer scales with unrelated parent cells.

### Dense overhead budget

```text
PASS 100000x64 full Lens read / direct ndarray
     0.990x <= 3.00x

PASS 100000x64 interior Lens read / direct ndarray
     0.921x <= 3.00x

PASS 100000x64 full LensMut transform / direct ndarray
     1.000x <= 3.00x

PASS 100000x64 interior LensMut transform / direct ndarray
     0.892x <= 3.00x

PASS 1024x64 full Gear read / Lens
     0.997x <= 1.25x

PASS 100000x64 full Gear read / Lens
     1.029x <= 1.25x

PASS 1024x64 full Gear mutation / LensMut
     1.196x <= 1.25x

PASS 100000x64 full Gear mutation / LensMut
     1.062x <= 1.25x
```

All predeclared performance gates pass.

## Lens construction tradeoff

The private view representation makes Lens construction slightly more expensive.

Fixed 4 x 4 Lens construction medians:

```text
baseline:
  32x24       1.819 ns
  1024x64     1.810 ns
  100000x64   1.782 ns

candidate:
  32x24       34.668 ns
  1024x64     34.838 ns
  100000x64   35.664 ns
```

Fixed 4 x 4 LensMut construction medians:

```text
baseline:
  32x24       1.629 ns
  1024x64     1.642 ns
  100000x64   1.550 ns

candidate:
  32x24       35.684 ns
  1024x64     36.198 ns
  100000x64   36.202 ns
```

R6 accepts this roughly 35–36 ns constant construction cost. It does not scale
with parent size and buys the removal of microseconds-to-milliseconds of
parent-wide traversal waste from every subsequent selected iteration/copy.

## Allocation and copy accounting

R6 source accounting identifies:

```text
Lens creation
  no selected-value copy
  no required heap allocation
  creates ndarray view metadata

LensMut creation
  no selected-value copy
  no required heap allocation
  creates mutable ndarray view metadata

Lens/LensMut iteration
  no selected-value copy
  no required heap allocation

SumGear body
  no Gear-owned selected-value allocation

AddScalarGear body
  in-place selected mutation
  no selected-value allocation

execute_read/execute_mut benchmark Tags
  Vec::new()
  no tag payload allocation

Tag::source
  accepts Into<String>
  borrowed non-empty &str normally requires owned String storage
  an existing String may transfer/reuse its allocation

to_row_major()
  allocates output Vec
  clones selected T values
```

R6 did not add a custom allocator or heap profiler. It therefore does not claim
a universal exact allocator-call count for `to_row_major()` or string creation.
The table records required semantic copy/allocation behavior rather than
allocator implementation details.

## Profiling disposition

R6 attempted to preserve a profiler signal without weakening host policy.

Preliminary GitHub runner:

```text
perf_event_paranoid=4
hardware perf denied
```

Owner machine:

```text
perf_event_paranoid=3
perf executable not installed
```

R6 did not use sudo, change kernel perf policy, or install system packages solely
for this slice.

The lack of a sampled profile is recorded as a limitation, not hidden. The
hotspot is nevertheless well supported by:

1. fixed-selection time growing with total parent size before the repair;
2. Lens creation remaining constant before the repair;
3. direct ndarray selected-view time remaining constant;
4. source inspection identifying full-parent enumerate/filter-map traversal;
5. candidate fixed-selection timing becoming constant immediately after that
   traversal is replaced with a checked selected view.

## Rayon decision

Rayon is not added in R6.

The evidence after the sequential repair is:

```text
dense Lens read / direct ndarray          ~0.99x
dense LensMut transform / direct ndarray  ~1.00x
large Gear read / Lens                    ~1.03x
large Gear mutation / LensMut             ~1.06x
```

That does not establish a remaining traversal bottleneck that warrants:

- a new runtime dependency;
- global/local thread-pool policy;
- scheduling overhead;
- workload-size crossover thresholds;
- deterministic/concurrency qualification;
- additional mutable-authority analysis.

Rayon remains an attractive future option for a workload with substantial
independent computation per selected element. R6 specifically finds no reason to
make it part of the default dense traversal path now.

## Authority and unsafe audit

R6 preserves:

```text
caller chooses Region
-> Matrix validates Region
-> caller obtains Lens / LensMut
-> Gear receives only that exact bounded capability
```

The private ndarray view cannot be obtained by downstream callers or by a Gear
through a new public API.

No R6 Rust source introduces:

```text
unsafe block
unsafe fn
unchecked public indexing
public ndarray view/storage exposure
Gear access to whole Matrix
Gear Region-selection authority
hidden parallel execution
```

## Benchmark/CI policy

Criterion timing is deliberately not a normal CI threshold.

Shared hosted runners are appropriate for:

```text
compile benchmark harness
detect API breakage
run correctness tests
run Clippy/docs
```

They are not authoritative for:

```text
wall-clock regression percentages
absolute latency guarantees
parallel speedup claims
```

The repository therefore keeps the benchmark source but not generated Criterion
reports or machine-specific timing assertions.

## Mechanical scope

Expected R6 paths are limited to:

```text
Cargo.toml
Cargo.lock
benches/r6_selection.rs
benches/r6_transform.rs
src/schematics/matrix.rs
src/strategies/lens.rs
tests/r6_performance_contract.rs
docs/performance.md
docs/README.md
docs/active-development.md
docs/roadmap.md
docs/architecture/vision.md
docs/development/2026-08-29-r6-measure-optimize.md
```

No temporary R6 workflow is part of the final candidate. No generated
`target/`, Criterion result directory, profiler file, or raw evidence archive is
committed.

## Residual limitations

- owner-machine timing is one stable host/toolchain measurement, not a universal
  performance guarantee;
- no sampled `perf` profile was available;
- R6 does not benchmark BLAS/linear-algebra kernels because Matrical does not
  claim to replace them;
- R6 does not introduce parallelism;
- R6 does not solve historical Crossbeam/prototype dependency residue;
- R6 does not begin optional backend abstraction or persistence.

## Exit result

Subject to final PR exact-head CI and Teamlead/owner acceptance:

```text
R6: COMPLETE — TEAMLEAD/OWNER ACCEPTANCE PENDING
R6 CODE/SEMANTICS: QUALIFIED
R6 PERFORMANCE BUDGET: PASS
R6 PARALLELISM DECISION: NO RAYON
R6 PR: READY FOR REVIEW AFTER FINAL CI
R6 MERGE: NOT AUTHORIZED
R7: BLOCKED UNTIL R6 IS ACCEPTED AND MERGED
```

The final Teamlead handoff records the documentation commit/head tree, final
changed paths, PR number, and exact-head ordinary two-lane CI result because this
report cannot self-record the SHA of the commit that contains it.
