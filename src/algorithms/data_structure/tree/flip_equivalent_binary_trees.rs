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
    pub fn flip_equiv(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        Solution::rec_check(root1, root2)
    }

    pub fn rec_check(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if root1.is_none() && root2.is_none() {
            return true;
        } else if root1.is_some() && root2.is_some() {
            let root1_ref = root1.as_ref().unwrap().borrow();
            let root2_ref = root2.as_ref().unwrap().borrow();

            if root1_ref.val != root2_ref.val {
                return false;
            }

            let root1_left_value = root1_ref.left.as_ref().map(|l| l.borrow().val);
            let root1_right_value = root1_ref.right.as_ref().map(|l| l.borrow().val);

            let root2_left_value = root2_ref.left.as_ref().map(|l| l.borrow().val);
            let root2_right_value = root2_ref.right.as_ref().map(|r| r.borrow().val);

            if root1_left_value != root2_left_value && root1_left_value != root2_right_value {
                return false;
            }

            if root1_right_value != root2_left_value && root1_right_value != root2_right_value {
                return false;
            }

            if !Solution::rec_check(root1_ref.left.clone(), root2_ref.left.clone()) {
                if !Solution::rec_check(root1_ref.left.clone(), root2_ref.right.clone()) {
                    return false;
                }
            }

            if !Solution::rec_check(root1_ref.right.clone(), root2_ref.right.clone()) {
                if !Solution::rec_check(root1_ref.right.clone(), root2_ref.left.clone()) {
                    return false;
                }
            }

            return true;
        } else {
            return false;
        }
    }
}
