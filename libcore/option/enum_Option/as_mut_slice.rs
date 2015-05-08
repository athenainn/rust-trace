#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    #[test]
    fn as_mut_slice_test() {
	type T = &'static str;
	let mut x: Option<T> = Some("Diamonds");
	{
	    let v: &mut [T] = x.as_mut_slice();
	    assert!(v == ["Diamonds"]);
	    v[0] = "Dirt";
	    assert!(v == ["Dirt"]);
	}
	assert_eq!(x, Some("Dirt"));
    }
}
