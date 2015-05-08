#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::option::IterMut;

    type T = u32;
    type O = Option<T>;
    type A = T;

    #[test]
    fn next_test1() {
	let mut x: O = Some(68);

	let mut it: IterMut<T> = x.iter_mut();

	let a: Option<&mut A> = it.next();
	match a {
	    Some(&mut ref mut x) => *x = 500,
	    None => {}
	}

	assert_eq!(a.unwrap(), &mut 500);
    }

    #[test]
    fn next_test2() {
	let mut x: O = None;

	let mut it: IterMut<T> = x.iter_mut();

	let a: Option<&mut A> = it.next();
	match a {
	    Some(&mut ref mut x) => *x = 500,
	    None => {}
	}

	let b: Option<&mut T> = it.next();

	assert_eq!(a.is_none(), true);
    }
}
