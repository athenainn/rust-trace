#![feature(core)]

extern crate core;

#[cfg(test)]
mod tests {
    //     impl<_R> PartialEq for extern "C" fn() -> _R {
    //         #[inline]
    //         fn eq(&self, other: &extern "C" fn() -> _R) -> bool {
    //             let self_: *const () = unsafe { mem::transmute(*self) };
    //             let other_: *const () = unsafe { mem::transmute(*other) };
    //             self_ == other_
    //         }
    //     }

    //     impl<_R> PartialEq for extern "C" fn() -> _R {
    //         #[inline]
    //         fn eq(&self, other: &extern "C" fn() -> _R) -> bool {
    //             let self_: *const () = unsafe { mem::transmute(*self) };
    //             let other_: *const () = unsafe { mem::transmute(*other) };
    //             self_ == other_
    //         }
    //     }
    //     macro_rules! fnptreq {
    //         ($($p:ident),*) => {
    //             #[stable(feature = "rust1", since = "1.0.0")]
    //             impl<_R,$($p),*> PartialEq for extern "C" fn($($p),*) -> _R {
    //                 #[inline]
    //                 fn eq(&self, other: &extern "C" fn($($p),*) -> _R) -> bool {
    //                     let self_: *const () = unsafe { mem::transmute(*self) };
    // 
    //                     let other_: *const () = unsafe { mem::transmute(*other) };
    //                     self_ == other_
    //                 }
    //             }
    //         }
    //     }
    //     fnptreq! { A }
    //     fnptreq! { A,B }
    //     fnptreq! { A,B,C }
    //     fnptreq! { A,B,C,D }
    //     fnptreq! { A,B,C,D,E }

    macro_rules! eq_check {
	($($p:ident),*) => ({
	    extern "C" fn bar<$($p),*>($(_: $p),*) -> () {}
	    extern "C" fn foo<$($p),*>($(_: $p),*) -> () {}

	    let f1: extern "C" fn($($p),*) -> () = bar::<$($p),*>;
	    let f2: extern "C" fn($($p),*) -> () = foo::<$($p),*>;

	    assert_eq!(f1.eq(&f1), true);
	    assert_eq!(f1.eq(&f2), false);
	})
    }

    type T = i32;
    type A = T;
    type B = T;
    type C = T;
    type D = T;
    type E = T;

    #[test]
    fn eq_test1() {
	eq_check! { };
	eq_check! { A };
	eq_check! { A,B };
	eq_check! { A,B,C };
	eq_check! { A,B,C,D };
	eq_check! { A,B,C,D,E };
    }
}
