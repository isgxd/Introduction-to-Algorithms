//! 矩阵乘法。

///
/// +-        -+   +-        -+   +-        -+
/// | C11  C12 |   | A11  A12 |   | B11  B12 |
/// |          | = |          | * |          |
/// | C21  C22 |   | A21  A22 |   | B21  B22 |
/// +-        -+   +-        -+   +-        -+
///
/// 等价于：
///
/// C11 = A11*B11 + A12*B21
/// C12 = A11*B12 + A12*B22
/// C21 = A21*B11 + A22*B21
/// C22 = A21*B12 + A22*B22
///
fn square_matrix_multiply(a: &[Vec<i32>], b: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let n = a.len();
    let mut c = new_matrix(n);
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    c
}

/// 递归。
fn square_matrix_multiply_recursive() {}

fn new_matrix(n: usize) -> Vec<Vec<i32>> {
    vec![vec![0; n]; n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_matrix_multiply_test() {
        let a = vec![vec![2, 3], vec![4, 5]];
        let b = vec![vec![1, 6], vec![7, 8]];
        let c = square_matrix_multiply(&a, &b);

        assert_eq!(vec![vec![23, 36], vec![39, 64],], c);
    }
}
