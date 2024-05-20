use super::DoubleLinkedList;
use core::fmt::Debug;
use std::rc::Weak;
use std::{cell::RefCell, rc::Rc};
use tests_api::TheAlloc;

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    prev: Option<NodeRef<T>>,
    next: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Debug, Clone)]
pub struct NodeRef<T>(Weak<RefCell<Node<T>>>);

impl<T> PartialEq for NodeRef<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.ptr_eq(&other.0)
    }
}

pub struct Implementation<'x, T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    alloc: &'x TheAlloc,
}

impl<'x, T> Implementation<'x, T> {
    fn allocate_node(&mut self, value: T) -> Rc<RefCell<Node<T>>> {
        // Rc::new_in(
        //     RefCell::new(Node {
        //         value,
        //         prev: None,
        //         next: None,
        //     }),
        //     self.alloc,
        // )
        Rc::new(RefCell::new(Node {
            value,
            prev: None,
            next: None,
        }))
    }
}

impl<'x, T: Clone + PartialEq + Debug> DoubleLinkedList<'x, T> for Implementation<'x, T> {
    type NodeRef = NodeRef<T>;

    fn new(alloc: &'x TheAlloc, _capacity: usize) -> Self {
        Implementation {
            head: None,
            tail: None,
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
                    self.tail = Some(new_node.clone());
                }

                node_borrow.next = Some(new_node.clone());
            }

            NodeRef(Rc::downgrade(&new_node))
        } else {
            NodeRef(Weak::new())
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
                    self.head = Some(new_node.clone());
                }

                node_borrow.prev = Some(NodeRef(Rc::downgrade(&new_node)));
            }

            NodeRef(Rc::downgrade(&new_node))
        } else {
            NodeRef(Weak::new())
        }
    }

    fn push_back(&mut self, value: T) -> Self::NodeRef {
        let new_node = self.allocate_node(value);
        {
            if let Some(tail_node) = self.tail.clone() {
                tail_node.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(NodeRef(Rc::downgrade(&tail_node)));
            } else {
                self.head = Some(new_node.clone());
            }
            self.tail = Some(new_node.clone());
        }

        NodeRef(Rc::downgrade(&new_node))
    }

    fn push_front(&mut self, value: T) -> Self::NodeRef {
        let new_node = self.allocate_node(value);
        {
            if let Some(head_node) = self.head.clone() {
                head_node.borrow_mut().prev = Some(NodeRef(Rc::downgrade(&new_node)));
                new_node.borrow_mut().next = Some(head_node);
            } else {
                self.tail = Some(new_node.clone());
            }
            self.head = Some(new_node.clone());
        }

        NodeRef(Rc::downgrade(&new_node))
    }

    unsafe fn delete(&mut self, node: Self::NodeRef) {
        if let Some(node_rc) = node.0.upgrade() {
            let node_borrow = node_rc.borrow();
            if let Some(prev_node) = node_borrow.prev.clone().and_then(|p| p.0.upgrade()) {
                prev_node.borrow_mut().next = node_borrow.next.clone();
            } else {
                self.head = node_borrow.next.clone();
            }

            if let Some(next_node) = node_borrow.next.clone() {
                next_node.borrow_mut().prev = node_borrow.prev.clone();
            } else {
                self.tail = node_borrow.prev.clone().and_then(|p| p.0.upgrade());
            }
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
        self.head.as_ref().map(|rc| NodeRef(Rc::downgrade(rc)))
    }

    fn last(&self) -> Option<Self::NodeRef> {
        self.tail.as_ref().map(|rc| NodeRef(Rc::downgrade(rc)))
    }

    fn value(&self, node: Self::NodeRef) -> Option<&T> {
        node.0.upgrade().map(|node_rc| {
            let node_ref: &Node<T> = &*node_rc.borrow();
            let x = &node_ref.value as *const T;
            unsafe { &*x as &T }
        })
    }

    fn value_mut(&mut self, node: Self::NodeRef) -> Option<&mut T> {
        node.0.upgrade().map(|node_rc| {
            let node_mut: &mut Node<T> = &mut *node_rc.borrow_mut();
            let x = &mut node_mut.value as *mut T;
            unsafe { &mut *x as &mut T }
        })
    }
}
