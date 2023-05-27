#![feature(test)]
#![feature(associated_type_defaults)]
use ndarray::{Array2, Data, DataMut, Shape};
use ndarray::{ArrayBase, Axis, Dim, Ix2, OwnedRepr};
use rayon::iter::plumbing::{
    bridge, Consumer, Producer, ProducerCallback, UnindexedConsumer, UnindexedProducer,
};
use rayon::iter::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator,
    IntoParallelRefMutIterator, ParallelIterator,
};

use crossbeam::atomic::AtomicCell;
use std::collections::VecDeque;
use std::ops::Range;
use std::sync::Mutex;
use std::fmt;
use std::sync::Arc;
use std::any::{Any, TypeId};
use std::collections::HashMap;
// mod bench;
// #[cfg(test)]
// mod tests;

// The Strategy trait declares operations common to all versions of some algorithm.
pub trait MatrixOperation {
    fn execute(
        &self,
        matrix: &AtomicBoolMatrix,
        index: (usize, usize),
        other: Option<bool>,
    ) -> Result<bool, AtomicBoolMatrixError>;
}



struct MyOperation;
impl Operation for MyOperation {
    fn execute(&self, matrix: &AtomicBoolMatrix, index: (usize, usize), other: Option<bool>) -> Result<bool, AtomicBoolMatrixError> {
        // Perform some operation on the matrix...
        Ok(true)
    }
}




///////////////////////////////////////////////////////////////////////
// A matrix of atomic bolean flags with a queue for lazy updates
pub struct AtomicBoolMatrix {
    // The data stored in the matrix
    data: Array2<AtomicCell<bool>>,
    // The attributes of the matrix
    attributes: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
    // The queue of updates to be applied to the matrix
    update_queue: Mutex<VecDeque<Box<dyn Fn(&Array2<AtomicCell<bool>>) + Send>>>,
}


impl std::ops::Deref for AtomicBoolMatrix {
    type Target = HashMap<TypeId, Arc<dyn Any + Send + Sync>>;
    fn deref(&self) -> &Self::Target {
        &self.attributes
    }
}

/////////////////////////////////////////////////////////////////////////

// The
//
impl AtomicBoolMatrix {

    // Create a new AtomicBoolMatrix with the given dimensions
    pub fn new(dim: (usize, usize)) -> Self {
        let data = Array2::from_shape_fn(dim, |_| AtomicCell::new(false));
        let update_queue = Mutex::new(VecDeque::new());
        let attributes = HashMap::new();
        Self { data, update_queue, attributes }
    }

    // Create a new AtomicBoolMatrix with the given dimensions and data
    pub fn new_with_data(data: Array2<AtomicCell<bool>>) -> Self {
        let update_queue = Mutex::new(VecDeque::new());
        let attributes = HashMap::new();
        Self { data, update_queue, attributes }
    }

    
    
    pub fn add_attribute<T: Any + Send + Sync>(&mut self, attribute: T) {
        self.attributes.insert(TypeId::of::<T>(), Arc::new(attribute));
    }

    pub fn get_attribute<T: Any + Send + Sync>(&self) -> Option<&T> {
        self.attributes.get(&TypeId::of::<T>()).and_then(|arc| arc.downcast_ref::<T>())
    }

    pub fn remove_attribute<T: Any + Send + Sync>(&mut self) {
        self.attributes.remove(&TypeId::of::<T>());
    }
    // pub fn execute_operation(
    //     &self,
    //     operation: Box<dyn MatrixOperation>,
    //     index: (usize, usize),
    //     other: Option<bool>,
    // ) -> Result<bool, AtomicBoolMatrixError> {
    //     // Check if the index is within the bounds of the matrix.
    //     if index.0 >= self.data.dim().0 || index.1 >= self.data.dim().1 {
    //         return Err(AtomicBoolMatrixError::IndexOutOfBounds);
    //     }
    pub fn execute_operation(
        &self,
        operation: Box<dyn Operation>,
        index: (usize, usize),
        other: Option<bool>,
    ) -> Result<bool, AtomicBoolMatrixError> {
        operation.execute(self, index, other)
    }

    //








    // Get the value at the given index in the matrix
    pub fn get(&self, index: (usize, usize)) -> Result<bool, AtomicBoolMatrixError> {
        if index.0 >= self.data.dim().0 || index.1 >= self.data.dim().1 {
            Err(AtomicBoolMatrixError::IndexOutOfBounds)
        } else {
            Ok(self.data[index].load())
        }
    }

    // pub fn execute_operation(
    //     &self,
    //     operation: Box<dyn MatrixOperation>,
    //     index: (usize, usize),
    //     other: Option<bool>,
    // ) -> Result<bool, AtomicBoolMatrixError> {
    //     // Check if the index is within the bounds of the matrix.
    //     if index.0 >= self.data.dim().0 || index.1 >= self.data.dim().1 {
    //         return Err(AtomicBoolMatrixError::IndexOutOfBounds);
    //     }
    
    //     // Execute the operation and propagate any errors upwards with `?`.
    //     operation.execute(self, index, other)
    // }

