#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::i32::MAX;

    // macro_rules! int_module { ($T:ty, $bits:expr) => (
    //
    // // FIXME(#11621): Should be deprecated once CTFE is implemented in favour of
    // // calling the `mem::size_of` function.
    // #[unstable(feature = "num_bits_bytes",
    //            reason = "may want to be an associated function")]
    // pub const BITS : usize = $bits;
    // // FIXME(#11621): Should be deprecated once CTFE is implemented in favour of
    // // calling the `mem::size_of` function.
    // #[unstable(feature = "num_bits_bytes",
    //            reason = "may want to be an associated function")]
    // pub const BYTES : usize = ($bits / 8);
    //
    // // FIXME(#11621): Should be deprecated once CTFE is implemented in favour of
    // // calling the `Bounded::min_value` function.
    // #[stable(feature = "rust1", since = "1.0.0")]
    // pub const MIN: $T = (-1 as $T) << (BITS - 1);
    // // FIXME(#9837): Compute MIN like this so the high bits that shouldn't exist are 0.
    // // FIXME(#11621): Should be deprecated once CTFE is implemented in favour of
    // // calling the `Bounded::max_value` function.
    // #[stable(feature = "rust1", since = "1.0.0")]
    // pub const MAX: $T = !MIN;
    //
    // ) }

    // int_module! { i32, 32 }

    #[test]
    #[allow(overflowing_literals)]
    fn max_test1() {
	assert_eq!(MAX, 0x7fffffff);
    }
}
