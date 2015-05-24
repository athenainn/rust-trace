#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::raw::Slice;
    use core::raw::Repr;

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
