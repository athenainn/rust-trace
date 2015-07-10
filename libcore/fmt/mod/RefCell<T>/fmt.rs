#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::cell::RefCell;

    // impl<T: ?Sized + Debug> Debug for RefCell<T> {
    //     fn fmt(&self, f: &mut Formatter) -> Result {
    //         match self.borrow_state() {
    //             BorrowState::Unused | BorrowState::Reading => {
    //                 write!(f, "RefCell {{ value: {:?} }}", self.borrow())
    //             }
    //             BorrowState::Writing => write!(f, "RefCell {{ <borrowed> }}"),
    //         }
    //     }
    // }

    type T = u32;

    #[test]
    fn fmt_test1() {
	let value: T = 68;
	let refcell: RefCell<T> = RefCell::<T>::new(value);
	let output: String = format!("{:?}", refcell);

	assert_eq!(output, "RefCell { value: 68 }".to_string());
    }
}
