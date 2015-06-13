#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::str::pattern::ReverseSearcher;
    use core::str::pattern::CharSearcher;
    use core::str::pattern::Pattern;

    // #[derive(Clone)]
    // pub struct CharSearcher<'a>(<CharEqPattern<char> as Pattern<'a>>::Searcher);

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

    // impl<'a> Pattern<'a> for char {
    //     pattern_methods!(CharSearcher<'a>, CharEqPattern, CharSearcher);
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

    // unsafe impl<'a, 'b> ReverseSearcher<'a> for CharSliceSearcher<'a, 'b> {
    //     searcher_methods!(reverse);
    // }

    #[test]
    fn next_reject_back_test1() {
	let c: char = '而';
	let haystack: &str = "我能吞下玻璃而不傷身體。";
	let mut searcher: CharSearcher = c.into_searcher(haystack);

	assert_eq!(searcher.next_reject_back(), Some::<(usize, usize)>((33, 36)));
	assert_eq!(searcher.next_reject_back(), Some::<(usize, usize)>((30, 33)));
	assert_eq!(searcher.next_reject_back(), Some::<(usize, usize)>((27, 30)));
	assert_eq!(searcher.next_reject_back(), Some::<(usize, usize)>((24, 27)));
	assert_eq!(searcher.next_reject_back(), Some::<(usize, usize)>((21, 24)));
	assert_eq!(searcher.next_reject_back(), Some::<(usize, usize)>((15, 18)));
	assert_eq!(searcher.next_reject_back(), Some::<(usize, usize)>((12, 15)));
	assert_eq!(searcher.next_reject_back(), Some::<(usize, usize)>((9, 12)));
	assert_eq!(searcher.next_reject_back(), Some::<(usize, usize)>((6, 9)));
	assert_eq!(searcher.next_reject_back(), Some::<(usize, usize)>((3, 6)));
	assert_eq!(searcher.next_reject_back(), Some::<(usize, usize)>((0, 3)));
	assert_eq!(searcher.next_reject_back(), None::<(usize, usize)>);
    }
}
