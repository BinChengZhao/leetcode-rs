pub struct Solution;

impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        let len = matrix[0].len();
        let dep = matrix.len();

        let mut row_index = 0;
        let mut col_index = 0;

        let mut row_first_element = 0;

        for i in 0..dep {
            for j in 0..len {
                if i > 0 && j > 0 {
                    continue;
                }

                row_first_element = matrix[i][j];

                row_index = j;
                col_index = i;

                loop {
                    row_index += 1;
                    col_index += 1;

                    if row_index >= len || col_index >= dep {
                        break;
                    }

                    if matrix[col_index][row_index] != row_first_element {
                        return false;
                    }
                }
            }
        }

        true
    }
}
