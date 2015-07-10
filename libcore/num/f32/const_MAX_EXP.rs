#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::f32::MAX_EXP;

    // pub const MAX_EXP: i32 = 128;

    #[test]
    fn max_exp_test1() {
	assert_eq!(MAX_EXP, 128);
    }
}
