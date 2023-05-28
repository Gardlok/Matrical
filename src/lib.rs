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

use crossbeam::atomic::AtomicCell;
use crossbeam_queue::ArrayQueue;
use crossbeam_queue::SegQueue;
use crossbeam_utils::sync::ShardedLock;






pub struct AtomicBool { atomic_bool: AtomicCell<bool> }
pub struct Attribute { _attri: PhantomData<Arc<dyn Fn()>> }
pub struct AttributesApplied { attri: SegQueue<PhantomData<Arc<dyn Any + Send + Sync>>> }
pub struct Element<V> { state: AtomicBool , _context: ElementContext<V> }
pub struct Matrix<V> { matrix: ArrayQueue<Element<V>>, _context: MatrixContext }

// The Matrix struct now holds a Box<dyn MatrixOperation> which allows for changing the operation at runtime
impl<V> Matrix<V> {
   pub fn new() -> Self {
        Self {
            matrix: ArrayQueue::new(100),
            _context: MatrixContext {
                attributes: HashMap::new(),
                functors: HashMap::new(),
                update_queue: SegQueue::new(),
            },
        }
    }

    pub fn set_operation(&mut self, operation: Box<dyn MatrixOperation>) {
        self._context.operation = operation;
    }

    pub fn execute_operation(&self) -> Result<(), MatricalError> {
        self._context.operation.execute(&self._context)
    }
}

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

// Element Operation
pub trait ElementOperation {
    fn execute(&self, context: &ElementContext) -> Result<(), MatricalError>;
}

// Attribute Operation
pub trait AttributeOperation {
    fn execute(&self, context: &AttributeContext) -> Result<(), MatricalError>;
}

// How to look at the matrix
pub struct ViewOperation;

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

// Set the state
pub struct StateSetOperation;
//
impl MatrixOperation for StateSetOperation {
    fn execute(&self, context: &MatrixContext, value: AtomicBool) -> Result<(), MatricalError> {
        // Here we'll set all the states to true
        for row in &mut context.matrix {
            for element in row {
                element.state.atomic_bool.store(value);
            }
        }
        Ok(())
    }
}

pub struct StateGetOperation;

impl MatrixOperation for StateGetOperation {
    fn execute(&self, context: &MatrixContext) -> Result<(), MatricalError> {
        // Implement the MatrixOperation trait
        
    }
}

pub struct StateToggleOperation;

impl MatrixOperation for StateToggleOperation {
    fn execute(&self, context: &MatrixContext) -> Result<(), MatricalError> {
        // Implement the MatrixOperation trait
    }
}

pub struct BitwiseAndOperation;

impl MatrixOperation for BitwiseAndOperation {
    fn execute(&self, context: &MatrixContext) -> Result<(), MatricalError> {
        // Implement the MatrixOperation trait
    }
}

pub struct BitwiseOrOperation;

impl MatrixOperation for BitwiseOrOperation {
    fn execute(&self, context: &MatrixContext) -> Result<(), MatricalError> {
        // Implement the MatrixOperation trait
    }
}

pub struct BitwiseXorOperation;

impl MatrixOperation for BitwiseXorOperation {
    fn execute(&self, context: &MatrixContext) -> Result<(), MatricalError> {
        // Implement the MatrixOperation trait
    }
}

pub struct BitwiseNotOperation;

impl MatrixOperation for BitwiseNotOperation {
    fn execute(&self, context: &MatrixContext) -> Result<(), MatricalError> {
        // Implement the MatrixOperation trait
    }
}

pub struct CreateAttributeOperation;

impl MatrixOperation for CreateAttributeOperation {
    fn execute(&self, context: &MatrixContext) -> Result<(), MatricalError> {
        // Implement the MatrixOperation trait
    }
}

pub struct GetAttributeOperation;

impl MatrixOperation for GetAttributeOperation {
    fn execute(&self, context: &MatrixContext) -> Result<(), MatricalError> {
        // Implement the MatrixOperation trait
    }
}

pub struct RemoveAttributeOperation;

