#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::any::Any;
    use core::any::TypeId;

    // #[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
    // #[stable(feature = "rust1", since = "1.0.0")]
    // pub struct TypeId {
    //     t: u64,
    // }

    // impl TypeId {
    //     /// Returns the `TypeId` of the type this generic function has been
    //     /// instantiated with
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn of<T: ?Sized + Reflect + 'static>() -> TypeId {
    //         TypeId {
    //             t: unsafe { intrinsics::type_id::<T>() },
    //         }
    //     }
    // }

    type T = i32;

    #[test]
    fn of_test1() {
	let x: T = 68;
	let x_typeid: TypeId = x.get_type_id();
	let typeid: TypeId = TypeId::of::<T>();

	assert_eq!(x_typeid, typeid);
    }

    #[test]
    #[should_panic]
    fn of_test2() {
	struct A;

	let x: T = 68;
	let x_typeid: TypeId = x.get_type_id();
	let typeid: TypeId = TypeId::of::<A>();

	assert_eq!(x_typeid, typeid);
    }
}
