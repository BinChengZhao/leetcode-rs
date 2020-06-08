mod algorithms;
mod thread;
use algorithms::a1::two_sum;
use algorithms::a55::can_jump;
use algorithms::google::eggs::{
    double_eggs, for_super_eggs, for_super_eggs2, for_super_eggs3, super_eggs,
};
use thread::t1114::exec;
fn main() {
    let v = vec![2, 7, 11, 15];
    let t = 9;
    two_sum(v, t);

    // exec([1, 2, 3]);
    // exec([3, 2, 1]);
    // exec([1, 3, 2]);
    // let result55 = can_jump(vec![3, 2, 1, 0, 4]);
    // println!("{:?}", result55);

    // let eggs = double_eggs(100);
    // println!("{:?}", eggs);

    // let super_eggs1 = super_eggs(6, 2);
    // println!("super_eggs:{:?}", super_eggs1);

    // let super_eggs2 = super_eggs(2, 1);
    // println!("super_eggs:{:?}", super_eggs2);

    // let super_eggs3 = super_eggs(14, 3);
    // println!("super_eggs:{:?}", super_eggs3);

    // let super_eggs4 = super_eggs(15, 2);
    // println!("super_eggs:{:?}", super_eggs4);

    // let super_eggs5 = for_super_eggs(15, 2);
    // println!("for_super_eggs:{:?}", super_eggs5);

    // let super_eggs6 = for_super_eggs(100, 2);
    // println!("for_super_eggs:{:?}", super_eggs6);

    // let super_eggs6 = for_super_eggs(0, 0);
    // println!("for_super_eggs:{:?}", super_eggs6);

    // let super_eggs6 = for_super_eggs(0, 1);
    // println!("for_super_eggs:{:?}", super_eggs6);

    // let super_eggs6 = for_super_eggs(1, 0);
    // println!("for_super_eggs:{:?}", super_eggs6);

    // let super_eggs6 = for_super_eggs(1, 1);
    // println!("for_super_eggs:{:?}", super_eggs6);

    // let super_eggs6 = for_super_eggs(300, 5);
    // println!("for_super_eggs:{:?}", super_eggs6);

    // let super_eggs6 = for_super_eggs2(300, 5);
    // println!("for_super_eggs:{:?}", super_eggs6);

    // for j in (1..5).rev() {
    //     println!("{}", j);
    // }

    // let super_eggs6 = for_super_eggs3(300, 5);
    // println!("for_super_eggs:{:?}", super_eggs6);
}
