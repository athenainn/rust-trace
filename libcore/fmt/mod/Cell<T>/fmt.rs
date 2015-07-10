#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::cell::Cell;

    // impl<T: Copy + Debug> Debug for Cell<T> {
    //     fn fmt(&self, f: &mut Formatter) -> Result {
    //         write!(f, "Cell {{ value: {:?} }}", self.get())
    //     }
    // }

    type T = u32;

    #[test]
    fn fmt_test1() {
	let value: T = 68;
	let cell: Cell<T> = Cell::<T>::new(value);
	let output: String = format!("{:?}", cell);

	assert_eq!(output, "Cell { value: 68 }".to_string());
    }
}
