

pub mod aggregate;
pub use aggregate::AggregateStrategy;

use crate::*;

pub trait MatrixStrategy {
    fn execute<V>(
        &self,
        matrix: &Matrix,
        index: Option<(usize, usize)>,
        other: Option<bool>,
    ) -> Result<(), MatricalError>;
}