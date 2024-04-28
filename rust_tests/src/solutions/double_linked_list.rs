use tests_api::TheAlloc;

#[allow(dead_code)] // TODO
pub trait DoubleLinkedList<'x, T> {
    type NodeRef: Copy + PartialEq + std::fmt::Debug;

    /// Creates a list.
    fn new(alloc: &'x TheAlloc, capacity: usize) -> Self;

    fn insert_after(&mut self, node: Self::NodeRef, value: T) -> Self::NodeRef;
    fn insert_before(&mut self, node: Self::NodeRef, value: T) -> Self::NodeRef;
    fn push_back(&mut self, value: T) -> Self::NodeRef;
    fn push_front(&mut self, value: T) -> Self::NodeRef;

    unsafe fn delete(&mut self, node: Self::NodeRef);

    fn next(&self, node: Self::NodeRef) -> Option<Self::NodeRef>;
    fn prec(&self, node: Self::NodeRef) -> Option<Self::NodeRef>;
    fn search<F: Fn(&T) -> bool>(&self, f: F) -> Option<Self::NodeRef> {
        let mut it = self.first();
        while let Some(x) = it {
            if f(self.value(x)?) {
                return Some(x);
            }
            it = self.next(x);
        }
        None
    }

    fn first(&self) -> Option<Self::NodeRef>;
    fn last(&self) -> Option<Self::NodeRef>;

    fn value(&self, node: Self::NodeRef) -> Option<&T>;
    fn value_mut(&mut self, node: Self::NodeRef) -> Option<&mut T>;
}