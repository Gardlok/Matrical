pub mod gears;
pub mod cog;

pub use gears::*;
pub use cog::*;

use crate::*;

pub trait MatrixOperation {
    fn execute(&self, context: &MatrixContext) -> Result<(), MatricalError>;
}

pub fn execute_functor<T, F>(element: &mut Element<T>, functor: F)
where
    F: Fn(&mut Element<T>) + Send + Sync,
    T: Send + Sync,
    Element<T>: Send + Sync,
{

    functor(element);
}
