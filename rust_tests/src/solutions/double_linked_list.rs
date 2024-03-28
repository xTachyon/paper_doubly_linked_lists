pub trait DoubleLinkedList<T> {
    type Node;
    fn insert_after(node: Node, value: T)->Node;
    fn insert_before(node: Node, value: T)->Node;
    fn push_back(value: T)->Node;
    fn push_top(value: T)->Node;
    fn delete(node: Node);
    fn next(node: Node)->Option<Node>;
    fn prec(node: Node)->Option<Node>;
    fn first()->Option<Node>;
    fn last()->Option<Node>;
}