use std::collections::HashMap;
pub(crate) struct Solution121;

impl Solution121 {
    // 起始的思路，没有什么是两层循环解决不了的。
    // 维护一个map<price, [profit]>，里面对应了每个 price 与之后日期计算出来的正收益。
    // 稍加思考，这个 `[profit]` 是不必要的，因为我只需要最佳的收益，所以优化为map<price, max_profit>
    // 但是map是各个价格最好的收益，被没有全局最佳的收益，所以需要一个 `max_profit_result` 在计算中进行维护。

    // step1，结果如下，验证后发现结果正确，但是leetcode上运行居然超时了。
    // 果然还是实现太粗糙。
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit_map: HashMap<i32, i32> = HashMap::default();
        let mut max_profit_result = 0;
        let mut tmp_profit;

        for new_price in prices.iter() {
            profit_map.insert(*new_price, 0);

            for (price, sell_out_price) in profit_map.iter_mut() {
                if new_price > sell_out_price {
                    *sell_out_price = *new_price;
                }

                tmp_profit = *sell_out_price - *price;
                if tmp_profit > max_profit_result {
                    max_profit_result = tmp_profit;
                }
            }
        }

        max_profit_result
    }

    // 优化后的版本:
    // 重点避免无用map带来的过多开销
    // 只在运行中维护最小购买价格 `min_price` & 最大收益 `max_profit_result`
    // 中间值 `tmp_profit` 计算规则为，遍历中当前价格 - 最小价格 ,
    // 如果结果大于 `max_profit_result` 则，更新最大化收益。
    // PS: `min_price` 这个值总是出现在 `price`(遍历中当前价格) 之前, 这样才能保证低买高卖
    pub fn max_profit_1(prices: Vec<i32>) -> i32 {
        let mut tmp_profit;
        let mut min_price = i32::MAX;
        let mut max_profit_result = 0;

        for new_price in prices.iter() {
            if new_price < &min_price {
                min_price = *new_price;
                continue;
            }

            tmp_profit = *new_price - min_price;
            if tmp_profit > max_profit_result {
                max_profit_result = tmp_profit;
            }
        }

        max_profit_result
    }
}
