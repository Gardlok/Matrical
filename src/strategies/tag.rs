
use crate::*;


// Defines Tag (Attribute) that can be applied to a given Element, Matrix, or Vector. Generally issued 
// by Cog to provide additional context for operations performed on the Element, Matrix, or Vector, however
// can serve many other purposes. Such as data profiling, data validation, and/or data transformation in
// effort to support the various operations performed within Matrical.
pub struct Tag {

    // The name of the Tag
    name: String,

} 

// Defines a parameterized query that can be used to perform various operations on a given data set.
//
pub struct ParameterizedQuery {
    query: String,
    parameters: Vec<String>,
}

// Defines a container for strategies and parameterized queries that can be used to perform various operations on a given data set.
//
pub struct DependencyInjectionContainer {
    strategies: Vec<Box<dyn Strategy>>,
    parameterized_queries: Vec<ParameterizedQuery>,
}
impl DependencyInjectionContainer {
    fn new() -> Self {
        Self {
            strategies: Vec::new(),
            parameterized_queries: Vec::new(),
        }
    }
}
