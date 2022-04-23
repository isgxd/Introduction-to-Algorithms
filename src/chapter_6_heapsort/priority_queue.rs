//! 最大优先队列。

use super::heapsort::{max_heapify, parent};

/// 将值（key）插入队列最后，随后维护整个队列。
fn max_heap_insert(a: &mut Vec<i32>, key: i32) {
    a.push(i32::MIN);
    let i = a.len() - 1;
    heap_increase_key(&mut a[..], i, key);
}

/// 在位置 i 设置一个更大的值（key），随后维护整个队列。
fn heap_increase_key(a: &mut [i32], mut i: usize, key: i32) {
    if key < a[i] {
        panic!("new key is smaller than current key");
    }

    a[i] = key;
    while i > 0 && a[parent(i)] < a[i] {
        a.swap(i, parent(i));
        i = parent(i);
    }
}

/// 提取堆的最大值，缩小规模。
fn heap_extract_max(a: &mut Vec<i32>) -> i32 {
    if a.is_empty() {
        panic!("heap underflow");
    }

    let max = a[0];
    a[0] = a.pop().unwrap();
    max_heapify(&mut a[..], 0);

    max
}

/// 获取堆的最大值，规模不变。
fn heap_maximum(a: &[i32]) -> i32 {
    a[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heap_extract_max_test() {
        let mut buf = vec![22, 18, 19, 9, 6, 12, 8];
        assert_eq!(7, buf.len());

        let val = heap_extract_max(&mut buf);
        assert_eq!(22, val);
        assert_eq!(6, buf.len());

        let expect = [19, 18, 12, 9, 6, 8];
        assert_eq!(&expect, &buf[..])
    }

    #[test]
    fn heap_increase_key_test() {
        let mut buf = [9, 3, 2];
        heap_increase_key(&mut buf, 2, 10);
        assert_eq!(&[10, 3, 9], &buf);
    }

    #[test]
    fn max_heap_insert_test() {
        let mut buf = vec![22, 18, 19, 9, 6, 12, 8];
        assert_eq!(7, buf.len());

        max_heap_insert(&mut buf, 13);
        assert_eq!(8, buf.len());

        let val = [22, 18, 19, 13, 6, 12, 8, 9];
        assert_eq!(&val, &buf[..]);
    }
}
