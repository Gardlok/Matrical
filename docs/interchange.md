# Matrical interchange boundary

## Purpose

`MatrixSnapshot<T>` is Matrical's versioned, inert representation for moving a
dense logical Matrix across a process, repository, storage, or integration
boundary without exposing the private `ndarray` backend.

```text
live Matrix<T>
      |
      | explicit snapshot
      v
MatrixSnapshot<T>
      |
      +-- optional Serde serialization
      +-- caller-owned file/database/network transport
      +-- checked reconstruction
              |
              v
          Matrix<T>
```

A snapshot is data, not a backend. It owns no file, database, socket, mapped
region, background worker, or other live storage/execution authority.

## Dense snapshot v1

The current logical schema is version 1:

```json
{
  "version": 1,
  "rows": 2,
  "columns": 3,
  "row_major": [1, 2, 3, 4, 5, 6]
}
```

The v1 field names and meanings are:

- `version` — schema version; currently exactly `1`.
- `rows` — row count encoded as `u64`.
- `columns` — column count encoded as `u64`.
- `row_major` — owned values in deterministic logical row-major order.

The public constant `DENSE_SNAPSHOT_VERSION` names the version Matrical emits
and accepts. Snapshot fields remain private; callers inspect them through
read-only accessors and cannot independently change version or shape metadata by
ordinary setters.

## Checked reconstruction

`MatrixSnapshot::into_matrix()` fails closed unless all reconstruction invariants
hold:

```text
version == DENSE_SNAPSHOT_VERSION
rows and columns fit the receiving platform's usize
rows * columns is representable by usize
row_major.len() == rows * columns
```

Reconstruction deliberately reuses the existing core invariant path:

```text
snapshot version check
-> checked u64 -> usize dimension conversion
-> Shape::new(...)
-> Matrix::from_row_major(...)
```

Matrical does not maintain a second independent Matrix-invariant implementation.
Unsupported versions return `MatricalError::UnsupportedSnapshotVersion`.
Dimensions that cannot fit the receiving platform return
`MatricalError::SnapshotDimensionOutOfRange`. Element-count overflow and
row-major length mismatch continue to use the existing structural errors.
Nothing is truncated.

A newer schema version is not interpreted as v1 merely because some fields look
familiar. Forward migration belongs to a future explicit version-aware layer.

## Ownership and copy behavior

Snapshot creation is explicit:

```text
matrix.snapshot()
    borrows Matrix
    O(n) clone of values
    requires T: Clone

matrix.into_snapshot()
    consumes Matrix
    transfers owned values
    does not require T: Clone

snapshot.into_matrix()
    consumes snapshot
    validates and reconstructs owned dense Matrix
```

The borrowed path is not zero-copy. The consuming path avoids cloning `T`, but
reconstruction still creates the owned dense Matrix representation required by
the current backend.

Zero-dimensional shapes remain valid: `0 x 0`, `0 x N`, and `N x 0` snapshots
carry zero row-major values and reconstruct successfully.

## Optional Serde feature

Serialization is opt-in:

```toml
matrical = { version = "0.1.0", features = ["serde"] }
```

The `serde` feature derives `Serialize` and `Deserialize` for
`MatrixSnapshot<T>` using ordinary generic Serde bounds. It does not make
`Matrix<T>` serializable and does not expose ndarray serialization.

The default Matrical runtime graph does not activate the optional Serde
dependency. `serde_json` is development/example-only and is not a Matrical
runtime dependency or storage engine.

For v1, Serde deserialization denies unknown fields. This prevents a v1 reader
from silently dropping unrecognized semantic data.

## Format neutrality and JSON limits

`MatrixSnapshot<T>` is format-neutral. Serde compatibility means a snapshot can
participate in a chosen Serde format when `T` supports that format.

The selected format owns its own representational limits. JSON, for example,
does not establish faithful representation for every possible `T` or every
possible floating-point bit pattern. The committed canonical fixture therefore
uses integers and is a schema/interoperability proof, not a claim about arbitrary
element-domain fidelity.

Matrical intentionally does not provide `save_json`, `load_json`, `write_file`,
`read_file`, or similar APIs. Callers own byte transport and storage policy.

## Resource-bounded input handling

Generic Serde deserialization is not itself a transport-level resource limiter.
Callers accepting untrusted documents must bound document size, nesting,
allocation, and related resource use at the transport/format layer.

R7-A's fail-closed guarantee is about semantic Matrix reconstruction after a
snapshot exists. Checking `row_major.len()` after deserialization does not by
itself guarantee bounded memory consumption while parsing input.

## Dense today; sparse and mapped later

Snapshot v1 is specifically a dense logical representation. Future sparse or
mapped representations are separate schema/storage decisions. R7-A does not add
`MatrixBackend`, `StorageBackend`, a lending/GAT/HRTB provider, a sparse Matrix,
or a mapped backend merely to make an inert DTO share a live-provider trait.

A real second live implementation must exist before Matrical generalizes the
live storage boundary.

## Future consumer and integration boundary

Matrical core remains independently useful and does not select or depend on any
particular external consumer. External callers or adapters may use Matrical's
public APIs and may transport or store `MatrixSnapshot` as inert interchange
data without becoming part of Matrical core.

If a concrete consumer later justifies an adapter, that adapter should live in
the consumer, a dedicated integration crate, or another explicit opt-in
boundary. No particular project is designated as the intended or preferred
snapshot consumer.

Gear execution authority remains bounded by caller-selected `Lens`/`LensMut`.
Transporting a snapshot does not create Gear authority or grant a caller a live
Matrix backend handle.
