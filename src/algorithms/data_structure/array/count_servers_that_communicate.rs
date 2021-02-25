pub(crate) struct Solution;

impl Solution {
    //https://leetcode-cn.com/problems/count-servers-that-communicate/
    //TODO:这个题还没解完。
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let max_row = m - 1;
        let max_col = n - 1;

        let mut count = 0;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 0 {
                    continue;
                }

                if i != 0 && grid[i - 1][j] == 1 {
                    count += 1;
                    continue;
                }

                if j != 0 && grid[i][j - 1] == 1 {
                    count += 1;
                    continue;
                }

                if i != max_row && grid[i + 1][j] == 1 {
                    count += 1;
                    continue;
                }

                if j != max_col && grid[i][j + 1] == 1 {
                    count += 1;
                    continue;
                }
            }
        }

        count
    }
}
