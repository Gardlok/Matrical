
pub mod cog;
pub mod gear;
pub mod lens;
pub mod tag;

pub use cog::*;
pub use gear::*;
pub use lens::*;
pub use tag::*;




use crate::*;


pub(crate) trait MatrixStrategy where Self: Send + Sync + Clone + Eq + 'static {
    fn execute<V: Clone + Send + Sync + Eq + Default +'static> (
        &self,
        matrix: &Matrix<V>,
        index: Option<(usize, usize)>,
        other: Option<bool>,
    ) -> Result<(), MatricalError>;
}

pub(crate) trait ElementStrategy<V> 
where V: Clone + Send + Sync + 'static + Default + PartialEq + Eq {
    fn execute(
        &self,
        element: &Element<V>,
        index: Option<(usize, usize)>,
        other: Option<bool>,
    ) -> Result<(), MatricalError>;
}




