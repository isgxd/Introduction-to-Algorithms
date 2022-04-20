#![cfg_attr(debug_assertions, allow(dead_code))]

mod chapter_2_algorithmic_basics;
mod chapter_4_divide_and_conquer;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
