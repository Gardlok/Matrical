


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

use crate::schematics::{Matrix, Element};
use std::sync::Arc;
use dashmap::DashMap;
use crossbeam::queue::{ArrayQueue, SegQueue};
use std::marker::PhantomData;

pub trait MatrixContextTrait {
    fn is_valid(&self) -> bool;
}

pub struct MatrixContext<T> {
    data: T,
}

impl<T> MatrixContext<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

impl<T> MatrixContextTrait for MatrixContext<T> {
    fn is_valid(&self) -> bool {
        // Check if the data is valid
        self.data.is_valid()
    }
}

pub trait IsValid {
    fn is_valid(&self) -> bool;
}

impl<T> IsValid for T {
    fn is_valid(&self) -> bool {
        // Check if the value is valid
        true
    }
}

pub struct CustomValidationStrategy<T, F> {
    validator: F,
    phantom: PhantomData<T>,
}

impl<T, F> CustomValidationStrategy<T, F> {
    pub fn new(validator: F) -> Self {
        Self {
            validator,
            phantom: PhantomData,
        }
    }
}

impl<T, F> MatrixValidationStrategy<T> for CustomValidationStrategy<T, F>
where
    F: 'static + Fn(&T) -> bool,
{
    fn is_valid(&self, value: &T) -> Result<(), MatricalError> {
        // If the value is valid, return Ok
        if (self.validator)(value) {
            Ok(())
        }
        // Otherwise, return an error
        else {
            Err(MatricalError::InvalidValue)
        }
    }
}

pub struct CustomContextStrategy<T, F> {
    validator: F,
    phantom: PhantomData<T>,
}



pub trait MatrixValidationStrategy<T> {
    fn is_valid(&self, value: &T) -> Result<(), MatricalError>;
}

pub struct IsValidStrategy<T> {
    validator: Box<dyn Fn(&T) -> bool>,
}

impl<T> IsValidStrategy<T> {
    pub fn new<F>(validator: F) -> Self
    where
        F: 'static + Fn(&T) -> bool,
    {
        Self {
            validator: Box::new(validator),
        }
    }
}

impl<T> MatrixValidationStrategy<T> for IsValidStrategy<T> {
    fn is_valid(&self, value: &T) -> Result<(), MatricalError> {
        // If the value is valid, return Ok
        if (self.validator)(value) {
            Ok(())
        }
        // Otherwise, return an error
        else {
            Err(MatricalError::InvalidValue)
        }
    }
}

pub struct MatrixContextStrategy<T> {
    validator: Box<dyn Fn(&MatrixContext<T>) -> bool>,
}

impl<T> MatrixContextStrategy<T> {
    pub fn new<F>(validator: F) -> Self
    where
        F: 'static + Fn(&MatrixContext<T>) -> bool,
    {
        Self {
            validator: Box::new(validator),
        }
    }
}

impl<T> MatrixValidationStrategy<MatrixContext<T>> for MatrixContextStrategy<T> {
    fn is_valid(&self, context: &MatrixContext<T>) -> Result<(), MatricalError> {
        // If the context is valid, return Ok
        if (self.validator)(context) {
            Ok(())
        }
        // Otherwise, return an error
        else {
            Err(MatricalError::InvalidContext)
        }
    }
}

pub struct MatrixValidation<T> {
    strategies: Vec<Box<dyn MatrixValidationStrategy<T>>>,
}

impl<T> MatrixValidation<T> {
    pub fn new() -> Self {
        Self { strategies: vec![] }
    }

    pub fn add_strategy<S>(&mut self, strategy: S)
    where
        S: 'static + MatrixValidationStrategy<T>,
    {
        self.strategies.push(Box::new(strategy));
    }

    pub fn is_valid(&self, value: &T) -> Result<(), MatricalError> {
        for strategy in &self.strategies {
            let result = self.validate(&value);
            if result.is_err() {
                return result;
            }
        }
        Ok(())
    }
}

pub struct MatrixValidationBuilder<T> {
    strategies: Vec<Box<dyn MatrixValidationStrategy<T>>>,
}

impl<T> MatrixValidationBuilder<T> {
    pub fn new() -> Self {
        Self { strategies: vec![] }
    }

    pub fn add_strategy<S>(&mut self, strategy: S)
    where 
        S: 'static + MatrixValidationStrategy<T>,
    {
        self.strategies.push(Box::new(strategy));
    }

    pub fn build(&self) -> MatrixValidation<T> {
        MatrixValidation {
            strategies: self.strategies.clone(),
        }
    }
}

// impl<T> From<T> for Result<(), MatricalError> {
//     fn from(value: T) -> Self {
//         let mut validation = MatrixValidation::new();
//         validation.add_strategy(IsValidStrategy::new(IsValid::is_valid));
//         validation.add_strategy(MatrixContextStrategy::new(MatrixContextTrait::is_valid));
//         validation.is_valid(&value)
//     }
// }

// fn validate_matrix<T>(matrix: &MatrixContext<T>) -> Result<(), MatricalError> {
//     let mut builder = MatrixValidationBuilder::new();
//     builder.add_strategy(IsValidStrategy::new(IsValid::is_valid));
//     builder.add_strategy(MatrixContextStrategy::new(MatrixContextTrait::is_valid));
//     builder.add_strategy(CustomValidationStrategy::new(|matrix| {
//         // Perform custom validation logic here
//         true
//     }));
//     let validation = builder.build();
//     validation.is_valid(matrix)
// }

