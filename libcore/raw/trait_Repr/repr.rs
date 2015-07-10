#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::raw::Repr;

    struct A;

    struct Pointer<T> {
	address: *const T
    }

    // pub unsafe trait Repr<T> {
    //     /// This function "unwraps" a rust value (without consuming it) into its raw
    //     /// struct representation. This can be used to read/write different values
    //     /// for the struct. This is a safe method because by default it does not
    //     /// enable write-access to the fields of the return value in safe code.
    //     #[inline]
    //     fn repr(&self) -> T { unsafe { mem::transmute_copy(&self) } }
    // }

    unsafe impl Repr<Pointer<A>> for A {}

    #[test]
    fn repr_test1 () {
	let a: A = A;
	let repr: Pointer<A> = a.repr();

	assert_eq!(repr.address as *const A, &a as *const A);
    }
}
