#![cfg_attr(debug_assertions, allow(dead_code))]

mod chapter_2_algorithmic_basics;
mod chapter_4_divide_and_conquer;
mod chapter_6_heapsort;
mod chapter_7_quicksort;

/// 判断数组是否按从小到大的顺序排列。
fn is_sorted(input: &[i32]) -> bool {
    for i in 1..input.len() {
        if input[i - 1] > input[i] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::is_sorted;

    #[test]
    fn test() {
        let buf = [1, 2, 3];
        assert!(is_sorted(&buf));

        let buf = [2, 1, 3];
        assert!(!is_sorted(&buf));
    }
}
