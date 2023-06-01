
use crate::error::MatricalError;
use ndarray::Array2;
use std::marker::PhantomData;
use std::sync::Arc;
use std::any::Any;




// The Cog struct
pub struct Cog {
    lens: Option<Arc<dyn Fn()>>,
    operation: Option<Box<dyn CogOperation>>,
    strategy: Option<Box<dyn CogStrategy>>,
    context: Option<Box<CogContext>>,
}

impl Cog {
    pub fn new(
        lens: Option<Arc<dyn Fn()>>,
        operation: Option<Box<dyn CogOperation>>,
        strategy: Option<Box<dyn CogStrategy>>,
        context: Option<Box<CogContext>>,
    ) -> Self {
        Self {
            lens,
            operation,
            strategy,
            context,
        }
    }
}

// Cog Operation
pub trait CogOperation: Send + Sync {
    fn apply(&self, context: &CogContext) -> Result<(), MatricalError>;
}

pub struct CogOperationImpl {
    top_left: (usize, usize),
    bottom_right: (usize, usize),
}

impl CogOperationImpl {
    pub fn new(top_left: (usize, usize), bottom_right: (usize, usize)) -> Self {
        Self {
            top_left,
            bottom_right,
        }
    }
}

impl CogOperation for CogOperationImpl {
    fn apply(&self, context: &CogContext) -> Result<(), MatricalError> {
        // Check if the coordinates are within the matrix dimensions
        if self.top_left.0 >= context.data.dim().0
            || self.top_left.1 >= context.data.dim().1
            || self.bottom_right.0 >= context.data.dim().0
            || self.bottom_right.1 >= context.data.dim().1
        {
            return Err(MatricalError::ShouldNotOccur); // TODO: Change this to IndexOutOfBounds
        }

        // Apply the cog operation logic here
        // ...

        Ok(())
    }
}

// Cog Strategy
pub trait CogStrategy: Send + Sync {
    fn execute(
        &self,
        cog: &Cog,
        index: Option<(usize, usize)>,
        other: Option<bool>,
    ) -> Result<(), MatricalError>;
}

pub struct CogStrategyImpl {
    top_left: (usize, usize),
    bottom_right: (usize, usize),
}

impl CogStrategyImpl {
    pub fn new(top_left: (usize, usize), bottom_right: (usize, usize)) -> Self {
        Self {
            top_left,
            bottom_right,
        }
    }
}

impl CogStrategy for CogStrategyImpl {
    fn execute(
        &self,
        cog: &Cog,
        index: Option<(usize, usize)>,
        other: Option<bool>,
    ) -> Result<(), MatricalError> {
        // Check if the coordinates are within the matrix dimensions
        if self.top_left.0 >= cog.context.as_ref().unwrap().data.dim().0
            || self.top_left.1 >= cog.context.as_ref().unwrap().data.dim().1
            || self.bottom_right.0 >= cog.context.as_ref().unwrap().data.dim().0
            || self.bottom_right.1 >= cog.context.as_ref().unwrap().data.dim().1
        {
            return Err(MatricalError::ShouldNotOccur);  // TODO: Change this to IndexOutOfBounds
        }

        // Execute the cog strategy logic here
        // ...

        Ok(())
    }
}

// Cog Context
pub struct CogContext {
    top_left: (usize, usize),
    bottom_right: (usize, usize),
    data: Array2<f64>,
}

impl CogContext {
    pub fn new(top_left: (usize, usize), bottom_right: (usize, usize), data: Array2<f64>) -> Self {
        Self {
            top_left,
            bottom_right,
            data,
        }
    }
}

// Cog Builder
pub struct CogBuilder {
    top_left: (usize, usize),
    bottom_right: (usize, usize),
    data: Option<Array2<f64>>,
}

impl CogBuilder {
    pub fn new(top_left: (usize, usize), bottom_right: (usize, usize)) -> Self {
        Self {
            top_left,
            bottom_right,
            data: None,
        }
    }

    pub fn data(mut self, data: Array2<f64>) -> Self {
        self.data = Some(data);
        self
    }

    pub fn build(self) -> Cog {
        let context = self
            .data
            .map(|data| Box::new(CogContext::new(self.top_left, self.bottom_right, data)) as Box<CogContext>);

        Cog::new(
            None,
            Some(Box::new(CogOperationImpl::new(self.top_left, self.bottom_right))),
            Some(Box::new(CogStrategyImpl::new(self.top_left, self.bottom_right))),
            context,
        )
    }
}

