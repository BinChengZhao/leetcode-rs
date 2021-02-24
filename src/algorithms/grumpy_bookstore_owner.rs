// 今天，书店老板有一家店打算试营业 customers.length 分钟。每分钟都有一些顾客（customers[i]）会进入书店，所有这些顾客都会在那一分钟结束后离开。

// 在某些时候，书店老板会生气。 如果书店老板在第 i 分钟生气，那么 grumpy[i] = 1，否则 grumpy[i] = 0。 当书店老板生气时，那一分钟的顾客就会不满意，不生气则他们是满意的。

// 书店老板知道一个秘密技巧，能抑制自己的情绪，可以让自己连续 X 分钟不生气，但却只能使用一次。

// 请你返回这一天营业下来，最多有多少客户能够感到满意的数量。
//

// 示例：

// 输入：customers = [1,0,1,2,1,1,7,5], grumpy = [0,1,0,1,0,1,0,1], X = 3
// 输出：16
// 解释：
// 书店老板在最后 3 分钟保持冷静。
// 感到满意的最大客户数量 = 1 + 1 + 1 + 1 + 7 + 5 = 16.
//

// x = n ;
// 这个n代表可以让 grumpy 连续n个元素为 1
//
// 也就是说，可以让连续n个在customers的元素被累加进结果
// 在 `customers`求相加是最大值的连续n个元素的范围， 并且考虑 `grumpy` 的影响
// 如果`grumpy`在那段范围上，本来下标对应的值就是1， 那就不用采用这个范围。。
//
// 综合考虑哪段范围相加的值最大， 但因为`grumpy`0较多 值没有累加入结果。
// 通过一个栈类的数据结构，记录n条数据中的没有累计计算的值，如果对应下标的值正常被累计则栈中存一个0
//
// 输入：customers = [1,0,1,2,1,1,7,5], grumpy = [0,1,0,1,0,1,0,1], X = 3
// stack-record   customers[0..=2][0,0,0] customers[1..=3][0,0,2] customers[2..=4][0,2,0]
// stack-record   customers[3..=5][2,0,1] customers[4..=6][0,1,0] customers[5..=7][1,0,5]
//
// 上述记录中可以看出，X 用在 5..=7 分钟最合适，能再增加6个满意度

pub struct Solution1052;
impl Solution1052 {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, x: i32) -> i32 {
        let mut total_num = 0;

        // let mut cursor_chunk_val: i32;
        let mut max_losses_val_of_chunk: i32;
        let mut current_losses_val_of_chunk: i32 = 0;

        // let mut cursor_stack = Vec::<i32>::with_capacity(x);
        let mut cursor_stack_of_losses = Vec::<i32>::with_capacity(x as usize);

        // let statify_vec = Vec::<i32>::with_capacity(customers.len() / x + 1);
        let mut customers_vec = customers.into_iter();
        let mut grumpy_vec = grumpy.into_iter();

        for _ in 0..x {
            let i = customers_vec.next().unwrap();
            // cursor_stack.push(i);
            // cursor_chunk_val += i;

            if grumpy_vec.next().unwrap() == 1 {
                current_losses_val_of_chunk += i;
                cursor_stack_of_losses.push(i);
            } else {
                total_num += i;

                cursor_stack_of_losses.push(0);
            }
        }

        max_losses_val_of_chunk = current_losses_val_of_chunk;
        // The chunk-val of customers[0..x].sum()
        // statify_vec.push(cursor_chunk_val);

        // Then the elements will be customers[1..x+1].sum customers[2..x+2].sum()
        for i in customers_vec {
            // cursor_chunk_val = cursor_chunk_val - cursor_stack.remove(0) + i;
            // cursor_stack.push(i);

            // v.remove(0) eq v.shift()
            current_losses_val_of_chunk =
                current_losses_val_of_chunk - cursor_stack_of_losses.remove(0);

            if grumpy_vec.next().unwrap() == 1 {
                current_losses_val_of_chunk += i;
                cursor_stack_of_losses.push(i);
            } else {
                total_num += i;

                // push the last one.
                cursor_stack_of_losses.push(0);
            }

            if current_losses_val_of_chunk > max_losses_val_of_chunk {
                max_losses_val_of_chunk = current_losses_val_of_chunk;
            }

            // Check what the value of this block of data is because grumpy-chunks is 0 and not cumulative.
        }
        total_num + max_losses_val_of_chunk
    }
}
