#![feature(core, flt2dec)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::num::flt2dec::strategy::grisu::ALPHA;

    // #[doc(hidden)] pub const ALPHA: i16 = -60;

    #[test]
    fn alpha_test1() {
	let alpha: i16 = -60;

	assert_eq!(ALPHA, alpha);
    }
}
