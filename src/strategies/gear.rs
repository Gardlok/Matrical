

use crate::schematics::Element;
use crate::error::MatricalError;
use ndarray::{Array2, s};




pub fn execute_functor<T, F>(element: &mut Element<T>, functor: F)
where
    F: Fn(&mut Element<T>) + Send + Sync + Clone + Eq + 'static,
    T: Send + Sync + Clone + Default + PartialEq + Eq + 'static,
    Element<T>: Send + Sync,
{

    functor(element);
}






// The Gear struct
#[derive(Debug, Clone)]
pub struct Gear {
    // The top left and bottom right coordinates of the sub-matrix
    top_left: (usize, usize),
    bottom_right: (usize, usize),
    // The data contained in the gear
    data: Array2<f64>,
}

impl Gear {
    // Create a new Gear with the given coordinates and data
    pub fn new(
        top_left: (usize, usize),
        bottom_right: (usize, usize),
        data: Array2<f64>,
    ) -> Self {
        Self {
            top_left,
            bottom_right,
            data,
        }
    }
}

// Gear Operation
pub trait GearOperation {
    fn apply(&self, gear: &mut Gear, context: &GearContext) -> Result<(), MatricalError>;
}

// Gear Strategy
pub trait GearStrategy {
    fn execute(
        &self,
        gear: &Gear,
        index: Option<(usize, usize)>,
        other: Option<bool>,
    ) -> Result<(), MatricalError>;
}

// The GearContext struct
pub struct GearContext {
    // The top left and bottom right coordinates of the sub-matrix
    top_left: (usize, usize),
    bottom_right: (usize, usize),
}

impl GearContext {
    // Create a new GearContext with the given coordinates
    pub fn new(top_left: (usize, usize), bottom_right: (usize, usize)) -> Self {
        Self {
            top_left,
            bottom_right,
        }
    }
}

// The GearFactory trait
pub trait GearFactory {
    fn create(&self, top_left: (usize, usize), bottom_right: (usize, usize)) -> Gear;
}

// The GearFactoryImpl struct
pub struct GearFactoryImpl {
    // The top left and bottom right coordinates of the sub-matrix
    top_left: (usize, usize),
    bottom_right: (usize, usize),
    // The data contained in the gear
    data: Array2<f64>,
}

impl GearFactoryImpl {
    // Create a new GearFactoryImpl with the given coordinates
    pub fn new(top_left: (usize, usize), bottom_right: (usize, usize)) -> Self {
        Self {
            top_left,
            bottom_right,
            data: Array2::zeros((0, 0)),
        }
    }

    // Set the data of the GearFactoryImpl
    pub fn data(mut self, data: Array2<f64>) -> Self {
        self.data = data;
        self
    }
}

impl GearFactory for GearFactoryImpl {
    fn create(&self, top_left: (usize, usize), bottom_right: (usize, usize)) -> Gear {
        Gear::new(top_left, bottom_right, self.data.clone())
    }
}

// The GearOperationImpl struct
pub struct GearOperationImpl {
    // The constant to add to the sub-matrix
    constant: f64,
}

impl GearOperationImpl {
    // Create a new GearOperation with the given constant
    pub fn new(constant: f64) -> Self {
        Self { constant }
    }
}

impl GearOperation for GearOperationImpl {
    fn apply(&self, gear: &mut Gear, context: &GearContext) -> Result<(), MatricalError> {
        // Check if the context is within the gear dimensions
        if context.top_left.0 >= gear.data.dim().0
            || context.top_left.1 >= gear.data.dim().1
            || context.bottom_right.0 >= gear.data.dim().0
            || context.bottom_right.1 >= gear.data.dim().1
        {
            return Err(MatricalError::IndexOutOfBounds);
        }

        let (start_row, start_col) = context.top_left;
        let (end_row, end_col) = context.bottom_right;

        let data = &mut gear.data.slice_mut(s![start_row..=end_row, start_col..=end_col]);
        *data += self.constant;

        Ok(())
    }
}