    // Set the value at the given index in the matrix

    // Apply the next update in the queue to the matrix
    pub fn apply_next_update(&self) -> Result<(), AtomicBoolMatrixError> {
        let mut guard = self
            .update_queue
            .lock()
            .map_err(|_| AtomicBoolMatrixError::MutexPoisoned)?;

        if let Some(update) = guard.pop_front() {
            update(&self.data);
        }
        Ok(())
    }

    // Queue an update to be applied to the matrix
    pub fn queue_update(
        &self,
        update: Box<dyn Fn(&Array2<AtomicCell<bool>>) + Send>,
    ) -> Result<(), AtomicBoolMatrixError> {
        let mut guard = self
            .update_queue
            .lock()
            .map_err(|_| {
                AtomicBoolMatrixError::MutexPoisoned
            })?;
        guard.push_back(update);

        Ok(())
    }

    // Apply all updates in the queue to the matrix
    pub fn execute_all_updates(&self) -> Result<(), AtomicBoolMatrixError> {
        let mut guard = self
            .update_queue
            .lock()
            .map_err(|_| AtomicBoolMatrixError::MutexPoisoned)?;
        while let Some(update) = guard.pop_front() {
            update(&self.data);
        }
        Ok(())
    }

   
}

/////////////////////////////////////////////////////////////////////////
 
// The MatrixStrategy trait
pub trait MatrixStrategy {
    type Output;

    fn execute(
        &self,
        matrix: &mut AtomicBoolMatrix,
        index: (usize, usize),
        other: Option<bool>,
    ) -> Self::Output;
}

// The Extractor trait
pub trait Extractor {
    fn extract(
        &self,
        matrix: &AtomicBoolMatrix,
        index: (usize, usize),
    ) -> Result<bool, AtomicBoolMatrixError>;
}

impl Extractor for GetOperation {
    fn extract(
        &self,
        matrix: &AtomicBoolMatrix,
        index: (usize, usize),
    ) -> Result<bool, AtomicBoolMatrixError> {
        if index.0 >= matrix.data.dim().0 || index.1 >= matrix.data.dim().1 {
            Err(AtomicBoolMatrixError::IndexOutOfBounds)
        } else {
            let value = matrix.data[index].load();

            Ok(value)
        }
    }
}

/////////////////////////////////////////////////////////////////////////
// The SetOperation is a strategy for setting a value at a specific
// index in the matrix.
//
pub struct SetOperation;
impl MatrixStrategy for SetOperation {
    type Output = Result<(), AtomicBoolMatrixError>;