fn main() {
    let builder = CogBuilder::new((0, 0), (1, 1)).data(Array2::ones((2, 2)));
    let cog = builder.build();
    let context = cog.context.unwrap();

    let operation = cog.operation.unwrap();
    operation.apply(&context).unwrap();

    // let strategy = cog.strategy.unwrap().clone();
    // strategy.execute(cog, None, None).unwrap();
}

// The Cog struct
// 
/*
 * An Cog, in Matrical, is a piece of metadata that can be associated with a matrix or a submatrix.
 * It can store any kind of information, such as tags, labels, or significant data about the matrix.
 * Cogs can be used to provide additional context for operations performed on the matrix.
 * When a Gear operates on a matrix, it can use the information from any associated Cogs to
 * modify its operation. This allows for operations to be tailored based on the specific Cogs
 * of matrix elements. Cogs can be applied to the entire matrix or to specific submatrices.
 */
// pub struct Cog { _attri: PhantomData<Arc<dyn Fn()>> }


// // The Cog (Attribute) struct
// #[derive(Debug, Clone)]
// pub struct Cog {

//     // For when a lens is required
//     lens: Option<Arc<dyn Fn()>>,

//     // The CogOperation to be applied to the Cog
//     operation: Option<Box<dyn CogOperation>>,

//     // The CogStrategy to be applied to the Cog
//     strategy: Option<Box<dyn Any + Send + Sync + CogStrategy>>,

//     // The CogContext to be applied to the Cog
//     context: Option<Box<CogContext>>,
   
// }

// impl Cog {
//     // Create a new Cog with the given coordinates and data
//     pub fn new(
//         lens: Option<Arc<dyn Fn()>>,
//         operation: Option<Box<dyn CogOperation>>,
//         strategy: Option<Box<dyn CogStrategy>>,
//         context: Option<Box<CogContext>>,
//     ) -> Self {
//         Self {
//             lens,
//             operation,
//             strategy,
//             context,
//         }
//     }
// }
        
// // Cog Operation
// // 
// pub trait CogOperation {
//     // Apply the CogOperation to the given Cog
//     fn apply(&self, context: &CogContext) -> Result<(), MatricalError>;
// }

// // Cog Strategy
// //
// // The CogStrategy trait is used to provide context for CogOperations.
// // Passing along the CogStrategy to the CogOperation allows the CogOperation
// // to access the CogStrategy's context and perform operations on the Cog.
// //  
// pub trait CogStrategy {
//     // Execute the CogStrategy on the given Cog
//     fn execute(
//         &self,
//         cog: &Cog,
//         // the index of the element in the matrix
//         index: Option<(usize, usize)>,
//         other: Option<bool>, // Placeholder TODO: What is this?
//     ) -> Result<(), MatricalError>;
// }
// // The CogContext struct
// //
// // The CogContext struct is used to provide Metadata for CogOperations.
// // It contains the dimensions coordinates of the sub-matrix
// // where the Cog is applied.
// //
// #[derive(Debug, Clone)]
// pub struct CogContext {
//     // The top left and bottom right coordinates of the sub-matrix
//     // where the Cog is applied
//     top_left: (usize, usize),
//     bottom_right: (usize, usize),
//     // The data contained in the Cog
//     data: Option<Array2<f64>>,
//     // The lens of the Cog if it exists
//     lens: Option<Arc<dyn Fn()>>,
    
// }

// impl CogContext {
//     // Create a new CogContext with the given coordinates
//     pub fn new(top_left: (usize, usize), bottom_right: (usize, usize)) -> Self {
//         Self {
//             top_left,
//             bottom_right,
//             lens: None,
//             data: None,
//         }
//     }
// }

// // The CogOperation Implementation struct 
// // 
// #[derive(Debug, Clone)]
// pub struct CogOperationImpl {
//     // The top left and bottom right coordinates of the sub-matrix
//     top_left: (usize, usize),
//     bottom_right: (usize, usize),
// }
// impl CogOperationImpl {
//     // Create a new CogOperation with the given coordinates
//     pub fn new(top_left: (usize, usize), bottom_right: (usize, usize)) -> Self {
//         Self {
//             top_left,
//             bottom_right,
//         }
//     }
// }

