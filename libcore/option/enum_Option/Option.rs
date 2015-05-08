#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    #[test]
    fn Option_test() {
	type T = u32;
	let x: Option<T> = Some(2);
	let y: Option<T> = None;

	match x {
	    Some(z) => { assert_eq!(z, 2); }
	    _ => { assert!(false); }
	}

	match y {
	    None => { assert!(true); }
	    _ => { assert!(false); }
	}
    }
}
