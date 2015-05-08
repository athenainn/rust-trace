#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    #[test]
    fn as_mut_test() {
	type T = u32;
	let mut x: Option<T> = Some(2);

	{
	    let a: Option<&mut T> = x.as_mut();

	    match a {
		Some(v) => *v = 42,
		None => {}
	    }
	}
	assert_eq!(x, Some(42));
    }
}
