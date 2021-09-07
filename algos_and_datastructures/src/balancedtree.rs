use std::fmt;

#[derive(Debug)]
pub struct BinTree<T>(Option<Box<BinData<T>>>);

impl<T> BinData<T> {
    pub fn rot_left(mut self) -> Box<Self> {
        // right is now root, self is left
        // right's left moves to self (left) right
        let mut res = match self.right.0.take() {
            Some(res) => res,
            None => return Box::new(self),
        };
        self.right = BinTree(res.left.0.take());
        self.right.set_height();
        res.left = BinTree(Some(Box::new(self)));
        res.left.set_height();
        res.h = 1 + std::cmp::max(res.left.height(), res.right.height());
        res
    }

    pub fn rot_right(mut self) -> Box<Self> {
        // right is now root, self is left
        // right's left moves to self (left) right
        let mut res = match self.left.0.take() {
            Some(res) => res,
            None => return Box::new(self),
        };
        self.left = BinTree(res.right.0.take());
        self.left.set_height();
        res.right = BinTree(Some(Box::new(self)));
        res.right.set_height();
        res.h = 1 + std::cmp::max(res.left.height(), res.right.height());
        res
    }
}

#[derive(Debug)]
pub struct BinData<T> {
    data: T,
    h: i8, // single byte 0-128 -> 10^30 nodes before overflow, Red and Black is other option with more complex math
    left: BinTree<T>,
    right: BinTree<T>,
}

impl<T> BinTree<T> {
    pub fn new() -> Self {
        BinTree(None)
    }

    pub fn height(&self) -> i8 {
        match self.0 {
            Some(ref t) => t.h,
            None => 0,
        }
    }

    pub fn set_height(&mut self) {
        if let Some(ref mut t) = self.0 {
            t.h = 1 + std::cmp::max(t.left.height(), t.right.height());
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
        let rot_dir = match self.0 {
            Some(ref mut bd) => {
                if data < bd.data {
                    bd.left.add_sorted(data);
                    if bd.left.height() - bd.right.height() > 1 {
                        1
                    } else {
                        0
                    }
                } else {
                    bd.right.add_sorted(data);
                    if bd.right.height() - bd.left.height() > 1 {
                        -1
                    } else {
                        0
                    }
                }
            }
            None => {
                self.0 = Some(Box::new(BinData {
                    data,
                    h: 0,
                    left: BinTree(None),
                    right: BinTree(None),
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

impl<T: fmt::Debug> BinTree<T> {
    pub fn print_left_first(&self, dp: i32) {
        if let Some(ref bd) = self.0 {
            bd.left.print_left_first(dp + 1);
            let mut spc = String::new();
            for _ in 0..dp {
                spc.push('.');
            }
            println!("{}{}{:?}", bd.h, spc, bd.data);
            bd.right.print_left_first(dp + 1);
        }
    }
}
