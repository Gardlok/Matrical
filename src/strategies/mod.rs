//! Borrowing views and typed transformation composition.
//!
//! [`Lens`] / [`LensMut`] express caller-selected authority. [`ReadGear`] and
//! [`MutGear`] consume those capabilities, [`Cog`] carries typed context, and
//! [`ExecutionReport`] plus [`Tag`] describe successful execution.

pub(crate) mod cog;
pub(crate) mod gear;
pub(crate) mod lens;
pub(crate) mod tag;

pub use cog::{ClampPolicy, Cog, ScalarPolicy, ValidateCog};
pub use gear::{
    execute_mut, execute_read, AddScalarGear, ClampGear, ExecutionReport, GearEffect, MutGear,
    ReadGear, ScaleGear, SumGear,
};
pub use lens::{Lens, LensMut};
pub use tag::{Tag, TagStage};

use crate::schematics::element::Element;
use crate::{MatricalError, Matrix};

pub(crate) trait MatrixStrategy
where
    Self: Send + Sync + Clone + Eq + 'static,
{
    fn execute<V: Clone + Send + Sync + Eq + Default + 'static>(
        &self,
        matrix: &Matrix<V>,
        index: Option<(usize, usize)>,
        other: Option<bool>,
    ) -> Result<(), MatricalError>;
}

pub(crate) trait ElementStrategy<V>
where
    V: Clone + Send + Sync + 'static + Default + PartialEq + Eq,
{
    fn execute(
        &self,
        element: &Element<V>,
        index: Option<(usize, usize)>,
        other: Option<bool>,
    ) -> Result<(), MatricalError>;
}
