pub mod gears;
pub mod cog;
pub mod lens;

pub use gears::*;
pub use cog::*;
pub use lens::*;


use crate::*;

pub trait MatrixOperation {
    fn execute(&self, context: &MatrixContext) -> Result<(), MatricalError>;
}

pub trait ElementOperation<V> {
    fn execute(&self, context: &ElementContext<V>) -> Result<(), MatricalError>;
}


pub fn execute_functor<T, F>(element: &mut Element<T>, functor: F)
where
    F: Fn(&mut Element<T>) + Send + Sync,
    T: Send + Sync,
    Element<T>: Send + Sync,
{

    functor(element);
}

