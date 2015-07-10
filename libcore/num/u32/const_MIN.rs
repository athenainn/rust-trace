#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::u32::MIN;

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

    // uint_module! { u32, i32, 32 }

    #[test]
    fn bits_test1() {
	assert_eq!(MIN, 0x00000000);
    }
}
