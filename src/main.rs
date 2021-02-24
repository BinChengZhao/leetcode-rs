#![allow(dead_code)]
mod algorithms;
mod thread;

use algorithms::a20::Solution20;

use algorithms::data_structure::array::toeplitz_matrix::Solution;
use algorithms::grumpy_bookstore_owner::Solution1052;

//TODO: 把不同的题解运行demo，分发到不同的 examples 下。
fn main() {
    let v = vec![vec![1, 2, 0, 4], vec![5, 1, 2, 3], vec![9, 5, 1, 2]];

    dbg!(Solution::is_toeplitz_matrix(v));

    let v = vec![vec![1, 2, 3, 4], vec![5, 1, 2, 3], vec![9, 5, 1, 2]];

    dbg!(Solution::is_toeplitz_matrix(v));
    dbg!(Solution20::is_valid("()[]{}".to_string()));
    dbg!(Solution20::is_valid("()[".to_string()));
    dbg!(Solution20::is_valid(")[".to_string()));
    dbg!(Solution20::is_valid("((".to_string()));
    dbg!(Solution20::is_valid("))".to_string()));
    dbg!(Solution20::is_valid("(((()))){}{}({[]})".to_string()));

    // let v = vec![2, 7, 11, 15];
    // let t = 9;
    // two_sum(v, t);

    dbg!(Solution1052::max_satisfied(
        vec![1, 0, 1, 2, 1, 1, 7, 5],
        vec![0, 1, 0, 1, 0, 1, 0, 1],
        3
    ));

    dbg!(Solution1052::max_satisfied(
        vec![1, 0, 1, 2, 1, 1, 7, 5],
        vec![0, 1, 0, 1, 0, 1, 0, 1],
        5
    ));

    dbg!(Solution1052::max_satisfied(
        vec![1, 0, 1, 9, 2, 1, 7, 5],
        vec![1, 1, 1, 0, 0, 1, 0, 1],
        5
    ));

    dbg!(Solution1052::max_satisfied(
        vec![1, 0, 1, 2, 0, 1, 0, 0],
        vec![0, 1, 0, 1, 0, 1, 0, 0],
        2
    ));

    dbg!(Solution1052::max_satisfied(
        vec![1, 9, 1, 2, 9, 9, 9, 5],
        vec![0, 0, 0, 1, 0, 0, 0, 1],
        7
    ));

    dbg!(Solution1052::max_satisfied(
        vec![1, 0, 1, 2, 1, 1, 7, 5],
        vec![0, 1, 0, 1, 0, 1, 0, 1],
        3
    ));
}
