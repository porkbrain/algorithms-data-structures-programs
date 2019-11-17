//! # Binary search algorithm
//!
//! > The search can be speeded up considerably if the elements are already
//!     ordered. In this case, the principle of repeated halving of the interval
//!     in which the desired element must be searched is most common. It is
//!     called _bisection_ or _binary search_. In each repetition, the inspected
//!     interval between indices *upper_bound* and *lower_bound* is bisected.
//!     The number of required comparisons is therefore at most log2(N).
//!     \
//!     \
//!     Niklaus Wirth 1976, 14
//!
//! The two bounds are inclusive points which the algorithm uses to remember
//! which parts of the array definitely don't contain the needle (an element we
//! search for).
//!
//! Consider following array. It starts with element `4` with index `0` up to
//! element `99` with index `7`. We start with lower bound (LB) on element `4`
//! and upper bound (UB) on element `99`, which means we consider ALL elements
//! of the array to potentially be the element we look for.
//!
//! ```text
//!     LB                                                          UB
//! ____\/__________________________________________________________\/____
//! |   4   ,   7   ,   9   ,   49   ,   50   ,   80   ,   85   ,   99   |
//! ----------------------------------------------------------------------
//! ```
//!
//! Let's say we are looking for element `9`. We find out which is the element
//! in the middle by adding together the bounds and dividing them by 2.
//! `MEDIAN_INDEX = (7 + 0) / 2 = 3` // See `integer_division_floors` test.
//! `MEDIAN_ELEMENT = array[MEDIAN_INDEX] = array[3] = 49`
//!
//! Is an element `MEDIAN_ELEMENT` equal to the element we search for?
//!     No. `9 != 49`
//!
//! Is an element `MEDIAN_ELEMENT` smaller that the element we search for?
//!     No. It is LARGER than `9`. Hence we move upper bound to the position
//!     to the `MEDIAN_INDEX - 1`. Why `- 1`? Because the bounds are
//!     **inclusive**. And we already checked that the element ON the
//!     `MEDIAN_INDEX` is not what we were looking for.
//!
//! Is lower bound larger than upper bound?
//!     No. If it would be, it'd mean we checked all elements and we didn't find
//!     the one we were looking for.
//!
//! In the next repetition, our bounds are refined and closer together.
//!
//! ```text
//!     LB              UB
//! ____\/______________\/________________________________________________
//! |   4   ,   7   ,   9   ,   49   ,   50   ,   80   ,   85   ,   99   |
//! ----------------------------------------------------------------------
//! ```
//!
//! We were able to disregard approximately half of the elements. This process
//! of checking the invariants and refining the bounds continues until one of
//! the loop invariants is met.
//!
//! Takeaway points from this algorithm are that it
//! - works with ordered comparable elements;
//! - uses two bounds which approach one another;
//! - first loop invariant is whether a `MEDIAN_ELEMENT` is what we want
//! - second loop invariant is whether lower bound is larger than higher bound
//! - runs in **log2(N)** which means it needs at most 20 repetitions to find an
//!      element in an array of one million elements.

/// Searches for given element in provided slice. The algorithm assumes that the
/// array is sorted. It returns element index if it's present or `None` if not.
///
/// NOTE:
/// We can make this algorithm generic by requiring only two properties of the
/// type `T`:
/// 1. We want to be able to tell whether two elements `T` are equal.
/// 2. We want to be able to tell which of two elements `T` is larger.
/// Rust has two traits which get the job done. Type `T` has to implement both
/// [`PartialEq`] and [`PartialOrd`].
///
/// [`PartialEq`]: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
/// [`PartialOrd`]: https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
pub fn binary_search<T>(element: &T, array: &[T]) -> Option<usize>
where
    T: PartialEq + PartialOrd,
{
    // We initialize the bounds to point to the first and last element.
    let mut lower_bound = 0;
    let mut upper_bound = array.len() - 1;

    loop {
        // Integer division always floors. See `integer_division_floors` test.
        let median = (lower_bound + upper_bound) / 2;

        // First loop invariant. If we found the element, return its index.
        if array[median] == *element {
            return Some(median);
        }

        // If the element we look for is larger, move the lower bound.
        // Otherwise move the upper bound. Bounds are inclusive.
        if array[median] < *element {
            lower_bound = median + 1;
        } else {
            upper_bound = median - 1;
        }

        // Second loop invariant. If lower bound is higher than upper bound,
        // the whole search space has been visited and the element is not
        // contained within it.
        if lower_bound > upper_bound {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_division_floors() {
        let median = 11usize / 2usize;

        assert_eq!(median, 5usize);
    }

    #[test]
    fn it_returns_index_if_element_is_present() {
        let needle = 30;
        let haystack: [u64; 10] = [1, 4, 6, 7, 12, 20, 30, 34, 40, 50];

        let search_result = binary_search(&needle, &haystack[..]);

        assert_eq!(search_result, Some(6));
    }

    #[test]
    fn it_returns_none_if_element_is_not_present() {
        let needle = 25;
        let haystack: [u64; 10] = [1, 4, 6, 7, 12, 20, 30, 34, 40, 50];

        let search_result = binary_search(&needle, &haystack[..]);

        assert_eq!(search_result, None);
    }

    #[test]
    fn it_is_generic() {
        let needle = "bcd";
        let haystack: [&str; 4] = ["abc", "abd", "bcd", "efg"];

        let search_result = binary_search(&needle, &haystack[..]);

        assert_eq!(search_result, Some(2));
    }
}
