//! 随机数算法。

const MBIG: i32 = i32::MAX;

/// 移植自 Net Framework 的随机数算法。
#[derive(Debug)]
pub struct Random {
    inext: usize,
    inextp: usize,
    seed_array: Vec<i32>,
}

impl Random {
    /// 根据指定的种子，创建一个 Random 对象。
    pub fn new(seed: i32) -> Self {
        let subtraction = if seed == i32::MIN {
            i32::MAX
        } else {
            seed.abs()
        };

        let mseed = 161803398;
        let mut mj = mseed - subtraction;

        let mut seed_array = vec![0; 56];
        seed_array[55] = mj;

        let mut mk = 1;
        for i in 1..55 {
            let ii = ((21 * i) % 55) as usize;
            seed_array[ii] = mk;
            mk = mj - mk;
            if mk < 0 {
                mk += MBIG;
            }
            mj = seed_array[ii];
        }

        for _ in 1..5 {
            for i in 1..56 {
                let ii = (1 + (i + 30) % 55) as usize;
                seed_array[i] -= seed_array[ii];
                if seed_array[i] < 0 {
                    seed_array[i] += MBIG;
                }
            }
        }

        Self {
            inext: 0,
            inextp: 21,
            seed_array,
        }
    }

    /// 获取 [min,max] 范围内的随机数。
    fn next_with(&mut self, min: i32, max: i32) -> i32 {
        if min > max {
            panic!("取值范围错误");
        }

        let range = max as f64 - min as f64;
        if range <= i32::MAX as f64 {
            (self.sample() * range) as i32 + min
        } else {
            (self.sample_for_large_range() * range) as i32 + min
        }
    }

    /// 获取一个 [0,1) 范围的样本。
    fn sample(&mut self) -> f64 {
        let val = self.next() as f64;
        let range = 1.0 / MBIG as f64;
        val * range
    }

    /// 针对大范围随机数，获取一个 [0,2*i32::MAX-1) 的样本。
    fn sample_for_large_range(&mut self) -> f64 {
        let mut result = self.next();
        if (self.next() % 2) == 0 {
            result = -result;
        }

        let mut d = result as f64;
        d += (i32::MAX - 1) as f64;
        d /= 2.0 * (i32::MAX as f64) - 1.0;
        d
    }

    /// 获取一个随机数。
    pub fn next(&mut self) -> i32 {
        let mut inext = self.inext;
        let mut inextp = self.inextp;

        inext += 1;
        if inext >= 56 {
            inext = 1;
        }
        inextp += 1;
        if inextp >= 56 {
            inextp = 1;
        }

        let mut val = self.seed_array[inext] - self.seed_array[inextp];

        if val == MBIG {
            val -= 1;
        }
        if val < 0 {
            val += MBIG;
        }

        self.seed_array[inext] = val;
        self.inext = inext;
        self.inextp = inextp;

        val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_test() {
        let mut r = Random::new(5);
        assert_eq!(726643700, r.next());
    }

    #[test]
    fn next_with_test() {
        let mut r = Random::new(5);
        let v = r.next_with(2, 4);
        println!("v = {v}");
        assert!(v <= 4);
        assert!(v >= 2);
    }
}
