#include <cassert>
#include <iostream>
#include <memory>   // for std::unique_ptr<T>
#include <utility>  // for std::move()
#include <vector>

/// A node of a binary search tree with element type T
template<typename T>
class bst {
    template<typename U>
    friend std::ostream& operator<< (std::ostream&, const bst<U>&);

    T value;                     ///< the value in this node
    std::unique_ptr<bst> left;   ///< The left (less-than) subtree
    std::unique_ptr<bst> right;  ///< The right (greater-than) subtree

    /// Returns either a deep clone of o, or nullptr if o is null
    static std::unique_ptr<bst> clone(const std::unique_ptr<bst>& o) {

        // If o is null, returns nullptr, otherwise returns a deep clone
        return (o) ? std::unique_ptr<bst>(new bst{*o}) : nullptr;
    }

public:
    /// Value constructor; constructs a node by copying the given value, with 
    /// empty left and right subtrees
    bst(const T& v) : value(v), left(), right() {}
    
    /// Use default destructor implementation (unique_ptr destructor will free subtrees)
    ~bst() = default;

    /// Copy constructor. Should perform a deep copy of o.
    bst(const bst& o) 
    : value(o.value), left(clone(o.left)), right(clone(o.right)) {}
    
    /// Move constructor. Steals o's subtrees without any new memory allocations.
    /// (already correct)
    bst(bst&& o) = default;

    /// Copy-assignment operator. Should perform a deep copy of o.
    bst& operator= (const bst& o) {

        // Checks if o is the same as self
        if (&o == this) { return *this; }

        // Reassigns value, left and right to o
        value = o.vlaue;
        left = clone(o.left);
        right = clone(o.left);

        return *this;
    }

    /// Move-assignment operator. Steals o's subtrees without any new memory allocation.
    /// (already correct)
    bst& operator= (bst&& o) = default;

    /// Tree iterator. Acts like a pointer to the tree values, can be advanced to the 
    /// next node using pointer arithmetic. An in-order traversal (left subtree, self, 
    /// right subtree).
    /// Design based on https://medium.com/algorithm-problems/binary-search-tree-iterator-19615ec585a
    class const_iterator {
        // allow BST class private access to implementation details
        friend bst;

        /// Stack of tree nodes that have the current node in their left subtree. 
        /// Equivalently, the path from the root to the current node, skipping nodes 
        /// that have already been seen. Current node is the top of the stack.
        std::vector<const bst*> nodes;

        /// Inserts this node into the iterator stack.
        /// Stores this node and its (recursive) left children on the stack.
        void fill_left(const bst* node) {

            // Checks if the node is non-null
            if (node) {
                // Inserts the node into the iterator stack
                nodes.push_back(node);

                // Recursively inserts the left children on the stack
                fill_left(node->left.get());
            }
        }

        /// Builds a const_iterator for the subtree rooted at node.
        /// Initializes the stack to the path to the leftmost child 
        /// of node.
        const_iterator(const bst& node) : nodes() {
            fill_left(&node);
        }

    public:
        /// Constructs an end iterator (empty list of nodes)
        const_iterator() = default;

        /// True if iterator is non-empty (has any nodes left).
        explicit operator bool() const {
            // Returns true if the iterator is non-empty
            return !nodes.empty();
        }

        /// Go to next node. The current node is removed from the 
        /// node stack, the next node is either the leftmost child 
        /// of the current node's right child, or, if no right child 
        /// exists, the previous node in the stack. This method may 
        /// assume the iterator is non-empty.
        const_iterator& operator++() {

            // Extracts the current node from the stack
            const bst* node = nodes.back();

            // Pops the current node
            nodes.pop_back();

            // Adds the right child and path to its leftmost descendants
            // fill_left checks if node exists
            fill_left(node->right.get());
            
            return *this;
        }

        /// Post-increment operator
        const_iterator operator++(int) {
            const_iterator tmp = *this;
            operator++();
            return tmp;
        }

        /// Dereference operator; returns value of current node.
        /// May assume iterator is non-empty.
        const T& operator* () const {
            
            // Extracts the current node from the stack
            const bst* node = nodes.back();

            return node->value;
        }

        /// Pointer-indirection operator.
        /// May assume iterator is non-empty.
        const T* operator-> () const {
            return &operator*();
        }

        /// Checks equality of two iterators based on both empty or
        /// both have the same current node
        bool operator== (const const_iterator& o) const {
            if ( nodes.empty() != o.nodes.empty() ) return false;
            return nodes.empty() || nodes.back() == o.nodes.back();
        }

        /// Checks inequality of two iterators.
        bool operator!= (const const_iterator& o) const {
            return !operator==(o);
        }
    };

    /// Get an iterator to the first node in the tree
    const_iterator begin() const { return const_iterator{*this}; }
    /// Get an iterator just after the last node in the tree
    const_iterator end() const { return const_iterator{}; }

    /// Add a new value to the tree. The value should be placed in its sorted 
    /// position according to the usual BST rules. Returns true if the value 
    /// was actually added, or false if it was already present.
    bool insert(const T& val) {

        bst* current = this;
        
        // Searches down the binary search tree
        while (current) {

            // If val is less than the current value, left branch is searched
            if (val < current->value) {

                // Creates a new tree is the left node doesn't exist
                if (!current->left) {
                    current->left = std::make_unique<bst<T>>(val);
                    return true;
                }
                current = current->left.get();

            // Otherwise, right branch is searched as val is greater
            } else {

                // Creates a new tree is the right node doesn't exist
                if (!current->right) {
                    current->right = std::make_unique<bst<T>>(val);
                    return true;
                }
                current = current->right.get();
            }
        }

        // Returns false if the value was already present
        return false;
    }
};

/// Prints space-separated in-order traversal of t
template<typename E>
std::ostream& operator<< (std::ostream& out, const bst<E>& t) {
    // note: this could have been implemented based on the iterator without making it 
    // a friend function, but that makes it harder to debug if your iterator 
    // implementation is incorrect

    if ( t.left ) { out << *t.left << " "; }
    out << t.value;
    if ( t.right ) { out << " " << *t.right; }
    return out;
}
