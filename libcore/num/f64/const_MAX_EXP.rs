#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f64::MAX_EXP;

    // pub const MAX_EXP: i32 = 1024;

    #[test]
    fn max_exp_test1() {
	assert_eq!(MAX_EXP, 1024);
    }
}
