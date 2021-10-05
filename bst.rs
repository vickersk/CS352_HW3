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
        unimplemented!()
    }

    /// Creates a new iterator pointing to the leftmost (least) 
    /// child of [node]
    pub fn new(node: &'a Bst<E>) -> BstIter<'a,E> {
        unimplemented!()
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
        unimplemented!()
    }
}

/// Methods for Bst, parameterized over its element type (which must be comparable).
impl<E : cmp::Ord> Bst<E> {
    /// Convenience construction method for BST from fields. 
    /// Should make a new Bst with the given value and empty left and right subtrees.
    pub fn new(value: E) -> Self {
        unimplemented!()
    }

    /// Gets the iterator for this BST, starting at the least element.
    pub fn iter(&self) -> BstIter<E> {
        BstIter::new(&self)
    }

    /// Inserts the value into the BST in the proper (sorted) position.
    /// Returns true if inserted, false if already present.
    pub fn insert(&mut self, val: E) -> bool {
        unimplemented!()
    }
}
