/* 
    # P1 #
    If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23. 
    Find the sum of all the multiples of 3 or 5 below 1000.
*/

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

fn compute(bound: u32) -> u32 {
    (1..bound).filter(|&n| n % 3 == 0|| n % 5 == 0).sum()
}

fn solve() -> String {
    compute(1000).to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn sum_below_ten() {
        assert_eq!(23, super::compute(10));
    }
}
