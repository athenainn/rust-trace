#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f64::MAX_10_EXP;

    // pub const MAX_10_EXP: i32 = 308;

    #[test]
    fn max_10_exp_test1() {
	assert_eq!(MAX_10_EXP, 308);
    }
}
