//! Matrical is a semantic matrix-transformation library built around validated
//! geometry, borrowing selections, typed transformations, contextual policy,
//! and inert provenance.
//!
//! The normal flow is deliberately small:
//!
//! ```text
//! Matrix -> Lens / LensMut -> Gear (+ Cog) -> ExecutionReport (+ Tags)
//! ```
//!
//! A [`Matrix`] owns dense two-dimensional data and a checked [`Shape`]. A
//! [`Region`] selects a half-open rectangle. [`Lens`] and [`LensMut`] borrow that
//! selection without exposing the underlying `ndarray` storage. Read-only and
//! mutating Gears receive only the Lens authority chosen by the caller; they do
//! not receive a Matrix or the ability to select a broader Region.
//!
//! [`Cog`] carries a concrete context or policy type. [`ValidateCog`] validates
//! that context before a Gear runs. [`Tag`] values are inert provenance attached
//! to the successful [`ExecutionReport`]; Tags are never passed into the Gear and
//! cannot steer execution.
//!
//! Fallible public operations return [`MatricalError`]. Construction, indexing,
//! Region validation, context resolution, and policy validation are therefore
//! ordinary `Result`-based caller boundaries rather than panic-based control
//! flow.
//!
//! For everyday use, import [`prelude`]. The [`schematics`] and [`strategies`]
//! modules provide the same supported API grouped by concept. Historical
//! prototype scaffolding is not part of the learning contract.
//!
//! # End-to-end example
//!
//! ```
//! use matrical::prelude::*;
//!
//! fn main() -> Result<(), MatricalError> {
//!     let shape = Shape::new(2, 3)?;
//!     let mut matrix = Matrix::from_row_major(
//!         shape,
//!         vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
//!     )?;
//!     let region = Region::new(shape, 0..2, 1..3)?;
//!
//!     {
//!         let lens = matrix.lens(region)?;
//!         let report = execute_read(
//!             &SumGear,
//!             &lens,
//!             &Cog::new(()),
//!             vec![Tag::source("crate-docs")],
//!         )?;
//!         assert_eq!(*report.output(), 16.0);
//!         assert_eq!(report.effect(), GearEffect::ReadOnly);
//!     }
//!
//!     {
//!         let mut lens = matrix.lens_mut(region)?;
//!         let report = execute_mut(
//!             &AddScalarGear,
//!             &mut lens,
//!             &Cog::new(ScalarPolicy::new(10.0)),
//!             vec![Tag::stage(TagStage::Transform)],
//!         )?;
//!         assert_eq!(*report.output(), 4);
//!         assert_eq!(report.effect(), GearEffect::Mutating);
//!     }
//!
//!     assert_eq!(
//!         matrix.into_row_major(),
//!         vec![1.0, 12.0, 13.0, 4.0, 15.0, 16.0]
//!     );
//!     Ok(())
//! }
//! ```
//!
//! Matrical remains version `0.1.0`; see the repository's API stability policy
//! before treating these APIs as a SemVer stability guarantee.

#[cfg(test)]
mod tests;

mod error;

/// Historical prototype operations retained for compatibility during the 0.1.0
/// rehabilitation campaign.
///
/// This namespace is intentionally hidden from generated documentation and is
/// not part of the recommended downstream API.
#[doc(hidden)]
pub mod operations;

/// Validated matrix geometry and owned storage.
pub mod schematics;
/// Borrowing views, typed transformations, context, and provenance.
pub mod strategies;

pub use error::MatricalError;
#[doc(hidden)]
pub use error::MatricalErrorType;

pub use schematics::{Index, Matrix, Region, Shape};
pub use strategies::{
    execute_mut, execute_read, AddScalarGear, ClampGear, ClampPolicy, Cog, ExecutionReport,
    GearEffect, Lens, LensMut, MutGear, ReadGear, ScalarPolicy, ScaleGear, SumGear, Tag, TagStage,
    ValidateCog,
};

/// Recommended everyday imports for constructing, selecting, transforming, and
/// inspecting matrices.
///
/// The prelude is intentionally curated. It contains the high-frequency R2–R4
/// API and excludes historical operation scaffolding, prototype Vector/Element
/// types, `MatrixContext`, dependency types, and implementation details.
pub mod prelude {
    pub use crate::{
        execute_mut, execute_read, AddScalarGear, ClampGear, ClampPolicy, Cog, ExecutionReport,
        GearEffect, Index, Lens, LensMut, MatricalError, Matrix, MutGear, ReadGear, Region,
        ScalarPolicy, ScaleGear, Shape, SumGear, Tag, TagStage, ValidateCog,
    };
}
