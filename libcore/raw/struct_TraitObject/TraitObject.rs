#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::raw::TraitObject;

    use core::intrinsics::transmute;

    trait Foo<T> {
	fn bar(&self) -> T;
    }

    impl Foo<T> for T {
	fn bar(&self) -> i32 {
	    *self + 1
	}
    }

    // #[repr(C)]
    // #[derive(Copy, Clone)]
    // pub struct TraitObject {
    //     pub data: *mut (),
    //     pub vtable: *mut (),
    // }

    type T = i32;

    #[test]
    fn traitobject_test1() {
	let value: T = 123;

	// let the compiler make a trait object
	let object: &Foo<T> = &value;

	// look at the raw representation
	let raw_object: TraitObject = unsafe {
	    transmute::<&Foo<T>, TraitObject>(object)
	};

	// the data pointer is the address of `value`
	assert_eq!(raw_object.data as *const T, &value as *const _);

	let other_value: T = 456;

	// construct a new object, pointing to a different `T`, being
	// careful to use the `T` vtable from `object`
	let synthesized: &Foo<T> = unsafe {
	    transmute::<TraitObject, &Foo<T>>(
		TraitObject {
		    data: &other_value as *const _ as *mut (),
		    vtable: raw_object.vtable
		}
	    )
	};

	// it should work just like we constructed a trait object out of
	// `other_value` directly
	assert_eq!(synthesized.bar(), 457);
    }
}
