///// 目前的策略是通过当前元素的最大步数走，如果走的下一个元素不是0 就继续走，如果是就步数-1
/// 但是会出错，[3,0,8,2,0,0,1] ，比如，[0] ==3 ,能走到 [3]=2 ,然后我就走了，结果往下没法走了（错过了最佳的8）
///
///
/// 策略二，比如当前能走 4步，我以最小维度试探，如果能走下去，就走
/// 也是错的。。[4,1,0,8,0,0,1]  ，[0]==4 ,最小维度走1步，走到了[1]==1结果没法走，（错过了最佳的8）
///
/// 策略三，把当前能走的每一条路都走一遍，不再单调， 如上两种
/// E1:[3,0,8,2,0,0,1]   3=》0  3=》8  3=》2 都走一遍
/// E2:[4,1,0,8,0,0,1]   4=》1  4=》0  4=》8 4=》0 都走一遍
/// “当前步长，内的步骤全部试完之后”，用最佳的下标开始走（也是把能走的范围内的，下标都试一遍）
/// (index,element) , 当前下标最远能走到  index + element
/// 例如：E1 ，第一步对应的是 ，(1,0)=1,(2,8)=10,(3,2)=5
/// E2 ，第一步对应的是 ，(1,1)=2,(2,0)=10,(3,8)=11,(4,0)=4
/// E1,最佳实践是  下标2对应的8，  E2最佳实践，下标3对应的8
/// 拿到最佳实践，然后开始在它的范围内一步一步走，拿到下一个最佳实践，如果拿不出来就GG
/// 补充：如果当前 index + element >= arr.len  ，直接返回true
///
///
/// 转载：策略4
/// 按当前能走的步数一直走，如果走到的位置 对应的新步数，比现在剩余的步数多，则更新新的步数
/// 如果步数不够了，还没到重点，则不能走了返回false
///
/// impl Solution {
///  pub fn can_jump(nums: Vec<i32>) -> bool {
///    if nums.len() <= 1 { return true; }
///
///    let mut max = 1;
///    for &num in nums.iter().take(nums.len() - 1)  {
///        max = i32::max(max - 1, num);
///        if max == 0 { return false; }
///    }
///    true
///}
///}
///
///
/// 转载：策略5（c++）
///
/// 一直更新，最新能走到的最远位置，如果最远位置小于现在遍历到的位置，则失败
///
/// 如果某一个作为 起跳点 的格子可以跳跃的距离是 3，那么表示后面 3 个格子都可以作为 起跳点。
/// 可以对每一个能作为 起跳点 的格子都尝试跳一次，把 能跳到最远的距离 不断更新。
/// 如果可以一直跳到最后，就成功了。
/// bool canJump(vector<int>& nums)
///{
///	int k = 0;
///	for (int i = 0; i < nums.size(); i++)
///	{
///		if (i > k) return false;
///		k = max(k, i + nums[i]);
///	}
///	return true;
///}
///
///
/// 下面的代码是自己写的，策略3：

pub fn can_jump(nums: Vec<i32>) -> bool {
    let nums_len: usize = nums.len();
    let mut index: usize = 0;
    let mut step: usize = 0;

    //临时位置对应的下标
    let mut tmp_index = 0;

    //在当前位置，配合下一步可以走出的最远位置
    let mut best_index = 0;

    //计步器
    let mut tmp_step = 0;

    //在当前位置，最佳走几步
    let mut best_step = 1;
    // let mut element = 0;
    loop {
        step = nums[index] as usize;

        //如果，可走步数+当前下标 >= 数组长度 ，则可以走完
        if (step + index) >= (nums_len - 1) {
            // println!(" index :{},can gone",index);
            return true;
        } else {
            // println!("try index: {},lelement : {}", index, step);
            //如果没走完，而且当前还是可走0步直接，卡死
            if step == 0 {
                return false;
            }

            best_step = 1;
            tmp_step = 1;
            //最新的位置 = 当前位置 + 当前位置能走的步数 + 走到的那步还可以走的步数 ，代表当前最远能走到那里
            tmp_index = index + best_step + nums[index + best_step] as usize;
            best_index = tmp_index;
            loop {
                if tmp_step >= step {
                    // println!("try all_step {}", tmp_step);

                    break;
                }

                tmp_step = tmp_step + 1;
                tmp_index = index + tmp_step + nums[index + tmp_step] as usize;

                //如果获取了更远的地址，就更新
                if tmp_index > best_index {
                    best_index = tmp_index;
                    best_step = tmp_step;
                }
            }

            //计算下标
            // println!("best _ step : {}", best_step);
            index = best_step + index;
        }
    }
}
