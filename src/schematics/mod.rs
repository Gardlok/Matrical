//! Checked geometry and owned dense matrix storage.
//!
//! Downstream users normally need [`Shape`], [`Index`], [`Region`], and
//! [`Matrix`]. Historical prototype data, Element, and MatrixContext structures
//! remain crate-internal or documentation-hidden during rehabilitation.

// These prototype modules were historically public. R5 intentionally removes
// them from downstream discovery; suppress dead-code diagnostics introduced
// solely by that visibility narrowing rather than rewriting their internals.
#[allow(dead_code)]
pub(crate) mod data;
#[allow(dead_code)]
pub(crate) mod element;
pub(crate) mod matrix;

pub use matrix::{Index, Matrix, Region, Shape};

#[doc(hidden)]
pub use element::ElementContext;
#[doc(hidden)]
pub use matrix::MatrixContext;
