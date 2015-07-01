#![feature(core, num_bits_bytes)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::u16::BYTES;

    // macro_rules! uint_module { ($T:ty, $T_SIGNED:ty, $bits:expr) => (
    //
    // #[unstable(feature = "num_bits_bytes",
    //            reason = "may want to be an associated function")]
    // pub const BITS : usize = $bits;
    // #[unstable(feature = "num_bits_bytes",
    //            reason = "may want to be an associated function")]
    // pub const BYTES : usize = ($bits / 8);
    //
    // #[stable(feature = "rust1", since = "1.0.0")]
    // pub const MIN: $T = 0 as $T;
    // #[stable(feature = "rust1", since = "1.0.0")]
    // pub const MAX: $T = !0 as $T;
    //
    // ) }

    // uint_module! { u16, i16, 16 }

    #[test]
    fn bytes_test1() {
	assert_eq!(BYTES, 2);
    }
}
