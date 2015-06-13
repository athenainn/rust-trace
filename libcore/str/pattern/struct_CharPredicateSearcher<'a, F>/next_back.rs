#![feature(core, unboxed_closures)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::str::pattern::ReverseSearcher;
    use core::str::pattern::SearchStep::{Match, Reject, Done};
    use core::str::pattern::CharPredicateSearcher;
    use core::str::pattern::Pattern;

    use core::ops::FnMut;
    use core::ops::FnOnce;

    // #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    // pub enum SearchStep {
    //     /// Expresses that a match of the pattern has been found at
    //     /// `haystack[a..b]`.
    //     Match(usize, usize),
    //     /// Expresses that `haystack[a..b]` has been rejected as a possible match
    //     /// of the pattern.
    //     ///
    //     /// Note that there might be more than one `Reject` between two `Match`es,
    //     /// there is no requirement for them to be combined into one.
    //     Reject(usize, usize),
    //     /// Expresses that every byte of the haystack has been visted, ending
    //     /// the iteration.
    //     Done
    // }

    // #[derive(Clone)]
    // pub struct CharPredicateSearcher<'a, F>(<CharEqPattern<F> as Pattern<'a>>::Searcher)
    //     where F: FnMut(char) -> bool;

    // pub trait Pattern<'a>: Sized {
    //     /// Associated searcher for this pattern
    //     type Searcher: Searcher<'a>;
    //
    //     /// Constructs the associated searcher from
    //     /// `self` and the `haystack` to search in.
    //     fn into_searcher(self, haystack: &'a str) -> Self::Searcher;
    //
    //     /// Checks whether the pattern matches anywhere in the haystack
    //     #[inline]
    //     fn is_contained_in(self, haystack: &'a str) -> bool {
    //         self.into_searcher(haystack).next_match().is_some()
    //     }
    //
    //     /// Checks whether the pattern matches at the front of the haystack
    //     #[inline]
    //     fn is_prefix_of(self, haystack: &'a str) -> bool {
    //         match self.into_searcher(haystack).next() {
    //             SearchStep::Match(0, _) => true,
    //             _ => false,
    //         }
    //     }
    //
    //     /// Checks whether the pattern matches at the back of the haystack
    //     //     #[inline]
    //     fn is_suffix_of(self, haystack: &'a str) -> bool
    //         where Self::Searcher: ReverseSearcher<'a>
    //     {
    //         match self.into_searcher(haystack).next_back() {
    //             SearchStep::Match(_, j) if haystack.len() == j => true,
    //             _ => false,
    //         }
    //     }
    // }

    // macro_rules! pattern_methods {
    //     ($t:ty, $pmap:expr, $smap:expr) => {
    //         type Searcher = $t;
    //
    //         #[inline]
    //         fn into_searcher(self, haystack: &'a str) -> $t {
    //             ($smap)(($pmap)(self).into_searcher(haystack))
    //         }
    //
    //         #[inline]
    //         fn is_contained_in(self, haystack: &'a str) -> bool {
    //             ($pmap)(self).is_contained_in(haystack)
    //         }
    //
    //         #[inline]
    //         fn is_prefix_of(self, haystack: &'a str) -> bool {
    //             ($pmap)(self).is_prefix_of(haystack)
    //         }
    //
    //         #[inline]
    //         fn is_suffix_of(self, haystack: &'a str) -> bool
    //             where $t: ReverseSearcher<'a>
    //         {
    //             ($pmap)(self).is_suffix_of(haystack)
    //         }
    //     }
    // }

    // impl<'a, F> Pattern<'a> for F where F: FnMut(char) -> bool {
    //     pattern_methods!(CharPredicateSearcher<'a, F>, CharEqPattern, CharPredicateSearcher);
    // }

    // macro_rules! searcher_methods {
    //     (forward) => {
    //         #[inline]
    //         fn haystack(&self) -> &'a str {
    //             self.0.haystack()
    //         }
    //         #[inline]
    //         fn next(&mut self) -> SearchStep {
    //             self.0.next()
    //         }
    //         #[inline]
    //         fn next_match(&mut self) -> Option<(usize, usize)> {
    //             self.0.next_match()
    //         }
    //         #[inline]
    //         fn next_reject(&mut self) -> Option<(usize, usize)> {
    //             self.0.next_reject()
    //         }
    //     };
    //     (reverse) => {
    //         #[inline]
    //         fn next_back(&mut self) -> SearchStep {
    //             self.0.next_back()
    //         }
    //         #[inline]
    //         fn next_match_back(&mut self) -> Option<(usize, usize)> {
    //             self.0.next_match_back()
    //         }
    //         #[inline]
    //         fn next_reject_back(&mut self) -> Option<(usize, usize)> {
    //             self.0.next_reject_back()
    //         }
    //     }
    // }

    // unsafe impl<'a, F> ReverseSearcher<'a> for CharPredicateSearcher<'a, F>
    //     where F: FnMut(char) -> bool
    // {
    //     searcher_methods!(reverse);
    // }

    struct F { c: char }

    type Args = (char,);

    impl FnOnce<Args> for F {
	type Output = bool;

	extern "rust-call" fn call_once(mut self, (c,): Args) -> Self::Output {
	    self.call_mut((c,))
	}
    }

    impl FnMut<Args> for F {
	extern "rust-call" fn call_mut(&mut self, (c,): Args) -> Self::Output {
	    self.c == c
	}
    }

    #[test]
    fn next_back_test1() {
	let f: F = F { c: '而' };
	let haystack: &str = "我能吞下玻璃而不傷身體。";
	let mut searcher: CharPredicateSearcher<F> = f.into_searcher(haystack);

	assert_eq!(searcher.next_back(), Reject(33, 36));
	assert_eq!(searcher.next_back(), Reject(30, 33));
	assert_eq!(searcher.next_back(), Reject(27, 30));
	assert_eq!(searcher.next_back(), Reject(24, 27));
	assert_eq!(searcher.next_back(), Reject(21, 24));
	assert_eq!(searcher.next_back(), Match(18, 21));
	assert_eq!(searcher.next_back(), Reject(15, 18));
	assert_eq!(searcher.next_back(), Reject(12, 15));
	assert_eq!(searcher.next_back(), Reject(9, 12));
	assert_eq!(searcher.next_back(), Reject(6, 9));
	assert_eq!(searcher.next_back(), Reject(3, 6));
	assert_eq!(searcher.next_back(), Reject(0, 3));
	assert_eq!(searcher.next_back(), Done);
    }
}
