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
}
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut vec_values = Vec::with_capacity(88);
        Solution::collect_to_vec(root, &mut vec_values);
        vec_values
    }

    pub fn collect_to_vec(root: Option<Rc<RefCell<TreeNode>>>, vec: &mut Vec<i32>) {
        match root {
            None => {
                return;
            }

            Some(t) => {
                let node = t.borrow();
                Solution::collect_to_vec(node.left.clone(), vec);
                vec.push(node.val);
                Solution::collect_to_vec(node.right.clone(), vec);
            }
        }
    }
}
