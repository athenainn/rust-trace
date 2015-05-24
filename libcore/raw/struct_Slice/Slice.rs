#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::raw::Slice;
    use core::raw::Repr;

    // pub unsafe trait Repr<T> {
    //     /// This function "unwraps" a rust value (without consuming it) into its raw
    //     /// struct representation. This can be used to read/write different values
    //     /// for the struct. This is a safe method because by default it does not
    //     /// enable write-access to the fields of the return value in safe code.
    //     #[inline]
    //     fn repr(&self) -> T { unsafe { mem::transmute_copy(&self) } }
    // }

    // #[repr(C)]
    // pub struct Slice<T> {
    //     pub data: *const T,
    //     pub len: usize,
    // }

    type T = i32;

    #[test]
    fn slice_test1 () {
	let slice: &[T] = &[1, 2, 3, 4];
	let repr: Slice<T> = slice.repr();

	assert_eq!(repr.len, 4);
    }

    #[test]
    fn slice_test2 () {
	let slice: &[T] = &[1, 2, 3, 4];
	let repr: Slice<T> = slice.repr();

	let copy: &[T] = slice;
	let copy_repr: Slice<T> = copy.repr();

	assert_eq!(copy_repr.data, repr.data);
	assert_eq!(copy_repr.len, repr.len);
    }
}
