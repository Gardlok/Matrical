use crossbeam::epoch::Atomic;


// use ndarray::{Array2, Data, DataMut, Shape};
// use ndarray::{ArrayBase, Axis, Dim, Ix2, OwnedRepr};
use serde::de::value::U32Deserializer;

use std::any::{Any, TypeId};
use std::fmt;
use std::marker::PhantomData;
use std::ops::Range;
use std::sync::{Arc, Mutex};
use std::error::Error;

use crossbeam::atomic::AtomicCell;
use crossbeam::queue::SegQueue;
use dashmap::DashMap as HashMap;

mod error;

use error::{AtomicBoolError, MatricalError, MatricalErrorType};



pub mod operations;
pub use operations::*;
pub use operations::mechanics::*;

pub mod strategies;
pub use strategies::cog::*;
pub use strategies::gear::*;
pub use strategies::lens::*;
pub use strategies::tag::*;

pub mod schematics;
pub use schematics::data::*;
pub use schematics::element::*;
pub use schematics::matrix::*;
pub use schematics::vector::*;



// Defines a set of methods that can be used to perform various operations on a given data set.
//
pub trait Strategy {
    fn prepare(&self, data: &HashMap<String, String>) -> Result<(), String>;
    fn execute(&self, data: &HashMap<String, String>) -> Result<(), String>;
    fn result(&self) -> Result<(), String>;
}





// pub trait FunctorHandler<T, F> where F: Fn() -> T {
//     fn execute(&self, context: &MatrixContext) -> Result<T, MatricalError>;
// }
// pub fn perform_execute<T, H>(context: MatrixContext, handler: &H) -> Result<(), MatricalError>
// where
//     H: FunctorHandler<T, H> + Fn() -> T
// {
//     handler.execute(&context)?

// }