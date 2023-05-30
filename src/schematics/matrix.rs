

use crate::*;




pub struct Matrix<V> { matrix: SegQueue<Element<V>>, _context: MatrixContext }

pub struct MatrixContext {
    dimensions: Option<(usize, usize)>,
    attributes: HashMap<TypeId, Arc<dyn Fn()>>,
    functors: HashMap<usize, Arc<Box<dyn Fn( dyn Any + Send + Sync )>>>,
}

// The Matrix struct now holds a Box<dyn MatrixOperation> which allows for changing the operation at runtime

impl Matrix {
    pub fn new() -> Self {
         Self {
             matrix: Arc::new(SurrealDBAdapter::new()),
             _context: MatrixContext { 
                 dimensions: None,
                 attributes: HashMap::new(),
                 functors: HashMap::new(),
             },
         }
     }
 
     pub fn set_operation(&mut self, operation: Box<MatrixOperation<V>>) {
         self._context.operation = operation;
     }
 
     pub fn execute_operation(&self) -> Result<(), MatricalError> {
         self._context.operation.execute(&self._context)
     }
     // Use the execute_strategies method within your matrix operation methods to apply the desired strategies:
     pub fn execute_strategies(&self, data: &HashMap<String, String>) -> Result<(), String> {
         let di_container = self.di_container.lock().unwrap();
         for strategy in &di_container.strategies {
             strategy.prepare(data)?;
             strategy.execute(data)?;
             strategy.result()?;
         }
         Ok(())
     }
     pub async fn async_execute_strategies(&self, data: &HashMap<String, String>) -> Result<(), String> {
         let di_container = self.di_container.lock().unwrap();
         for strategy in &di_container.strategies {
             strategy.prepare(data)?;
             strategy.execute(data)?;
             strategy.result()?;
         }
         Ok(())
     }
 
     pub fn add_strategy(&mut self, strategy: Box<dyn Strategy>) {
         let mut di_container = self.di_container.lock().unwrap();
         di_container.strategies.push(strategy);
     }
 
     pub fn add_parameterized_query(&mut self, query: ParameterizedQuery) {
         let mut di_container = self.di_container.lock().unwrap();
         di_container.parameterized_queries.push(query);
     }
 
    pub fn get_matrix_value(&self, data: &HashMap<String, String>) -> Result<(), String> {
        let di_container = self.di_container.lock().unwrap();
        for strategy in &di_container.strategies {
            strategy.prepare(data)?;
            strategy.execute(data)?;
            strategy.result()?;
        }
        Ok(())
    }

    // set_matrix_value()
    // Sets a matrix value in the given data set.
    pub fn set_matrix_value(&mut self, data: &HashMap<String, String>) -> Result<(), String> {
        let di_container = self.di_container.lock().unwrap();
        for strategy in &di_container.strategies {
            strategy.prepare(data)?;
            strategy.execute(data)?;
            strategy.result()?;
        }
        Ok(())
    }

    // get_matrix_operation()
    // Retrieves a matrix operation from the given data set.
    pub fn set_matrix_operation(&self, data: &HashMap<String, String>) -> Result<(), String> {
        let di_container = self.di_container.lock().unwrap();
        for strategy in &di_container.strategies {
            strategy.prepare(data)?;
            strategy.execute(data)?;
            strategy.result()?;
        }
        Ok(())
    }

    // set_matrix_operation()
    // Sets a matrix operation in the given data set.
    pub fn set_matrix_operation(&mut self, data: &HashMap<String, String>) -> Result<(), String> {
        let di_container = self.di_container.lock().unwrap();
        for strategy in &di_container.strategies {
            strategy.prepare(data)?;
            strategy.execute(data)?;
            strategy.result()?;
        }
        Ok(())
    }

