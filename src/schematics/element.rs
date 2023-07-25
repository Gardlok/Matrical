

use crossbeam::queue::{SegQueue, ArrayQueue};
use crossbeam::atomic::AtomicCell;
use std::any::Any;
use std::marker::PhantomData;
use std::sync::Arc;


use crate::strategies::Tag;

pub struct AtomicBool { atomic_bool: AtomicCell<bool> }


pub struct Element<V>{
    pub state: AtomicBool,
    pub _context: ElementContext<V>,
}

// Move the following to a separate file
pub struct AttributesApplied { attri: SegQueue<PhantomData<Arc<dyn Any + Send + Sync>>> }
pub struct AttributeContext {
    pub attri: Option<SegQueue<Box<dyn Any + Send + Sync>>>,
}




pub struct ElementContext<V> {
    pub _x_idx: usize,
    pub _y_idx: usize,
    pub attri: SegQueue<Tag>,
    pub workq: SegQueue<Box<dyn GearOperation>>,
    pub value: AtomicCell<V>,
}

impl<V> ElementContext<V> 
where V: Any + Send + Sync + Clone + Default {
    pub fn new() -> Self {
        Self {
            _x_idx: 0,
            _y_idx: 0,
            attri: SegQueue::new(),
            workq: SegQueue::new(),
            value: AtomicCell::<V>::new(Default::default()),
        }
    }
}




impl <V> Default for ElementContext<V>
where V: Any + Send + Sync + Clone + Default {
    fn default() -> Self {
        Self::new()
    }
}

impl AttributeContext {
    pub fn new() -> Self {
        Self {
            attri: None,
        }
    }
}



impl<V: Default> Element<V> 
where V: Any + Send + Sync + Clone + Default {
    pub fn new() -> Self {
        Self {
            state: AtomicBool::new(),
            _context: ElementContext::new(),
        }
    }
}


impl AttributesApplied {
    pub fn new() -> Self {
        Self {
            attri: SegQueue::new(),
        }
    }
}

impl AtomicBool {
    pub fn new() -> Self {
        Self {
            atomic_bool: AtomicCell::new(false),
        }
    }
}




pub trait GearOperation: Send + Sync {
    fn execute(&self) -> ();
}


impl GearOperation for Box<dyn Fn() -> () + Send + Sync> {
    fn execute(&self) -> () {
        self()
    }
}
