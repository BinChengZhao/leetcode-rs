pub struct Solution;

impl Solution {
    pub fn flip_and_invert_image(mut a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let f = |e: &mut Vec<i32>| {
            e.reverse();
            e.iter_mut().for_each(|e: &mut i32| {
                *e = (*e - 1).abs();
            })
        };
        a.iter_mut().for_each(f);
        a
    }
}
