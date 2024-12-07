extern crate alloc;

use alloc::boxed::Box;
use core::ptr;
use core::mem;

struct Node<T> {
    data: T,
    next: *mut Node<T>
}

impl<T> Node<T> {
    fn new(data: T, next: *mut Node<T>) -> Node<T> {
        Node { data, next }
    }
}

pub struct LinkedList<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>
}

impl<T: PartialEq> LinkedList<T> {
    fn new() -> LinkedList<T> {
        LinkedList {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }

    /// Add node at the end of the linked list
    pub fn add(&mut self, data: T) {
        let new_node = Box::into_raw(Box::new(Node::new(data, ptr::null_mut())));

        if self.head.is_null() {
            self.head = new_node;
            self.tail = self.head;
        } else if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = new_node;
                self.tail = new_node;
            }
        }
    }

    /// Delete specific node in the linked list 
    pub fn delete(&mut self, data: T) -> bool {
        let mut node: *mut Node<T> = self.head;
        let mut prev: *mut Node<T> = ptr::null_mut();
    
        while !node.is_null() {
            unsafe {
                if (*node).data == data {
                    if prev.is_null() {
                        self.head = (*node).next;
                        mem::drop(node);
                    } else if self.tail == node {
                        mem::drop(self.tail);
                        self.tail = node;
                        (*node).next = ptr::null_mut();
                    } else {
                        (*prev).next = (*node).next;
                        mem::drop(node);
                    }
                    return true;
                } else {
                    prev = node;
                    node = (*node).next;
                }
            }
        }
        return false;
    }


    pub fn iter(&mut self) -> LinkedListIter<T> {
        LinkedListIter {
            current: self.head,
        }
    }
}

pub struct LinkedListIter<T> {
    current: *mut Node<T>,
}

impl<T> Iterator for LinkedListIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            None
        } else {
            unsafe {
                // Temporarily hold current node's data
                let data = ptr::read(&(*self.current).data);
                // Move to the next node
                self.current = (*self.current).next;
                Some(data)
            }
        }
    }
}