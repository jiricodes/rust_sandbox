//! Skiplist is a stacked linked list where every layer has about half nodes than the previous one
//! Some skiplists implement also up and left connections

use rand;
use std::cell::RefCell;
use std::rc::Rc;

type Rcc<T> = Rc<RefCell<T>>;
pub fn rcc<T>(t: T) -> Rcc<T> {
    Rc::new(RefCell::new(t))
}

#[derive(Debug)]
pub struct SkipNode<T: PartialOrd> {
    right: Option<Rcc<SkipNode<T>>>,
    down: Option<Rcc<SkipNode<T>>>,
    data: Rcc<T>,
}

impl<T: PartialOrd> SkipNode<T> {
    pub fn new(data: T) -> Self {
        SkipNode {
            right: None,
            down: None,
            data: rcc(data),
        }
    }

    pub fn insert(&mut self, dt: T) -> Option<Rcc<SkipNode<T>>> {
        // bigger than right, go right
        if let Some(ref mut rt) = self.right {
            if dt > *rt.borrow().data.borrow() {
                return rt.borrow_mut().insert(dt);
            }
        }

        // has lower children, try them
        if let Some(ref dw) = self.down {
            return match dw.borrow_mut().insert(dt) {
                Some(child) => match rand::random::<bool>() {
                    true => {
                        let down_data = child.borrow().data.clone();
                        let new_node = SkipNode {
                            right: self.right.take(),
                            data: down_data,
                            down: Some(child),
                        };
                        let res = rcc(new_node);
                        self.right = Some(res.clone());
                        Some(res)
                    }
                    false => None,
                },
                None => None,
            };
        }

        // we're in correct place. need to add node in between this and right
        let mut new_node = SkipNode::new(dt);
        new_node.right = self.right.take();
        let res = rcc(new_node);
        self.right = Some(res.clone());
        Some(res)
    }
}
