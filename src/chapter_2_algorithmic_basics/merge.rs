//! 归并排序。

/// 假定：lo..min、min..hi 的部分已经排序。
fn sort(input: &mut [i32], lo: usize, mid: usize, hi: usize) {
    let mut left = input[lo..mid].to_vec();
    let mut right = input[mid..hi].to_vec();

    // 哨兵，不会两个同时出现。
    left.push(i32::MAX);
    right.push(i32::MAX);

    let mut i = 0;
    let mut j = 0;
    for r in lo..hi {
        if left[i] <= right[j] {
            input[r] = left[i];
            i += 1;
        } else {
            input[r] = right[j];
            j += 1;
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::chapter_2_algorithmic_basics::{merge::sort, is_sorted};

    #[test]
    fn test() {
        let mut buf = [2, 3, 5, 1, 4, 6, 9];
        let len = buf.len();
        sort(&mut buf, 0, len / 2, len);

        println!("{buf:?}");
        assert!(is_sorted(&buf));
    }
}
