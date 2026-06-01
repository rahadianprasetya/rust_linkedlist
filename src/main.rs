use core::str;
// Refcell, Immutable outside, but can mutate interior
// Rc, Reference counting pointer
use std::{
    cell::RefCell,
    char,
    cmp::max,
    collections::BTreeMap,
    fmt::{Debug, Display, Write},
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
    h: i32,
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
    pub fn height(&mut self) -> i32 {
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
            -1 => self.rot_right(),
            1 => self.rot_left(),
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

// Skipp List

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

#[derive(Debug)]
pub struct SkipList<T: PartialOrd>(Vec<SkipNode<T>>);

impl<T: PartialOrd> SkipNode<T> {
    pub fn new(t: T) -> Self {
        SkipNode {
            right: None,
            down: None,
            data: rcc(t),
        }
    }

    pub fn insert(&mut self, dt: T) -> Option<Rcc<SkipNode<T>>> {
        // bigger than right -> go right
        if let Some(ref mut rt) = self.right {
            if dt > *rt.borrow().data.borrow() {
                return rt.borrow_mut().insert(dt);
            }
        }

        // try lower layer
        if let Some(ref dw) = self.down {
            return match dw.borrow_mut().insert(dt) {
                Some(child) => match rand::random::<bool>() {
                    true => {
                        let dt = child.borrow().data.clone();

                        let nn = SkipNode {
                            right: self.right.take(),
                            data: dt,
                            down: Some(child),
                        };

                        let res = rcc(nn);

                        self.right = Some(res.clone());

                        Some(res)
                    }

                    false => None,
                },

                None => None,
            };
        }

        // bottom level insertion
        let mut nn = SkipNode::new(dt);

        nn.right = self.right.take();

        let res = rcc(nn);

        self.right = Some(res.clone());

        Some(res)
    }
}

impl<T: PartialOrd> Default for SkipList<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T: PartialOrd> SkipList<T> {
    pub fn new() -> Self {
        SkipList(Vec::new())
    }

    pub fn insert(&mut self, data: T) {
        if self.0.is_empty() {
            self.0.push(SkipNode::new(data));
            return;
        }
        // our vec will have the lowest row, with the lowes number
        for i in (0..self.0.len()).rev() {
            if data > *self.0[i].data.borrow() {
                if let Some(ch) = self.0[i].insert(data) {
                    // TODO loop up on 50:50 chance
                    self.loop_up(ch, i + 1);
                }
                return;
            }
        }
        // if none those succeeded, that means we have an element to replace the first
        let mut nn = SkipNode::new(data);
        // put our new element on the front of the row
        std::mem::swap(&mut nn, &mut self.0[0]);
        let res = rcc(nn);
        self.0[0].right = Some(res.clone());
        self.loop_up(res, 1);
        // TODO loop up on 50:50 chance
    }

    pub fn loop_up(&mut self, ch: Rcc<SkipNode<T>>, n: usize) {
        if rand::random::<bool>() == true {
            return;
        }
        let dt = ch.borrow().data.clone();

        let mut nn = SkipNode {
            right: None,
            down: Some(ch),
            data: dt,
        };

        if n >= self.0.len() {
            self.0.push(nn);
            return;
        }

        std::mem::swap(&mut nn, &mut self.0[n]);
        let res = rcc(nn);
        self.0[n].right = Some(res.clone());
        self.loop_up(res, n + 1);
    }
}

impl<T: Debug + PartialOrd> SkipNode<T> {
    pub fn print_row<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        write!(w, ",{:?}", self.data.borrow())?;
        if let Some(ref r) = self.right {
            r.borrow().print_row(w)?;
        }
        Ok(())
    }
}

impl<T: Debug + PartialOrd> Display for SkipList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            return write!(f, "SkipList<Empty>");
        }
        for sn in &self.0 {
            //write!(f, "\n")?;
            writeln!(f)?;
            sn.print_row(f)?;
        }
        Ok(())
    }
}

// HUFFMAN
#[derive(Debug)]
pub enum HuffNode {
    Tree(Box<HuffNode>, Box<HuffNode>),
    Leaf(char),
}

pub struct HScore {
    h: HuffNode,
    s: i32,
}

impl HuffNode {
    pub fn print_lfirst(&self, depth: i32, dir: char) {
        match self {
            HuffNode::Tree(l, r) => {
                l.print_lfirst(depth + 1, '/');
                let mut spc = String::new();
                for _ in 0..depth {
                    spc.push('.');
                }
                println!("{}{}*", spc, dir);
                r.print_lfirst(depth + 1, '\\');
            }
            HuffNode::Leaf(c) => {
                let mut spc = String::new();
                for _ in 0..depth {
                    spc.push('.');
                }
                println!("{}{}{}", spc, dir, c);
            }
        }
    }

