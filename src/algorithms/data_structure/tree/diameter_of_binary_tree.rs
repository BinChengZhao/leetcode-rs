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
use std::cmp::max;
use std::rc::Rc;
struct Solution;
impl Solution {
    // For every node, length of longest path which pass it = MaxDepth of its left subtree + MaxDepth of its right subtree.

    // The root node is not necessarily the longest diameter node,
    // example：
    // [4,-7,-3,null,null,-9,-3,9,-7,-4,null,6,null,-6,-6,null,null,0,6,5,null,9,null,null,-1,-4,null,null,null,-2]

    // So each node needs to calculate whether its own diameter is the longest
    // when calculating depth.

    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_diameter = 0;
        Solution::check_level(root, &mut max_diameter);
        max_diameter
    }

    pub fn check_level(root: Option<Rc<RefCell<TreeNode>>>, max_diameter: &mut i32) -> i32 {
        match root {
            None => 0,

            Some(root_node) => {
                let left_level =
                    Solution::check_level(root_node.borrow().left.clone(), max_diameter);
                let right_level =
                    Solution::check_level(root_node.borrow().right.clone(), max_diameter);

                let node_max_level = max(left_level, right_level) + 1;

                *max_diameter = max(*max_diameter, left_level + right_level);
                node_max_level
            }
        }
    }
}
