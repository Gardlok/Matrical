


use std::any::Any;
use std::marker::PhantomData;

pub struct Context {
    param: String,
    id: u32,
}

pub trait FromContext {
    fn from_context(context: &Context) -> Self;
}

pub struct Param(pub String);

impl FromContext for Param {
    fn from_context(context: &Context) -> Self {
        Param(context.param.clone())
    }
}

trait Handler<T> {
    fn call(&self, arg: T);
}

impl<T, F> Handler<T> for F
where
    F: Fn(T),
{
    fn call(&self, arg: T) {
        (self)(arg);
    }
}

// Can this help?
pub struct Functor<T, F> {
    handler: F,
    _phantom: PhantomData<T>,
}

pub trait FunctorHandler<T, F> where F: Fn() -> T {
    fn execute(&self, context: ()) -> Result<T, ()>;
}

impl<T, F> FunctorHandler<T, F> for Functor<T, F>
where
    F: Fn() -> T,
{
    fn execute(&self, context: ()) -> Result<T, ()> {
        Ok((self.handler)())
    }
}

pub fn perform_execute<T, H>(context: (), handler: &H) -> Result<(), ()>
where
    H: FunctorHandler<T, H> + Fn() -> T
{
    handler.execute(context)?;
    Ok(())
}

pub struct ComboValidationStrategy<T: 'static> {
    static_strategies: Vec<Box<dyn ValidationStrategy<T>>>,
    dynamic_strategies: Vec<Box<dyn ValidationStrategy<T>>>,
}

impl<T: 'static> ComboValidationStrategy<T> {
    // Creates a new ComboValidationStrategy with the given static and dynamic strategies to be used
    // when validating input. The static strategies are executed first, followed by the dynamic
    // strategies. If any of the static strategies fail, the input is considered invalid and the
    // dynamic strategies are not executed. If any of the dynamic strategies fail, the input is 
    // considered invalid. If all of the static and dynamic strategies pass, the input is considered
    // valid. 
    pub fn new(static_strategies: Vec<Box<dyn ValidationStrategy<T>>>, dynamic_strategies: Vec<Box<dyn ValidationStrategy<T>>>) -> Self {
        ComboValidationStrategy {
            static_strategies,
            dynamic_strategies,
        }
    }
}

