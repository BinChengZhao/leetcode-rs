mod algorithms;
mod thread;
use algorithms::a1::two_sum;
use algorithms::a20::Solution20;
use algorithms::a55::can_jump;
use algorithms::data_structure::array::transpose_matrix;
use algorithms::google::eggs::{
    double_eggs, for_super_eggs, for_super_eggs2, for_super_eggs3, super_eggs,
};
use thread::t1114::exec;
//TODO: 把不同的题解运行demo，分发到不同的 examples 下。
fn main() {
    dbg!(Solution20::is_valid("()[]{}".to_string()));
    dbg!(Solution20::is_valid("()[".to_string()));
    dbg!(Solution20::is_valid(")[".to_string()));
    dbg!(Solution20::is_valid("((".to_string()));
    dbg!(Solution20::is_valid("))".to_string()));
    dbg!(Solution20::is_valid("(((()))){}{}({[]})".to_string()));

    // let v = vec![2, 7, 11, 15];
    // let t = 9;
    // two_sum(v, t);
}
