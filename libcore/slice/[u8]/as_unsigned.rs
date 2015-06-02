#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::slice::IntSliceExt;

    // pub trait IntSliceExt<U, S> {
    //     /// Converts the slice to an immutable slice of unsigned integers with the same width.
    //     fn as_unsigned<'a>(&'a self) -> &'a [U];
    //     /// Converts the slice to an immutable slice of signed integers with the same width.
    //     fn as_signed<'a>(&'a self) -> &'a [S];
    //
    //     /// Converts the slice to a mutable slice of unsigned integers with the same width.
    //     fn as_unsigned_mut<'a>(&'a mut self) -> &'a mut [U];
    //     /// Converts the slice to a mutable slice of signed integers with the same width.
    //     fn as_signed_mut<'a>(&'a mut self) -> &'a mut [S];
    // }

    // macro_rules! impl_int_slice {
    //     ($u:ty, $s:ty, $t:ty) => {
    //         #[unstable(feature = "core")]
    //         impl IntSliceExt<$u, $s> for [$t] {
    //             #[inline]
    //             fn as_unsigned(&self) -> &[$u] { unsafe { transmute(self) } }
    //             #[inline]
    //             fn as_signed(&self) -> &[$s] { unsafe { transmute(self) } }
    //             #[inline]
    //             fn as_unsigned_mut(&mut self) -> &mut [$u] { unsafe { transmute(self) } }
    //             #[inline]
    //             fn as_signed_mut(&mut self) -> &mut [$s] { unsafe { transmute(self) } }
    //         }
    //     }
    // }

    // macro_rules! impl_int_slices {
    //     ($u:ty, $s:ty) => {
    //         impl_int_slice! { $u, $s, $u }
    //         impl_int_slice! { $u, $s, $s }
    //     }
    // }

    // impl_int_slices! { u8,   i8  }

    type U = u8;
    type S = i8;
    type T = U;

    #[test]
    fn as_unsigned_test1() {
	let slice: &[T] = &[0xff];
	let as_unsigned: &[U] = slice.as_unsigned();

	assert_eq!(as_unsigned[0], 255);
    }
}
