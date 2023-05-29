



// Attribute Operation
pub trait AttributeOperation {
    fn execute(&self, context: &AttributeContext) -> Result<(), MatricalError>;
}

// Attribute Strategy
pub trait AttributeStrategy {
    fn execute(
        &self,
        attribute: &Attribute,
        index: Option<(usize, usize)>,
        other: Option<bool>,
    ) -> Result<(), MatricalError>;
}

// The AttributeOperation struct
pub struct AttributeOperationImpl {
    // The top left and bottom right coordinates of the sub-matrix
    top_left: (usize, usize),
    bottom_right: (usize, usize),
}

impl AttributeOperationImpl {
    // Create a new AttributeOperation with the given coordinates
    pub fn new(top_left: (usize, usize), bottom_right: (usize, usize)) -> Self {
        Self {
            top_left,
            bottom_right,
        }
    }
}

impl AttributeOperation for AttributeOperationImpl {
    fn execute(&self, context: &AttributeContext) -> Result<(), MatricalError> {
        unimplemented!()
    }
}

impl AttributeStrategy for AttributeOperationImpl {
    fn execute(
        &self,
        attribute: &Attribute,
        _index: Option<(usize, usize)>,
        _other: Option<bool>,
    ) -> Result<(), MatricalError> {
        // Check if the coordinates are within the matrix dimensions
        if self.top_left.0 >= attribute.data.dim().0
            || self.top_left.1 >= attribute.data.dim().1
            || self.bottom_right.0 >= attribute.data.dim().0
            || self.bottom_right.1 >= attribute.data.dim().1
        {
            return Err(MatricalError::IndexOutOfBounds);
        }

        Ok(())
    }
}

// The AttributeStrategy struct
pub struct AttributeStrategyImpl {
    // The top left and bottom right coordinates of the sub-matrix
    top_left: (usize, usize),
    bottom_right: (usize, usize),
}

impl AttributeStrategyImpl {
    // Create a new AttributeStrategy with the given coordinates
    pub fn new(top_left: (usize, usize), bottom_right: (usize, usize)) -> Self {
        Self {
            top_left,
            bottom_right,
        }
    }
}

impl AttributeStrategy for AttributeStrategyImpl {
    fn execute(
        &self,
        attribute: &Attribute,
        _index: Option<(usize, usize)>,
        _other: Option<bool>,
    ) -> Result<(), MatricalError> {
        // Check if the coordinates are within the matrix dimensions
        if self.top_left.0 >= attribute.data.dim().0
            || self.top_left.1 >= attribute.data.dim().1
            || self.bottom_right.0 >= attribute.data.dim().0
            || self.bottom_right.1 >= attribute.data.dim().1
        {
            return Err(MatricalError::IndexOutOfBounds);
        }

        Ok(())
    }
}

// The AttributeContext struct
pub struct AttributeContext {
    // The top left and bottom right coordinates of the sub-matrix
    top_left: (usize, usize),
    bottom_right: (usize, usize),
}

impl AttributeContext {
    // Create a new AttributeContext with the given coordinates
    pub fn new(top_left: (usize, usize), bottom_right: (usize, usize)) -> Self {
        Self {
            top_left,
            bottom_right,
        }
    }
}

// The Attribute struct
pub struct Attribute {
    // The top left and bottom right coordinates of the sub-matrix
    top_left: (usize, usize),
    bottom_right: (usize, usize),
    // The data contained in the attribute
    data: Array2<f64>,
}

impl Attribute {
    // Create a new Attribute with the given coordinates and data
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

// The AttributeBuilder struct
pub struct AttributeBuilder {
    // The top left and bottom right coordinates of the sub-matrix
    top_left: (usize, usize),
    bottom_right: (usize, usize),
    // The data contained in the attribute
    data: Array2<f64>,
}

impl AttributeBuilder {
    // Create a new AttributeBuilder with the given coordinates
    pub fn new(top_left: (usize, usize), bottom_right: (usize, usize)) -> Self {
        Self {
            top_left,
            bottom_right,
            data: Array2::zeros((0, 0)),
        }
    }

    // Set the data of the AttributeBuilder
    pub fn data(mut self, data: Array2<f64>) -> Self {
        self.data = data;
        self
    }

    // Build the AttributeBuilder into an Attribute
    pub fn build(self) -> Attribute {
        Attribute::new(self.top_left, self.bottom_right, self.data)
    }
}

// The AttributeBuilder struct
pub struct AttributeBuilderImpl {
    // The top left and bottom right coordinates of the sub-matrix
    top_left: (usize, usize),
    bottom_right: (usize, usize),
    // The data contained in the attribute
    data: Array2<f64>,
}

impl AttributeBuilderImpl {
    // Create a new AttributeBuilder with the given coordinates
    pub fn new(top_left: (usize, usize), bottom_right: (usize, usize)) -> Self {
        Self {
            top_left,
            bottom_right,
            data: Array2::zeros((0, 0)),
        }
    }

    // Set the data of the AttributeBuilder
    pub fn data(mut self, data: Array2<f64>) -> Self {
        self.data = data;
        self
    }

    // Build the AttributeBuilder into an Attribute
    pub fn build(self) -> Attribute {
        Attribute::new(self.top_left, self.bottom_right, self.data)
    }
}

