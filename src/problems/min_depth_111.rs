use std::cell::{RefCell};
use std::cmp;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
    // pub fn from(vals: Vec<Option<i32>>)-> Option<Rc<RefCell<Self>>> {
    //   if vals.is_empty() {
    //     return None;
    //   }
    //   let root = Rc::new(RefCell::new(Self::new(val)));
    //   for val in vals[1..].to_vec() {
    //     if let Some(val) = val {
    //       let node = Rc::new(RefCell::new(Self::new(val)));
    //     }
    //   }
    //   let first = vals[0];

    //   let rest = vals[1..].to_vec();

    //   let root = Self::new(vals);
    //   vals.iter().next().fold(init, f)
    // }
}

pub struct Solution;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let node = node.borrow();
                // let left = node.left.clone();
                // let right = node.right.clone();
                match (&node.left, &node.right) {
                    (None, None) => 1,
                    (None, Some(right)) => 1 + Self::min_depth(Some(right.clone())),
                    (Some(left), None) => 1 + Self::min_depth(Some(left.clone())),
                    (Some(left), Some(right)) => {
                        1 + cmp::min(
                            Self::min_depth(Some(left.clone())),
                            Self::min_depth(Some(right.clone())),
                        )
                    }
                }
            }
        }
    }
}