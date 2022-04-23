//! 计数排序。

/// 对 a 排序，结果保存在 b 中。
/// r 表示临时空间的大小，必须大于 a 中最大元素。
fn sort(a: &[i32], b: &mut [i32], r: usize) {
    let mut c = vec![0; r];
    // 记录待排序元素的个数。
    for val in a {
        c[*val as usize] += 1;
    }
    // 计算待排序元素的位置（相同元素从高到低）。
    for i in 1..r {
        c[i] += c[i - 1];
    }
    // 倒序输出排序结果。
    for val in a.iter().rev() {
        b[c[*val as usize]] = *val;
        c[*val as usize] -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_test() {
        let a = [3, 2, 2, 3, 5];
        let mut b = [0; 6];
        sort(&a, &mut b, 6);
        assert_eq!(&[0, 2, 2, 3, 3, 5], &b);
    }
}
