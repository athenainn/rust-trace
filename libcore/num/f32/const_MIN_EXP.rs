#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f32::MIN_EXP;

    // pub const MIN_EXP: i32 = -125;

    #[test]
    fn min_exp_test1() {
	assert_eq!(MIN_EXP, -125);
    }
}
