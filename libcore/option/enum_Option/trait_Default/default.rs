#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    #[derive(Debug)]
    struct A;

    #[test]
    fn default_test1() {
	let x: Option<A> = Option::<A>::default();
	assert_eq!(x.is_none(), true);
    }

    #[test]
    fn default_test2() {
	let x: Option<A> = Default::default();
	assert_eq!(x.is_none(), true);
    }
}