    fn execute(
        &self,
        matrix: &mut AtomicBoolMatrix,
        index: (usize, usize),
        other: Option<bool>,
    ) -> Self::Output {
        // Check if the index is within the bounds of the matrix.
        if index.0 >= matrix.data.dim().0 || index.1 >= matrix.data.dim().1 {
            Err(AtomicBoolMatrixError::IndexOutOfBounds)
        } else {
            // If 'other' is Some(value), set the value at the given index in the matrix.
            // If 'other' is None, return an error because a value is required for this operation.
            if let Some(value) = other {
                matrix.data[index].store(value);
                Ok(())
            } else {
                Err(AtomicBoolMatrixError::MissingOperand)
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////
// The GetOperation is a strategy for getting a value at a specific index in the matrix.
//
pub struct GetOperation;
impl MatrixStrategy for GetOperation {
    type Output = Result<bool, AtomicBoolMatrixError>;

    fn execute(
        &self,
        matrix: &mut AtomicBoolMatrix,
        index: (usize, usize),
        _other: Option<bool>,
    ) -> Self::Output {
        // Check if the index is within the bounds of the matrix.
        if index.0 >= matrix.data.dim().0 || index.1 >= matrix.data.dim().1 {
            Err(AtomicBoolMatrixError::IndexOutOfBounds)
        } else {
            // Get the value at the given index in the matrix.
            let value = matrix.data[index].load();

            Ok(value)
        }
    }
}

/////////////////////////////////////////////////////////////////////////
//// The `BitwiseAndOperation` strategy performs a bitwise AND operation
// on the value at the given index in the matrix and the provided value.
//
pub struct BitwiseAndOperation;
impl MatrixStrategy for BitwiseAndOperation {
    fn execute(
        &self,
        matrix: &AtomicBoolMatrix,
        index: (usize, usize),
        other: Option<bool>,
    ) -> Result<(), AtomicBoolMatrixError> {
        if index.0 >= matrix.data.dim().0 || index.1 >= matrix.data.dim().1 {
            Err(AtomicBoolMatrixError::IndexOutOfBounds)
        } else {
            let other = other.ok_or(AtomicBoolMatrixError::MissingOperand)?;
            let value = matrix.data[index].load() & other;
            matrix.data[index].store(value);
            Ok(())
        }
    }
}

/////////////////////////////////////////////////////////////////////////
/// The `BitwiseOrOperation` strategy performs a bitwise OR operation on 
/// the value at the given index in the matrix and the provided value.
pub struct BitwiseOrOperation;
impl MatrixStrategy for BitwiseOrOperation {
    fn execute(
        &self,
        matrix: &AtomicBoolMatrix,
        index: (usize, usize),
        other: Option<bool>,
    ) -> Result<(), AtomicBoolMatrixError> {
        if index.0 >= matrix.data.dim().0 || index.1 >= matrix.data.dim().1 {
            Err(AtomicBoolMatrixError::IndexOutOfBounds)
        } else {
            let other = other.ok_or(AtomicBoolMatrixError::MissingOperand)?;
            let value = matrix.data[index].load() | other;
            matrix.data[index].store(value);
            Ok(())
        }
    }
}


/////////////////////////////////////////////////////////////////////////
/// The `BitwiseXorOperation` strategy performs a bitwise XOR operation on
///  the value at the given index in the matrix and the provided value.
pub struct BitwiseXorOperation;
impl MatrixStrategy for BitwiseXorOperation {
    fn execute(
        &self,
        matrix: &AtomicBoolMatrix,
        index: (usize, usize),
        other: Option<bool>,
    ) -> Result<(), AtomicBoolMatrixError> {
        if index.0 >= matrix.data.dim().0 || index.1 >= matrix.data.dim().1 {
            Err(AtomicBoolMatrixError::IndexOutOfBounds)
        } else {
            let other = other.ok_or(AtomicBoolMatrixError::MissingOperand)?;
            let value = matrix.data[index].load() ^ other;
            matrix.data[index].store(value);
            Ok(())
        }
    }
}

//
/// The `BitwiseNotOperation` strategy performs a bitwise NOT operation on
///  the value at the given index in the matrix.
pub struct BitwiseNotOperation;
impl MatrixStrategy for BitwiseNotOperation {
    fn execute(
        &self,
        matrix: &AtomicBoolMatrix,
        index: (usize, usize),
        _other: Option<bool>,
    ) -> Result<(), AtomicBoolMatrixError> {
        if index.0 >= matrix.data.dim().0 || index.1 >= matrix.data.dim().1 {
            Err(AtomicBoolMatrixError::IndexOutOfBounds)
        } else {
            let value = !matrix.data[index].load();
            matrix.data[index].store(value);

            Ok(())
        }
    }
}
//////////////////////////////////////////////////////////////////////////////////

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
        matrix: &AtomicBoolMatrix,
        _index: Option<(usize, usize)>,
        _other: Option<bool>,
    ) -> Result<(), AtomicBoolMatrixError> {
        // Check if the coordinates are within the matrix dimensions
        if self.top_left.0 >= matrix.data.dim().0
            || self.top_left.1 >= matrix.data.dim().1
            || self.bottom_right.0 >= matrix.data.dim().0
            || self.bottom_right.1 >= matrix.data.dim().1
        {
            return Err(AtomicBoolMatrixError::IndexOutOfBounds);
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

//////////////////////////////////////////////////////////////////////////////////
//
// Error types for AtomicBoolMatrix operations
//
#[derive(Debug, PartialEq)]
pub enum AtomicBoolMatrixError {
    MutexPoisoned,
    IndexOutOfBounds,
}

// An AtomicBoolMatrixError is returned when an operation on an AtomicBoolMatrix fails.
impl fmt::Display for AtomicBoolMatrixError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AtomicBoolMatrixError::MutexPoisoned => write!(f, "Mutex was poisoned"),
            AtomicBoolMatrixError::IndexOutOfBounds => write!(f, "Index out of bounds"),
        }
    }
}



///////////////////////////////////////////////////////////////////////////////////////////
//
// Sum of all elements
//
pub struct SumOperation;
impl MatrixOperation for SumOperation {
    fn execute(
        &self,
        matrix: &AtomicBoolMatrix,
        _index: (usize, usize),
        _other: Option<bool>,
    ) -> Result<bool, AtomicBoolMatrixError> {
        let mut sum = 0;
        for i in 0..matrix.data.dim().0 {
            for j in 0..matrix.data.dim().1 {
                sum += matrix.data[(i, j)].load() as i32;
            }
        }
        println!("Sum of all elements: {}", sum);
        Ok(true)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////
//
// Transposing
//

// pub struct TransposedMatrix<'a> {
//     matrix: &'a AtomicBoolMatrix,
// }
// impl<'a> TransposedMatrix<'a> {
//     pub fn new(matrix: &'a AtomicBoolMatrix) -> Self {
//         Self { matrix }
//     }
//     pub fn get(&self, index: (usize, usize)) -> Result<bool, AtomicBoolMatrixError> {
//         self.matrix.get((index.1, index.0))
//     }
//     pub fn set(&self, index: (usize, usize), value: bool) -> Result<bool, AtomicBoolMatrixError> {
//         self.matrix
//             .execute_operation(index, Some(value), &SetOperation)
//     }
// }

// fn main() {
//     let mut matrix = AtomicBoolMatrix::new((10, 10));
//     matrix.add_attribute("some attribute".to_string());

//     let operation = MyOperation;
//     let result = matrix.execute_operation(Box::new(operation), (5, 5), Some(true));
// }
