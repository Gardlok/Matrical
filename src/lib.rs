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


use db::surreal_db::*;

mod operations;
use operations::cog::*;
use operations::gears::*;
mod lenses;
use lenses::*;
mod strategies;
use strategies::*;

mod matrix;
use matrix::*;

//use operations::matrix::*;
mod db;



// Defines a set of methods that can be used to perform various operations on a given data set.
//
pub trait Strategy {
    fn prepare(&self, data: &HashMap<String, String>) -> Result<(), String>;
    fn execute(&self, data: &HashMap<String, String>) -> Result<(), String>;
    fn result(&self) -> Result<(), String>;
}

// Defines a parameterized query that can be used to perform various operations on a given data set.
//
pub struct ParameterizedQuery {
    query: String,
    parameters: Vec<String>,
}

// Defines a container for strategies and parameterized queries that can be used to perform various operations on a given data set.
//
pub struct DependencyInjectionContainer {
    strategies: Vec<Box<dyn Strategy>>,
    parameterized_queries: Vec<ParameterizedQuery>,
}
impl DependencyInjectionContainer {
    fn new() -> Self {
        Self {
            strategies: Vec::new(),
            parameterized_queries: Vec::new(),
        }
    }
}




pub trait FunctorHandler<T, F> where F: Fn() -> T {
    fn execute(&self, context: &MatrixContext) -> Result<T, MatricalError>;
}
pub fn perform_execute<T, H>(context: MatrixContext, handler: &H) -> Result<(), MatricalError>
where
    H: FunctorHandler<T, H> + Fn() -> T
{
    let result: Result<T, H> = handler.execute(&context);  
    match result {
        Ok(value) => {
            context.update_queue.lock().unwrap().push_back(Box::new(move |matrix| {
                matrix.set_value(value);
            }));
            Ok(())
        }
        Err(error) => Err(error),
    }

}