// impl CogOperation for CogOperationImpl {
//     fn apply(&self, context: &CogContext) -> Result<(), MatricalError> {
//         // Check if the coordinates are within the matrix dimensions
//         // TODO: Check if the coordinates are within the matrix dimensions
//         //
//         if self.top_left.0 >= context.top_left.0
//             || self.top_left.1 >= context.top_left.1
//             || self.bottom_right.0 >= context.bottom_right.0
//             || self.bottom_right.1 >= context.bottom_right.1
//         {
//             return Err(MatricalError::IndexOutOfBounds);
//         }

//         Ok(())
//     }
// }

// impl CogStrategy for CogOperationImpl {
//     fn execute(
//         &self,
//         cog: &Cog,
//         // the index of the element in the matrix
//         _index: Option<(usize, usize)>,
//         _other: Option<bool>, // Placeholder TODO: What is this?
//     ) -> Result<(), MatricalError> {
//         // Check if the coordinates are within the matrix dimensions
//         if self.top_left.0 >= cog.context.as_ref().unwrap().data.as_ref().unwrap().dim().0
//             || self.top_left.1 >= cog.context.as_ref().unwrap().data.as_ref().unwrap().dim().1
//             || self.bottom_right.0 >= cog.context.as_ref().unwrap().data.as_ref().unwrap().dim().0
//             || self.bottom_right.1 >= cog.context.as_ref().unwrap().data.as_ref().unwrap().dim().1
//         {
//             return Err(MatricalError::IndexOutOfBounds);
//         }

//         Ok(())
//     }
// }


// // The CogStrategy struct
// #[derive(Debug, Clone)]
// pub struct CogStrategyImpl {
//     // The top left and bottom right coordinates of the sub-matrix
//     top_left: (usize, usize),
//     bottom_right: (usize, usize),
// }

// impl CogStrategyImpl {
//     // Create a new CogStrategy with the given coordinates
//     pub fn new(top_left: (usize, usize), bottom_right: (usize, usize)) -> Self {
//         Self {
//             top_left,
//             bottom_right,
//         }
//     }
// }

// impl CogStrategy for CogStrategyImpl {
//     fn execute(
//         &self,
//         cog: &Cog,
//         _index: Option<(usize, usize)>,
//         _other: Option<bool>, // Placeholder TODO: What is this?
//     ) -> Result<(), MatricalError> {
//         // Check if the coordinates are within the matrix dimensions
//         if self.top_left.0 >= cog.context.as_ref().unwrap().data.as_ref().unwrap().dim().0
//             || self.top_left.1 >= cog.context.as_ref().unwrap().data.as_ref().unwrap().dim().1
//             || self.bottom_right.0 >= cog.context.as_ref().unwrap().data.as_ref().unwrap().dim().0
//             || self.bottom_right.1 >= cog.context.as_ref().unwrap().data.as_ref().unwrap().dim().1
//         {
//             return Err(MatricalError::IndexOutOfBounds);
//         }

//         Ok(())
//     }
// }



// // The CogBuilder struct
// #[derive(Debug, Clone)]
// pub struct CogBuilder {
//     // The top left and bottom right coordinates of the sub-matrix
//     // where the Cog is applied 
//     // TODO: Should this be a reference to the Cog* struct?
//     top_left: (usize, usize),
//     bottom_right: (usize, usize),
//     // The data contained in the Cog 
//     // TODO: Should this be a reference to the Cog* struct?
//     data: Array2<f64>,
// }

// impl CogBuilder {
//     // Create a new CogBuilder with the given coordinates
//     pub fn new(top_left: (usize, usize), bottom_right: (usize, usize)) -> Self {
//         Self {
//             top_left,
//             bottom_right,
//             data: Array2::zeros((0, 0)),
//         }
//     }

//     // Set the data of the CogBuilder
//     // TODO: Should this be a reference to the Cog* struct?
//     pub fn data(mut self, data: Array2<f64>) -> Self {
//         self.data = data;
//         self
//     }

//     // Build the Cog from the CogBuilder 
//     // TODO: Should this be a reference to the Cog* struct?
//     pub fn build(self) -> Cog {
//         Cog::new(
//             None,
//             Some(Box::new(CogOperationImpl::new(
//                 self.top_left,
//                 self.bottom_right,
//             ))),
//             Some(Box::new(CogStrategyImpl::new(
//                 self.top_left,
//                 self.bottom_right,
//             ))),
//             Some(Box::new(CogContext::new(
//                 self.top_left,
//                 self.bottom_right,
//             ))),
//         )
//     }
// }
