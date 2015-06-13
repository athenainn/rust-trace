#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::str::pattern::Searcher;
    use core::str::pattern::SearchStep::{Match, Reject, Done};
    use core::str::pattern::CharSliceSearcher;
    use core::str::pattern::Pattern;

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
    // pub struct CharSliceSearcher<'a, 'b>(<CharEqPattern<&'b [char]> as Pattern<'a>>::Searcher);

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

    // impl<'a, 'b> Pattern<'a> for &'b [char] {
    //     pattern_methods!(CharSliceSearcher<'a, 'b>, CharEqPattern, CharSliceSearcher);
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

    // unsafe impl<'a, 'b> Searcher<'a> for CharSliceSearcher<'a, 'b> {
    //     searcher_methods!(forward);
    // }

    #[test]
    fn next_test1() {
	let cset: &[char] = &['能', '而'];
	let haystack: &str = "我能吞下玻璃而不傷身體。";
	let mut searcher: CharSliceSearcher = cset.into_searcher(haystack);

	assert_eq!(searcher.next(), Reject(0, 3));
	assert_eq!(searcher.next(), Match(3, 6));
	assert_eq!(searcher.next(), Reject(6, 9));
	assert_eq!(searcher.next(), Reject(9, 12));
	assert_eq!(searcher.next(), Reject(12, 15));
	assert_eq!(searcher.next(), Reject(15, 18));
	assert_eq!(searcher.next(), Match(18, 21));
	assert_eq!(searcher.next(), Reject(21, 24));
	assert_eq!(searcher.next(), Reject(24, 27));
	assert_eq!(searcher.next(), Reject(27, 30));
	assert_eq!(searcher.next(), Reject(30, 33));
	assert_eq!(searcher.next(), Reject(33, 36));
	assert_eq!(searcher.next(), Done);
    }
}
