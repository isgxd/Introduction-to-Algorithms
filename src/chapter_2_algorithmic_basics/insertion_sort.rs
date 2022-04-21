//! 插入排序。

fn sort(input: &mut [i32]) {
    for j in 1..input.len() {
        let key = input[j];

        // i 必须包含 0，并且可以为负，否则第 0 个元素不参与排序。
        let mut i = j as i32 - 1;
        while i >= 0 && input[i as usize] > key {
            input[i as usize + 1] = input[i as usize];
            i -= 1;
        }
        // i 可能为 -1，必须在 +1 后再转为无符号数。
        input[(i + 1) as usize] = key;
    }
}

#[cfg(test)]
mod tests {
    use crate::{chapter_2_algorithmic_basics::insertion_sort::sort, is_sorted};

    #[test]
    fn test() {
        let mut buf = [3, 2, 5, 1, 9, 6, 4];
        sort(&mut buf);

        assert!(is_sorted(&buf));
    }
}
