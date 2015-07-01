#![feature(core, flt2dec)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::num::flt2dec::strategy::grisu::CACHED_POW10_FIRST_E;

    // #[doc(hidden)] pub const CACHED_POW10_FIRST_E: i16 = -1087;

    #[test]
    fn alpha_test1() {
	let cached_pow10_first_e: i16 = -1087;

	assert_eq!(CACHED_POW10_FIRST_E, cached_pow10_first_e);
    }
}
