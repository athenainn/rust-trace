#![feature(core, flt2dec)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::num::flt2dec::strategy::grisu::GAMMA;

    // #[doc(hidden)] pub const GAMMA: i16 = -32;

    #[test]
    fn gamma_test1() {
	let gamma: i16 = -32;

	assert_eq!(GAMMA, gamma);
    }
}
