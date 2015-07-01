#![feature(core, zero_one)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::num::Zero;

    // pub trait Zero {
    //     /// The "zero" (usually, additive identity) for this type.
    //     fn zero() -> Self;
    // }

    // pub trait One {
    //     /// The "one" (usually, multiplicative identity) for this type.
    //     fn one() -> Self;
    // }

    // macro_rules! zero_one_impl {
    //     ($($t:ty)*) => ($(
    //         impl Zero for $t {
    //             #[inline]
    //             fn zero() -> Self { 0 }
    //         }
    //         impl One for $t {
    //             #[inline]
    //             fn one() -> Self { 1 }
    //         }
    //     )*)
    // }
    // zero_one_impl! { u8 u16 u32 u64 usize i8 i16 i32 i64 isize }

    // macro_rules! zero_one_impl_float {
    //     ($($t:ty)*) => ($(
    //         impl Zero for $t {
    //             #[inline]
    //             fn zero() -> Self { 0.0 }
    //         }
    //         impl One for $t {
    //             #[inline]
    //             fn one() -> Self { 1.0 }
    //         }
    //     )*)
    // }
    // zero_one_impl_float! { f32 f64 }

    type T = u64;

    #[test]
    fn zero_test1() {
	let value: T = T::zero();

	assert_eq!(value, 0x0000000000000000);
    }
}
