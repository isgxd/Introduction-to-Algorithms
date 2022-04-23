//! 最大堆排序。

/// 堆排序。
fn sort(a: &mut [i32]) {
    build_max_heap(a);

    for i in (1..a.len()).rev() {
        a.swap(0, i);
        max_heapify(&mut a[..i], 0);
    }
}

/// 将切片转换为最大堆。
fn build_max_heap(a: &mut [i32]) {
    let len = a.len() / 2;
    for i in (0..len).rev() {
        max_heapify(a, i);
    }
}

/// 从指定索引开始往下调整最大堆。
pub(crate) fn max_heapify(a: &mut [i32], i: usize) {
    let l = left(i);
    let r = right(i);
    let mut largest = if l < a.len() && a[l] > a[i] { l } else { i };
    if r < a.len() && a[r] > a[largest] {
        largest = r;
    }
    if largest != i {
        a.swap(i, largest);
        max_heapify(a, largest);
    }
}

/// 获取父节点的索引。
pub(crate) fn parent(i: usize) -> usize {
    (i - 1) >> 1
}

/// 获取左叶子的索引。
fn left(i: usize) -> usize {
    (i << 1) + 1
}

/// 获取右叶子的索引。
fn right(i: usize) -> usize {
    left(i) + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::is_sorted;

    #[test]
    fn left_test() {
        assert_eq!(1, left(0));
        assert_eq!(3, left(1));
    }

    #[test]
    fn right_test() {
        assert_eq!(2, right(0));
        assert_eq!(4, right(1));
    }

    #[test]
    fn parent_test() {
        assert_eq!(0, parent(1));
        assert_eq!(0, parent(2));
        assert_eq!(1, parent(3));
        assert_eq!(1, parent(4));
    }

    #[test]
    fn sort_test() {
        let mut a = [16, 4, 10, 14, 7, 9, 3, 2, 8, 1];
        sort(&mut a);

        println!("{a:?}");
        assert!(is_sorted(&a));
    }

    #[test]
    fn max_heapify_test() {
        let mut buf = [2, 8, 7, 9, 5, 6, 3];
        max_heapify(&mut buf, 0);
        assert_eq!(&[8, 9, 7, 2, 5, 6, 3], &buf);
    }

    #[test]
    fn build_max_heap_test() {
        let mut buf = [2, 8, 7, 9, 5, 6, 3];
        build_max_heap(&mut buf);
        assert_eq!(&[9, 8, 7, 2, 5, 6, 3], &buf);
    }
}