impl<T: 'static> ValidationStrategy<T> for ComboValidationStrategy<T> {
    fn is_valid(&self, input: &T) -> bool {
        // All static strategies must pass
        if !self.static_strategies.iter().all(|strategy| strategy.is_valid(input)) {
            return false;
        }

        // All dynamic strategies must pass
        if !self.dynamic_strategies.iter().all(|strategy| strategy.is_valid(input)) {
            return false;
        }

        // If all static and dynamic strategies pass, the input is valid
        true
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct NestedValidationStrategy {
    nested_validation: Validation<i32>,
}

impl ValidationStrategy<i32> for NestedValidationStrategy {
    fn is_valid(&self, data: &i32) -> bool {
        self.nested_validation.is_valid(data)
    }

    fn as_any(&self) -> &dyn Any {
        todo!()
    }
}

// A validation strategy that always returns true
pub struct AlwaysValid;
impl<T: 'static> ValidationStrategy<T> for AlwaysValid {
    fn is_valid(&self, _input: &T) -> bool {
        true
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// A validation strategy that always returns false
pub struct AlwaysInvalid;
impl<T: 'static> ValidationStrategy<T> for AlwaysInvalid {
    fn is_valid(&self, _input: &T) -> bool {
        false
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// A validation strategy that returns true if the input is equal to the given value
pub struct Equals<T: 'static>(T);
impl<T: 'static + PartialEq> Equals<T> {
    pub fn new(value: T) -> Self {
        Equals(value)
    }
}

impl<T: 'static + PartialEq> ValidationStrategy<T> for Equals<T> {
    fn is_valid(&self, input: &T) -> bool {
        input == &self.0
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// A validation strategy that returns true if the input is not equal to the given value
pub struct NotEquals<T: 'static>(T);
impl<T: 'static + PartialEq> NotEquals<T> {
    pub fn new(value: T) -> Self {
        NotEquals(value)
    }
}
impl<T: 'static + PartialEq> ValidationStrategy<T> for NotEquals<T> {
    fn is_valid(&self, input: &T) -> bool {
        input != &self.0
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// A validation strategy that returns true if the input is greater than the given value
pub struct GreaterThan<T: 'static>(T);
impl<T: 'static + PartialOrd> GreaterThan<T> {
    pub fn new(value: T) -> Self {
        GreaterThan(value)
    }
}
impl<T: 'static + PartialOrd> ValidationStrategy<T> for GreaterThan<T> {
    fn is_valid(&self, input: &T) -> bool {
        input > &self.0
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// A validation strategy that returns true if the input is greater than or equal to the given value
pub struct GreaterThanOrEqual<T: 'static>(T);
impl<T: 'static + PartialOrd> GreaterThanOrEqual<T> {
    pub fn new(value: T) -> Self {
        GreaterThanOrEqual(value)
    }
}
impl<T: 'static + PartialOrd> ValidationStrategy<T> for GreaterThanOrEqual<T> {
    fn is_valid(&self, input: &T) -> bool {
        input >= &self.0
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// A validation strategy that returns true if the input is less than the given value
pub struct LessThan<T: 'static>(T);
impl<T: 'static + PartialOrd> LessThan<T> {
    pub fn new(value: T) -> Self {
        LessThan(value)
    }
}
impl<T: 'static + PartialOrd> ValidationStrategy<T> for LessThan<T> {
    fn is_valid(&self, input: &T) -> bool {
        input < &self.0
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// A validation strategy that returns true if the input is less than or equal to the given value
pub struct LessThanOrEqual<T: 'static>(T);
impl<T: 'static + PartialOrd> LessThanOrEqual<T> {
    pub fn new(value: T) -> Self {
        LessThanOrEqual(value)
    }
}
impl<T: 'static + PartialOrd> ValidationStrategy<T> for LessThanOrEqual<T> {
    fn is_valid(&self, input: &T) -> bool {
        input <= &self.0
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// A validation strategy that returns true if the input is between the given values
pub struct Between<T: 'static>(T, T);
impl<T: 'static + PartialOrd> Between<T> {
    pub fn new(min: T, max: T) -> Self {
        Between(min, max)
    }
}
impl<T: 'static + PartialOrd> ValidationStrategy<T> for Between<T> {
    fn is_valid(&self, input: &T) -> bool {
        input >= &self.0 && input <= &self.1
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub struct CustomValidationStrategy<T: 'static, F: Fn(&T) -> bool + 'static>(
    F,
    PhantomData<T>,
);

impl<T: 'static, F: Fn(&T) -> bool + 'static> CustomValidationStrategy<T, F> {
    pub fn new(strategy: F) -> Self {
        CustomValidationStrategy(strategy, PhantomData)
    }
}

impl<T: 'static, F> ValidationStrategy<T> for CustomValidationStrategy<T, F>
where
    F: Fn(&T) -> bool + 'static,
{
    fn is_valid(&self, input: &T) -> bool {
        (self.0)(input)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}



pub struct LengthValidation;

impl ValidationStrategy<String> for LengthValidation {
    fn is_valid(&self, data: &String) -> bool {
        data.len() > 5
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct NumberValidation;

impl ValidationStrategy<i32> for NumberValidation {
    fn is_valid(&self, data: &i32) -> bool  {
        *data > 5
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}



pub trait ValidationStrategy<T: 'static> {
    fn is_valid(&self, input: &T) -> bool;
    fn as_any(&self) -> &dyn Any;
}

pub struct Validation<T: 'static> {
    strategies: Vec<Box<dyn ValidationStrategy<T>>>,
    children: Vec<Validation<T>>,
}

impl<T: 'static> Validation<T> {
    pub fn new() -> Self {
        Validation {
            strategies: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn add_strategy<S>(&mut self, strategy: S)
    where
        S: ValidationStrategy<T> + 'static,
    {
        self.strategies.push(Box::new(strategy));
    }
    
    pub fn add_strategies<S>(&mut self, strategies: Vec<S>)
    where
        S: ValidationStrategy<T> + 'static,
    {
        for strategy in strategies {
            self.add_strategy(strategy);
        }
    }

    pub fn add_child(&mut self, child: Validation<T>) {
        self.children.push(child);
    }

    pub fn remove_strategy(&mut self, strategy: &dyn Any) {
        self.strategies.retain(|s| !std::ptr::eq(s.as_any(), strategy));
    }

    pub fn remove_child(&mut self, child: &Validation<T>) {
        self.children.retain(|c| !std::ptr::eq(c, child));
    }

    pub fn batch_process(&self, inputs: &[T]) -> Vec<bool> {
        inputs.iter().map(|input| self.is_valid(input)).collect()
    }

    pub fn is_valid(&self, input: &T) -> bool {
        self.strategies.iter().all(|strategy| strategy.is_valid(input)) &&
        self.children.iter().all(|child| child.is_valid(input))
    }
}



trait ValidatorStrategy<T> {
// type that implements the trait:
    // any 
    // Eq 
    // PartialEq 
    // PartialOrd 
    // Ord 
    // Hash 
    // Debug 
    // Display 
    // Default 
    // Copy 
    fn validate(&self, data: &T) -> bool;
}

pub struct Validator<T> {
    // Stores the data that will be validated by the strategies in the vector
    // T is a generic type that will be specified when the Validator is created
    // 'static is a lifetime specifier that means the Validator will live for 
    // the entire duration of the program (which is what we want) 
    data: T,
    // Stores a list of strategies that will be used to validate the data
    // Box<dyn ValidatorStrategy<T>> is a trait object that can hold any type
    // that implements the ValidatorStrategy<T> trait (which is all of them)
    strategies: Vec<Box<dyn ValidationStrategy<T>>>,
}

impl<T: 'static> Validator<T> {
    // Creates a new Validator with the given data and an empty vector of strategies
    // TODO: Consider Atomic queue from crossbeam crate it could look like this:
    pub fn new(data: T) -> Self {
        Validator {
            data,
            strategies: Vec::new(),
        }
    }


    pub fn add_strategy(&mut self, strategy: Box<dyn ValidationStrategy<T>>) {
        self.strategies.push(strategy);
    }

    pub fn add_strategies(&mut self, strategies: Vec<Box<dyn ValidationStrategy<T>>>) {
        for strategy in strategies {
            self.add_strategy(strategy);
        }
    }

    pub fn remove_strategy(&mut self, strategy: &dyn Any) {
        self.strategies.retain(|s| !std::ptr::eq(s.as_any(), strategy));
    }


    pub fn validate(&self) -> bool {
        self.strategies.iter().all(|strategy| strategy.is_valid(&self.data))
    }
}



pub struct ValidatorFactory<T: 'static> {
    validators: Vec<Validation<T>>,
}

impl<T: 'static> ValidatorFactory<T> {
    pub fn new() -> Self {
        Self {
            validators: Vec::new(),
        }
    }

    pub fn create_validator(&mut self) -> &mut Validation<T> {
        let validator = Validation::new();
        self.validators.push(validator);
        self.validators.last_mut().unwrap()
    }

    pub fn remove_validator(&mut self, validator: &Validation<T>) {
        self.validators.retain(|v| !std::ptr::eq(v, validator));
    }

    pub fn remove_strategy(&mut self, validator: &mut Validation<T>, strategy: &dyn Any) {
        validator.remove_strategy(strategy);
    }
}

pub struct ValidationBuilder<T: 'static> {
    strategies: Vec<Box<dyn ValidationStrategy<T>>>,
}

impl<T: 'static> ValidationBuilder<T> {
    pub fn new() -> Self {
        Self {
            strategies: Vec::new(),
        }
    }

    pub fn add_strategy<S: ValidationStrategy<T> + 'static>(mut self, strategy: S) -> Self {
        self.strategies.push(Box::new(strategy));
        self
    }

    pub fn add_strategies<S: ValidationStrategy<T> + 'static>(mut self, strategies: Vec<S>) -> Self {
        for strategy in strategies {
            self.strategies.push(Box::new(strategy));
        }
        self
    }   

    pub fn build(self) -> Validation<T> {
        Validation {
            strategies: self.strategies,
            children: Vec::new(),
        }
    }
}

