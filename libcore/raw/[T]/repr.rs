#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::raw::Repr;
    use core::raw::Slice;

    // pub unsafe trait Repr<T> {
    //     /// This function "unwraps" a rust value (without consuming it) into its raw
    //     /// struct representation. This can be used to read/write different values
    //     /// for the struct. This is a safe method because by default it does not
    //     /// enable write-access to the fields of the return value in safe code.
    //     #[inline]
    //     fn repr(&self) -> T { unsafe { mem::transmute_copy(&self) } }
    // }

    // pub struct Slice<T> {
    //     pub data: *const T,
    //     pub len: usize,
    // }

    // unsafe impl<T> Repr<Slice<T>> for [T] {}

    type T = i32;

    #[test]
    fn repr_test1 () {
	let slice: &[T] = &[1, 2, 3, 4, 5, 6];
	let repr: Slice<T> = slice.repr();

	assert_eq!(repr.data, &slice[0]);
	assert_eq!(repr.len, slice.len());
    }
}
