struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}