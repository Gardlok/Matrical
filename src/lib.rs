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

mod bench;
#[cfg(test)]
mod tests;

// The Strategy trait declares operations common to all versions of some algorithm.
pub trait MatrixOperation {
    fn execute(
        &self,
        matrix: &AtomicFlagMatrix,
        index: (usize, usize),
        other: Option<bool>,
    ) -> Result<bool, AtomicFlagMatrixError>;
}

///////////////////////////////////////////////////////////////////////
// A matrix of atomic bolean flags with a queue for lazy updates
pub struct AtomicFlagMatrix {
    // The data stored in the matrix
    data: Array2<AtomicCell<bool>>,

    // The queue of updates to be applied to the matrix
    update_queue: Mutex<VecDeque<Box<dyn Fn(&Array2<AtomicCell<bool>>) + Send>>>,
}

/////////////////////////////////////////////////////////////////////////

// The
//
impl AtomicFlagMatrix {
    // Create a new AtomicFlagMatrix with the given dimensions
    pub fn new(dim: (usize, usize)) -> Self {
        let data = Array2::from_shape_fn(dim, |_| AtomicCell::new(false));
        let update_queue = Mutex::new(VecDeque::new());
        Self { data, update_queue }
    }

    // Get the value at the given index in the matrix
    pub fn get(&self, index: (usize, usize)) -> Result<bool, AtomicFlagMatrixError> {
        if index.0 >= self.data.dim().0 || index.1 >= self.data.dim().1 {
            Err(AtomicFlagMatrixError::IndexOutOfBounds)
        } else {
            Ok(self.data[index].load())
        }
    }

    pub fn execute_operation(
        &self,
        operation: Box<dyn MatrixOperation>,
        index: (usize, usize),
        other: Option<bool>,
    ) -> Result<bool, AtomicFlagMatrixError> {
        operation.execute(self, index, other)
    }

    // Apply the next update in the queue to the matrix
    pub fn apply_next_update(&self) -> Result<(), AtomicFlagMatrixError> {
        let mut guard = self
            .update_queue
            .lock()
            .map_err(|_| AtomicFlagMatrixError::MutexPoisoned)?;

        if let Some(update) = guard.pop_front() {
            update(&self.data);
        }

        Ok(())
    }

    // Queue an update to be applied to the matrix
    pub fn queue_update(
        &self,
        update: Box<dyn Fn(&Array2<AtomicCell<bool>>) + Send>,
    ) -> Result<(), AtomicFlagMatrixError> {
        let mut guard = self
            .update_queue
            .lock()
            .map_err(|_| AtomicFlagMatrixError::MutexPoisoned)?;
        guard.push_back(update);

        Ok(())
    }

    // Apply all updates in the queue to the matrix
    pub fn execute_all_updates(&self) -> Result<(), AtomicFlagMatrixError> {
        let mut guard = self
            .update_queue
            .lock()
            .map_err(|_| AtomicFlagMatrixError::MutexPoisoned)?;
        while let Some(update) = guard.pop_front() {
            update(&self.data);
        }

        Ok(())
    }

    pub fn transpose(&self) -> Result<Self, AtomicFlagMatrixError> {
        let data = self.data.read().unwrap();
        let transposed_data = Array2::from_shape_fn((data.dim().1, data.dim().0), |(i, j)| {
            AtomicCell::new(data[(j, i)].load())
        });

        Ok(Self {
            data: Arc::new(RwLock::new(transposed_data)),
            update_queue: Arc::new(Mutex::new(VecDeque::new())),
        })
    }
}

/////////////////////////////////////////////////////////////////////////

// The MatrixStrategy trait
pub trait MatrixStrategy {
    type Output = Result<(), AtomicFlagMatrixError>;
    fn execute(
        &self,
        matrix: &AtomicFlagMatrix,
        index: (usize, usize),
        other: Option<bool>,
    ) -> Result<(), AtomicFlagMatrixError>;
}

// The Extractor trait
pub trait Extractor {
    fn extract(
        &self,
        matrix: &AtomicFlagMatrix,
        index: (usize, usize),
    ) -> Result<bool, AtomicFlagMatrixError>;
}

