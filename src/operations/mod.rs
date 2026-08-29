//! Historical operation scaffolding retained only for 0.1.0 source compatibility.
//!
//! New downstream code should use the Lens/Gear API exported from the crate root
//! or [`crate::prelude`].

pub mod aggregate;
pub mod arithmetic;
pub mod bitwise;
pub mod boolean;
pub mod filter;
pub mod mechanics;
pub mod sort;

pub use aggregate::*;
pub use arithmetic::*;
pub use bitwise::*;
pub use boolean::*;
pub use filter::*;
pub use mechanics::*;
pub use sort::*;

use crate::error::MatricalError;
use crate::schematics::{ElementContext, MatrixContext};

pub trait MatrixOperation {
    fn execute<T>(&self, context: &mut MatrixContext) -> Result<(), MatricalError>;
}

pub trait ElementOperation<V>
where
    V: Clone + Send + Sync + 'static + Default + PartialEq + Eq,
{
    fn execute(&self, context: &ElementContext<V>) -> Result<(), MatricalError>;
}
