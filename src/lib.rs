#![feature(test)]
use ndarray::{Array2, Data, DataMut, Shape};
use ndarray::{ArrayBase, Axis, Dim, Ix2, OwnedRepr};
use rayon::iter::plumbing::{
    bridge, Consumer, Producer, ProducerCallback, UnindexedConsumer, UnindexedProducer,
};
use rayon::iter::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator,
    IntoParallelRefMutIterator, ParallelIterator,
};
use rayon::prelude::*;
use std::cell::UnsafeCell;
use std::error::Error;
use std::fmt;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::{Arc, RwLock};

use crossbeam::atomic::AtomicCell;
use std::collections::VecDeque;
use std::sync::Mutex;

mod bench;
#[cfg(test)]
mod tests;

// Error types for AtomicFlagMatrix operations

#[derive(Debug, PartialEq)]
pub enum AtomicFlagMatrixError {
    MutexPoisoned,
    IndexOutOfBounds,
}

// A matrix of atomic boolean flags with a queue for lazy updates
pub struct AtomicFlagMatrix {
    // The data stored in the matrix
    data: Array2<AtomicCell<bool>>,

    // The queue of updates to be applied to the matrix
    update_queue: Mutex<VecDeque<Box<dyn Fn(&Array2<AtomicCell<bool>>) + Send>>>,
}

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
}

// The Strategy trait declares operations common to all versions of some algorithm.
pub trait MatrixOperation {
    fn execute(
        &self,
        matrix: &AtomicFlagMatrix,
        index: (usize, usize),
        other: Option<bool>,
    ) -> Result<(), AtomicFlagMatrixError>;
}

// Concrete Strategies implement the algorithm while following the base Strategy interface.
#[derive(Clone)]
struct SetOperation;
impl MatrixOperation for SetOperation {
    fn execute(
        &self,
        matrix: &AtomicFlagMatrix,
        index: (usize, usize),
        other: Option<bool>,
    ) -> Result<(), AtomicFlagMatrixError> {
        if index.0 >= matrix.data.dim().0 || index.1 >= matrix.data.dim().1 {
            Err(AtomicFlagMatrixError::IndexOutOfBounds)
        } else {
            matrix.data[index].store(other.unwrap());
            Ok(())
        }
    }
}

struct BitwiseAndOperation;
impl MatrixOperation for BitwiseAndOperation {
    fn execute(
        &self,
        matrix: &AtomicFlagMatrix,
        index: (usize, usize),
        other: Option<bool>,
    ) -> Result<(), AtomicFlagMatrixError> {
        if index.0 >= matrix.data.dim().0 || index.1 >= matrix.data.dim().1 {
            Err(AtomicFlagMatrixError::IndexOutOfBounds)
        } else {
            let value = matrix.data[index].load() & other.unwrap();

            matrix.data[index].store(value);

            Ok(())
        }
    }
}

impl AtomicFlagMatrix {
    pub fn execute_operation(
        &self,
        operation: Box<dyn MatrixOperation>,
        index: (usize, usize),
        other: Option<bool>,
    ) -> Result<(), AtomicFlagMatrixError> {
        operation.execute(self, index, other)
    }
}

struct BitwiseOrOperation;
impl MatrixOperation for BitwiseOrOperation {
    fn execute(
        &self,
        matrix: &AtomicFlagMatrix,
        index: (usize, usize),
        other: Option<bool>,
    ) -> Result<(), AtomicFlagMatrixError> {
        if index.0 >= matrix.data.dim().0 || index.1 >= matrix.data.dim().1 {
            Err(AtomicFlagMatrixError::IndexOutOfBounds)
        } else {
            let value = matrix.data[index].load() | other.unwrap();
            matrix.data[index].store(value);
            Ok(())
        }
    }
}

struct BitwiseXorOperation;
impl MatrixOperation for BitwiseXorOperation {
    fn execute(
        &self,
        matrix: &AtomicFlagMatrix,
        index: (usize, usize),
        other: Option<bool>,
    ) -> Result<(), AtomicFlagMatrixError> {
        if index.0 >= matrix.data.dim().0 || index.1 >= matrix.data.dim().1 {
            Err(AtomicFlagMatrixError::IndexOutOfBounds)
        } else {
            let value = matrix.data[index].load() ^ other.unwrap();
            matrix.data[index].store(value);

            Ok(())
        }
    }
}

struct BitwiseNotOperation;
impl MatrixOperation for BitwiseNotOperation {
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
