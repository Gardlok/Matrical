

use crossbeam::queue::SegQueue;
use crossbeam::atomic::AtomicCell;
use std::any::Any;
use std::marker::PhantomData;
use std::sync::Arc;




pub struct AtomicBool { atomic_bool: AtomicCell<bool> }

pub struct Element<V> { state: AtomicBool , _context: ElementContext<V> }


// Move the following to a separate file
pub struct AttributesApplied { attri: SegQueue<PhantomData<Arc<dyn Any + Send + Sync>>> }
pub struct AttributeContext {
    pub attri: Option<SegQueue<Box<dyn Any + Send + Sync>>>,
}

pub struct ElementContext<V> {
    pub state: AtomicBool,
    pub x_idx: AtomicCell<usize>,
    pub y_idx: AtomicCell<usize>,
    pub attri: Option<SegQueue<Cog>>, 
    pub workq: SegQueue<Box<dyn Fn (&mut dyn FunctorHandler)>>,  // TODO
    pub value: Option<V>,  // Not thread safe
}


impl <V>ElementContext<V> {
    pub fn new() -> Self {
        Self {
            state: AtomicBool::new(),
            workq: SegQueue::new(),
            x_idx: None,
            y_idx: None,
            attri: None,
            value: None,
        }
    }
}

impl <V> Default for ElementContext<V> {
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



impl<V> Element<V> {
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
