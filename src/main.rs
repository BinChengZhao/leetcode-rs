#![allow(dead_code)]
mod algorithms;
mod thread;

use algorithms::bit_manipulation::missing_number::Solution268;

//TODO: 把不同的题解运行demo，分发到不同的 examples 下。
fn main() {
    debug_assert_eq!(Solution268::missing_number(vec![1, 2, 3, 4, 5, 6, 7]), 0);
    debug_assert_eq!(Solution268::missing_number(vec![0, 1, 2, 4, 5, 6, 7]), 3);
}
