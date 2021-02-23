///https://www.zhihu.com/question/19690210
/// https://zhuanlan.zhihu.com/p/39816314
/// 知乎提解
use std::cmp::max;

/// 两个蛋时，用的函数
/// 求的是：最坏多少次能找出来，结果要的是一个最优策略（不是临界层在哪一个，因为根本不知道哪一个）
/// 当我们采用固定间隔法丢的话，最坏的次数会动态的增长
/// 例如：50层开始丢，如果碎了 就1此一次往50层找（最坏50次）
/// ， 如果不碎就100（最坏51次） 或 75（最坏26次）层 丢一下，再一层一层寻找
/// 这样的话，我们策略对应的‘最坏次数’总是变化的
///
/// 假设，有没有一种策略能保持，我们‘最坏次数’是固定且最小的
///
/// 嵌入逻辑：假设每次丢鸡蛋是 K层，如果k 是固定的比如10层
/// 第一次，10（如果碎了-最坏情况是10次），如果不碎则在20层丢（如果碎了-最坏情况是11次）
/// 如果不碎则在30层丢（如果碎了-最坏情况是12次）。。。一直不碎到100层，（最坏则是19次）
///
/// 这种情况下，最坏次数还在变动并且增加
///
/// 回到逻辑：假设，有没有一种策略能保持，我们‘最坏次数’是固定且最小的
/// 当间隔（k）固定，则每一次的最坏情况都是k+1
///
/// 反过来想，如果我们间隔不固定，每次的间隔（k） 都是上一次的  （k-1），则范围跟 最坏次数都锁定在了k
/// 完成了第一步：固定的最坏情况
/// 第二步，需要找到最小的起始间隔（k）
/// 涉及到一个数学公式
/// 推到公式 (k)+ (k-1) + (k-2) + (k-n) = 100 = k(k+1)/2
pub fn double_eggs(height: usize) -> usize {
    if height == 1 {
        return 1;
    }

    for i in 0..height {
        if (i * (i + 1) / 2) >= height {
            return i;
        }
    }

    return 0;
}

///现在我们知道问题，不在于100层楼两个蛋，而是在于不定的楼层，与不定个数的蛋，最优的策略跟最坏的次数
/// 当我们推理出，100层楼2个蛋存在‘不定的间隔’的最优解，及这种情况下的最坏次数
/// 也可以推理一下，不定高度的楼层，与不定个数的蛋
///
/// 在一些特定情况下，我们是可以直接推理出策略的：
/// 比如1个鸡蛋（我们别无选择，只能一层一层扔），或者是只有一层楼（最优永远都是在第一层楼扔，不论几个鸡蛋）
///
/// 除了除特殊的边界场景，我们运用数学的逻辑我们来推理：
/// 假设，每次最优的间隔是K，总楼层高度是T，鸡蛋的个数是N
/// 那我们第一次在K楼扔的时候，后会两种情况：碎或者不碎
/// Case1：如果碎了那就，在 需要用 ，N -1 个蛋，在 k-1层中寻找
/// Case2：如果不碎我们就得，在 T-K 个楼层用 N个蛋寻找
/// 总的可能性就是 Max（（k-1，n-1），（T-k，n））+1
/// +1 是因为，初始在k层扔了一次  ，max 中是扔了一次之后，两种情况对应的次数中：挑一个最大值（因为我们求得是最坏情况）
/// 当我们在公式推倒出来后，我们可以使用遍历的方法，从k=1 到 k<=T ,种种情况挨个计算
/// 求出一个最优解的最坏次数，也就是求一个最小值
/// ps：感谢李永乐老师
pub fn super_eggs(height: usize, eggs_num: usize) -> usize {
    if height == 0 {
        return 0;
    }

    if height == 1 {
        return 1;
    }

    if eggs_num == 1 {
        return height;
    }

    let mut results = Vec::new();

    for i in 1..height {
        let tmp_result = max(
            super_eggs(i - 1, eggs_num - 1),
            super_eggs(height - i, eggs_num),
        ) + 1;
        results.push(tmp_result);
    }

    results.sort();

    *results.first().unwrap()
}

