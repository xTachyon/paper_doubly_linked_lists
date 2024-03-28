pub trait DoubleLinkedList<T> {
    type Node;
    fn insert_after(&mut self, node: Node, value: T)->Node;
    fn insert_before(&mut self, node: Node, value: T)->Node;
    fn push_back(&mut self, value: T)->Node;
    fn push_top(&mut self, value: T)->Node;
    fn delete(&mut self, node: Node);
    fn next(&self, node: Node)->Option<Node>;
    fn prec(&self, node: Node)->Option<Node>;
    fn first(&self)->Option<Node>;
    fn last(&self)->Option<Node>;
    fn value(&self, node: Node)->Option<&T>;
    fn value_mut(&mut self, node: Node)->Option<&mut T>;
}