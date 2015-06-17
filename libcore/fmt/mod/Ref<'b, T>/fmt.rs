#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::cell::RefCell;
    use core::cell::Ref;

    // impl<'b, T: ?Sized + Debug> Debug for Ref<'b, T> {
    //     fn fmt(&self, f: &mut Formatter) -> Result {
    //         Debug::fmt(&**self, f)
    //     }
    // }

    type T = u32;

    #[test]
    fn fmt_test1() {
	let value: T = 68;
	let refcell: RefCell<T> = RefCell::<T>::new(value);
	let value_ref: Ref<T> = refcell.borrow();
	let output: String = format!("{:?}", value_ref);

	assert_eq!(output, "68".to_string());
    }
}
