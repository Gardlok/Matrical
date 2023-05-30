


/*
RowLens: This lens would focus on a specific row of the matrix. It would allow operations to be performed on that row without affecting the rest of the matrix.

ColumnLens: Similar to the RowLens, this lens would focus on a specific column of the matrix.

SubmatrixLens: This lens would focus on a specific submatrix within the matrix. The submatrix could be defined by specifying the top left and bottom right coordinates.

DiagonalLens: This lens would focus on the diagonal elements of the matrix. This could be usefulcog for certain types of operations, such as calculating the trace of the matrix.

BandLens: This lens would focus on a band of elements around the diagonal of the matrix. The width of the band could be specified by the user.

UpperTriangularLens: This lens would focus on the upper triangular part of the matrix (i.e., all elements above the main diagonal).

LowerTriangularLens: This lens would focus on the lower triangular part of the matrix (i.e., all elements below the main diagonal).

SparseLens: This lens would focus on the non-zero elements of the matrix. This could be useful for operations on sparse matrices, where most of the elements are zero.
 */

use crate::error::MatricalError;
use ndarray::{Array2, s};
use crate::{Matrix, MatrixStrategy};
 
 // The LensOperation struct
 
 #[derive(Debug, Clone)]
 pub struct LensOperation {
     // The top left and bottom right coordinates of the sub-matrix
     top_left: (usize, usize),
     bottom_right: (usize, usize),
 }
 
 impl LensOperation {
     // Create a new LensOperation with the given coordinates
     pub fn new(top_left: (usize, usize), bottom_right: (usize, usize)) -> Self {
         Self {
             top_left,
             bottom_right,
         }
     }
 }
 
 impl MatrixStrategy for LensOperation {
     fn execute(&self, matrix: &mut Matrix) -> Result<(), MatricalError> {
         // Check if the coordinates are within the matrix dimensions
         if self.top_left.0 >= matrix.data.shape()[0]
             || self.top_left.1 >= matrix.data.shape()[1]
             || self.bottom_right.0 >= matrix.data.shape()[0]
             || self.bottom_right.1 >= matrix.data.shape()[1]
         {
             return Err(MatricalError::Custom(String::from("Coordinates are out of bounds")));
         }
 
         // Create a mutable sub-matrix lens
         // The lens is a mutable reference to the original matrix's data
         // It represents a lens into the sub-matrix
         let lens = matrix.data.slice_mut(s![self.top_left.0..=self.bottom_right.0, self.top_left.1..=self.bottom_right.1]);
 
         // Iterate over the lens and modify the values
         for i in self.top_left.0..=self.bottom_right.0 {
             for j in self.top_left.1..=self.bottom_right.1 {
                 let value = &mut lens[[i - self.top_left.0, j - self.top_left.1]];
                 // Modify the value as desired
                 *value += 1.0;
             }
         }
 
         Ok(())
     }
 }
 



// The Lens trait
pub trait Lens {
    fn execute(&self, matrix: &mut Matrix) -> Result<(), MatricalError>;
}

// The UpperTriangularLens struct
pub struct UpperTriangularLens;

impl Lens for UpperTriangularLens {
    fn execute(&self, matrix: &mut Matrix) -> Result<(), MatricalError> {
        let (rows, cols) = matrix.data.dim();

        // Iterate over the lower triangular part of the matrix
        for i in 0..rows {
            for j in 0..i {
                matrix.data[[i, j]] = 0.0;
            }
        }

        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::Array2;

    #[test]
    fn test_upper_triangular_lens() {
        // Create a matrix
        let matrix = Matrix::new(Array2::from_shape_vec((3, 3), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap());

        // Apply the upper triangular lens
        let lens = UpperTriangularLens::new();
        let view = lens.apply(&matrix);

        // Check the dimensions of the view
        assert_eq!(view.shape(), (3, 3));

        // Check the values of the view
        assert_eq!(view[(0, 0)], 1);
        assert_eq!(view[(0, 1)], 2);
        assert_eq!(view[(0, 2)], 3);
        assert_eq!(view[(1, 0)], 0);
        assert_eq!(view[(1, 1)], 5);
        assert_eq!(view[(1, 2)], 6);
        assert_eq!(view[(2, 0)], 0);
        assert_eq!(view[(2, 1)], 0);
        assert_eq!(view[(2, 2)], 9);
    }

    #[test]
    fn test_row_lens() {
        // Create a matrix
        let matrix = Matrix::new(Array2::from_shape_vec((3, 3), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap());

        // Apply the row lens
        let lens = RowLens::new(1);
        let view = lens.apply(&matrix);

        // Check the dimensions of the view
        assert_eq!(view.shape(), (1, 3));

        // Check the values of the view
        assert_eq!(view[(0, 0)], 4);
        assert_eq!(view[(0, 1)], 5);
        assert_eq!(view[(0, 2)], 6);
    }

    // Additional tests for other lens types...

}