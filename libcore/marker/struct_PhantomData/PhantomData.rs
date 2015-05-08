#![feature(core)]
extern crate core;

trait ResType { fn foo(&self); }
struct ParamType;
mod foreign_lib {
    pub fn new(_: usize) -> *mut () { 42 as *mut () }
    pub fn do_stuff(_: *mut (), _: usize) {}
}
fn convert_params(_: ParamType) -> usize { 42 }

#[cfg(test)]
mod tests {
    use core::marker::PhantomData;
    use core::mem;

    // Unused lifetime parameter
    #[test]
    fn PhantomData_test1() {
	struct Slice<'a, T:'a> {
	    start: *const T,
	    end: *const T,
	    phantom: PhantomData<&'a T>
	}
    }

    // Unused type parameters
    #[test]
    fn PhantomData_test2() {
	use super::ResType;
	use super::ParamType;
	use super::foreign_lib;
	use super::convert_params;

	struct ExternalResource<R> {
	    resource_handle: *mut (),
	    resource_type: PhantomData<R>
	}

	impl<R: ResType> ExternalResource<R> {
	    fn new() -> ExternalResource<R> {
		let size_of_res = mem::size_of::<R>();
		ExternalResource {
		    resource_handle: foreign_lib::new(size_of_res),
		    resource_type: PhantomData
		}
	    }

	    fn do_stuff(&self, param: ParamType) {
		let foreign_params = convert_params(param);
		foreign_lib::do_stuff(self.resource_handle, foreign_params);
	    }
	}
    }
}
