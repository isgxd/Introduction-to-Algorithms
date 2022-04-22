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