    // get_matrix_strategy()
    // Retrieves a matrix strategy from the given data set.
    pub fn get_matrix_strategy(&self, data: &HashMap<String, String>) -> Result<(), String> {
        let di_container = self.di_container.lock().unwrap();
        for strategy in &di_container.strategies {
            strategy.prepare(data)?;
            strategy.execute(data)?;
            strategy.result()?;
        }
        Ok(())
    }
    
    // set_matrix_strategy()
    // Sets a matrix strategy in the given data set.
    pub fn set_matrix_strategy(&mut self, data: &HashMap<String, String>) -> Result<(), String> {
        let di_container = self.di_container.lock().unwrap();
        for strategy in &di_container.strategies {
            strategy.prepare(data)?;
            strategy.execute(data)?;
            strategy.result()?;
        }
        Ok(())
    }

    // get_matrix_context()
    // Retrieves a matrix context from the given data set.
    pub fn get_matrix_context(&self, data: &HashMap<String, String>) -> Result<(), String> {
        let di_container = self.di_container.lock().unwrap();
        for strategy in &di_container.strategies {
            strategy.prepare(data)?;
            strategy.execute(data)?;
            strategy.result()?;
        }
        Ok(())
    }

    // set_matrix_context()
    // Sets a matrix context in the given data set.
    pub fn set_matrix_context(&mut self, data: &HashMap<String, String>) -> Result<(), String> {
        let di_container = self.di_container.lock().unwrap();
        for strategy in &di_container.strategies {
            strategy.prepare(data)?;
            strategy.execute(data)?;
            strategy.result()?;
        }
        Ok(())
    }

    // get_matrix_attributes()
    // Retrieves matrix attributes from the given data set.
    pub fn get_matrix_attributes(&self, data: &HashMap<String, String>) -> Result<(), String> {
        let di_container = self.di_container.lock().unwrap();
        for strategy in &di_container.strategies {
            strategy.prepare(data)?;
            strategy.execute(data)?;
            strategy.result()?;
        }
        Ok(())
    }

    // set_matrix_attributes()
    // Sets matrix attributes in the given data set.
    pub fn set_matrix_attributes(&mut self, data: &HashMap<String, String>) -> Result<(), String> {
        let di_container = self.di_container.lock().unwrap();
        for strategy in &di_container.strategies {
            strategy.prepare(data)?;
            strategy.execute(data)?;
            strategy.result()?;
        }
        Ok(())
    }

    // get_matrix_functors()
    // Retrieves matrix functors from the given data set.
    pub fn get_matrix_functors(&self, data: &HashMap<String, String>) -> Result<(), String> {
        let di_container = self.di_container.lock().unwrap();
        for functor in &di_container.functors {
            functor.prepare(data)?;
            functor.execute(data)?;
            functor.result()?;
        }
        Ok(())
    }

    // set_matrix_functors()
    // Sets matrix functors in the given data set.
    pub fn set_matrix_functors(&mut self, data: &HashMap<String, String>) -> Result<(), String> {
        let di_container = self.di_container.lock().unwrap();
        for functor in &di_container.functors {
            functor.prepare(data)?;
            functor.execute(data)?;
            functor.result()?;
        }
        Ok(())
    }

    // get_matrix_dimensions()
    // Retrieves matrix dimensions from the given data set.
    pub fn get_matrix_dimensions(&self, data: &HashMap<String, String>) -> Result<(), String> {
        let di_container = self.di_container.lock().unwrap();
        for functor in &di_container.functors {
            functor.prepare(data)?;
            functor.execute(data)?;
            functor.result()?;
        }
        Ok(())
    }

    // set_matrix_dimensions()
    // Sets matrix dimensions in the given data set.
    pub fn set_matrix_dimensions(&mut self, data: &HashMap<String, String>) -> Result<(), String> {
        let di_container = self.di_container.lock().unwrap();
        for functor in &di_container.functors {
            functor.prepare(data)?;
            functor.execute(data)?;
            functor.result()?;
        }
        Ok(())
    }
}
 
 


