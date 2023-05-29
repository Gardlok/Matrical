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
mod strategies;
use error::{AtomicBoolError, MatricalError, MatricalErrorType};


use db::surreal_db::SurrealDBAdapter;

mod operations;
use operations::attribute::Attribute;

mod matrix;
use matrix::AtomicFlagMatrix;

mod strategy;
use strategy::MatrixStrategy;

mod operation;


mod handler;
use handler::Handler;



// Strategy trait
// Defines a set of methods that can be used to perform various operations on a given data set.
trait Strategy {
    fn prepare(&self, data: &HashMap<String, String>) -> Result<(), String>;
    fn execute(&self, data: &HashMap<String, String>) -> Result<(), String>;
    fn result(&self) -> Result<(), String>;
}


// Defines a parameterized query that can be used to perform various operations on a given data set.
//
struct ParameterizedQuery {
    query: String,
    parameters: Vec<String>,
}


// Defines a container for strategies and parameterized queries that can be used to perform various operations on a given data set.
//
struct DependencyInjectionContainer {
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



// The Matrix struct now holds a Box<dyn MatrixOperation> which allows for changing the operation at runtime
impl<V> Matrix<V> {
   pub fn new() -> Self {
        Self {
            matrix: Arc::new(SurrealDBAdapter::new()),
            _context: MatrixContext {
                attributes: HashMap::new(),
                functors: HashMap::new(),
            },
        }
    }

    pub fn set_operation(&mut self, operation: Box<dyn MatrixOperation>) {
        self._context.operation = operation;
    }

    pub fn execute_operation(&self) -> Result<(), MatricalError> {
        self._context.operation.execute(&self._context)
    }
    // Use the execute_strategies method within your matrix operation methods to apply the desired strategies:


}

pub struct AtomicBool { atomic_bool: AtomicCell<bool> }
pub struct Attribute { _attri: PhantomData<Arc<dyn Fn()>> }
pub struct AttributesApplied { attri: SegQueue<PhantomData<Arc<dyn Any + Send + Sync>>> }
pub struct Element<V> { state: AtomicBool , _context: ElementContext<V> }
// pub struct Matrix<V> { matrix: SegQueue<Element<V>>, _context: MatrixContext }
pub struct Matrix<V> { matrix: SegQueue<SurrealDBAdapter>, _context: MatrixContext }

pub struct AttributeContext {
    pub attri: Option<SegQueue<dyn Any + Send + Sync>>,
}

pub struct ElementContext<V> {
    pub state: AtomicBool,
    pub x_idx: AtomicCell<usize>,
    pub y_idx: AtomicCell<usize>,
    pub attri: Option<SegQueue<Attribute>>,
    pub workq: SegQueue<dyn Fn (&mut Element<V>)>,  // TODO
    pub value: Option<V>,  // Not thread safe
}

pub struct MatrixContext {
    attributes: HashMap<TypeId, Arc<dyn Fn()>>,
    functors: HashMap<usize, Arc<dyn Fn( dyn Any + Send + Sync )>>,
}



// Matrix Strategy
pub trait MatrixStrategy {
    fn execute<V>(
        &self,
        matrix: &Matrix<V>,
        index: Option<(usize, usize)>,
        other: Option<bool>,
    ) -> Result<(), MatricalError>;
}



// Matrix Operation
pub trait MatrixOperation {
    fn execute(&self, context: &MatrixContext) -> Result<(), MatricalError>;
}

impl MatrixOperation for ViewOperation {
    fn execute(&self, context: &MatrixContext) -> Result<(), MatricalError> {
        unimplemented!()
    }
}

// The ViewOperation struct
pub struct ViewOperation {
    // The top left and bottom right coordinates of the sub-matrix
    top_left: (usize, usize),
    bottom_right: (usize, usize),
}
impl ViewOperation {
    // Create a new ViewOperation with the given coordinates
    pub fn new(top_left: (usize, usize), bottom_right: (usize, usize)) -> Self {
        Self {
            top_left,
            bottom_right,
        }
    }
}

impl MatrixStrategy for ViewOperation {
    fn execute<V>(
        &self,
        matrix: &Matrix<V>,
        _index: Option<(usize, usize)>,
        _other: Option<bool>,
    ) -> Result<(), MatricalError> {
        // Check if the coordinates are within the matrix dimensions
        if self.top_left.0 >= matrix.data.dim().0
            || self.top_left.1 >= matrix.data.dim().1
            || self.bottom_right.0 >= matrix.data.dim().0
            || self.bottom_right.1 >= matrix.data.dim().1
        {
            return Err(MatricalError::IndexOutOfBounds);
        }

        // Iterate over the sub-matrix and print the values
        for i in self.top_left.0..=self.bottom_right.0 {
            for j in self.top_left.1..=self.bottom_right.1 {
                let value = matrix.data[(i, j)].load();
                println!("Value at ({}, {}): {}", i, j, value);
            }
        }
        Ok(())
    }
}



// Functors are functions that operate on the matrix
// Execute a function on the value of a Element
pub trait FunctorHandler<T, F> where F: Fn() -> T {
    fn execute(&self, context: &MatrixContext) -> Result<T, MatricalError>;
}
pub fn perform_execute<T, H>(context: MatrixContext, handler: &H) -> Result<(), MatricalError>
where
    H: FunctorHandler<T, H>
{
    let result: Result<T, _> = handler.execute(&context);  
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






// // Handler trait definition TODO: Implement this
// pub trait Handler<T> {
//     fn call(&self, context: &Context<T>);
// }

// // An ErrorHandler
// pub struct ErrorHandler;

// impl Handler<Error> for ErrorHandler {
//     fn call(&self, context: &Context<Error>) {
//         // Here you can handle the error in any way you want
//         println!("Error occurred in '{}': {:?}", context.name, context.data);
//     }
// }

