#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::str::pattern::Searcher;
    use core::str::pattern::SearchStep::{self, Match, Reject, Done};
    use core::str::pattern::Pattern;

    use core::str::CharIndices;

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

    trait CharEq {
	fn matches(&mut self, char) -> bool;
	fn only_ascii(&self) -> bool;
    }

    impl CharEq for char {
	#[inline]
	fn matches(&mut self, c: char) -> bool { *self == c }

	#[inline]
	fn only_ascii(&self) -> bool { (*self as u32) < 0xf0 }
    }

    #[derive(Clone)]
    struct CharEqSearcher<'a, C: CharEq> {
	char_eq: C,
	haystack: &'a str,
	char_indices: CharIndices<'a>,
	ascii_only: bool
    }

    unsafe impl<'a, C: CharEq> Searcher<'a> for CharEqSearcher<'a, C> {
	#[inline]
	fn haystack(&self) -> &'a str {
	    self.haystack
	}

	#[inline]
	fn next(&mut self) -> SearchStep {
	    let s: &mut CharIndices<'a> = &mut self.char_indices;
	    let pre_len: usize = s.size_hint().1.unwrap();

	    if let Some::<(usize, char)>((i, c)) = s.next() {
		let len: usize = s.size_hint().1.unwrap();
		let char_len = pre_len - len;

		if self.char_eq.matches(c) {
		    return SearchStep::Match(i, i + char_len);
		} else {
		    return SearchStep::Reject(i, i + char_len);
		}
	    }

	    SearchStep::Done
	}

	// #[inline]
	// fn next_match(&mut self) -> Option<(usize, usize)> {
	//     loop {
	//         match self.next() {
	//             SearchStep::Match(a, b) => return Some((a, b)),
	//             SearchStep::Done => return None,
	//             _ => continue,
	//         }
	//     }
	// }

	// #[inline]
	// fn next_reject(&mut self) -> Option<(usize, usize)> {
	//     loop {
	//         match self.next() {
	//             SearchStep::Reject(a, b) => return Some((a, b)),
	//             SearchStep::Done => return None,
	//             _ => continue,
	//         }
	//     }
	// }
    }

    struct CharEqPattern<C: CharEq>(C);

    impl<'a, C: CharEq> Pattern<'a> for CharEqPattern<C> {
	type Searcher = CharEqSearcher<'a, C>;

	#[inline]
	fn into_searcher(self, haystack: &'a str) -> CharEqSearcher<'a, C> {
	    CharEqSearcher {
		ascii_only: self.0.only_ascii(),
		char_eq: self.0,
		haystack: haystack,
		char_indices: haystack.char_indices()
	    }
	}

	// fn is_contained_in(self, haystack: &'a str) -> bool {
	//     self.into_searcher(haystack).next_match().is_some()
	// }

        // fn is_prefix_of(self, haystack: &'a str) -> bool {
	//     match self.into_searcher(haystack).next() {
	//         SearchStep::Match(0, _) => true,
	//         _ => false,
	//     }
	// }

        // fn is_suffix_of(self, haystack: &'a str) -> bool
	//     where Self::Searcher: ReverseSearcher<'a>
	// {
	//     match self.into_searcher(haystack).next_back() {
	//         SearchStep::Match(_, j) if haystack.len() == j => true,
	//         _ => false,
	//     }
	// }
    }

    #[test]
    fn is_prefix_of_test1() {
	let c: char = '我';
	let haystack: &str = "我能吞下玻璃而不傷身體。";
	let pattern: CharEqPattern<char> = CharEqPattern::<char>(c);
	let result: bool = pattern.is_prefix_of(haystack);

	assert_eq!(result, true);
    }

    #[test]
    fn is_prefix_of_test2() {
	let c: char = '能';
	let haystack: &str = "我能吞下玻璃而不傷身體。";
	let pattern: CharEqPattern<char> = CharEqPattern::<char>(c);
	let result: bool = pattern.is_prefix_of(haystack);

	assert_eq!(result, false);
    }
}
