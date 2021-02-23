use std::mem::swap;
pub struct Solution;

impl Solution {
    //写的不好，应该直接用 row_len 跟 col_len两层遍历

    pub fn transpose(mut a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let row_len = a.len();
        let first = a.remove(0);
        let col_len = first.len();

        //  a.swap(0, row_len - 2);

        //预分配内存
        let mut result: Vec<Vec<i32>> = Vec::with_capacity(col_len);
        for i in first.into_iter() {
            let mut v = Vec::with_capacity(row_len);
            v.push(i);
            result.push(v);
        }
        for v in a.into_iter() {
            for (k, i) in v.into_iter().enumerate() {
                result[k].push(i);
            }
        }
        result
    }
}
