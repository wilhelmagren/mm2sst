/// Implementation of a generic Radix Tree node.
pub struct Node<T> {
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    val: Option<T>,
}

/// Implementation of a generic Radix Tree.
pub struct Tree<T> {
    root: Node<T>,
}
