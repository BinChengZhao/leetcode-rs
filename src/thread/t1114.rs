use std::ops::{Deref, DerefMut};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Barrier,
};
use std::thread::*;

///  线程1114题，总结：
///
/// 通过原子类型的条件变量做的
///
/// 1.AtomicUsize，类型不要 mut  也可以通过 AtomicUsize   store 与 store 设置新值
/// 但是，要get_mut() 等就需要mut 提前声明了
///
/// 2.自己一开始代码写的对但是没有打印出来，是因为没有把子线程最后join起来
/// 因为我的主线程退出，调试的终端都已经关闭了，但是子线程却还在输出，就看不见了
/// join是用作，所有的子线程都在父线程的生命周期内退出，所以调试终端关闭之前结果都能看见
///
/// 3.滥用递归会导致，爆栈内存 ，可以用循环实现
trait syncOpration {
    fn one(&self) {
        println!("one");
    }

    fn two(&self) {
        println!("two");
    }

    fn three(&self) {
        println!("three");
    }
}

impl syncOpration for AtomicUsize {
    fn one(&self) {
        println!("one");
        self.fetch_add(1, Ordering::SeqCst);
    }

    fn two(&self) {
        loop {
            if self.load(Ordering::SeqCst) == 1 {
                println!("two");
                self.fetch_add(1, Ordering::SeqCst);
                return;
            }
        }
    }

    fn three(&self) {
        loop {
            if self.load(Ordering::SeqCst) == 2 {
                println!("three");
                return;
            }
        }
    }
}

pub fn exec(arr: [u32; 3]) {
    let num = Arc::new(AtomicUsize::new(0));

    // let mut joinArr :[JoinHandle<()>;3] = [];
    let mut joinArr = Vec::new();

    for i in arr.iter() {
        let syncNum = num.clone();

        let join: JoinHandle<()> = match i {
            1 => spawn(move || syncNum.one()),
            2 => spawn(move || syncNum.two()),

            3 => spawn(move || syncNum.three()),

            _ => {
                println!("???");

                return;
            }
        };
        joinArr.push(join);
    }

    for join in joinArr {
        join.join();
    }
}
