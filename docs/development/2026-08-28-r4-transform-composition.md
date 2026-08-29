# R4 transformation composition

Date: 2026-08-28

## Starting identity

```text
repository Gardlok/Matrical
branch     main
commit     9fbc712084a78570e8ac2b980ff0d4474c90ee7f
tree       4db71daeb50553edc6cdc69a2986f93087be4f35
version    0.1.0
```

This is the verified merge of owner-accepted PR #8. The accepted `Cargo.lock`
SHA-256 is `8abe052c6d793e87df19c1e6ade379caf3cad562eea693a946dc39c9e7180020`.

## Historical prototype disposition

R4 replaces, rather than preserves, the unfinished Gear/Cog/Tag prototype.
Historical Gear-owned `ndarray::Array2<f64>`, coordinate duplication, direct
ndarray slicing, factory/strategy ceremony, Cog callback/trait-object graphs,
Cog-owned ndarray data, private demonstration `main()`, bare string Tag, and
`ParameterizedQuery`/DI residue are not compatibility requirements for 0.1.0.

The reconstructed transformation layer owns no independent matrix substrate.
The selected Lens is the capability boundary.

## Gear contracts

R4 defines two ordinary static Rust traits:

```rust
pub trait ReadGear<T> {
    type Context: ValidateCog;
    type Output;

    fn name(&self) -> &'static str;
    fn apply(
        &self,
        lens: &Lens<'_, T>,
        context: &Self::Context,
    ) -> Result<Self::Output, MatricalError>;
}

pub trait MutGear<T> {
    type Context: ValidateCog;
    type Output;

    fn name(&self) -> &'static str;
    fn apply(
        &self,
        lens: &mut LensMut<'_, T>,
        context: &Self::Context,
    ) -> Result<Self::Output, MatricalError>;
}
```

A read-only Gear receives immutable Lens authority only. A mutating Gear receives
exclusive `LensMut` authority only. Neither contract gives a Gear `&Matrix<T>`,
`&mut Matrix<T>`, ndarray storage, or region-selection authority.

A compile-fail rustdoc example proves that a `ReadGear` implementation cannot
call `get_mut` through its supplied `Lens`.

Downstream crates implement these traits directly. Static dispatch is the
default; there is no registry, factory map, string dispatch, `Any` lookup, DI
container, or mandatory boxed trait object.

## Built-in Gears

R4 provides deterministic `f64` examples:

- `SumGear`: read-only aggregation; empty Lens result is `0.0`;
- `AddScalarGear`: mutating scalar addition using `ScalarPolicy` Cog context;
- `ScaleGear`: mutating scalar multiplication using `ScalarPolicy` Cog context;
- `ClampGear`: mutating inclusive clamping using validated `ClampPolicy` context.

Mutating Gears report the number of affected selected elements. Empty mutable
Lenses succeed and report zero affected elements.

## Cog representation and validation

`Cog<C>` is a typed optional context container. `Cog::new(C)` carries context of
known Rust type `C`; `Cog::<C>::empty()` represents absence. `Cog::context()`
returns `MatricalError::InvalidContext` rather than panicking when required
context is absent.

`ValidateCog` is intentionally small:

```rust
pub trait ValidateCog {
    fn validate(&self) -> Result<(), MatricalError>;
}
```

The central execution path resolves the context, validates it, and only then
invokes the Gear. `ScalarPolicy` rejects non-finite values as `InvalidValue`.
`ClampPolicy` rejects non-finite bounds and reversed `minimum > maximum` ranges as
`InvalidValue`. No downcast or string-identified context is used.

## Central execution path

`execute_read` and `execute_mut` are the two generic execution boundaries. Each:

1. resolves typed Cog context;
2. validates the context;
3. captures the Lens Region;
4. invokes the statically selected Gear;
5. returns an `ExecutionReport<O>` on success.

Failures return `Err(MatricalError)` directly. No success report is fabricated
around failed execution.

## Tag and provenance

R4 replaces the bare historical `Tag { name: String }` and removes
`ParameterizedQuery`. The public Tag namespace is bounded:

```text
Tag::Source(String)
Tag::Stage(TagStage)
Tag::Sequence(u64)
```

`TagStage` is a finite enum (`Input`, `Transform`, `Output`, `Review`). Source
text has one explicit semantic role: an inert provenance label. Matrical never
interprets Tag text as code, SQL, a query, a Gear selector, or a command.

Tags are not passed into Gear execution. They are attached to the successful
report only after the Gear returns. This structural separation prevents Tag from
becoming a hidden transformation command channel.

## Execution report

`ExecutionReport<O>` preserves a strongly typed output and records:

- static Gear identity;
- exact selected `Region`;
- typed `GearEffect::{ReadOnly, Mutating}`;
- output `O` without `Any` erasure;
- provenance Tags in caller-supplied deterministic order.

A caller can therefore inspect which Gear ran, where it ran, its effect class,
its typed result, and its provenance.

## Lens authority evidence

The transformation API consumes already-selected `Lens`/`LensMut` values. A Gear
cannot choose or enlarge its Region through the trait contract. Integration and
unit regressions apply mutation to an interior Region and then compare the
entire parent Matrix, proving values outside the selected Region are unchanged.

Read-only authority is also compile-time separated: `ReadGear` receives no
mutable Lens API. No runtime effect flag controls authority.

## Empty selections and determinism

`SumGear` returns its natural empty sum `0.0`. Built-in mutating Gears succeed on
empty `LensMut` values and report zero affected elements. No built-in Gear
fabricates data or panics for an empty selection.

