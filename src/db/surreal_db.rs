


use::surrealdb::dbs::Session;
use::surrealdb::kvs::Datastore;
use::dashmap::DashMap as HashMap;

use crate::MatricalError;
use crate::DependencyInjectionContainer;
use crate::ParameterizedQuery;
use crate::Strategy;





// SurrealDBAdapter struct
// Defines an adapter for the SurrealDB database that can be used to execute strategies and parameterized queries.
use std::sync::{Arc, Mutex};

pub struct SurrealDBAdapter {
    datastore: Datastore,
    session: Session,
    di_container: Arc<Mutex<DependencyInjectionContainer>>,
}

impl SurrealDBAdapter {
    pub fn new_mut() -> Result<Self, MatricalError> {
        let datastore = Datastore::new("memory")?;
        let session = Session::for_kv().with_ns("test").with_db("test");
        let di_container = Arc::new(Mutex::new(DependencyInjectionContainer::new()));
        Ok(Self {
            datastore,
            session,
            di_container,
        })
    }



    pub fn add_strategy(&mut self, strategy: Box<dyn Strategy>) {
        let mut di_container = self.di_container.lock().unwrap();
        di_container.strategies.push(strategy);
    }

    pub fn add_parameterized_query(&mut self, query: ParameterizedQuery) {
        let mut di_container = self.di_container.lock().unwrap();
        di_container.parameterized_queries.push(query);
    }

    pub async fn execute_strategies(&self, data: &HashMap<String, String>) -> Result<(), String> {
        let di_container = self.di_container.lock().unwrap();
        for strategy in &di_container.strategies {
            strategy.prepare(data)?;
            strategy.execute(data)?;
            strategy.result()?;
        }
        Ok(())
    }
    // get_matrix_value()
	// Retrieves a matrix value from the given data set.
    pub fn get_matrix_value(&self, row: usize, col: usize) -> Result<bool, MatricalError> {
        // Get data from SurrealDB or other source
        let data = HashMap::new();  // Placeholder, replace with actual data retrieval

        // Apply strategies
        self.execute_strategies(&data)?;
    }

	// set_matrix_value()
	// Sets a matrix value in the given data set.
    pub fn set_matrix_value(&mut self, row: usize, col: usize, value: bool) -> Result<(), MatricalError> {
        // Get data from SurrealDB or other source
        let data = HashMap::new();  // Placeholder, replace with actual data retrieval

        // Apply strategies
        self.execute_strategies(&data)?;

        // Perform the matrix value setting operation
      

        Ok(())
 
    }

}
// impl SurrealDBAdapter {
//     pub fn new() -> Result<Self, MatricalError> {
//         let datastore = Datastore::new("memory")?;
//         let session = Session::for_kv().with_ns("test").with_db("test");
//         let di_container = DependencyInjectionContainer::new();
//         Ok(Self {
//             datastore,
//             session,
//             di_container,
//         })
//     }

// 	// add_strategy()
// 	// Adds a strategy to the Dependency Injection Container.
//     pub fn add_strategy(&mut self, strategy: Box<dyn Strategy>) {
//         self.di_container.strategies.push(strategy);

//     }

// 	// add_parameterized_query()
// 	// Adds a parameterized query to the Dependency Injection Container.
//     pub fn add_parameterized_query(&mut self, query: ParameterizedQuery) {
//         self.di_container.parameterized_queries.push(query);
//     }

// 	// execute_strategies()
// 	// Executes the strategies in the Dependency Injection Container on the given data set.
//     pub async fn execute_strategies(&self, data: &HashMap<String, String>) -> Result<(), String> {
//         for strategy in &self.di_container.strategies {
//             strategy.prepare(data)?;
//             strategy.execute(data)?;
//             strategy.result()?;
//         }
//         Ok(())
//     }


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