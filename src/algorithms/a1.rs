///给定一个整数数组 nums 和一个目标值 target，
/// 请你在该数组中找出和为目标值的那 两个 整数，并返回他们的数组下标。

///你可以假设每种输入只会对应一个答案。但是，数组中同一个元素不能使用两遍。


/// 给定 nums = [2, 7, 11, 15], target = 9

///因为 nums[0] + nums[1] = 2 + 7 = 9
///所以返回 [0, 1]

// | ：表示或 

///该算法采用hash_map 
///假设两个数分别为：i ， j   ， i+j = t
/// 那么，我们建立一个hash_map ，K 是 k = t - （i|j） ，里面存的V 是对应的下标
/// 当 用2作为k，能在hash_map 查到时，说明已经有一个  T-（i|j） = 2 的了
/// 这是拿着 2在数组的下标，跟2在hash_map对应的下标 返回即可
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut index_arr: HashMap<i32, i32> = HashMap::new();


    for (i, num) in nums.iter().enumerate() {

        if let Some(j) = index_arr.get(num) {
            return vec![i as i32, *j];
        }

        index_arr.insert(target-num, i as i32);
    }
    return vec![];
}
