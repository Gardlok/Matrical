
use crate::schematics::element::Element;

use crossbeam::queue::SegQueue;



// Struct to organize elements into a vector
//
// The vector is a SegQueue of elements that are aligned in a single dimension
// 
pub struct Vector<V> 
where Element<V>: PartialEq + Eq + Clone {
    elements: SegQueue<Element<V>>,
}


impl<V> Vector<V> where Element<V>: PartialEq + Eq + Clone {

    // Create a new vector with the given elements
    pub fn with_elements(elements: SegQueue<Element<V>>) -> Self {
        Self {
            elements,
        }
    }

    // Add an element to the vector
    pub fn add_element(&self, element: Element<V>) {
        self.elements.push(element);
    }

    // Pop an element from the vector
    pub fn pop_element(&self) -> Option<Element<V>> {
        self.elements.pop()
    }

    // Pop n elements from the vector
    pub fn pop_elements(&self, n: usize) -> Option<Vec<Element<V>>> {
        let mut elements = Vec::new();
        for _ in 0..n {
            if let Some(element) = self.elements.pop() {
                elements.push(element);
            } else {
                return None;
            }
        }
        Some(elements)
    }


    // Get the number of elements in the vector
    pub fn len(&self) -> usize {
        self.elements.len()
    }


   
}