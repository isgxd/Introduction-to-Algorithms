//! 最大子数组。

fn find_maximum_subarray(input: &[i32]) -> (usize, usize, i32) {
    if input.len() == 1 {
        return (0, 1, input[0]);
    }

    let mid = input.len() / 2;
    let (left_low, left_high, left_sum) = find_maximum_subarray(&input[..mid]);
    let (right_low, right_high, right_sum) = find_maximum_subarray(&input[mid..]);
    let (cross_low, cross_high, cross_sum) = find_max_crossing_subarray(input, mid);
    if left_sum >= right_sum && left_sum >= cross_sum {
        (left_low, left_high, left_sum)
    } else if right_sum >= left_sum && right_sum >= cross_sum {
        (right_low, right_high, right_sum)
    } else {
        (cross_low, cross_high, cross_sum)
    }
}

fn find_max_crossing_subarray(input: &[i32], mid: usize) -> (usize, usize, i32) {
    let mut left_sum = i32::MIN;
    let mut max_left = 0;
    let mut sum = 0;
    for i in (0..mid).rev() {
        sum += input[i];
        if sum > left_sum {
            left_sum = sum;
            max_left = i;
        }
    }

    let mut right_sum = i32::MIN;
    let mut max_right = 0;
    sum = 0;
    for j in mid..input.len() {
        sum += input[j];
        if sum > right_sum {
            right_sum = sum;
            max_right = j;
        }
    }

    (max_left, max_right, left_sum + right_sum)
}

#[cfg(test)]
mod tests {
    use super::find_maximum_subarray;

    #[test]
    fn test() {
        let array = [12, -4, -6, 9, 32, -33, 4];
        let r = find_maximum_subarray(&array);
        assert_eq!((0, 4, 43), r);
    }
}
