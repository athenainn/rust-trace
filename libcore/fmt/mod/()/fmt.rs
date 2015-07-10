#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    // impl Debug for () {
    //     fn fmt(&self, f: &mut Formatter) -> Result {
    //         f.pad("()")
    //     }
    // }

    type T = u32;

    #[test]
    fn fmt_test1() {
	let value: () = (); 
	let output: String = format!("{:?}", value);

	assert_eq!(output, "()".to_string());
    }
}
