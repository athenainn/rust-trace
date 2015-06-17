#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::cell::RefCell;
    use core::cell::RefMut;

    // impl<'b, T: ?Sized + Debug> Debug for RefMut<'b, T> {
    //     fn fmt(&self, f: &mut Formatter) -> Result {
    //         Debug::fmt(&*(self.deref()), f)
    //     }
    // }

    type T = u32;

    #[test]
    fn fmt_test1() {
	let value: T = 68;
	let refcell: RefCell<T> = RefCell::<T>::new(value);
	let value_refmut: RefMut<T> = refcell.borrow_mut();
	let output: String = format!("{:?}", value_refmut);

	assert_eq!(output, "68".to_string());
    }
}
