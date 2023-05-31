

// use crate::*;
// use crate::strategies::Tag;
// use crate::strategies::ElementStrategy;
// use crate::strategies::MatrixStrategy;
// use crate::schematics::Element;
// use crate::Element;
// use crate::schematics::Matrix;
// use crate::schematics::MatrixContext;
// use crate::LensContext;
// use crate::LensStrategy;
// use crate::LensCalibration;
// use crate::Calibration;



use crate::{Element, Tag};


use std::any::Any;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;

use crossbeam::atomic::AtomicCell;
use crossbeam::queue::{ArrayQueue, SegQueue};

pub struct Matrix<V> {
    matrix: ArrayQueue<Element<V>>,
    _context: MatrixContext,
}

pub struct MatrixContext {
    dimensions: Option<(usize, usize)>,
    attributes: Vec<Tag>,
    functors: HashMap<usize, Arc<Box<dyn Fn(dyn Any + Send + Sync) -> ()>>>,
}

impl<V> Matrix<V> {
    pub fn new_by_size(size: usize) -> Self {
        Self {
            matrix: ArrayQueue::new(size),
            _context: MatrixContext {
                dimensions: None,
                attributes: Vec::new(),
                functors: HashMap::new(),
            },
        }
    }
}


// pub struct Matrix<V> { matrix: ArrayQueue<Element<V>>, _context: MatrixContext }

// pub struct MatrixContext {
//     dimensions: Option<(usize, usize)>,
//     attributes: Vec<Tag>,
//     functors: HashMap<usize, Arc<Box<dyn Fn( dyn Any + Send + Sync )>>>,
// }

// // The Matrix struct now holds a Box<dyn MatrixOperation> which allows for changing the operation at runtime

// impl<V> Matrix<V> {
//     pub fn new_by_size(size: usize) -> Self {
//          Self {
//              matrix: ArrayQueue::new(size),
//              _context: MatrixContext { 
//                  dimensions: None,
//                  attributes: HashMap::new(),
//                  functors: HashMap::new(),
//              },
//          }
//      }
// }
 
 


