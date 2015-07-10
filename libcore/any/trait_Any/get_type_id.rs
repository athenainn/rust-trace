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

    // impl<T: Reflect + 'static> Any for T {
    //     fn get_type_id(&self) -> TypeId { TypeId::of::<T>() }
    // }

    type T = i32;

    #[test]
    fn get_type_id_test1() {
	let x: T = 68;
	let y: T = 500;
	let x_typeid: TypeId = x.get_type_id();
	let y_typeid: TypeId = y.get_type_id();

	assert_eq!(x_typeid, y_typeid);
    }

    #[test]
    fn get_type_id_test2() {
	struct A;

	let x: A = A;
	let _: TypeId = x.get_type_id();
    }
}
