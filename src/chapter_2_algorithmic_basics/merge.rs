//! 归并排序。

/// 排序。
fn merge_sort(input: &mut [i32]) {
    if input.len() < 2 {
        return;
    }

    let mid = input.len() / 2;
    merge_sort(&mut input[..mid]);
    merge_sort(&mut input[mid..]);
    merge(input, 0, mid, input.len());
}

/// 假定：lo..mid、mid..hi 的部分已经排序。
fn merge(input: &mut [i32], lo: usize, mid: usize, hi: usize) {
    let left = copy_from(&input[lo..mid]);
    let right = copy_from(&input[mid..hi]);

    let mut i = 0;
    let mut j = 0;
    for k in lo..hi {
        if left[i] <= right[j] {
            input[k] = left[i];
            i += 1;
        } else {
            input[k] = right[j];
            j += 1;
        }
    }
}

// 从切片中拷贝全部元素，并在最后添加一个哨兵。
fn copy_from(input: &[i32]) -> Vec<i32> {
    let mut buf = Vec::with_capacity(input.len() + 1);
    buf.extend(input.iter());
    buf.push(i32::MAX); // 哨兵。
    buf
}

#[cfg(test)]
mod tests {
    use crate::{chapter_2_algorithmic_basics::merge::*, is_sorted};

    #[test]
    fn sort_test() {
        let mut buf = [2, 3, 5, 1, 4, 6, 9];
        let len = buf.len();
        merge(&mut buf, 0, len / 2, len);

        println!("{buf:?}");
        assert!(is_sorted(&buf));
    }

    #[test]
    fn merge_test() {
        let mut buf = [3, 2, 5, 1, 9, 6, 4];
        merge_sort(&mut buf);

        println!("{buf:?}");
        assert!(is_sorted(&buf));
    }
}
