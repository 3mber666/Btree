use std::{cmp::{PartialEq, PartialOrd}};
use std::iter::{Extend};


#[derive(Debug)]
struct Btree<T: Ord> {
    root: Tree<T>
}


#[derive(Debug)]
struct Node<T: Ord> {
    payload: T,
    left: Tree<T>,
    right: Tree<T>
}


#[derive(Debug)]
struct Tree<T: Ord>(Option<Box<Node<T>>>);


impl<T: Ord> Btree<T> {
    fn new() -> Self {
        Btree { root: Tree(None) }
    }

    fn insert(&mut self, payload: T) {
        self.root.insert(payload)
    }

    fn to_sorted_vector(self) -> Vec<T> {
        self.root.to_sorted_vector()
    }

    fn search_node(self, target: T) -> bool {
        self.root.search_node(target)
    }
}

impl<T: Ord> Node<T> {
    fn new(payload: T) -> Self {
        Node { 
            payload, 
            left: Tree(None), 
            right: Tree(None) 
        }
    }
}


impl<T: Ord> Extend<T> for Btree<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        iter.into_iter().for_each(move |elem| {
            self.insert(elem);
        });
    }
} 


impl<T: Ord> Tree<T> {

    fn insert(&mut self, payload: T) {
        let mut current = self;

        while let Some(ref mut node) = current.0 {
            match node.payload.cmp(&payload) {
                std::cmp::Ordering::Greater => current = &mut node.left,
                std::cmp::Ordering::Less => current = &mut node.right,
                std::cmp::Ordering::Equal => { todo!() }
            }
        }

        current.0 = Some(Box::new(Node::new(payload)))
    }

    fn to_sorted_vector(self) -> Vec<T> {
        let mut elements = Vec::new();
        

        if let Some(node) = self.0 {
            elements.extend(node.left.to_sorted_vector());
            elements.push(node.payload);
            elements.extend(node.right.to_sorted_vector());
        }

        elements
    }

    fn search_node(self, target: T) -> bool {
        let isTrue = self.to_sorted_vector().contains(&target);
        isTrue
    } 
}


#[test]
fn test_to_sorted_vec() {

    let mut root: Btree<i32>  = Btree::new();
    root.insert(1);
    root.insert(2);
    root.insert(3);
    root.insert(4);
    root.insert(5);
    let sorted = root.to_sorted_vector();

    assert_eq!(sorted, vec![1, 2, 3, 4, 5]);
}



#[test]
fn search_node_test_true() {
    let mut root: Btree<i32>  = Btree::new();
    root.insert(1);
    root.insert(2);
    root.insert(3);
    root.insert(4);
    root.insert(5);

    assert_eq!(root.search_node(5), true);
}


#[test]
fn search_node_test_false() {
    let mut root: Btree<i32>  = Btree::new();
    root.insert(1);
    root.insert(2);
    root.insert(3);
    root.insert(4);
    root.insert(5);

    assert_eq!(root.search_node(99), !true);
}


fn main() {}
