//! 快速排序。

/// 排序。
fn sort(a: &mut [i32]) {
    if a.is_empty() {
        return;
    }

    let q = partition(a);
    sort(&mut a[..q]);
    sort(&mut a[q + 1..]);
}

/// 尾递归版排序。
fn tail_recursive_sort(a: &mut [i32]) {
    let mut low = 0;
    // 确保每次可划分的切片长度大于 1。
    while low + 1 < a.len() {
        let mid = partition(&mut a[low..]);

        tail_recursive_sort(&mut a[low..mid]);
        low = mid + 1;
    }
}

/// 将切片划分为两部分，返回主元索引。
fn partition(a: &mut [i32]) -> usize {
    // 最后一个元素作为主元，应排除。
    let len = a.len() - 1;
    let x = a[len];

    let mut i = 0;
    for j in 0..len {
        if a[j] <= x {
            a.swap(j, i);
            i += 1;
        }
    }
    a.swap(i, len);
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partition_test() {
        let mut buf = [4, 9, 3, 5, 6];
        assert_eq!(3, partition(&mut buf));
        assert_eq!(&[4, 3, 5, 6, 9], &buf);
    }

    #[test]
    fn sort_test() {
        let mut buf = [4, 9, 3, 5, 6];
        sort(&mut buf);
        assert_eq!(&[3, 4, 5, 6, 9], &buf);
    }

    #[test]
    fn tail_recursive_sort_test() {
        let mut buf = [4, 9, 3, 5, 6];
        tail_recursive_sort(&mut buf);
        assert_eq!(&[3, 4, 5, 6, 9], &buf);
    }
}
