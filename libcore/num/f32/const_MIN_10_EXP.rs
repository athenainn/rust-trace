#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f32::MIN_10_EXP;

    // pub const MIN_10_EXP: i32 = -37;

    #[test]
    fn min_10_exp_test1() {
	assert_eq!(MIN_10_EXP, -37);
    }
}