// impl<T> From<T> for Result<(), MatricalError> {
//     fn from(value: T) -> Self {
//         let mut validation = MatrixValidation::new();
//         validation.add_strategy(IsValidStrategy::new(IsValid::is_valid));
//         validation.add_strategy(MatrixContextStrategy::new(MatrixContextTrait::is_valid));
//         validation.add_strategy(CustomContextStrategy::new(MatrixContextTrait::is_valid));
//         validation.is_valid(&value)
//     }
// }

// impl<T> From<T> for Result<(), MatricalError> {
//     fn from(value: T) -> Self {
//         let mut validation = MatrixValidation::new();
//         validation.add_strategy(IsValidStrategy::new(IsValid::is_valid));
//         validation.add_strategy(MatrixContextStrategy::new(MatrixContextTrait::is_valid));
//         validation.add_strategy(CustomValidationStrategy::new(|matrix| {
//             // Perform custom validation logic here
//             true
//         }));
//         validation.is_valid(&value)
//     }
// }




// impl<T> From<T> for Result<(), MatricalError> {
//     fn from(value: T) -> Self {
//         let mut validation = MatrixValidation::new();
//         validation.add_strategy(IsValidStrategy::new(IsValid::is_valid));
//         validation.add_strategy(MatrixContextStrategy::new(MatrixContextTrait::is_valid));
//         validation.add_strategy(CustomContextStrategy::new(MatrixContextTrait::is_valid));
//         validation.is_valid(&value)
//     }
// }







///////////////////////////////




    // Sizing
    // fn execute(&self, Lens: &mut Lens<V>) -> Result<(), MatricalError> {
    //     // Check if the coordinates are within the Lens dimensions
    //     if self.top_left.0 >= Lens.data.shape()[0]
    //         || self.top_left.1 >= Lens.data.shape()[1]
    //         || self.bottom_right.0 >= Lens.data.shape()[0]
    //         || self.bottom_right.1 >= Lens.data.shape()[1]
    //     {
    //         return Err(MatricalError::Custom(String::from("Coordinates are out of bounds")));
    //     }

    //     // Create a mutable sub-Lens lens
    //     // The lens is a mutable reference to the original Lens's data
    //     // It represents a lens into the sub-Lens
    //     let lens = Lens.data.slice_mut(s![self.top_left.0..=self.bottom_right.0, self.top_left.1..=self.bottom_right.1]);

    //     // Iterate over the lens and modify the values
    //     for i in self.top_left.0..=self.bottom_right.0 {
    //         for j in self.top_left.1..=self.bottom_right.1 {
    //             let value = &mut lens[[i - self.top_left.0, j - self.top_left.1]];
    //             // Modify the value as desired
    //             *value += 1.0;
    //         }
    //     }

    //     Ok(())
    // }






/*
// Add the Lens Strategy instances to the DependencyInjectionContainer
fn add_lens_strategies(container: &mut DependencyInjectionContainer) {
    // Create an instance of StructureToMatrixLens and add it to the container
    let structure_to_matrix_lens = Arc::new(StructureToMatrixLens::new());
    container.add_strategy(Box::new(structure_to_matrix_lens));
}

// Initialize the DependencyInjectionContainer and add the lens strategies
fn initialize_di_container() -> DependencyInjectionContainer {
    let mut container = DependencyInjectionContainer::new();
    add_lens_strategies(&mut container);
    container
}
*/













 
 #[cfg(test)]
 mod tests {
     use super::*;
     use crossbeam::queue::SegQueue;
 
    //  #[test]
    // //  fn test_upper_triangular_lens() {
    // //      // Create a matrix
    // //      let matrix = Matrix::with_elements(SegQueue::from(vec![
    // //          Element::new(1.0, (0, 0)),
    // //          Element::new(2.0, (0, 1)),
    // //          Element::new(3.0, (0, 2)),
    // //          Element::new(4.0, (1, 0)),
    // //          Element::new(5.0, (1, 1)),
    // //          Element::new(6.0, (1, 2)),
    // //          Element::new(7.0, (2, 0)),
    // //          Element::new(8.0, (2, 1)),
    // //          Element::new(9.0, (2, 2)),
    // //      ]));
 
    //      // Apply the upper triangular lens
    //      let lens = UpperTriangularLens::new();
    //      let view = lens.apply(&matrix);
 
    //      // Check the dimensions of the view
    //      assert_eq!(view.shape(), (3, 3));
 
    //      // Check the values of the view
    //      assert_eq!(view[(0, 0)], 1);
    //      assert_eq!(view[(0, 1)], 2);
    //      assert_eq!(view[(0, 2)], 3);
    //      assert_eq!(view[(1, 0)], 0);
    //      assert_eq!(view[(1, 1)], 5);
    //      assert_eq!(view[(1, 2)], 6);
    //      assert_eq!(view[(2, 0)], 0);
    //      assert_eq!(view[(2, 1)], 0);
    //      assert_eq!(view[(2, 2)], 9);
    //  }
 
    //  #[test]
    //  fn test_row_lens() {
    //      // Create a matrix
    //      let matrix = Matrix::new(Array2::from_shape_vec((3, 3), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap());
 
    //      // Apply the row lens
    //      let lens = RowLens::new(1);
    //      let view = lens.apply(&matrix);
 
    //      // Check the dimensions of the view
    //      assert_eq!(view.shape(), (1, 3));
 
    //      // Check the values of the view
    //      assert_eq!(view[(0, 0)], 4);
    //      assert_eq!(view[(0, 1)], 5);
    //      assert_eq!(view[(0, 2)], 6);
    //  }
 
     // Additional tests for other lens types...
 
 }