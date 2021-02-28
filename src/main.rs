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

    let v1 = vec![1, 0, 1, 2, 1, 1, 7, 5];
    let v2 = vec![0, 1, 0, 1, 0, 1, 0, 1];
    let l = 3;
    debug_assert_eq!(
        Solution1052::max_satisfied(v1.clone(), v2.clone(), l),
        Solution1052::max_satisfied2(v1, v2, l as usize)
    );

    let v1 = vec![1, 0, 1, 2, 1, 1, 7, 5];
    let v2 = vec![0, 1, 0, 1, 0, 1, 0, 1];
    let l = 5;
    debug_assert_eq!(
        Solution1052::max_satisfied(v1.clone(), v2.clone(), l),
        Solution1052::max_satisfied2(v1, v2, l as usize)
    );

    let v1 = vec![1, 0, 1, 9, 2, 1, 7, 5];
    let v2 = vec![1, 1, 1, 0, 0, 1, 0, 1];
    let l = 5;
    debug_assert_eq!(
        Solution1052::max_satisfied(v1.clone(), v2.clone(), l),
        Solution1052::max_satisfied2(v1, v2, l as usize)
    );

    let v1 = vec![1, 0, 1, 2, 0, 1, 0, 0];
    let v2 = vec![0, 1, 0, 1, 0, 1, 0, 0];
    let l = 2;
    debug_assert_eq!(
        Solution1052::max_satisfied(v1.clone(), v2.clone(), l),
        Solution1052::max_satisfied2(v1, v2, l as usize)
    );

    let v1 = vec![1, 9, 1, 2, 9, 9, 9, 5];
    let v2 = vec![0, 0, 0, 1, 0, 0, 0, 1];
    let l = 7;
    debug_assert_eq!(
        Solution1052::max_satisfied(v1.clone(), v2.clone(), l),
        Solution1052::max_satisfied2(v1, v2, l as usize)
    );

    let v1 = vec![1, 0, 1, 2, 1, 1, 7, 5];
    let v2 = vec![0, 1, 0, 1, 0, 1, 0, 1];
    let l = 3;
    debug_assert_eq!(
        Solution1052::max_satisfied(v1.clone(), v2.clone(), l),
        Solution1052::max_satisfied2(v1, v2, l as usize)
    );
}
