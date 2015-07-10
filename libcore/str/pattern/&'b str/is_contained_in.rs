#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::str::pattern::Pattern;

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

    // impl<'a, 'b> Pattern<'a> for &'b str {
    //     type Searcher = StrSearcher<'a, 'b>;
    //
    //     #[inline]
    //     fn into_searcher(self, haystack: &'a str) -> StrSearcher<'a, 'b> {
    //         StrSearcher {
    //             haystack: haystack,
    //             needle: self,
    //             start: 0,
    //             end: haystack.len(),
    //             state: State::NotDone,
    //         }
    //     }
    // }

    #[test]
    fn is_contained_in_test1() {
	let string: &str = "玻璃";
	let haystack: &str = "我能吞下玻璃而不傷身體。";
	let result: bool = string.is_contained_in(haystack);

	assert_eq!(result, true);
    }

    #[test]
    fn is_contained_in_test2() {
	let string: &str = "木頭";
	let haystack: &str = "我能吞下玻璃而不傷身體。";
	let result: bool = string.is_contained_in(haystack);

	assert_eq!(result, false);
    }

}
