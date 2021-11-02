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

struct Solution;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let (_, node) = Solution::check_num_and_remove(head.unwrap(), n);
        node
    }

    // 1. The first step is to get the number of each node.
    // 2. Remove the node with number = N.
    pub fn check_num_and_remove(mut node: Box<ListNode>, n: i32) -> (i32, Option<Box<ListNode>>) {
        // Access the node recursively,
        // And mark the number 1 at the end of the last node as the end of the recursion.
        if node.next.is_none() {
            if n == 1 {
                return (1, node.next);
            }
            return (1, Some(node));
        }

        // Set self number by next node number + 1.
        let (next_num, next) = Solution::check_num_and_remove(node.next.take().unwrap(), n);
        let num = next_num + 1;
        // If self.number == N, So ignore it just return next node.
        if num == n {
            return (num, next);
        }

        node.next = next;
        (num, Some(node))
    }
}