All R4 built-ins are sequential and deterministic. They use Lens's documented
logical row-major iteration, contain no random behavior, and use no global
mutable state.

## Static versus dynamic dispatch

R4 uses static dispatch. The external integration test defines its own
`CountAboveGear` and typed `ThresholdPolicy`, then executes them through public
exports only. This is sufficient evidence for first-class downstream extension.

R4 has no demonstrated requirement for heterogeneous runtime Gear collections,
so dynamic dispatch, runtime registries, factory maps, and string lookup remain
deferred. R5/R7 may revisit this if a concrete consumer requires it.

## GAT reassessment

### Design A — Gear consumes the capability it is given

The caller selects a `Region`, obtains `Lens` or `LensMut`, and passes that
already-bounded capability into Gear execution. This directly encodes least
authority: the Gear cannot enlarge the caller's selection.

### Design B — Gear receives a lending provider

A public GAT provider could expose lifetime-indexed `View<'a>` and
`ViewMut<'a>` associated types and let Gear request views from a generic provider.
That preserves static dispatch and may help if multiple future providers share a
real lending contract.

### Authority analysis

For R4, giving a Gear the provider would also risk giving it selection authority:
a Gear that can request arbitrary Regions from a Matrix-like provider has more
authority than a Gear handed one caller-selected Lens. Avoiding that escalation
would require additional provider restrictions whose only current purpose would
be recreating the Lens boundary indirectly.

R4 therefore provides stronger evidence than R3's single-provider rationale:
transformation composition actively prefers the narrower already-selected Lens
capability. A public GAT lending-provider trait is deferred. It does not improve
current downstream Gear reuse, static dispatch, ergonomics, Rust 1.85 diagnostics,
or authority safety enough to justify its extra public surface.

No fake second provider is introduced to manufacture a GAT use case.

## HRTB evaluation

R4 found no execution adapter or callback that genuinely needs to accept a Lens
borrow of every possible lifetime. Gear methods naturally accept `Lens<'_, T>` or
`LensMut<'_, T>` directly. Introducing `for<'a> Fn(&Lens<'a, T>)` would add
indirection without a current requirement, so no public HRTB abstraction is
added.

## Unsafe audit

R4 transformation code requires no unsafe block, unsafe function, raw pointer, or
unsafe trait implementation. Project-authored unsafe introduced by R4 is zero.

## Tests and example

R4 coverage includes:

- full, partial, and empty read-only aggregation;
- deterministic repeated read result;
- selected-only mutation with whole-Matrix comparison;
- empty mutable execution reporting zero affected elements;
- multiple distinct mutating transformations;
- typed context present, missing, invalid, and successfully delivered;
- exact report Gear identity, Region, effect, output, and Tag order;
- inert Tag metadata that cannot influence Gear execution;
- an external-crate downstream-defined Gear and policy;
- compile-fail read/mutate authority evidence;
- runnable `examples/r4_transform.rs` composition workflow.

The first published PR candidate ran 61 library unit tests, 9 integration tests
(1 R2, 3 R3, 5 R4), and 5 doctests successfully in each CI lane. The R4 example
compiled under `--all-targets`. The new read-Gear compile-fail doctest passed on
both Rust 1.85.0 and stable.

## Qualification

PR #9 (`R4: establish typed transformation composition`) is the review unit. Its
first complete candidate head was
`f83e6481d34ef10970773d8a60a3aa1ea22adac0`. GitHub Qualification run #13
checked the clean PR merge candidate and completed both lanes successfully:

```text
Rust 1.85.0
  rustc 1.85.0 (4d91de4e4 2025-02-17)
  cargo check --locked --all-targets  PASS
  cargo test --locked --all-targets   PASS
  cargo test --locked --doc           PASS (5/5)
  cargo clippy --locked --all-targets PASS
  cargo doc --locked --no-deps        PASS

stable
  rustc 1.98.0 (88d9e12ae 2026-08-18)
  cargo check --locked --all-targets  PASS
  cargo test --locked --all-targets   PASS
  cargo test --locked --doc           PASS (5/5)
  cargo clippy --locked --all-targets PASS
  cargo doc --locked --no-deps        PASS
```

The existing workflow invokes the Cargo binary belonging to each selected
rustup toolchain but does not print `cargo --version`; only the exact observed
rustc versions above are claimed here.

Run #13 surfaced one avoidable R4-only warning: an unused `MutGear` import in the
external integration test. Commit `92d9601e2846941e0283965e02a7c7205d6386ba`
removed only that import. The inherited warning set remains outside R4. The
final PR head is requalified by the same two-lane workflow after this evidence
record is committed; that final external check is the exact-head merge gate and
is reported in the Teamlead handoff rather than creating an endless
self-referential documentation-SHA cycle.

The dependency graph, MSRV, edition, version, workflow, and lockfile are unchanged
by R4. `Cargo.lock` therefore remains byte-identical to the accepted baseline,
with SHA-256
`8abe052c6d793e87df19c1e6ade379caf3cad562eea693a946dc39c9e7180020`.

## Residual historical debt

R4 does not reconstruct Vector, unrelated operation modules, broader historical
warning/formatting residue, detached `MatrixContext`, or dependency cleanup.
Those remain outside the transformation contract.

## R4 exit result

The implementation establishes the complete intended R4 architecture. Once the
final exact PR head has both Qualification lanes green, the candidate state is:

```text
R4: COMPLETE — TEAMLEAD/OWNER ACCEPTANCE PENDING
R4 PR: READY — NOT DRAFT
R4 MERGE: NOT AUTHORIZED
```

Recommended next phase after merge: R5 — API ergonomics and learning surface.
