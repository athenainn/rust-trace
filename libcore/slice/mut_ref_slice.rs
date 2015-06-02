#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::slice::mut_ref_slice;

    struct A<T> {
	value: T
    }

    // pub fn mut_ref_slice<'a, A>(s: &'a mut A) -> &'a mut [A] {
    //     unsafe {
    //         from_raw_parts_mut(s, 1)
    //     }
    // }

    type T = i32;

    #[test]
    fn mut_ref_slice_test1() {
	let mut a: A<T> = A { value: 68 };

	{
	    let mut_ref_slice: &mut [A<T>] = mut_ref_slice(&mut a);
	    mut_ref_slice[0].value = 500;
	}

	assert_eq!(a.value, 500);
    }
}
