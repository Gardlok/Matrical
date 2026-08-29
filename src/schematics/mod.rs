//! Checked geometry and owned dense matrix storage.
//!
//! Downstream users normally need [`Shape`], [`Index`], [`Region`], and
//! [`Matrix`]. Historical prototype data, Element, Vector, and MatrixContext
//! structures remain crate-internal during rehabilitation.

pub(crate) mod data;
pub(crate) mod element;
pub(crate) mod matrix;
pub(crate) mod vector;

pub use matrix::{Index, Matrix, Region, Shape};

#[doc(hidden)]
pub use element::ElementContext;
#[doc(hidden)]
pub use matrix::MatrixContext;
