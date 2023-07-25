



pub trait SqlContextTrait {
    fn is_valid(&self) -> bool;
}

pub struct SqlContext<T> {
    data: T,
}

impl<T> SqlContext<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

impl<T> SqlContextTrait for SqlContext<T> {
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

pub enum SqlError {
    InvalidValue,
    InvalidContext,
}

pub trait SqlValidationStrategy<T> {
    fn is_valid(&self, value: &T) -> Result<String, SqlError>;
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

impl<T> SqlValidationStrategy<T> for IsValidStrategy<T> {
    fn is_valid(&self, value: &T) -> Result<String, SqlError> {
        // If the value is valid, return Ok
        if (self.validator)(value) {
            Ok(String::from(""))
        }
        // Otherwise, return an error
        else {
            Err(SqlError::InvalidValue)
        }
    }
}

pub struct SqlContextStrategy<T> {
    validator: Box<dyn Fn(&SqlContext<T>) -> bool>,
}

impl<T> SqlContextStrategy<T> {
    pub fn new<F>(validator: F) -> Self
    where
        F: 'static + Fn(&SqlContext<T>) -> bool,
    {
        Self {
            validator: Box::new(validator),
        }
    }
}

impl<T> SqlValidationStrategy<SqlContext<T>> for SqlContextStrategy<T> {
    fn is_valid(&self, context: &SqlContext<T>) -> Result<String, SqlError> {
        // If the context is valid, return Ok
        if (self.validator)(context) {
            Ok(String::from(""))
        }
        // Otherwise, return an error
        else {
            Err(SqlError::InvalidContext)
        }
    }
}

pub struct SqlValidation<T> {
    strategies: Vec<Box<dyn SqlValidationStrategy<T>>>,
}

impl<T> SqlValidation<T> {
    pub fn new() -> Self {
        Self { strategies: vec![] }
    }

    pub fn add_strategy<S>(&mut self, strategy: S)
    where
        S: 'static + SqlValidationStrategy<T>,
    {
        self.strategies.push(Box::new(strategy));
    }

    pub fn is_valid(&self, value: &T) -> Result<String, SqlError> {
        let mut sql = String::new();
        for strategy in &self.strategies {
            // let result = self.validate_strategy(strategy, value);
            // if result.is_err() {
            //     return result;
            // }
            // sql.push_str(&result.unwrap());
        }
        Ok(sql)
    }
}

pub struct SqlValidationBuilder<T> {
    strategies: Vec<Box<dyn SqlValidationStrategy<T>>>,
}

impl<T> SqlValidationBuilder<T> {
    pub fn new() -> Self {
        Self { strategies: vec![] }
    }

    pub fn add_strategy<S>(&mut self, strategy: S)
    where
        S: 'static + SqlValidationStrategy<T>,
    {
        self.strategies.push(Box::new(strategy));
    }

    // pub fn build(&self) -> SqlValidation<T> {
    //     SqlValidation {
    //         strategies: self.strategies.clone(),
    //     }
    // }
}

// impl<T> From<T> for Result<String, SqlError> {
//     fn from(value: T) -> Self {
//         let mut validation = SqlValidation::new();
//         validation.add_strategy(IsValidStrategy::new(IsValid::is_valid));
//         validation.add_strategy(SqlContextStrategy::new(SqlContextTrait::is_valid));
//         validation.is_valid(&value)
//     }
// }

// fn validate_sql<T>(sql: &SqlContext<T>) -> Result<String, SqlError> {
//     let mut builder = SqlValidationBuilder::new();
//     builder.add_strategy(IsValidStrategy::new(IsValid::is_valid));
//     builder.add_strategy(SqlContextStrategy::new(SqlContextTrait::is_valid));
    // builder.add_strategy(CustomValidationStrategy::new(|sql| {
    //     // Perform custom validation logic here
    //     true
    // }));
    // let validation = builder.build();
    // validation.is_valid(sql)
// }



/*
pub fn get_matrix_value(&self, row: usize, col: usize) -> Result<bool, MatricalError> {
    // Get data from SurrealDB or other source
    let data = HashMap::new();  // Placeholder, TODO: replace with actual data retrieval
    // Apply strategies
    self.execute_strategies(&data)?;
    Ok(value)
}

pub fn set_matrix_value(&mut self, row: usize, col: usize, value: bool) -> Result<(), MatricalError> {
    // Get data from SurrealDB or other source
    let data = HashMap::new();  // Placeholder, replace with actual data retrieval
    // Apply strategies
    self.execute_strategies(&data)?;
    Ok(())
} */