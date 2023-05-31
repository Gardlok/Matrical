// Module: operations
    
pub mod arithmetic;
pub mod aggregate;
pub mod mechanics;
pub mod bitwise;
pub mod boolean;
pub mod filter;
pub mod sort;

pub use arithmetic::*;
pub use aggregate::*;
pub use mechanics::*;
pub use bitwise::*;
pub use boolean::*;
pub use filter::*;
pub use sort::*;

use crate::{ElementContext, error::MatricalError};
    



pub trait MatrixOperation {
    fn execute(&self, context: &dyn MatrixContext) -> Result<(), MatricalError>;
}

pub trait ElementOperation<V>where V: Clone + Send + Sync + 'static + Default + PartialEq + Eq {
    fn execute(&self, context: &ElementContext<V>) -> Result<(), MatricalError>;
}




