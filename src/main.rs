#![allow(dead_code)]
mod algorithms;
mod thread;

use algorithms::data_structure::array::best_time_to_buy_and_sell_stock::Solution121;

//TODO: 把不同的题解运行demo，分发到不同的 examples 下。
fn main() {
    dbg!(Solution121::max_profit(vec![7, 1, 5, 3, 6, 4]));
    dbg!(Solution121::max_profit(vec![7, 6, 5, 4, 3, 2, 1]));
    dbg!(Solution121::max_profit(vec![7, 7, 7, 7, 5, 4]));
    dbg!(Solution121::max_profit(vec![1, 2, 3, 4, 5, 6, 7]));
    dbg!(Solution121::max_profit(vec![
        8, 0, 2, 5, 3, 6, 9, 3, 65, 57, 57, 16, 50, 27
    ]));
    dbg!(Solution121::max_profit_1(vec![7, 1, 5, 3, 6, 4]));
    dbg!(Solution121::max_profit_1(vec![7, 6, 5, 4, 3, 2, 1]));
    dbg!(Solution121::max_profit_1(vec![7, 7, 7, 7, 5, 4]));
    dbg!(Solution121::max_profit_1(vec![1, 2, 3, 4, 5, 6, 7]));
    dbg!(Solution121::max_profit_1(vec![
        8, 0, 2, 5, 3, 6, 9, 3, 65, 57, 57, 16, 50, 27
    ]));
}
