
use crate::*;

use crossbeam::sync::SegQueue;




// Struct to organize elements into a vector
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Vector {
    // The elements of the vector are stored in a SegQueue
    // This allows for concurrent access to the elements
    // The elements are stored in the order they are added
    pub elements: SegQueue<Element<V>>,
}


impl Vector {
    // Create a new vector
    pub fn new() -> Self {
        Self {
            elements: SegQueue::new(),
        }
    }

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

    // pop from the front of the vector

    // Add an element to the vector at the given index
    pub fn add_element_at(&self, element: Element<V>, index: usize) {
        self.elements.insert(index, element);
    }

    // Remove an element from the vector
    pub fn remove_element(&self, element: Element<V>) {
        self.elements.remove(element);
    }

    // Get the number of elements in the vector
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    // Get the element at the given index
    pub fn get_element(&self, index: usize) -> Option<Element<V>> {
        self.elements.get(index)
    }

    // Get the index of the given element
    pub fn get_index(&self, element: Element<V>) -> Option<usize> {
        self.elements.get_index(element)
    }

    
}