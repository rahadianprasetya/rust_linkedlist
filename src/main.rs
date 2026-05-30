// Refcell, Immutable outside, but can mutate interior
// Rc, Reference counting pointer
use std::{
    cell::RefCell,
    cmp::max,
    fmt::Debug,
    rc::{Rc, Weak},
};

fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
pub struct LinkedList<T>(Option<(T, Box<LinkedList<T>>)>);

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList(None)
    }

    pub fn push_front(&mut self, data: T) {
        let t = self.0.take();
        self.0 = Some((data, Box::new(LinkedList(t))));
    }

    pub fn push_back(&mut self, data: T) {
        match self.0 {
            Some((_, ref mut child)) => child.push_back(data),
            None => self.push_front(data),
        }
    }
}

/*
 * Double LinkedList
 */
#[derive(Debug)]
#[allow(dead_code)]
pub struct DbNode<T> {
    data: T,
    next: Option<Rc<RefCell<DbNode<T>>>>,
    prev: Option<Weak<RefCell<DbNode<T>>>>,
}

#[derive(Debug)]
pub struct DbList<T> {
    first: Option<Rc<RefCell<DbNode<T>>>>,
    last: Option<Weak<RefCell<DbNode<T>>>>,
}

impl<T> Default for DbList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> DbList<T> {
    pub fn new() -> Self {
        DbList {
            first: None,
            last: None,
        }
    }

    pub fn push_front(&mut self, data: T) {
        match self.first.take() {
            Some(r) => {
                // create new front_object
                let new_front = Rc::new(RefCell::new(DbNode {
                    data,
                    next: Some(r.clone()),
                    prev: None,
                }));
                // tell the object now this is now in front of it
                let mut m = r.borrow_mut();
                m.prev = Some(Rc::downgrade(&new_front));
                // put this on the front
                self.first = Some(new_front);
            }
            None => {
                let new_data = Rc::new(RefCell::new(DbNode {
                    data,
                    next: None,
                    prev: None,
                }));
                self.last = Some(Rc::downgrade(&new_data));
                self.first = Some(new_data);
            }
        }
    }

    pub fn push_back(&mut self, data: T) {
        match self.last.take() {
            Some(r) => {
                // create new back object
                let new_back = Rc::new(RefCell::new(DbNode {
                    data,
                    prev: Some(r.clone()),
                    next: None,
                }));
                // tell the object now this is now in behind of it
                let st = Weak::upgrade(&r).unwrap();
                let mut m = st.borrow_mut();
                self.last = Some(Rc::downgrade(&new_back));
                m.next = Some(new_back);
                // put this on the front
            }
            None => {
                let new_data = Rc::new(RefCell::new(DbNode {
                    data,
                    next: None,
                    prev: None,
                }));
                self.last = Some(Rc::downgrade(&new_data));
                self.first = Some(new_data);
            }
        }
    }
}

// binary tree
#[derive(Debug)]
pub struct BinTree<T>(Option<Box<BinData<T>>>);

#[derive(Debug)]
pub struct BinData<T> {
    data: T,
    h: i8,
    left: BinTree<T>,
    right: BinTree<T>,
}

impl<T> BinData<T> {
    pub fn rot_left(mut self) -> Box<Self> {
        // result is the right node
        let mut res = match self.right.0.take() {
            Some(res) => res,
            None => return Box::new(self), // No right node how can we wrote?
        };
        // move left of right node to right of start node
        self.right = BinTree(res.left.0.take());
        self.right.set_height();

        res.left = BinTree(Some(Box::new(self)));
        res.left.set_height();
        res.h = 1 + max(res.left.height(), res.right.height());
        res
    }

    pub fn rot_right(mut self) -> Box<Self> {
        // result is the right node
        let mut res = match self.left.0.take() {
            Some(res) => res,
            None => return Box::new(self), // No right node how can we wrote?
        };
        // move left of right node to right of start node
        self.left = BinTree(res.right.0.take());
        self.left.set_height();

        res.right = BinTree(Some(Box::new(self)));
        res.right.set_height();
        res.h = 1 + max(res.left.height(), res.right.height());
        res
    }
}

impl<T> Default for BinTree<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> BinTree<T> {
    pub fn new() -> Self {
        BinTree(None)
    }
    pub fn height(&mut self) -> i8 {
        match self.0 {
            Some(ref t) => t.h,
            None => 0,
        }
    }
    pub fn set_height(&mut self) {
        if let Some(ref mut t) = self.0 {
            t.h = 1 + max(t.left.height(), t.right.height())
        }
    }

    pub fn rot_left(&mut self) {
        self.0 = self.0.take().map(|v| v.rot_left());
    }

    pub fn rot_right(&mut self) {
        self.0 = self.0.take().map(|v| v.rot_right());
    }
}

impl<T: PartialOrd> BinTree<T> {
    pub fn add_sorted(&mut self, data: T) {
        let rot_dir = match self.0.as_mut() {
            Some(bd) => {
                let dir = if data < bd.data {
                    bd.left.add_sorted(data);

                    if bd.left.height() - bd.right.height() > 1 {
                        -1
                    } else {
                        0
                    }
                } else {
                    bd.right.add_sorted(data);

                    if bd.right.height() - bd.left.height() > 1 {
                        1
                    } else {
                        0
                    }
                };

                self.set_height();

                dir
            }

            None => {
                self.0 = Some(Box::new(BinData {
                    data,
                    h: 0,
                    left: BinTree::new(),
                    right: BinTree::new(),
                }));

                0
            }
        };

        match rot_dir {
            1 => self.rot_right(),
            -1 => self.rot_left(),
            _ => self.set_height(),
        }
    }
}

impl<T: Debug> BinTree<T> {
    pub fn print_lfirst(&self, dp: i32) {
        if let Some(ref bd) = self.0 {
            bd.left.print_lfirst(dp + 1);
            let mut spaces = String::new();
            for _ in 0..dp {
                spaces.push('.');
            }
            println!("{}:{}{:?}", bd.h, spaces, bd.data);
            bd.right.print_lfirst(dp + 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::LinkedList;

    use crate::{BinTree, DbList};

    #[test]
    fn test_linkedlist() {
        let mut ll = LinkedList::new();
        ll.push_front(3);
        ll.push_back(12);
        ll.push_front(1);

        println!("ll {:?}", ll);
    }

    #[test]
    fn test_dblist() {
        let mut dl = DbList::new();
        dl.push_front(6);
        dl.push_back(11);
        dl.push_front(5);
        println!("DbList {:?}", dl);
    }

    #[test]
    fn test_bintree() {
        let mut t = BinTree::new();
        t.add_sorted(4);
        t.add_sorted(5);
        t.add_sorted(6);
        t.add_sorted(3);
        t.add_sorted(2);
        t.add_sorted(10);
        t.print_lfirst(0);
    }
}
