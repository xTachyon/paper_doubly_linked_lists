use super::DoubleLinkedList;
use core::fmt::Debug;
use std::rc::Weak;
use std::{cell::RefCell, rc::Rc};
use tests_api::TheAlloc;

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    prev: Option<NodeRef<T>>,
    next: Option<R<T>>,
}

impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        loop {
            let Some(next) = self.next.take() else {
                return;
            };
            let next = next.borrow_mut().next.take();
            self.next = next;
        }
    }
}

type R<T> = Rc<RefCell<Node<T>>, &'static TheAlloc>;
type W<T> = Weak<RefCell<Node<T>>, &'static TheAlloc>;

#[derive(Debug, Clone)]
pub struct NodeRef<T>(W<T>);

impl<T> PartialEq for NodeRef<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.ptr_eq(&other.0)
    }
}

pub struct Implementation<T> {
    first: Option<R<T>>,
    last: Option<R<T>>,
    alloc: &'static TheAlloc,
}

impl<T> Implementation<T> {
    fn allocate_node(&mut self, value: T) -> R<T> {
        // Rc::new_in(
        //     RefCell::new(Node {
        //         value,
        //         prev: None,
        //         next: None,
        //     }),
        //     self.alloc,
        // )
        Rc::new_in(
            RefCell::new(Node {
                value,
                prev: None,
                next: None,
            }),
            self.alloc,
        )
    }
}

impl<'x, T: Clone + Debug> DoubleLinkedList<'x, T> for Implementation<T> {
    type NodeRef = NodeRef<T>;

    fn new(alloc: &'static TheAlloc, _capacity: usize) -> Self {
        Implementation {
            first: None,
            last: None,
            alloc,
        }
    }

    fn insert_after(&mut self, node: Self::NodeRef, value: T) -> Self::NodeRef {
        if let Some(node_rc) = node.0.upgrade() {
            let new_node = self.allocate_node(value);
            {
                let mut node_borrow = node_rc.borrow_mut();
                new_node.borrow_mut().prev = Some(NodeRef(Rc::downgrade(&node_rc)));
                new_node.borrow_mut().next = node_borrow.next.clone();

                if let Some(next_node) = node_borrow.next.clone() {
                    next_node.borrow_mut().prev = Some(NodeRef(Rc::downgrade(&new_node)));
                } else {
                    self.last = Some(new_node.clone());
                }

                node_borrow.next = Some(new_node.clone());
            }

            NodeRef(Rc::downgrade(&new_node))
        } else {
            panic!("insert_after called with node not in list");
        }
    }

    fn insert_before(&mut self, node: Self::NodeRef, value: T) -> Self::NodeRef {
        if let Some(node_rc) = node.0.upgrade() {
            let new_node = self.allocate_node(value);
            {
                let mut node_borrow = node_rc.borrow_mut();
                new_node.borrow_mut().next = Some(node_rc.clone());
                new_node.borrow_mut().prev = node_borrow.prev.clone();

                if let Some(prev_node) = node_borrow.prev.clone().and_then(|p| p.0.upgrade()) {
                    prev_node.borrow_mut().next = Some(new_node.clone());
                } else {
                    self.first = Some(new_node.clone());
                }

                node_borrow.prev = Some(NodeRef(Rc::downgrade(&new_node)));
            }

            NodeRef(Rc::downgrade(&new_node))
        } else {
            panic!("insert_before called with node not in list");
        }
    }

    fn push_back(&mut self, value: T) -> Self::NodeRef {
        let new_node = self.allocate_node(value);
        {
            if let Some(tail_node) = self.last.clone() {
                tail_node.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(NodeRef(Rc::downgrade(&tail_node)));
            } else {
                self.first = Some(new_node.clone());
            }
            self.last = Some(new_node.clone());
        }

        NodeRef(Rc::downgrade(&new_node))
    }

    fn push_front(&mut self, value: T) -> Self::NodeRef {
        let new_node = self.allocate_node(value);
        {
            if let Some(head_node) = self.first.clone() {
                head_node.borrow_mut().prev = Some(NodeRef(Rc::downgrade(&new_node)));
                new_node.borrow_mut().next = Some(head_node);
            } else {
                self.last = Some(new_node.clone());
            }
            self.first = Some(new_node.clone());
        }

        NodeRef(Rc::downgrade(&new_node))
    }

    unsafe fn delete(&mut self, node: Self::NodeRef) {
        if let Some(node_rc) = node.0.upgrade() {
            let node_borrow = node_rc.borrow();
            if let Some(prev_node) = node_borrow.prev.clone().and_then(|p| p.0.upgrade()) {
                prev_node.borrow_mut().next = node_borrow.next.clone();
            } else {
                self.first = node_borrow.next.clone();
            }

            if let Some(next_node) = node_borrow.next.clone() {
                next_node.borrow_mut().prev = node_borrow.prev.clone();
            } else {
                self.last = node_borrow.prev.clone().and_then(|p| p.0.upgrade());
            }
        } else {
            panic!("delete called with node not in list");
        }
    }

    fn next(&self, node: Self::NodeRef) -> Option<Self::NodeRef> {
        if let Some(node_rc) = node.0.upgrade() {
            node_rc
                .borrow()
                .next
                .as_ref()
                .map(|rc| NodeRef(Rc::downgrade(rc)))
        } else {
            None
        }
    }

    fn prec(&self, node: Self::NodeRef) -> Option<Self::NodeRef> {
        if let Some(node_rc) = node.0.upgrade() {
            node_rc.borrow().prev.clone()
        } else {
            None
        }
    }

    fn first(&self) -> Option<Self::NodeRef> {
        self.first.as_ref().map(|rc| NodeRef(Rc::downgrade(rc)))
    }

    fn last(&self) -> Option<Self::NodeRef> {
        self.last.as_ref().map(|rc| NodeRef(Rc::downgrade(rc)))
    }

    fn value(&self, node: Self::NodeRef) -> Option<&T> {
        node.0.upgrade().map(|node_rc| {
            let node_ref: &Node<T> = &node_rc.borrow();
            let x = &node_ref.value as *const T;
            unsafe { &*x as &T }
        })
    }

    fn value_mut(&mut self, node: Self::NodeRef) -> Option<&mut T> {
        node.0.upgrade().map(|node_rc| {
            let node_mut: &mut Node<T> = &mut node_rc.borrow_mut();
            let x = &mut node_mut.value as *mut T;
            unsafe { &mut *x as &mut T }
        })
    }
}
