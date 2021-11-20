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
pub struct Solution;
impl Solution {
    // Recursively traverses whether two trees are equal.
    pub fn is_subtree(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if root.is_none() && sub_root.is_none() {
            return true;
        } else if root.is_some() && sub_root.is_some() {
            if Solution::compare_tree(root.clone(), sub_root.clone()) {
                return true;
            }

            // If they are not equal
            // The left and right subtrees are used to make the determination.

            let left = root.as_mut().map(|r| r.borrow_mut().left.take()).flatten();

            if Solution::is_subtree(left, sub_root.clone()) {
                return true;
            }

            let right = root.as_mut().map(|r| r.borrow_mut().right.take()).flatten();

            if Solution::is_subtree(right, sub_root) {
                return true;
            }

            return false;
        } else {
            return false;
        }
    }

    pub fn compare_tree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (root, sub_root) {
            (None, None) => true,
            (Some(root_node), Some(sub_node)) => {
                let root_node = root_node.borrow_mut();
                let sub_node = sub_node.borrow_mut();
                if root_node.val != sub_node.val {
                    return false;
                }

                // When comparing, don't using `take`,
                // Otherwise all subsequent subtrees will be destroyed during the first comparison,
                // and subsequent comparisons will not be possible.
                if !Solution::compare_tree(root_node.left.clone(), sub_node.left.clone()) {
                    return false;
                }

                if !Solution::compare_tree(root_node.right.clone(), sub_node.right.clone()) {
                    return false;
                }

                return true;
            }
            _ => {
                return false;
            }
        }
    }
}

// This is written by others is really too beautiful.
// impl Solution {
//     pub fn is_subtree(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
//         match (root, sub_root) {
//             (Some(root_node), Some(sub_root_node))if root_node == sub_root_node => return true,
//             (None, _) => return false,
//             (Some(root_node), Some(sub_root_node)) => {
//                 let left = root_node.borrow_mut().left.take();
//                 let right = root_node.borrow_mut().right.take();

//                 Self::is_subtree(left, Some(sub_root_node.clone())) ||
//                     Self::is_subtree(right, Some(sub_root_node))
//             }
//             _ => return false
//         }
//     }
// }
