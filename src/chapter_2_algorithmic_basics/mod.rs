//! 第2章 算法基础

mod insertion_sort;
mod merge;

/// 判断数组是否按从小到大的顺序排列。
fn is_sorted(input: &[i32]) -> bool {
    for i in 1..input.len() {
        if input[i - 1] > input[i] {
            return false;
        }
    }
    true
}

#[test]
fn test() {
    let buf = [1, 2, 3];
    assert!(is_sorted(&buf));

    let buf = [2, 1, 3];
    assert!(!is_sorted(&buf));
}