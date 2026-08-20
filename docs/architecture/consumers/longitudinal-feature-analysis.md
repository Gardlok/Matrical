# Longitudinal feature-analysis consumer

**Status:** non-binding downstream design input

This note records requirements from a prospective analytical typing application
without making Matrical responsible for typing, health interpretation, or
application persistence. The exact Matrical APIs remain subject to their roadmap
slices, executable evidence, Teamlead review, and owner acceptance.

## Representative data model

The application can present a validated numeric dataset to Matrical using this
conceptual mapping:

| Concept | Representative meaning |
| --- | --- |
| Matrix rows | trials or fixed time windows |
| Matrix columns | versioned numerical features |
| Lens | historical-window or feature selection |
| Gear | deterministic transformation |
| Cog | baseline or transformation policy |
| Tag | schema and derivation provenance |

Typical features might include timing distributions, error rates, corrections,
rhythm measures, or derived changes. Their scientific meaning belongs to the
consumer, not to Matrical.

## Ownership boundary

The downstream application owns:

- `TrialId`, `FeatureId`, and other domain identifiers;
- feature names, units, and versioned schemas;
- capture capabilities and device/session provenance;
- the meaning of missing or unavailable measurements;
- analyzer versions and cognitive-health interpretation;
- storage, synchronization, retention, and access policy;
- database record identifiers and wall-clock event metadata.

Matrical owns only the contracts it implements, such as:

- validated numeric shape, index, and region behavior;
- bounded borrowed selection;
- deterministic transformations;
- typed transformation policy;
- bounded schema or derivation identifiers supplied by a consumer;
- reports describing the transformation Matrical actually performed.

Matrical must not become a dataframe, typing-event model, database abstraction,
or cognitive-health assessment library to serve this consumer.

## Transformation boundaries

Read-only and mutating transformations should have different visible effects.
R4 should evaluate distinct `Gear` and `GearMut` contracts after R3 establishes
working `Lens` and `LensMut` borrowing semantics. The illustrative signatures
from consumer discussion are design input, not an accepted API.

Application crates must be able to define Gears through ordinary Rust traits and
static dispatch without registering them in a global runtime registry. Candidate
downstream Gears include:

- robust scaling;
- baseline residuals;
- column standardization;
- session slopes;
- capability masking;
- rolling differences;
- change-point evidence.

These remain downstream operations unless repeated use demonstrates that a
transformation is broadly useful and belongs in Matrical.

## Missing measurements

Different capture modes expose different measurements. A press-only terminal
session, for example, cannot supply key-release or hold-duration features.
"Not captured" must not silently become numeric zero.

The preferred research direction is a dense numeric matrix paired with an
explicit validity mask. `NaN` alone is insufficient because it can conflate an
unavailable measurement with a legitimate floating-point result. R2 must decide
whether mask ownership belongs in Matrical core, a paired Matrical type, or the
downstream wrapper. Until that decision is accepted, downstream code must not
feed unsupported columns into transforms as though they were valid values.

## Deterministic provenance

R4 transformation reports should evaluate recording:

- operation identity and version;
- selected region;
- consumer-supplied feature-schema identifier;
- Cog or policy identifier;
- input and output shape;
- whether allocation occurred;
- success or typed failure.

Wall-clock duration, database identifiers, and other run-instance metadata
remain downstream concerns. Excluding them from Matrical's logical report allows
identical historical replays to produce equivalent transformation records.

The precise report schema and the distinction between Tag and execution-report
fields remain R4 design decisions.

## Performance workloads

R6 should benchmark shapes resembling longitudinal feature analysis rather than
only giant square matrices:

| Shape | Representative workload |
| --- | --- |
| `32 x 24` | recent personal window |
| `1,024 x 64` | mature personal history |
| `100,000 x 64` | future research cohort or detailed segmentation |

Candidate measurements include Matrix construction, rectangular Lens creation,
column and window selection, row iteration, robust column transforms,
allocations, and Matrical overhead relative to direct `ndarray` operations.

R6 should propose and justify an overhead budget before turning it into a release
gate. A semantic layer that adds material cost must demonstrate corresponding
correctness, traceability, or usability value.

## Advanced type-system boundary

Borrowed Lenses justify explicit lifetimes. They do not yet justify a universal
storage-backend abstraction.

The first implementation should borrow directly from concrete `ndarray`
storage. A lending backend trait using generic associated types should wait
until a second real backend exposes the shared requirement. Higher-ranked trait
bounds may be appropriate when a transformation genuinely needs to operate over
any Lens lifetime, but must still improve a demonstrated contract.

## Roadmap acceptance inputs

- **R2:** validate tall and narrow Matrix construction and decide missing-data
  ownership without importing the typing domain model.
- **R3:** support bounded rectangular, row, and column borrowing without
  undocumented allocation.
- **R4:** make downstream-defined transformations first-class, keep read and
  mutation effects visible, and produce deterministic bounded provenance.
- **R5:** prove the extension model in a downstream example that defines its own
  Gear and domain wrapper.
- **R6:** benchmark representative shapes, allocation behavior, and overhead
  against direct `ndarray` use.

This consumer may influence acceptance criteria, but it does not bypass the
ordered invariant work in R1 through R3.
