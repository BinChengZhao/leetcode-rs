// convert-sorted-list-to-binary-search-tree
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

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
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut bst: Option<Rc<RefCell<TreeNode>>> = None;

        ListNode::seperate(head, &mut bst);

        return bst;
    }
}

impl TreeNode {
    // TODO:
    // These are unnecessary codes because the input data is an ordered linkedlist
    // I don't need to judge the order with rotation when inserting.

    pub fn push(tree: &mut Option<Rc<RefCell<TreeNode>>>, val: i32) {
        if let Some(mut tree_node) = tree.as_mut().map(|t| t.borrow_mut()) {
            if tree_node.val < val {
                return TreeNode::push(&mut tree_node.right, val);
            } else {
                return TreeNode::push(&mut tree_node.left, val);
            }
        }
        *tree = Some(Rc::new(RefCell::new(TreeNode::new(val))));
    }

    // pub fn left_rotation(&mut self) {
    //     let mut temp_node = None;
    //     swap(&mut temp_node, self);

    //     let mut left = temp_node.left;
    //     swap(&mut left, self);
    //     self.right = temp_node;
    // }

    // pub fn right_rotation(&mut self) {
    //     let mut temp_node = None;
    //     swap(&mut temp_node, self);
    //     let mut right = temp_node.right.take();
    //     swap(self, &mut right);
    //     self.left = temp_node;
    // }

    // pub fn height(&self) -> i32 {
    //     let height = match (&self.left, &self.right) {
    //         (None, None) => 0,
    //         (Some(left), None) => left.borrow().height(),

    //         (None, Some(right)) => right.borrow().height(),
    //         (Some(left), Some(right)) => {
    //             std::cmp::max(right.borrow().height(), left.borrow().height())
    //         }
    //     };
    //     height + 1
    // }
}

impl ListNode {
    pub fn count(list: Option<&Self>) -> usize {
        match list {
            None => 0,
            Some(listnode) => ListNode::count(listnode.next.as_deref()) + 1,
        }
    }

    pub fn seperate(mut list: Option<Box<ListNode>>, tree: &mut Option<Rc<RefCell<TreeNode>>>) {
        let count = ListNode::count(list.as_deref());
        let mut list_ref_mut = list.as_deref_mut();

        if count == 0 {
            return;
        }

        if count == 1 {
            TreeNode::push(tree, list.unwrap().val);

            return;
        }

        let mid = (count / 2) - 1;
        let mut offset = 0;

        while let Some(node) = list_ref_mut {
            if offset == mid {
                // seperate mid..right.
                let mut mid_node = node.next.take().unwrap();
                TreeNode::push(tree, mid_node.val);

                ListNode::seperate(list, tree);

                let right = mid_node.next.take();
                ListNode::seperate(right, tree);
                return;
            }

            list_ref_mut = node.next.as_deref_mut();
            offset += 1;
        }
    }
}

// TODO: This is someone else's solution, too powerful, I have to learn it well.
// type Node = Option<Rc<RefCell<TreeNode>>>;
// impl Solution {
//     pub fn sorted_list_to_bst(mut head: Option<Box<ListNode>>) -> Node {
//         let mut stack = Vec::new();
//         while let Some(node) = head {
//             stack.push(node.val);
//             head = node.next;
//         }
//         Solution::s(&stack[..])
//     }
//      pub fn s(nums: &[i32]) -> Node {
//         let mut n = nums.len();
//         if n == 0 { return  None; }
//         let m = n/2;
//         let mut node = TreeNode::new(nums[m]);
//         node.left = Solution::s(&nums[..m]);
//         node.right = Solution::s(&nums[m + 1..]);
//         Some(Rc::new(RefCell::new(node)))
//     }
// }
