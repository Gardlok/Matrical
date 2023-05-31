
use dashmap::DashMap as HashMap;


use crate::schematics::data::*;
use crate::schematics::element::*;
use crate::schematics::matrix::*;





// Defines a set of methods that can be used to sort a given data set.
// //
// trait SortStrategy: Strategy {
//     fn sort(&self, data: &HashMap<String, String>) -> Result<(), String>;
// }


// // Defines a set of methods that can be used to sort a given data set.
// //
// struct SortStrategyImpl {
//     data: HashMap<String, String>,
// }

// impl SortStrategyImpl {
//     fn new() -> Self {
//         Self {
//             data: HashMap::new(),
//         }
//     }
// }

// impl Strategy for SortStrategyImpl {
//     fn prepare(&self, data: &HashMap<String, String>) -> Result<(), String> {
//         Ok(())
//     }

//     fn execute(&self, data: &HashMap<String, String>) -> Result<(), String> {
//         Ok(())
//     }

//     fn result(&self) -> Result<(), String> {
//         Ok(())
//     }
// }

// impl SortStrategy for SortStrategyImpl {
//     fn sort(&self, data: &HashMap<String, String>) -> Result<(), String> {
//         Ok(())
//     }
// }

// // Defines a set of methods that can be used to sort a given data set.
//
// struct SortStrategyImpl2 {
//     data: HashMap<String, String>,
// }