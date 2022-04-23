//! 寻找最小值。

/// 获取切片中的最小值。
///
/// # Arguments
///
/// * `a`: 一个非空切片。
///
/// returns: i32
///
/// # Examples
///
/// ```
/// let a = [8, 7, 9, 4, 6];
/// assert_eq!(4, min_num(&a));
/// ```
fn min_num(a: &[i32]) -> i32 {
    let mut min = a[0];
    for it in a {
        if it < &min {
            min = *it;
        }
    }
    min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let a = [8, 7, 9, 4, 6];
        assert_eq!(4, min_num(&a));
    }
}
