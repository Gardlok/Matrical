
use rayon::result;

use crate::error::MatricalError;
// use crate::MatrixContext;


use std::any::Any;
use std::ops::Rem;
use std::cmp::PartialOrd;
use std::marker::PhantomData;
use std::rc::Rc;

use std::sync::Arc;







// 
trait ValidationStrategy<T> {
    fn is_valid(&self, input: &T) -> bool;
    fn as_any(&self) -> &dyn Any;
}

pub struct CustomValidationStrategy<F>(F);

impl<T, F> ValidationStrategy<T> for CustomValidationStrategy<F>
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

pub struct Validation<T> {
    strategies: Vec<Box<dyn ValidationStrategy<T>>>,
}

impl<T> Validation<T> {
    fn new() -> Self {
        Validation {
            strategies: Vec::new(),
        }
    }

    fn add_strategy<S>(&mut self, strategy: S)
    where
        S: ValidationStrategy<T> + 'static,
    {
        self.strategies.push(Box::new(strategy));
    }
    
    fn remove_strategy(&mut self, strategy: &dyn Any) {
        self.strategies.retain(|s| {
            let s_any: &dyn Any = s.as_any();
            !Rc::ptr_eq(&Rc::new(s_any), &Rc::new(strategy))
        });
    }

    fn is_valid(&self, input: &T) -> bool {
        self.strategies.iter().all(|strategy| strategy.is_valid(input))
    }
}


pub struct ValidatorFactory<T> {
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
        self.validators.retain(|v| Rc::ptr_eq(&Rc::new(v), &Rc::new(validator)));
    }
}




