#![feature(core, flt2dec)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::num::flt2dec::MAX_SIG_DIGITS;

    // pub const MAX_SIG_DIGITS: usize = 17;

    #[test]
    fn max_sig_digits_test1() {
	let max_sig_digits: usize = 17;

	assert_eq!(MAX_SIG_DIGITS, max_sig_digits);
    }
}
