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
    pub fn merge_two_lists(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut list_option: Option<Box<ListNode>> = None;
        let mut vec_values: Vec<i32> = Vec::with_capacity(48);

        let mut value1;
        let mut value2;
        loop {
            if l1.is_none() {
                while let Some(node) = l2 {
                    vec_values.push(node.val);
                    l2 = node.next;
                }

                break;
            }

            if l2.is_none() {
                while let Some(node) = l1 {
                    vec_values.push(node.val);
                    l1 = node.next;
                }
                break;
            }

            value1 = l1.as_ref().map(|n| n.val).unwrap();
            value2 = l2.as_ref().map(|n| n.val).unwrap();

            if value1 > value2 {
                vec_values.push(value2);
                l2 = l2.unwrap().next;
            } else if value2 > value1 {
                vec_values.push(value1);
                l1 = l1.unwrap().next;
            } else {
                vec_values.push(value1);
                vec_values.push(value2);

                l1 = l1.unwrap().next;
                l2 = l2.unwrap().next;
            }
        }

        for v in vec_values.into_iter().rev() {
            let mut node = Box::new(ListNode::new(v));

            if let Some(root) = list_option {
                node.next = Some(root);
                list_option = Some(node);
                continue;
            }

            list_option = Some(node);
        }

        list_option
    }
}

////////// Copied Best performance code /////////////
// impl Solution {
//     pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//         match (l1, l2) {
//             (None, None) => None,
//             (l1, None) => l1,
//             (None, l2) => l2,
//             (Some(s1), Some(s2)) => {
//                 if s1.val < s2.val {
//                     Some(Box::new(ListNode {
//                         val: s1.val,
//                         next: Self::merge_two_lists(s1.next, Some(s2)),
//                     }))
//                 } else {
//                     Some(Box::new(ListNode {
//                         val: s2.val,
//                         next: Self::merge_two_lists(s2.next, Some(s1)),
//                     }))
//                 }
//             }
//         }
//     }
// }
////////// Copied Best performance code /////////////

////////// The code for my ideal effect copied from leetcode. /////////////
// impl Solution {
//     pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//         let mut result = Box::new(ListNode::new(-1));
//         let mut to_add = &mut result.next;

//         let (mut l1, mut l2) = (&l1, &l2);

//         loop {
//             match (l1, l2){
//                 (Some(l), Some(r)) if l.val < r.val => {
//                     *to_add = Some(Box::new(ListNode::new(l.val)));
//                     to_add = &mut to_add.as_mut().unwrap().next;
//                     l1 = &l.next;
//                 }
//                 (Some(l), Some(r)) => {
//                     *to_add = Some(Box::new(ListNode::new(r.val)));
//                     to_add = &mut to_add.as_mut().unwrap().next;
//                     l2 = &r.next;
//                 }
//                 (Some(l), None) => {
//                     *to_add = Some(Box::new(ListNode::new(l.val)));
//                     to_add = &mut to_add.as_mut().unwrap().next;
//                     l1 = &l.next;
//                 }
//                 (None, Some(r)) => {
//                     *to_add = Some(Box::new(ListNode::new(r.val)));
//                     to_add = &mut to_add.as_mut().unwrap().next;
//                     l2 = &r.next;
//                 }
//                 _ => return result.next
//             }
//         }

//     }
// }
////////// The code for my ideal effect copied from leetcode. /////////////
