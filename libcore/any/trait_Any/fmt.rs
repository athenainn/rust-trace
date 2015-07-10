#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::any::Any;

    // pub trait Any: Reflect + 'static {
    //     /// Gets the `TypeId` of `self`.
    //     #[unstable(feature = "core",
    //                reason = "this method will likely be replaced by an associated static")]
    //     fn get_type_id(&self) -> TypeId;
    // }
    //
    // impl<T: Reflect + 'static> Any for T {
    //     fn get_type_id(&self) -> TypeId { TypeId::of::<T>() }
    // }

    // impl fmt::Debug for Any {
    //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //         f.pad("Any")
    //     }
    // }

    type T = i32;

    #[test]
    fn fmt_test1() {
	let x: T = 68;
	let any: &Any = &x;
	let message: String = format!("{:?}", any);

	assert_eq!(message, "Any".to_string());
    }
}
