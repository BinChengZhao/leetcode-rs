#[derive(Debug, Default)]
pub struct Solution;

impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        let first = edges[0][0];

        let second = edges[0][1];

        if first == edges[1][0] || first == edges[1][1] {
            return first;
        }

        second
    }
}
