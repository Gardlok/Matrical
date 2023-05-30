



// Defines a set of methods that can be used to filter a given data set
//
trait FilterStrategy: Strategy {
    fn filter(&self, data: &HashMap<String, String>) -> Result<(), String>;
}

// Defines a set of methods that can be used to filter a given data set
//
struct FilterStrategyImpl {
    data: HashMap<String, String>,
}

impl FilterStrategyImpl {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

impl Strategy for FilterStrategyImpl {
    fn prepare(&self, data: &HashMap<String, String>) -> Result<(), String> {
        Ok(())
    }

    fn execute(&self, data: &HashMap<String, String>) -> Result<(), String> {
        Ok(())
    }

    fn result(&self) -> Result<(), String> {
        Ok(())
    }
}

impl FilterStrategy for FilterStrategyImpl {
    fn filter(&self, data: &HashMap<String, String>) -> Result<(), String> {
        Ok(())
    }
}

// // Defines a set of methods that can be used to filter a given data set
// //
// struct FilterStrategyImpl2 {
//     data: HashMap<String, String>,
// }