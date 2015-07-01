#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f64::MIN_EXP;

    // pub const MIN_EXP: i32 = -1021;

    #[test]
    fn min_exp_test1() {
	assert_eq!(MIN_EXP, -1021);
    }
}
