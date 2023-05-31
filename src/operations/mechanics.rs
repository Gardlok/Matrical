
use crate::*;



// pub trait MatrixContext {
//     fn is_valid(&self) -> bool;
// }

pub trait IsValid {
    fn is_valid(&self) -> bool;
}

impl<T> IsValid for T {
    fn is_valid(&self) -> bool {
        // Check if the value is valid
        true
    }
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
        if self.validator(value) {
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
        F: 'static + Fn(&MatrixContext) -> bool,
    {
        Self {
            validator: Box::new(validator),
        }
    }
}

impl<T> MatrixValidationStrategy<&MatrixContext> for MatrixContextStrategy<T>{
    fn is_valid(&self, context: &MatrixContext) -> Result<(), MatricalError> {
        // If the context is valid, return Ok
        if self.validator(context) {
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
            let result = self.validate_strategy(strategy, value);
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
//         validation.add_strategy(MatrixContextStrategy::new(MatrixContext::is_valid));
//         validation.is_valid(&value)
//     }
// }

fn validate_matrix(matrix: &MatrixContext) -> Result<(), MatricalError> {
    let mut builder = MatrixValidationBuilder::new();
    builder.add_strategy(IsValidStrategy::new(IsValid::is_valid));
    builder.add_strategy(MatrixContextStrategy::new(MatrixContext::is_valid));
    builder.add_strategy(CustomValidationStrategy::new(|matrix| {
        // Perform custom validation logic here
        true
    }));
    let validation = builder.build();
    validation.is_valid(&matrix)
}

// 1. Create a MatrixValidationBuilder instance: This creates an instance of the MatrixValidationBuilder, which is used to build a MatrixValidation instance.
// 2. Add strategies to the builder: This adds the strategies that will be used to validate the matrix. In this example, we are adding an IsValidStrategy, a MatrixContextStrategy, and a CustomValidationStrategy.
// 3. Build the MatrixValidation instance: This builds the MatrixValidation instance using the strategies that were added to the builder.
// 4. Validate the matrix: This uses the MatrixValidation instance to validate the matrix. If any of the strategies return an error, then the validation will fail.