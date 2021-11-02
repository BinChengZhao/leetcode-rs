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

pub struct Solution;

impl Solution {
    // https://leetcode.com/problems/add-two-numbers/
    //     You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order, and each of their nodes contains a single digit.
    //  Add the two numbers and return the sum as a linked list.

    // You may assume the two numbers do not contain any leading zero,
    // except the number 0 itself.

    // Input: l1 = [2,4,3], l2 = [5,6,4]
    // Output: [7,0,8]
    // Explanation: 342 + 465 = 807.
    // Example 2:

    // Input: l1 = [0], l2 = [0]
    // Output: [0]
    // Example 3:

    // Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
    // Output: [8,9,9,9,0,0,0,1]
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut list_vec: Vec<i32> = Vec::new();
        let mut list_option: Option<Box<ListNode>> = None;
        let mut overflow: i32 = 0;

        // Loop it.
        loop {
            let mut value1 = 0;
            let mut value2 = 0;

            // Break loop when both list is none .
            if l1.is_none() && l2.is_none() {
                break;
            }

            // if l1 is Some, take the value and put the `next` to l1 for next loop.
            if let Some(node1) = l1 {
                value1 = node1.val;
                l1 = node1.next;
            }

            // if l2 is Some, take the value and put the `next` to l2 for next loop.
            if let Some(node2) = l2 {
                value2 = node2.val;
                l2 = node2.next;
            }

            // Add value1 + value2;
            let mut value = value1 + value2 + overflow;

            // If it gq > 10 it has overflow, so mod it.
            // the overflow value plus to next value.
            if value >= 10 {
                overflow = value / 10;
                value %= 10;
            } else {
                overflow = 0;
            }

            // collect values.
            list_vec.push(value);
        }

        // The last pair may overflow ,so need collect again.
        if overflow > 0 {
            list_vec.push(overflow);
        }

        // The list_vec value is reversed order, so we need to reverse it.

        // This implementation is roughly 2n in time complexity,
        // avoiding the need to build the List in the first loop .
        for v in list_vec.into_iter().rev() {
            let mut node = ListNode::new(v);

            if list_option.is_some() {
                node.next = list_option;
                list_option = Some(Box::new(node));
                continue;
            }

            list_option = Some(Box::new(node));
        }
        list_option
    }
}

////////// Copied Best performance code /////////////
// fn add(l1: &Option<Box<ListNode>>, l2: &Option<Box<ListNode>>, carry: bool) -> (i32, bool) {
//     let val1 = match l1 {
//         None => 0,
//         Some(node) => node.val,
//     };
//     let val2 = match l2 {
//         None => 0,
//         Some(node) => node.val,
//     };
//     let mut calc = val1 + val2;
//     if carry { calc += 1 }
//     return (calc % 10, calc > 9)
// }

// fn adder(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, carry: bool) -> Option<Box<ListNode>> {
//     let (result, carry) = add(&l1, &l2, carry);
//     let mut node = ListNode::new(result);
//     let next1 = match l1 {
//         None => None,
//         Some(l1) => l1.next
//     };
//     let next2 = match l2 {
//         None => None,
//         Some(l2) => l2.next
//     };
//     if next1 != None || next2 != None {
//         node.next = adder(next1, next2, carry);
//     } else if carry {
//         node.next = Some(Box::new(ListNode::new(1)))
//     }
//     Some(Box::new(node))
// }

// impl Solution {
//     pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//         adder(l1, l2, false)
//     }
// }
////////// Copied Best performance code /////////////
