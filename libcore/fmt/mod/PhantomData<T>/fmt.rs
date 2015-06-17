#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::marker::PhantomData;

    // impl<T> Debug for PhantomData<T> {
    //     fn fmt(&self, f: &mut Formatter) -> Result {
    //         f.pad("PhantomData")
    //     }
    // }

    type T = u32;

    #[test]
    fn fmt_test1() {
	let value: PhantomData<T> = PhantomData::<T>;
	let output: String = format!("{:?}", value);

	assert_eq!(output, "PhantomData".to_string());
    }
}
