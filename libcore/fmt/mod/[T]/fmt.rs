#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    // impl<T: Debug> Debug for [T] {
    //     fn fmt(&self, f: &mut Formatter) -> Result {
    //         f.debug_list().entries(self.iter()).finish()
    //     }
    // }

    type T = u32;

    #[test]
    fn fmt_test1() {
	let value: [T; 3] = [68, 500, 999]; 
	let output: String = format!("{:?}", value);

	assert_eq!(output, "[68, 500, 999]".to_string());
    }
}
