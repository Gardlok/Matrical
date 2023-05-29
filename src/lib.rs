use crossbeam::epoch::Atomic;
use dashmap::DashMap;

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
use crossbeam_queue::ArrayQueue;
use crossbeam_queue::SegQueue;
use crossbeam_utils::sync::ShardedLock;

use surrealdb::Datastore;
use surrealdb::Session;
use surrealdb::Value;
use surrealdb::Error;

use std::collections::HashMap;

// Strategy trait
// Defines a set of methods that can be used to perform various operations on a given data set.
trait Strategy {
    fn prepare(&self, data: &HashMap<String, String>) -> Result<(), String>;
    fn execute(&self, data: &HashMap<String, String>) -> Result<(), String>;
    fn result(&self) -> Result<(), String>;
}

// FilterStrategy trait
// Defines a set of methods that can be used to filter a given data set.
trait FilterStrategy: Strategy {
    fn filter(&self, data: &HashMap<String, String>) -> Result<(), String>;
}

// SortStrategy trait
// Defines a set of methods that can be used to sort a given data set.
trait SortStrategy: Strategy {
    fn sort(&self, data: &HashMap<String, String>) -> Result<(), String>;
}

// AggregateStrategy trait
// Defines a set of methods that can be used to aggregate a given data set.
trait AggregateStrategy: Strategy {
    fn aggregate(&self, data: &HashMap<String, String>) -> Result<(), String>;
}

// ParameterizedQuery struct
// Defines a parameterized query that can be used to perform various operations on a given data set.
struct ParameterizedQuery {
    query: String,
    parameters: Vec<String>,
}

// DependencyInjectionContainer struct
// Defines a container for strategies and parameterized queries that can be used to perform various operations on a given data set.
struct DependencyInjectionContainer {
    strategies: Vec<Box<dyn Strategy>>,
    parameterized_queries: Vec<ParameterizedQuery>,
}

// SurrealDBAdapter struct
// Defines an adapter for the SurrealDB database that can be used to execute strategies and parameterized queries.
pub struct SurrealDBAdapter {
    datastore: Datastore,
    session: Session,
    di_container: DependencyInjectionContainer,
}

impl SurrealDBAdapter {

    pub fn new() -> Result<Self, Error> {
        let datastore = Datastore::new("memory")?;
        let session = Session::for_kv().with_ns("test").with_db("test");
        let di_container = DependencyInjectionContainer::new();
        Ok(Self {
            datastore,
            session,
            di_container,
        })
    }

	// add_strategy()
	// Adds a strategy to the Dependency Injection Container.
    pub fn add_strategy(&mut self, strategy: Box<dyn Strategy>) {
        self.di_container.strategies.push(strategy);
    }

	// add_parameterized_query()
	// Adds a parameterized query to the Dependency Injection Container.
    pub fn add_parameterized_query(&mut self, query: ParameterizedQuery) {
        self.di_container.parameterized_queries.push(query);
    }

	// execute_strategies()
	// Executes the strategies in the Dependency Injection Container on the given data set.
    pub async fn execute_strategies(&self, data: &HashMap<String, String>) -> Result<(), String> {
        for strategy in &self.di_container.strategies {
            strategy.prepare(data)?;
            strategy.execute(data)?;
            strategy.result()?;
        }
        Ok(())
    }

	// get_matrix_value()
	// Retrieves a matrix value from the given data set.
    pub fn get_matrix_value(&self, row: usize, col: usize) -> Result<bool, MatricalError> {
        // Get data from SurrealDB or other source
        let data = HashMap::new();  // Placeholder, replace with actual data retrieval

        // Apply strategies
        self.execute_strategies(&data)?;

        // Perform the matrix value retrieval operation

        Ok(value)
    }

	// set_matrix_value()
	// Sets a matrix value in the given data set.
    pub fn set_matrix_value(&mut self, row: usize, col: usize, value: bool) -> Result<(), MatricalError> {
        // Get data from SurrealDB or other source
        let data = HashMap::new();  // Placeholder, replace with actual data retrieval

        // Apply strategies
        self.execute_strategies(&data)?;

        // Perform the matrix value setting operation
      

        Ok(())
    }
}

