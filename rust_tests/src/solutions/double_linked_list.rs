#[allow(dead_code)] // TODO
pub trait DoubleLinkedList<T> {
    type Node: Copy + PartialEq + std::fmt::Debug;

    fn new(capacity: usize) -> Self;
    fn insert_after(&mut self, node: Self::Node, value: T) -> Self::Node;
    fn insert_before(&mut self, node: Self::Node, value: T) -> Self::Node;
    fn push_back(&mut self, value: T) -> Self::Node;
    fn push_top(&mut self, value: T) -> Self::Node;
    fn delete(&mut self, node: Self::Node);
    fn next(&self, node: Self::Node) -> Option<Self::Node>;
    fn prec(&self, node: Self::Node) -> Option<Self::Node>;
    fn first(&self) -> Option<Self::Node>;
    fn last(&self) -> Option<Self::Node>;
    fn value(&self, node: Self::Node) -> Option<&T>;
    fn value_mut(&mut self, node: Self::Node) -> Option<&mut T>;
}
