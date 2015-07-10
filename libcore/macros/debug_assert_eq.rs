// runs rustc with `-C debug-assertions=off` that can disable `debug_assert!`

#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    // macro_rules! debug_assert_eq {
    //     ($($arg:tt)*) => (if cfg!(debug_assertions) { assert_eq!($($arg)*); })
    // }

    // macro_rules! assert_eq {
    //     ($left:expr , $right:expr) => ({
    //         match (&($left), &($right)) {
    //             (left_val, right_val) => {
    //                 if !(*left_val == *right_val) {
    //                     panic!("assertion failed: `(left == right)` \
    //                            (left: `{:?}`, right: `{:?}`)", *left_val, *right_val)
    //                 }
    //             }
    //         }
    //     })
    // }

    #[test]
    #[should_panic]
    fn debug_assert_eq_test() {
        let left: usize = 68;
        let right: usize = 500;

        debug_assert_eq!(left, right); // panicked at 'assertion failed: `(left == right)` (left: `68`, right: `500`)'
    }
}