impl DependencyInjectionContainer {
    fn new() -> Self {
        Self {
            strategies: Vec::new(),
            parameterized_queries: Vec::new(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////
// // TEMPLATE Access to SurrealDB
// impl Strategy for YourStrategyType {
//     fn prepare(&self, data: &HashMap<String, String>) -> Result<(), String> {
//         // Implement prepare method for your strategy

        

//     }

//     fn execute(&self, data: &HashMap<String, String>) -> Result<(), String> {
//         // Implement execute method for your strategy
//     }

//     fn result(&self) -> Result<(), String> {
//         // Implement result method for your strategy
//     }
// }

// impl FilterStrategy for YourFilterStrategyType {
//     fn filter(&self, data: &HashMap<String, String>) -> Result<(), String> {
//         // Implement filter method for your filter strategy
//     }
// }
// impl SortStrategy for YourSortStrategyType {
//     fn sort(&self, data: &HashMap<String, String>) -> Result<(), String> {
//         // Implement sort method for your sort strategy
//     }
// }
// impl AggregateStrategy for YourAggregateStrategyType {
//     fn aggregate(&self, data: &HashMap<String, String>) -> Result<(), String> {
//         // Implement aggregate method for your aggregate strategy
//     }
// }



// The Matrix struct now holds a Box<dyn MatrixOperation> which allows for changing the operation at runtime
impl<V> Matrix<V> {
   pub fn new() -> Self {
        Self {
            matrix: ArrayQueue::new(100),
            _context: MatrixContext {
                attributes: HashMap::new(),
                functors: HashMap::new(),
                // update_queue: SegQueue::new(),
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
pub struct Matrix<V> { matrix: ArrayQueue<Element<V>>, _context: MatrixContext }

pub struct AttributeContext {
    pub attri: Option<SegQueue<dyn Any + Send + Sync>>,
}

pub struct ElementContext<V> {
    pub state: AtomicBool,
    pub x_idx: AtomicCell<usize>,
    pub y_idx: AtomicCell<usize>,
    pub attri: Option<SegQueue<Attribute>>,
    pub workq: SegQueue<dyn Fn(MatrixOperation) + Send + Sync>,  // Worker, stealer?
    pub value: Option<V>,  // Not thread safe
}

pub struct MatrixContext {
    attributes: HashMap<Any::TypeId<Attribute>, Arc<dyn Any + Send + Sync>>,
    functors: HashMap<usize, Arc<dyn Fn( Any + Send + Sync )>>,

}

// Matrix Operation
pub trait MatrixOperation {
    fn execute(&self, context: &MatrixContext) -> Result<(), MatricalError>;
}


pub fn get_matrix_value(&self, row: usize, col: usize) -> Result<bool, MatricalError> {
    // Get data from SurrealDB or other source
    let data = HashMap::new();  // Placeholder, TODO: replace with actual data retrieval
    // Apply strategies
    self.execute_strategies(&data)?;
    Ok(value)
}

pub fn set_matrix_value(&mut self, row: usize, col: usize, value: bool) -> Result<(), MatricalError> {
    // Get data from SurrealDB or other source
    let data = HashMap::new();  // Placeholder, replace with actual data retrieval
    // Apply strategies
    self.execute_strategies(&data)?;
    Ok(())
}


// Element Operation
pub trait ElementOperation {
    fn execute(&self, context: &ElementContext) -> Result<(), MatricalError>;
}

// Attribute Operation
pub trait AttributeOperation {
    fn execute(&self, context: &AttributeContext) -> Result<(), MatricalError>;
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
    fn execute(
        &self,
        matrix: &Matrix,
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


// The Matrix struct now holds a Box<dyn MatrixOperation> which allows for changing the operation at runtime
impl<V> Matrix {
    pub fn set_operation(&mut self, operation: Box<dyn MatrixOperation>) {
        self._context.operation = operation;
    }

    pub fn execute_operation(&self) -> Result<(), MatricalError> {
        self._context.operation.execute(&self._context)
    }
}

// Functors are functions that operate on the matrix
// Execute a function on the value of a Element
pub trait FunctorHandler<T, F> where F: Fn() -> Sync + Send {
    fn execute(&self, context: &MatrixContext<T>) -> Result<T, MatricalError>;
}
pub fn perform_execute<T, H>(context: MatrixContext<T>, handler: &H) -> Result<(), MatricalError>
where
    H: FunctorHandler<T, F>
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


// // Error handling
pub enum MatricalError {
    Regular(MatricalErrorType),
    Custom(String),
    ShouldNotOccur,
}

pub enum AtomicBoolError {
    MutexPoisoned,
    IndexOutOfBounds,
    MissingOperand,
}

pub enum MatricalErrorType {
    IncorrectDimensions,
    IncorrectFormat,
}

impl fmt::Debug for MatricalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for MatricalError {}

impl fmt::Display for MatricalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MatricalError::Regular(err) => write!(f, "Regular error: {}", err.as_str()),
            MatricalError::Custom(err) => write!(f, "Custom error: {}", err),
            MatricalError::ShouldNotOccur => write!(f, "Other error"),
        }
    }
}

impl MatricalErrorType {
    fn as_str(&self) -> &str {
        match *self {
            MatricalErrorType::IncorrectDimensions => "IncorrectDimensions",
            MatricalErrorType::IncorrectFormat => "IncorrectFormat",
        }
    }
}

// ERROR
struct ErrorHandler;
use std::fmt::Debug;

// Assuming you have an Error type
// #[derive(Debug)]
// pub struct Error {
//     message: String,
//     // you can add more fields here
// }

// impl Error {
//     pub fn new(message: &str) -> Self {
//         Error {
//             message: message.to_string(),
//         }
//     }
// }

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

