#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::raw::Slice;
    use core::raw::Repr;

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
