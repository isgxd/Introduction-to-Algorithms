//! 最大堆排序。

fn sort(a: &mut [i32]) {
    build_max_heap(a);

    for i in (1..a.len()).rev() {
        a.swap(0, i);
        max_heapify(&mut a[..i], 0);
    }
}

fn build_max_heap(a: &mut [i32]) {
    let len = a.len();
    for i in (0..len / 2).rev() {
        max_heapify(a, i);
    }
}

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

pub(crate) fn parent(i: usize) -> usize {
    i / 2
}

fn left(i: usize) -> usize {
    i << 1
}

fn right(i: usize) -> usize {
    left(i) + 1
}

#[cfg(test)]
mod tests {
    use crate::is_sorted;

    use super::*;

    #[test]
    fn test() {
        let mut a = [16, 4, 10, 14, 7, 9, 3, 2, 8, 1];
        sort(&mut a);

        println!("{a:?}");
        assert!(is_sorted(&a));
    }
}