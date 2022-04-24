//! 桶排序。

use crate::chapter_2_algorithmic_basics::insertion_sort;

fn sort(a: &mut [i32]) {
    // 查询最大、最小值。
    let mut min = a[0];
    let mut max = a[0];
    for i in 1..a.len() {
        min = min.min(a[i]);
        max = max.max(a[i]);
    }

    // 建桶。
    let num = ((max - min) as usize) / a.len() + 1;
    let mut b = vec![vec![]; num];

    // 装桶。
    for val in a.iter() {
        let i = ((*val - min) as usize) / a.len();
        b[i].push(*val);
    }

    // 排序。
    for it in b.iter_mut() {
        insertion_sort::sort(&mut it[..]);
    }

    // 归并。
    let mut i = 0;
    for buf in b.iter() {
        for val in buf {
            a[i] = *val;
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::is_sorted;

    #[test]
    fn sort_test() {
        let mut a = [12, 19, 10, 15, 18, 13];
        sort(&mut a);
        assert!(is_sorted(&a));
    }
}
