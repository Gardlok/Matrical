




// Defines a set of methods that can be used to aggregate a given data set.
//
trait AggregateStrategy: Strategy {
    fn aggregate(&self, data: &HashMap<String, String>) -> Result<(), String>;
}