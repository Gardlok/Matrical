/*
use arc_swap::{ArcSwap, ArcSwapAny};
use bung::{decode::from_slice, to_vec};
use ndarray::{Array2, Axis};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::{Cell, Mapping, Matrix, PhantomCell};

// #[derive(Clone, Serialize, Deserialize)]
// pub struct AttributeMatrix<T> {
//     matrix: ArcSwapAny<Arc<ArrayBase<OwnedRepr<(AtomicBool, T)>, Dim<[usize; 2]>>>>,
// }
#[derive(Clone, Serialize, Deserialize)]
pub struct AttributeMatrix<T> {
    matrix: ArcSwap<Array2<(AtomicBool, T)>>,
}

impl<T> fmt::Debug for AttributeMatrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AttributeMatrix")
    }
}

impl<T> Matrix for AttributeMatrix<T> {
    // constructor
    fn new(rows: usize, cols: usize) -> Self {
        let mut data = Vec::with_capacity(rows * cols);
        for _ in 0..rows * cols {
            data.push((false, Default::default()));
        }
        AttributeMatrix {
            matrix: ArcSwap::new(Arc::new(Array2::from_shape_vec((rows, cols), data))),
        }
    }

    fn get_value(&self, row: usize, col: usize) -> Result<bool, String> {
        let matrix = self.matrix.load();
        let rows = matrix.nrows();
        let cols = matrix.ncols();
        if row >= rows || col >= cols {
            Err(format!(
                "Indices out of range. Matrix size is {}x{}, but received row={}, col={}",
                rows, cols, row, col
            ))
        } else {
            Ok(matrix.index((row, col)).load(Ordering::SeqCst))
        }
    }
    // update value with priority
    fn update_value(
        &self,
        row: usize,
        col: usize,
        val: bool,
        priority: bool,
    ) -> Result<(), String> {
        let rows = self.matrix.load().nrows();
        let cols = self.matrix.load().ncols();
        if row >= rows || col >= cols {
            return Err(format!(
                "Indices out of range. Matrix size is {}x{}, but received row={}, col={}",
                rows, cols, row, col
            ));
        }

        self.matrix.load().index_mut((row, col)).0 = val;
        self.matrix.load().index_mut((row, col)).1 = priority;
        Ok(())
    }

    fn from_enum_matrix<E: Into<Self>, M: Mapping<E>>(matrix: &Array2<E>, mapper: &M) -> Self {
        let rows = matrix.nrows();
        let cols = matrix.ncols();
        let mut data = Vec::with_capacity(rows * cols);
        for element in matrix.iter() {
            let value = mapper.map(*element);

            let cell = if value {
                Cell::Value(value)
            } else {
                Cell::Phantom(PhantomCell::new())
            };

            data.push(cell);
        }

        let attribute_matrix = Array2::from_shape_vec((rows, cols), data).unwrap();
        AttributeMatrix::from(attribute_matrix)
    }
    // Implement other methods for AttributeMatrix
}

impl<T> AttributeMatrix<T> {
    // search for values in the matrix using rayon parallelism
    pub fn search(&self, value: bool) -> Vec<(usize, usize, &T)> {
        self.matrix
            .load()
            .indexed_iter()
            .filter(|(_, (val, _))| *val == value)
            .par_bridge() // Use par_bridge() to parallelize the iteration
            .map(|((row, col), (_, priority))| (row, col, priority))
            .collect()
    }

    // // get value
    // pub fn get_value(&self, row: usize, col: usize) -> Result<Option<bool>, String> {
    //     let rows = self.matrix.load().nrows();
    //     let cols = self.matrix.load().ncols();
    //     if row >= rows || col >= cols {
    //         return Err(format!(
    //             "Indices out of range. Matrix size is {}x{}, but received row={}, col={}",
    //             rows, cols, row, col
    //         ));
    //     }

    //     Ok(Some(self.matrix.load().index((row, col)).0))
    // }

    // get priority
    pub fn get_priority(&self, row: usize, col: usize) -> Result<Option<&T>, String> {
        let rows = self.matrix.load().nrows();
        let cols = self.matrix.load().ncols();
        if row >= rows || col >= cols {
            return Err(format!(
                "Indices out of range. Matrix size is {}x{}, but received row={}, col={}",
                rows, cols, row, col
            ));
        }
        Ok(Some(&self.matrix.load().index((row, col)).1))
    }

    // load precompiled matrix
    pub fn load_matrix(&self, matrix: Array2<(bool, T)>) {
        self.matrix.swap(Arc::new(matrix));
    }

    // save as mask
    pub fn save_as_mask(&self) -> Vec<bool> {
        self.matrix.load().iter().map(|(value, _)| *value).collect()
    }

    // perform mean, square root, dot product, etc.
    // these operations may require a conversion of the matrix to float
    pub fn mean(&self) -> Option<f64> {
        let count = self.matrix.load().len();
        if count == 0 {
            return None;
        }
        let sum: f64 = self
            .matrix
            .load()
            .iter()
            .map(|(value, _)| *value as f64)
            .sum();
        Some(sum / count as f64)
    }

    // parallel update with priority using rayon parallelism
    pub fn parallel_update<F>(&self, update_func: F)
    where
        F: Fn(usize, usize) -> (bool, T) + Sync,
    {
        let mut matrix = Arc::make_mut(&mut Arc::clone(&self.matrix));
        matrix.par_map_inplace(|(value, priority), (row, col)| {
            let (new_value, new_priority) = update_func(row, col);
            *value = new_value;
            *priority = new_priority;
        });
    }

    // stack matrices
    pub fn stack(&self, other: &AttributeMatrix<T>) -> Option<AttributeMatrix<T>> {
        let self_rows = self.matrix.load().nrows();
        let self_cols = self.matrix.load().ncols();
        let other_rows = other.matrix.load().nrows();
        let other_cols = other.matrix.load().ncols();
        if self_cols != other_cols {
            return None; // Matrices have incompatible column sizes
        }

        let stacked_rows = self_rows + other_rows;
        let mut stacked_data = Vec::with_capacity(stacked_rows * self_cols);

        // Copy self matrix
        stacked_data.extend_from_slice(self.matrix.load().as_slice().unwrap());

        // Copy other matrix
        stacked_data.extend_from_slice(other.matrix.load().as_slice().unwrap());
        let stacked_matrix =
            Array2::from_shape_vec((stacked_rows, self_cols), stacked_data).unwrap();
        Some(AttributeMatrix {
            matrix: ArcSwap::new(Arc::new(stacked_matrix)),
        })
    }

    // append matrix
    pub fn append(&self, other: &AttributeMatrix<T>) -> Option<AttributeMatrix<T>> {
        let self_rows = self.matrix.load().nrows();
        let self_cols = self.matrix.load().ncols();
        let other_rows = other.matrix.load().nrows();
        let other_cols = other.matrix.load().ncols();
        if self_rows != other_rows {
            return None; // Matrices have incompatible row sizes
        }

        let appended_cols = self_cols + other_cols;
        let mut appended_data = Vec::with_capacity(self_rows * appended_cols);

        // Copy self matrix with additional columns
        for row in 0..self_rows {
            let self_row_data = self.matrix.load().row(row).as_slice();
            appended_data.extend_from_slice(self_row_data.unwrap());
        }

        // Copy other matrix with additional columns
        for row in 0..other_rows {
            let other_row_data = other.matrix.load().row(row).as_slice();
            appended_data.extend_from_slice(other_row_data);
        }

        let appended_matrix =
            Array2::from_shape_vec((self_rows, appended_cols), appended_data).unwrap();
        Some(AttributeMatrix {
            matrix: ArcSwap::new(Arc::new(appended_matrix)),
        })
    }

    // grow matrix
    pub fn grow(&self, additional_rows: usize, additional_cols: usize) -> AttributeMatrix<T> {
        let self_rows = self.matrix.load().nrows();
        let self_cols = self.matrix.load().ncols();
        let new_rows = self_rows + additional_rows;
        let new_cols = self_cols + additional_cols;
        let mut new_data = Vec::with_capacity(new_rows * new_cols);

        // Copy self matrix
        new_data.extend_from_slice(self.matrix.load().as_slice().unwrap());

        // Add additional rows
        for _ in 0..additional_rows {
            new_data.extend(vec![(false, Default::default()); new_cols]);
        }

        let new_matrix = Array2::from_shape_vec((new_rows, new_cols), new_data).unwrap();
        AttributeMatrix {
            matrix: ArcSwap::new(Arc::new(new_matrix)),
        }
    }

    // merge matrices
    pub fn merge(&self, other: &AttributeMatrix<T>) -> Option<AttributeMatrix<T>> {
        let self_rows = self.matrix.load().nrows();
        let self_cols = self.matrix.load().ncols();
        let other_rows = other.matrix.load().nrows();
        let other_cols = other.matrix.load().ncols();
        if self_rows != other_rows {
            return None; // Matrices have incompatible row sizes
        }

        let merged_cols = self_cols + other_cols;
        let mut merged_data = Vec::with_capacity(self_rows * merged_cols);

        // Copy self matrix
        merged_data.extend_from_slice(self.matrix.load().as_slice().unwrap());

        // Copy other matrix
        merged_data.extend_from_slice(other.matrix.load().as_slice().unwrap());
        let merged_matrix = Array2::from_shape_vec((self_rows, merged_cols), merged_data).unwrap();
        Some(AttributeMatrix {
            matrix: ArcSwap::new(Arc::new(merged_matrix)),
        })
    }

    // invert matrix values
    pub fn invert(&self) -> AttributeMatrix<T> {
        let mut inverted_data = Vec::with_capacity(self.matrix.load().len());

        // Invert matrix values
        for (value, priority) in self.matrix.load().iter() {
            inverted_data.push((value.fetch_not(Ordering::Relaxed), priority.clone()));
        }
        let inverted_matrix =
            Array2::from_shape_vec(self.matrix.load().raw_dim(), inverted_data).unwrap();
        AttributeMatrix {
            matrix: ArcSwap::new(Arc::new(inverted_matrix)),
        }
    }
}
*/
