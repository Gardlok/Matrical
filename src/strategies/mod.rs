



use crate::*;

pub trait MatrixStrategy {
    fn execute<V>(
        &self,
        matrix: &Matrix,
        index: Option<(usize, usize)>,
        other: Option<bool>,
    ) -> Result<(), MatricalError>;
}

pub trait ElementStrategy<V> {
    fn execute(
        &self,
        element: &Element<V>,
        index: Option<(usize, usize)>,
        other: Option<bool>,
    ) -> Result<(), MatricalError>;
}




