#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::slice::ref_slice;

    use core::raw::Repr;

    struct A<T> {
	value: T
    }

    // pub fn ref_slice<'a, A>(s: &'a A) -> &'a [A] {
    //     unsafe {
    //         from_raw_parts(s, 1)
    //     }
    // }

    type T = i32;

    #[test]
    fn ref_slice_test1() {
	let a: A<T> = A { value: 68 };
	let ref_slice: &[A<T>] = ref_slice(&a);

	assert_eq!(ref_slice.repr().data as *const A<T>, &a as *const A<T>);
	assert_eq!(ref_slice[0].value, a.value);
    }
}
