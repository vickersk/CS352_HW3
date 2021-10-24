// Code for CS 352 HW 3
// Modified by: Kai Vickers

use std::cmp;
use std::fmt;

/// A binary search tree with element type E
#[derive(Clone,Debug)]
pub struct Bst<E> {
    value: E,
    left: Option<Box<Bst<E>>>,
    right: Option<Box<Bst<E>>>
}

/// Print space-separated in-order traversal of a BST
impl<E: fmt::Display> fmt::Display for Bst<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(left) = &self.left {
            write!(f, "{} ", left)?;
        }
        write!(f, "{}", self.value)?;
        if let Some(right) = &self.right {
            write!(f, " {}", right)?;
        }
        Ok(())
    }
}

/// Iterator for a BST, parameterized over lifetime and element type of the BST.
/// Design based on https://medium.com/algorithm-problems/binary-search-tree-iterator-19615ec585a
#[derive(Debug)]
pub struct BstIter<'a,E> {
    /// Stack of references to tree nodes that have the current node in their left 
    /// subtree. Equivalently, the path from the root to the current node, skipping nodes 
    /// that have already been seen. Current node is the top of the stack.
    nodes: Vec<&'a Bst<E>>
}

/// Methods for BstIter, parameterized over lifetime and element type of the BST 
impl<'a,E> BstIter<'a,E> {
    /// Modifies the current iterator to add [node] and all its 
    /// (recursive) left children
    fn fill_left(&mut self, mut node: &'a Bst<E>) {

        // Adds the node to the iterator stack.
        self.nodes.push(node);

        // Recursively adds the left children if it has a value.
        if let Some(node_left) = &node.left {
            self.fill_left(&node_left)
        }
    }

    /// Creates a new iterator pointing to the leftmost (least) 
    /// child of [node]
    pub fn new(node: &'a Bst<E>) -> BstIter<'a,E> {

        // Builds a BstIter class object with a new Vec.
        let mut iter = BstIter { nodes: Vec::<&Bst<E>>::new() };

        // Initializes the iterator for the root node.
        iter.fill_left(node);
        return iter
    }
}

/// Implements the built-in Iterator trait for BstIter.
/// Allows use of BstIter in, e.g. for loops
impl<'a,E> Iterator for BstIter<'a,E> {
    /// Item type of a BST iterator is a reference to the current 
    /// node's value
    type Item = &'a E;

    /// Returns the current node value (if present), and updates the 
    /// iterator to the next node. The current node is always 
    /// removed from the iterator stack; the next node is either the 
    /// leftmost child of the current node's right child, or, if no 
    /// right child exists, the previous node in the stack.
    fn next(&mut self) -> Option<Self::Item> {
        
        // Pops the current node from the stack, None if otherwise.
        let node = self.nodes.pop();

        // Returns None if nodes is empty
        if let None = node {
            return None
        }

        // If the current node has a value, the right child and path are added.
        if let Some(node_right) = &node.unwrap().right {
            self.fill_left(&node_right);
        }
        
        // Returns the current node value as an Option.
        return Some(&node.unwrap().value)
    }
}

/// Methods for Bst, parameterized over its element type (which must be comparable).
impl<E : cmp::Ord> Bst<E> {
    /// Convenience construction method for BST from fields. 
    /// Should make a new Bst with the given value and empty left and right subtrees.
    pub fn new(value: E) -> Self {

        // Creates a new Bst node with the given value.
        // Sets the left and right subtrees to empty.
        Bst {
            value,
            left: None,
            right: None
        }
    }

    /// Gets the iterator for this BST, starting at the least element.
    pub fn iter(&self) -> BstIter<E> {
        BstIter::new(&self)
    }

    /// Inserts the value into the BST in the proper (sorted) position.
    /// Returns true if inserted, false if already present.
    pub fn insert(&mut self, val: E) -> bool {

        // Checks if the value is already in the tree.
        if val == self.value {
            return false
        }
        
        // If val is less than the current value, the left branch is searched.
        if val < self.value {
            if let Some(left_subtree) = &mut self.left {
                return left_subtree.insert(val)
            }

            // Creates a new tree if the left node doesn't exist.
            self.left = Some(Box::new(Bst::new(val)));
            return true

        // Otherwise, the right branch is searched
        } else {
            if let Some(right_subtree) = &mut self.right {
                return right_subtree.insert(val)
            }

            // Creates a new tree if the right node doesn't exist.
            self.right = Some(Box::new(Bst::new(val)));
            return true
        }
    }
}