// The GearStrategyImpl struct
pub struct GearStrategyImpl {
    // The top left and bottom right coordinates of the sub-matrix
    top_left: (usize, usize),
    bottom_right: (usize, usize),
}

impl GearStrategyImpl {
    // Create a new GearStrategyImpl with the given coordinates
    pub fn new(top_left: (usize, usize), bottom_right: (usize, usize)) -> Self {
        Self {
            top_left,
            bottom_right,
        }
    }
}

impl GearStrategy for GearStrategyImpl {
    fn execute(
        &self,
        gear: &Gear,
        index: Option<(usize, usize)>,
        other: Option<bool>,
    ) -> Result<(), MatricalError> {
        // Check if the coordinates are within the matrix dimensions
        if self.top_left.0 >= gear.data.dim().0
            || self.top_left.1 >= gear.data.dim().1
            || self.bottom_right.0 >= gear.data.dim().0
            || self.bottom_right.1 >= gear.data.dim().1
        {
            return Err(MatricalError::IndexOutOfBounds);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gear_operation_apply() {
        // Create a new gear
        let gear_data = Array2::zeros((5, 5));
        let mut gear = Gear::new((0, 0), (2, 2), gear_data);

        // Create a new gear operation
        let operation = GearOperationImpl::new(1.0);

        // Create a new gear context
        let context = GearContext::new((0, 0), (2, 2));

        // Apply the operation to the gear
        let result = operation.apply(&mut gear, &context);

        assert!(result.is_ok());

        // Check the updated gear data
        let expected_data = Array2::from_shape_vec(
            (5, 5),
            vec![
                1.0, 1.0, 1.0, 0.0, 0.0, //
                1.0, 1.0, 1.0, 0.0, 0.0, //
                1.0, 1.0, 1.0, 0.0, 0.0, //
                0.0, 0.0, 0.0, 0.0, 0.0, //
                0.0, 0.0, 0.0, 0.0, 0.0,
            ],
        )
        .unwrap();

        assert_eq!(gear.data, expected_data);
    }

    #[test]
    fn test_basics() {
        use super::*;
        
        // Create a new gear
        let mut gear = Gear::new((0, 0), (2, 2), Array2::zeros((3, 3)));
    
        // Create a new gear operation
        let operation = GearOperationImpl::new(1.0);
    
        // Create a new gear context
        let context = GearContext::new((0, 0), (2, 2));
    
        // Apply the operation to the gear
        match operation.apply(&mut gear, &context) {
            Ok(()) => println!("Operation applied successfully"),
            Err(err) => println!("Failed to apply operation: {:?}", err),
        }
    
        // Print the gear data
        println!("{:?}", gear.data);
    }

    #[test]
    fn test_gear_factory() {
        let top_left = (0, 0);
        let bottom_right = (2, 2);
        let data = Array2::zeros((3, 3));

        let gear_factory = GearFactoryImpl::new(top_left, bottom_right).data(data.clone());
        let gear = gear_factory.create(top_left, bottom_right);

        assert_eq!(gear.top_left, top_left);
        assert_eq!(gear.bottom_right, bottom_right);
        assert_eq!(gear.data, data);
    }

    #[test]
    fn test_gear_operation() {
        let top_left = (0, 0);
        let bottom_right = (2, 2);
        let data = Array2::zeros((3, 3));

        let mut gear = Gear::new(top_left, bottom_right, data);
        let operation = GearOperationImpl::new(1.0);
        let context = GearContext::new(top_left, bottom_right);

        operation.apply(&mut gear, &context).unwrap();

        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(gear.data[[i, j]], 1.0);
            }
        }
    }

    #[test]
    fn test_gear_strategy() {
        let top_left = (0, 0);
        let bottom_right = (2, 2);
        let data = Array2::zeros((3, 3));

        let gear = Gear::new(top_left, bottom_right, data);
        let strategy = GearStrategyImpl::new(top_left, bottom_right);

        strategy.execute(&gear, None, None).unwrap();
    }
}