impl Extractor for GetOperation {
    fn extract(
        &self,
        matrix: &AtomicFlagMatrix,
        index: (usize, usize),
    ) -> Result<bool, AtomicFlagMatrixError> {
        if index.0 >= matrix.data.dim().0 || index.1 >= matrix.data.dim().1 {
            Err(AtomicFlagMatrixError::IndexOutOfBounds)
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
    type Output = Result<(), AtomicFlagMatrixError>;

    fn execute(
        &self,
        matrix: &mut AtomicFlagMatrix,
        index: (usize, usize),
        other: Option<bool>,
    ) -> Self::Output {
        // Check if the index is within the bounds of the matrix.
        if index.0 >= matrix.data.dim().0 || index.1 >= matrix.data.dim().1 {
            Err(AtomicFlagMatrixError::IndexOutOfBounds)
        } else {
            // If 'other' is Some(value), set the value at the given index in the matrix.
            // If 'other' is None, return an error because a value is required for this operation.
            if let Some(value) = other {
                matrix.data[index].store(value);
                Ok(())
            } else {
                Err(AtomicFlagMatrixError::MissingOperand)
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////
// The GetOperation is a strategy for getting a value at a specific index in the matrix.
//
pub struct GetOperation;
impl MatrixStrategy for GetOperation {
    type Output = Result<bool, AtomicFlagMatrixError>;

    fn execute(
        &self,
        matrix: &mut AtomicFlagMatrix,
        index: (usize, usize),
        _other: Option<bool>,
    ) -> Self::Output {
        // Check if the index is within the bounds of the matrix.
        if index.0 >= matrix.data.dim().0 || index.1 >= matrix.data.dim().1 {
            Err(AtomicFlagMatrixError::IndexOutOfBounds)
        } else {
            // Get the value at the given index in the matrix.
            let value = matrix.data[index].load();

            Ok(value)
        }
    }
}

/////////////////////////////////////////////////////////////////////////
//
// The `BitwiseAndOperation` strategy performs a bitwise AND operation
// on the value at the given index in the matrix and the provided value.
//
pub struct BitwiseAndOperation;
impl MatrixStrategy for BitwiseAndOperation {
    fn execute(
        &self,
        matrix: &AtomicFlagMatrix,
        index: (usize, usize),
        other: Option<bool>,
    ) -> Result<(), AtomicFlagMatrixError> {
        if index.0 >= matrix.data.dim().0 || index.1 >= matrix.data.dim().1 {
            Err(AtomicFlagMatrixError::IndexOutOfBounds)
        } else {
            let other = other.ok_or(AtomicFlagMatrixError::MissingOperand)?;
            let value = matrix.data[index].load() & other;
            matrix.data[index].store(value);
            Ok(())
        }
    }
}

/// The `BitwiseOrOperation` strategy performs a bitwise OR operation on the value at the given index in the matrix and the provided value.
pub struct BitwiseOrOperation;
impl MatrixStrategy for BitwiseOrOperation {
    fn execute(
        &self,
        matrix: &AtomicFlagMatrix,
        index: (usize, usize),
        other: Option<bool>,
    ) -> Result<(), AtomicFlagMatrixError> {
        if index.0 >= matrix.data.dim().0 || index.1 >= matrix.data.dim().1 {
            Err(AtomicFlagMatrixError::IndexOutOfBounds)
        } else {
            let other = other.ok_or(AtomicFlagMatrixError::MissingOperand)?;
            let value = matrix.data[index].load() | other;
            matrix.data[index].store(value);
            Ok(())
        }
    }
}

//
/// The `BitwiseXorOperation` strategy performs a bitwise XOR operation on the value at the given index in the matrix and the provided value.
pub struct BitwiseXorOperation;
impl MatrixStrategy for BitwiseXorOperation {
    fn execute(
        &self,
        matrix: &AtomicFlagMatrix,
        index: (usize, usize),
        other: Option<bool>,
    ) -> Result<(), AtomicFlagMatrixError> {
        if index.0 >= matrix.data.dim().0 || index.1 >= matrix.data.dim().1 {
            Err(AtomicFlagMatrixError::IndexOutOfBounds)
        } else {
            let other = other.ok_or(AtomicFlagMatrixError::MissingOperand)?;
            let value = matrix.data[index].load() ^ other;
            matrix.data[index].store(value);
            Ok(())
        }
    }
}

//
/// The `BitwiseNotOperation` strategy performs a bitwise NOT operation on the value at the given index in the matrix.
pub struct BitwiseNotOperation;
impl MatrixStrategy for BitwiseNotOperation {
    fn execute(
        &self,
        matrix: &AtomicFlagMatrix,
        index: (usize, usize),
        _other: Option<bool>,
    ) -> Result<(), AtomicFlagMatrixError> {
        if index.0 >= matrix.data.dim().0 || index.1 >= matrix.data.dim().1 {
            Err(AtomicFlagMatrixError::IndexOutOfBounds)
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
        matrix: &AtomicFlagMatrix,
        _index: Option<(usize, usize)>,
        _other: Option<bool>,
    ) -> Result<(), AtomicFlagMatrixError> {
        // Check if the coordinates are within the matrix dimensions
        if self.top_left.0 >= matrix.data.dim().0
            || self.top_left.1 >= matrix.data.dim().1
            || self.bottom_right.0 >= matrix.data.dim().0
            || self.bottom_right.1 >= matrix.data.dim().1
        {
            return Err(AtomicFlagMatrixError::IndexOutOfBounds);
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
// Error types for AtomicFlagMatrix operations
//
#[derive(Debug, PartialEq)]
pub enum AtomicFlagMatrixError {
    MutexPoisoned,
    IndexOutOfBounds,
}

// An AtomicFlagMatrixError is returned when an operation on an AtomicFlagMatrix fails.
impl std::error::Error for AtomicFlagMatrixError {}

impl std::fmt::Display for AtomicFlagMatrixError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            AtomicFlagMatrixError::IndexOutOfBounds => write!(f, "Index out of bounds"),
            AtomicFlagMatrixError::MutexPoisoned => write!(f, "Mutex poisoned"),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////
//
// Transposing
//
pub struct TransposedMatrix<'a> {
    matrix: &'a AtomicFlagMatrix,
}

impl<'a> TransposedMatrix<'a> {
    pub fn new(matrix: &'a AtomicFlagMatrix) -> Self {
        Self { matrix }
    }

    pub fn get(&self, index: (usize, usize)) -> Result<bool, AtomicFlagMatrixError> {
        self.matrix.get((index.1, index.0))
    }

    pub fn set(&self, index: (usize, usize), value: bool) -> Result<bool, AtomicFlagMatrixError> {
        self.matrix
            .execute_operation(self.matrix.execute_operation(index.1, index.0), value)
    }
}
