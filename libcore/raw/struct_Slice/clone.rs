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
    fn clone_test2 () {
	let slice: &[T] = &[1, 2, 3, 4];
	let repr: Slice<T> = slice.repr();

	let clone: &[T] = slice.clone();
	let clone_repr: Slice<T> = clone.repr();

	assert_eq!(clone_repr.data, repr.data);
	assert_eq!(clone_repr.len, repr.len);
    }
}
