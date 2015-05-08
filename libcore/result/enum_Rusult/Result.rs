#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    #[test]
    fn Result_test() {
	type T = i32;
	type E = &'static str;
	let x: Result<T, E> = Ok(-3);
	let y: Result<T, E> = Err("Some error message");

	match x {
	    Ok(z) => { assert_eq!(z, -3); }
	    _ => { assert!(false); }
	}
	match y {
	    Err(m) => { assert_eq!(m, "Some error message"); }
	    _ => { assert!(false); }
	}
    }
}