pub struct ValidationStrategyBuilder<T: 'static> {
    strategies: Vec<Box<dyn ValidationStrategy<T>>>,
}

impl<T: 'static> ValidationStrategyBuilder<T> {
    pub fn new() -> Self {
        Self {
            strategies: Vec::new(),
        }
    }

    pub fn add_strategy<S: ValidationStrategy<T> + 'static>(mut self, strategy: S) -> Self {
        self.strategies.push(Box::new(strategy));
        self
    }

    pub fn build(self) -> Vec<Box<dyn ValidationStrategy<T>>> {
        self.strategies
    }
}











#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_param_from_context() {
        let context = Context {
            param: "test_param".to_string(),
            id: 1,
        };
        let param = Param::from_context(&context);
        assert_eq!(param.0, "test_param");
    }

    #[test]
    fn test_length_validation() {
        let strategy = LengthValidation;
        assert!(strategy.is_valid(&"abcdef".to_string()));
        assert!(!strategy.is_valid(&"abcde".to_string()));
    }

    #[test]
    fn test_number_validation() {
        let strategy = NumberValidation;
        assert!(strategy.is_valid(&6));
        assert!(!strategy.is_valid(&5));
    }

    #[test]
    fn test_validator() {
        let mut validator = Validator::new(6);
        validator.add_strategy(Box::new(NumberValidation));
        assert!(validator.validate());
    }

    #[test]
    fn test_validator_factory() {
        let mut factory: ValidatorFactory<i32> = ValidatorFactory::new();
        let mut validator = factory.create_validator();
        validator.add_strategy(NumberValidation);
        assert!(factory.validators[0].is_valid(&6));
        assert!(!factory.validators[0].is_valid(&5));
    }

    #[test]
    fn test_always_valid() {
        let strategy = AlwaysValid;
        assert!(strategy.is_valid(&5));
        assert!(strategy.is_valid(&"hello"));
    }

    #[test]
    fn test_always_invalid() {
        let strategy = AlwaysInvalid;
        assert!(!strategy.is_valid(&5));
        assert!(!strategy.is_valid(&"hello"));
    }

    #[test]
    fn test_equals() {
        let strategy = Equals::new(5);
        assert!(strategy.is_valid(&5));
        assert!(!strategy.is_valid(&6));
    }

    #[test]
    fn test_not_equals() {
        let strategy = NotEquals::new(5);
        assert!(!strategy.is_valid(&5));
        assert!(strategy.is_valid(&6));
    }

    #[test]
    fn test_greater_than() {
        let strategy = GreaterThan::new(5);
        assert!(strategy.is_valid(&6));
        assert!(!strategy.is_valid(&5));
    }

    #[test]
    fn test_greater_than_or_equal() {
        let strategy = GreaterThanOrEqual::new(5);
        assert!(strategy.is_valid(&5));
        assert!(strategy.is_valid(&6));
        assert!(!strategy.is_valid(&4));
    }

    #[test]
    fn test_less_than() {
        let strategy = LessThan::new(5);
        assert!(strategy.is_valid(&4));
        assert!(!strategy.is_valid(&5));
    }

    #[test]
    fn test_less_than_or_equal() {
        let strategy = LessThanOrEqual::new(5);
        assert!(strategy.is_valid(&5));
        assert!(strategy.is_valid(&4));
        assert!(!strategy.is_valid(&6));
    }

    #[test]
    fn test_between() {
        let strategy = Between::new(5, 10);
        assert!(strategy.is_valid(&5));
        assert!(strategy.is_valid(&7));
        assert!(strategy.is_valid(&10));
        assert!(!strategy.is_valid(&4));
        assert!(!strategy.is_valid(&11));
    }

    // #[test]
    // fn test_combo_validation_strategy() {
    //     let static_strategies = vec![Box::new(AlwaysValid)];
    //     let dynamic_strategies:  <Box<AlwaysInvalid>> + 'static = vec![Box::new(AlwaysInvalid)];
    //     let strategy = ComboValidationStrategy::new(static_strategies, dynamic_strategies  );
    //     assert!(!strategy.is_valid(&5));
    // }

    #[test]
    fn test_validation() {
        let mut validation = Validation::new();
        validation.add_strategy(AlwaysValid);
        assert!(validation.is_valid(&5));

        validation.add_strategy(AlwaysInvalid);
        assert!(!validation.is_valid(&5));
    }

    #[test]
    fn test_custom_validation_strategy() {
        let strategy = CustomValidationStrategy::new(|data: &i32| data > &5);
        assert!(strategy.is_valid(&6));
        assert!(!strategy.is_valid(&5));
    }

    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn test_complex_validation() {
            let mut validation = Validation::new();
            validation.add_strategy(Equals::new(5));
            validation.add_strategy(GreaterThan::new(3));
            validation.add_strategy(LessThan::new(10));
    
            assert!(validation.is_valid(&5));
            assert!(!validation.is_valid(&3));
            assert!(!validation.is_valid(&10));
        }
    
        #[test]
        fn test_nested_validation() {
            let mut inner_validation = Validation::new();
            inner_validation.add_strategy(Equals::new(5));
    
            let mut outer_validation = Validation::new();
            outer_validation.add_child(inner_validation);
    
            assert!(outer_validation.is_valid(&5));
            assert!(!outer_validation.is_valid(&6));
        }
    
        #[test]
        fn test_validation_builder() {
            let validation = ValidationBuilder::new()
                .add_strategy(Equals::new(5))
                .add_strategy(GreaterThan::new(3))
                .add_strategy(LessThan::new(10))
                .build();
    
            assert!(validation.is_valid(&5));
            assert!(!validation.is_valid(&3));
            assert!(!validation.is_valid(&10));
        }
    
        // #[test]
        // fn test_nested_validation_builder() {
        //     let inner_validation = ValidationBuilder::new()
        //         .add_strategy(Equals::new(5))
        //         .build();
    
        //     let outer_validation = ValidationBuilder::new()
        //         .add_strategies(inner_validation)           
        //         .build();
    
        //     assert!(outer_validation.is_valid(&5));
        //     assert!(!outer_validation.is_valid(&6));
        // }
    }
    



}