impl MatrixOperation for RemoveAttributeOperation {
    fn execute(&self, context: &MatrixContext) -> Result<(), MatricalError> {
        // Implement the MatrixOperation trait
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
// pub trait FunctorHandler<T, F> where F: Fn() -> Sync + Send {
//     fn execute(&self, context: &MatrixContext<T>) -> Result<T, MatricalError>;
// }
// pub fn perform_execute<T, H>(context: MatrixContext<T>, handler: &H) -> Result<(), MatricalError>
// where
//     H: FunctorHandler<T, F>
// {
//     let result: Result<T, _> = handler.compute(&context.);
//     match result {
//         Ok(value) => {
//             context.update_queue.lock().unwrap().push_back(Box::new(move |matrix| {
//                 matrix.set_value(value);
//             }));
//             Ok(())
//         }
//         Err(error) => Err(error),
//     }

// }

// pub fn from_enum_matrix(matrix: Array2<MatrixElement>) -> Self {
//     let rows = matrix.nrows();
//     let cols = matrix.ncols();
//     let mut data = Vec::with_capacity(rows * cols);
//     for element in matrix.iter() {
//         let val = match element {
//             MatrixElement::VariantA => true,  // or whatever logic you want
//             MatrixElement::VariantB => false, // or whatever logic you want
//             MatrixElement::VariantC => true,  // or whatever logic you want
//             // ...
//         };
//         data.push(AtomicBool::new(val));
//     }
//     AttributeMatrix {
//         matrix: ArcSwap::new(Arc::new(Array2::from_shape_vec((rows, cols), data).unwrap())),
//     }
// }

// // example of a method for updating the matrix using parallel processing
// pub fn parallel_update<F>(&self, update_func: F)
// where
//     F: Fn(usize, usize) -> bool + Sync,
// {
//     let rows = self.matrix.load().rows();
//     let cols = self.matrix.load().cols();
//     (0..rows).into_par_iter().for_each(|row| {
//         for col in 0..cols {
//             let new_val = update_func(row, col);
//             self.update_value(row, col, new_val);
//         }
//     });
// }

// mod Mask {
//     //////////////////////////////////////////////////
//     // DASHER
//     use dashmap::DashMap;
//     type MaskId = String;

//     // Our concurrent mask storage.
//     struct MaskStorage {
//         masks: DashMap<MaskId, AttributeMatrix>,
//     }
//     impl MaskStorage {
//         fn new() -> Self {
//             MaskStorage {
//                 masks: DashMap::new(),
//             }
//         }
//         fn insert(&self, mask_id: MaskId, mask: AttributeMatrix) {
//             self.masks.insert(mask_id, mask);
//         }
//         fn get(&self, mask_id: &MaskId) -> Option<AttributeMatrix> {
//             self.masks.get(mask_id).map(|mask_ref| mask_ref.clone())
//         }
//         fn remove(&self, mask_id: &MaskId) -> Option<AttributeMatrix> {
//             self.masks.remove(mask_id).map(|(_, mask)| mask)
//         }
//     }
// }

//     // The error type returned by operations

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
#[derive(Debug)]
pub struct Error {
    message: String,
    // you can add more fields here
}

impl Error {
    pub fn new(message: &str) -> Self {
        Error {
            message: message.to_string(),
        }
    }
}

// Handler trait definition TODO: Implement this
pub trait Handler<T> {
    fn call(&self, context: &Context<T>);
}

// An ErrorHandler
pub struct ErrorHandler;

impl Handler<Error> for ErrorHandler {
    fn call(&self, context: &Context<Error>) {
        // Here you can handle the error in any way you want
        println!("Error occurred in '{}': {:?}", context.name, context.data);
    }
}

use surrealdb::Datastore;
use surrealdb::Session;
use surrealdb::Value;
use surrealdb::Error;

pub struct SurrealDBAdapter {
    datastore: Datastore,
    session: Session,
}

impl SurrealDBAdapter {
    pub async fn new() -> Result<Self, Error> {
        let datastore = Datastore::new("memory").await?;
        let session = Session::for_kv().with_ns("test").with_db("test");
        Ok(SurrealDBAdapter {
            datastore,
            session,
        })
    }

    pub async fn run_query(&self, sql: &str) -> Result<Vec<Value>, Error> {
        let result = self.datastore.execute(sql, &self.session, None, false).await?;
        Ok(result.into_iter().map(|res| res.result).collect())
    }

    pub fn get_matrix_value(&self, row: usize, col: usize) -> Result<bool, MatricalError> {
        let sql = format!("SELECT matrix[{}][{}] AS value FROM matrical;", row, col);
        let result = self.run_query(&sql)?;
        if let Some(value) = result.first() {
            if let Value::Bool(b) = value {
                Ok(*b)
            } else {
                Err(MatricalError::ValueError)
            }
        } else {
            Err(MatricalError::NotFoundError)
        }
    }

    pub fn set_matrix_value(&mut self, row: usize, col: usize, value: bool) -> Result<(), MatricalError> {
        let sql = format!("UPDATE matrical SET matrix[{}][{}] = {};", row, col, value);
        self.run_query(&sql)?;
        Ok(())
    }

    // Implement the remaining matrix operations methods
    // ...

}
