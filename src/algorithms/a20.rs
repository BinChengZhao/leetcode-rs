pub struct Solution20;

impl Solution20 {
    pub fn is_valid(mut s: String) -> bool {
        if (s.len() % 2) != 0 {
            return false;
        }

        let s_chars = s.as_mut_str().char_indices();

        let mut v = Vec::<char>::new();

        for (_, c) in s_chars {
            match c {
                '(' => v.push(')'),
                '[' => v.push(']'),
                '{' => v.push('}'),
                _ => {
                    if v.is_empty() {
                        return false;
                    }

                    if v.pop().unwrap() != c {
                        return false;
                    }
                }
            };
        }

        v.is_empty()
    }
}
