#![feature(core, alloc)]
extern crate core;
extern crate alloc;

#[cfg(test)]
mod tests {
    use alloc::boxed::Box;

    use core::hash::Hash;
    use core::hash::Hasher;
    use core::hash::SipHasher;

    // pub struct Box<T>(Unique<T>);

    // #[stable(feature = "rust1", since = "1.0.0")]
    // impl<T: Default> Default for Box<T> {
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn default() -> Box<T> { box Default::default() }
    // }

    // impl<T: ?Sized + Hash> Hash for Box<T> {
    //     fn hash<H: hash::Hasher>(&self, state: &mut H) {
    //         (**self).hash(state);
    //     }
    // }

    type T = i32; // T: ?Sized + Hash
    type H = SipHasher; // H: Hasher + Default

    #[test]
    fn default_test1() {
	let b: Box<T> = Box::<T>::default();
	let mut state: SipHasher = SipHasher::new();

	let finish: u64 = state.finish();
	assert_eq!(finish, 0x1e924b9d737700d7);

	b.hash::<H>(&mut state);

	let finish: u64 = state.finish();
	assert_eq!(finish, 0x7bf55e51b22b9698);
    }
}