    pub fn encode_char(&self, c: char) -> Option<Vec<char>> {
        // could return vec of bool but chars print nicer
        // once you have this converting it to a byte stream is pretty straight forward
        match self {
            HuffNode::Tree(l, r) => {
                if let Some(mut v) = l.encode_char(c) {
                    v.insert(0, '0');
                    return Some(v);
                }

                if let Some(mut v) = r.encode_char(c) {
                    v.insert(0, '1');
                    return Some(v);
                }
                None
            }
            HuffNode::Leaf(nc) => {
                if c == *nc {
                    return Some(Vec::new());
                } else {
                    None
                }
            }
        }
    }

    pub fn encode_str(&self, s: &str) -> Option<Vec<char>> {
        let mut res = Vec::new();
        for c in s.chars() {
            let v = self.encode_char(c)?;
            res.extend(v.into_iter());
        }
        Some(res)
    }

    pub fn decode_bits(&self, bits: Option<Vec<char>>) -> Option<String> {
        let bits = bits?;
        let mut result = String::new();
        let mut current = self;

        for &bit in &bits {
            current = match (current, bit) {
                (HuffNode::Tree(l, _), '0') => l,
                (HuffNode::Tree(_, r), '1') => r,
                _ => return None,
            };

            if let HuffNode::Leaf(c) = current {
                result.push(*c);
                current = self;
            }
        }

        if !std::ptr::eq(current, self) {
            return None;
        }

        Some(result)
    }
}

pub fn build_tree(s: &str) -> HuffNode {
    let mut map = BTreeMap::new();

    for c in s.chars() {
        // if map has already add 1 else put 1
        let n = *map.get(&c).unwrap_or(&0);
        map.insert(c, n + 1);
    }

    let mut tlist: Vec<HScore> = map
        .into_iter()
        .map(|(k, s)| HScore {
            h: HuffNode::Leaf(k),
            s,
        })
        .collect();

    while tlist.len() > 1 {
        let last = tlist.len() - 1;
        for i in 0..last - 1 {
            if tlist[i].s < tlist[last - 1].s {
                tlist.swap(i, last - 1);
            }
            if tlist[last - 1].s < tlist[last].s {
                tlist.swap(last - 1, last);
            }
        }
        let a_node = tlist.pop().unwrap(); // len >=2
        let b_node = tlist.pop().unwrap();
        let nnode = HuffNode::Tree(Box::new(a_node.h), Box::new(b_node.h));
        tlist.push(HScore {
            h: nnode,
            s: a_node.s + b_node.s,
        });
    }
    tlist.pop().unwrap().h
}

#[cfg(test)]
mod tests {
    use std::collections::LinkedList;

    use crate::{BinTree, DbList, SkipList, SkipNode, build_tree};

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

    #[test]
    fn test_balance_bintree() {
        let mut t = BinTree::new();
        t.add_sorted(4);
        t.add_sorted(5);
        t.add_sorted(6);
        t.add_sorted(3);
        t.add_sorted(2);
        t.add_sorted(10);
        t.print_lfirst(0);

        println!("--------");
        t.rot_left();
        t.print_lfirst(0);
    }

    #[test]
    fn test_balance_bintree_latest() {
        let mut t = BinTree::new();
        t.add_sorted(4);
        t.add_sorted(5);
        t.add_sorted(6);
        t.add_sorted(3);
        t.add_sorted(2);
        t.add_sorted(10);

        for i in 0..100000 {
            t.add_sorted(i);
        }
        t.print_lfirst(0);
    }

    #[test]
    fn test_skipp_node() {
        let mut snode = SkipNode::new(4);
        snode.insert(4);
        snode.insert(6);
        snode.insert(77);
        snode.insert(88);
        snode.insert(23);
        println!("s-{:?}", snode);
    }

    #[test]
    fn test_skipp_list() {
        let mut s = SkipList::new();
        s.insert(4);
        s.insert(6);
        s.insert(77);
        s.insert(88);
        s.insert(23);
        println!("s={}", s);
    }

    #[test]
    fn test_huffman() {
        let s = "at an apple app";
        let t = build_tree(s);
        t.print_lfirst(0, '<');
    }

    #[test]
    fn test_huffman_encode() {
        let s = "at an apple app";
        let t = build_tree(s);
        t.print_lfirst(0, '<');

        println!("n = {:?}", t.encode_char('n'));
    }

    #[test]
    fn test_huffman_encode_str() {
        let s = "at an apple app";
        let t = build_tree(s);
        t.print_lfirst(0, '<');

        println!("n = {:?}", t.encode_char('n'));

        let encoded = t.encode_str(s).expect("encode gagal");
        println!("encoded = {:?}", encoded);
    }

    #[test]
    fn test_huffman_decode() {
        let s = "at an apple app";
        let t = build_tree(s);
        t.print_lfirst(0, '<');

        println!("n = {:?}", t.encode_char('n'));

        let res = t.encode_str(s);
        println!("en = {:?}", res);
        println!("db = {:?}", t.decode_bits(res));
    }
}