/// for 循环版本
/// 接着上面，用循环写，递归容易爆栈
/// 例如14 （100，2），这个例子中，k：假设为14 ，楼层 T：100 ，蛋数 N： 2
/// 当在14楼扔下，形成如下可能  Max( (13,1) , (86,2) )+1
/// Max  中的 (13,1) 与 (86,2)，我们分别要用他们两个的最优解 ，去找分别的最坏情况
/// (13,1) 就是 13，
/// 那(86,2) 我们 又要推理 每一种k可能性对应的 最优策略中最坏的次数
/// 到底是 Max( (1,1) , (85,2) )+1   ，K假设是1
/// 还是 Max( (12,1) , (74,2) )+1    ，K假设是13
/// 所以我们需要优先把每一种可能性的，最优策略中最坏的次数 计算出来，再去计算
/// 这个算法是解出来了，但是空间复杂度比较高是，O(he) 的
/// 分析一下时间复杂度和空间复杂度，时间复杂度为 O(mh^2)
/// 空间复杂度为 O(hn)
///   ，很明显，空间上可以优化到  O(h)  ，原因是状态转移方程只和  m 与 m-1有关
///  有关，使用两个数组滚动即可
pub fn for_super_eggs(height: i32, eggs_num: i32) -> i32 {
    let height = height as usize;
    let eggs_num = eggs_num as usize;
    if height == 0 {
        return 0;
    }

    if eggs_num == 0 {
        return 0;
    }

    if height == 1 {
        return 1;
    }

    if eggs_num == 1 {
        return height as i32;
    }

    // let mut results = Vec::new();
    //let mut state = [[0u8; 4]; 6];  创建二维数组，但是大小得提前知道

    // 创建二维向量 Base 1d array
    //let mut grid_raw = vec![0; width * height];

    // Vector of 'width' elements slices
    // let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(width).collect();

    // Final 2d array
    // let grid: &mut [&mut [_]] = grid_base.as_mut_slice();

    // Accessing data
    //grid[0][0] = 4;

    let mut grid_raw = vec![0; (height + 1) * (eggs_num + 1)];
    let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(eggs_num + 1).collect();
    let height_t_eggs: &mut [&mut [_]] = grid_base.as_mut_slice();

    for i in 0..(height + 1) {
        for j in 0..(eggs_num + 1) {
            if i == 0 {
                height_t_eggs[i][j] = 0;
                continue;
            }

            if j == 0 {
                height_t_eggs[i][j] = 0;
                continue;
            }

            if i == 1 {
                height_t_eggs[i][j] = 1;
                continue;
            }

            if j == 1 {
                height_t_eggs[i][j] = i;
                continue;
            }

            //todo 提现拿出第一个，循环里面就少一个判断
            let mut tmp_max = max(height_t_eggs[1 - 1][j - 1], height_t_eggs[i - 1][j]) + 1;

            for k in 2..i {
                let tmp_choice = max(height_t_eggs[k - 1][j - 1], height_t_eggs[i - k][j]) + 1;
                if tmp_max > tmp_choice {
                    tmp_max = tmp_choice;
                }
            }

            height_t_eggs[i][j] = tmp_max;
            //当下的最优解
            // println!("height_t_eggs[{}][{}]:{}", i, j, height_t_eggs[i][j]);
        }
        // println!("height_t_eggs[{}]:{:?}", i,  height_t_eggs[i]);
    }

    *(height_t_eggs.last().unwrap()).last().unwrap() as i32
}

///空间复杂度优化版本
pub fn for_super_eggs2(height: i32, eggs_num: i32) -> i32 {
    let height = height as usize;
    let eggs_num = eggs_num as usize;

    if eggs_num == 0 {
        return 0;
    }

    //最少有1层楼
    if height <= 1 {
        return 1;
    }

    if eggs_num == 1 {
        return height as i32;
    }

    let mut result = Vec::with_capacity(height + 1);
    let mut current = Vec::with_capacity(height + 1);

    for i in 0..height + 1 {
        result.push(i);

        current.push(i);
    }

    for _j in 2..(eggs_num + 1) {
        for i in 1..(height + 1) {
            for k in 1..i {
                if result[i] > (max(current[k - 1], result[i - k]) + 1) {
                    result[i] = max(current[k - 1], result[i - k]) + 1;
                }
            }
        }
        current = result.clone();
    }

    *result.last().unwrap() as i32
}

///
///
pub fn for_super_eggs3(height: i32, eggs_num: i32) -> i32 {
    let N = height as usize;
    let K = eggs_num as usize;
    let mut dp = vec![0; N + 1];

    //N<=1,都返回1
    if N == 0 {
        return 1;
    }

    for i in 0..N + 1 {
        //记录一个错误，一开始把
        // dp[i] = i; 写成 dp.push(i)了
        //因为一开始已经初始化过了，导致正确的值都追加到后面了
        dp[i] = i;
    }

    for _ in 2..K + 1 {
        let mut dp2 = vec![0; N + 1];
        let mut x = 1;
        dp2[0] = 0;

        for n in 1..N + 1 {
            loop {
                if x < n && max(dp[x - 1], dp2[n - x]) >= max(dp[x], dp2[n - x - 1]) {
                    x = x + 1;
                } else {
                    break;
                }
            }

            dp2[n] = 1 + max(dp[x - 1], dp2[n - x]);
        }

        dp = dp2;
    }

    return dp[N] as i32;
}
