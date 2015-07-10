#![feature(core, flt2dec)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::num::flt2dec::bignum::Digit32;

    use core::default::Default;

    use core::any::Any;

    // pub type Digit32 = u32;

    #[test]
    fn digit32_test1() {
	let x: Digit32 = Digit32::default();
	let result: bool = (&x as &Any).is::<u32>();

	assert_eq!(result, true);
    }
}
