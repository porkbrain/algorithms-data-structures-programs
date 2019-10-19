//! # Insertion Sort by Diminishing Increment: Shellsort
//!
//! > A refinement of the straight insertion sort was proposed by [D. L. Shell]
//!     in 1959.The method is explain and demonstrated on our standard example
//!     of eight items. First, all items which are four positions apart are
//!     grouped and sorted separately.
//!     \
//!     ...
//!     \
//!     After this first pass, the items are regrouped into groups with items
//!     two positions apart and then sorted anew. Finally, in a third pass, all
//!     items are sorted din an ordinary sort.
//!     \
//!     ...
//!     \
//!     It is obvious that any sequence of increments will be acceptable, as
//!     long as the last one is unity, because in the worst case the last pass
//!     will do all the work. It is, however, much less obvious that the method
//!     of diminishing increments yields even better results with increments
//!     other than powers of 2.
//!     \
//!     ...
//!     \
//!     [Donald E. Knuth] recommends a choice of increments, the sequence
//!     \
//!     _1, 4, 13, 40, 121_
//!     \
//!     where `h(k - 1) = 2h(k) + 1`, `h(t) = 1` and `t = floor(log(2) n) - 1`.
//!     Mathematical analysis yields an effort proportional to n^1.2 required
//!     for sorting n items.
//!     \
//!     \
//!     Niklaus Wirth 1976, 68-70
//!
//! First iteration starts with gap of four. With gap this large the chains are
//! groups of two items.
//! ```text
//!                     55 > 18 implies swapping
//!             +--------------------------------+
//!             |                                |      42 <= 67 implies not
//!             |               +--------------------------------+  swapping
//!             |               |                |               |
//!             \/              \/              \/              \/
//!     44      55      12      42      94      18      06      67
//!     /\              /\              /\              /\
//!     | 44 <= 9       |                |               |
//!     +--------------------------------+               |
//!       implies not   |                                |
//!       swapping      +--------------------------------+
//!                                 12 > 06 implies swapping
//! ```
//! In next iteration, we use gap of two. In this case, the chain contains four
//! items. Each iteration runs until all items in the chain have been sorted.
//! Even thought during the second comparison, 44 and 94 don't swap places, in
//! third comparison 12 and 94 do swap. And the algorithm continues, swapping
//! 44 for 12.
//! ```text
//!     1) swaps 44 with 06
//!                                      2) swaps 94 with 12
//!                      3) swaps 44 with 12
//!       /------------\  /------------\  /------------\
//!      /              \/              \/              \
//!     44      18      06      42      94      55      12      67
//!              \              /\              /\              /
//!               \------------/  \------------/  \------------/
//!                            all sorted already
//! ```
//! Final gap is a unity. This is equivalent to sort by straight insertion.
//! However, generally when the sort gets to this point, most of array elements
//! are very close to being already sorted.
//! ```text
//!         X      SWAP     X       X       X       X      SWAP
//!       /----\  /----\  /----\  /----\  /----\  /----\  /----\
//!      /      \/      \/      \/      \/      \/      \/      \
//!     06      18      12      42      44      55      94      67
//! ```
//! After the unity swaps last elements which weren't sorted prior, it leaves
//! the array sorted.
//! ```text
//!     06      12      18      42      44      55      67      94
//! ```
//!
//! [Donald E. Knuth]: https://www-cs-faculty.stanford.edu/~knuth/
//! [D. L. Shell]: https://en.wikipedia.org/wiki/Donald_Shell

/// Takes a mutable slice of comparable elements and sorts them in ASC order.
pub fn shell_sort<T>(array: &mut [T])
    where
        T: PartialEq + PartialOrd,
{
    // Guard for small arrays which are already "sorted".
    if array.len() < 2 {
        return;
    }

    // We use formula `t = floor( log(2) n ) - 1`. However, we want at least one
    // sort iteration, so a `max` function is used to prevent `t == 0`.
    let sort_gaps_len = ((array.len() as f64).log2().floor() as usize - 1).max(1);
    // Based on the length of gaps, we calculate each gap with formula
    // `gap = 2^i - 1`.
    let sort_gaps: Vec<_> = (1..=sort_gaps_len)
        .map(|x| 2f64.powi(x as i32) as usize - 1)
        .collect();

    // We want to start with the largest gap and work our way down to unity gap.
    for gap_index in (0..sort_gaps_len).rev() {
        let gap = sort_gaps[gap_index];

        // In standard straight insertion sort, we skipped first element. In
        // this refined version we have to skip first `gap` elements. These are
        // going to be accounted for thanks to the fact that we use
        // `array[tracker - gap]`. (Note `- gap`.)
        for index in gap..array.len() {
            let mut tracker = index;

            // We decrement the tracker until we hit sentinel mark or element
            // on the right is larger/equal to it's group mate on the left.
            while tracker >= gap && array[tracker] < array[tracker - gap] {
                array.swap(tracker, tracker - gap);

                tracker -= gap;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::*;

    #[test]
    fn it_handles_empty_array() {
        let mut array: Vec<u8> = Vec::new();

        shell_sort(&mut array);
    }

    #[test]
    fn it_handles_array_of_one_element() {
        let mut array = vec![4];

        shell_sort(&mut array);

        assert_eq!(array[0], 4);
    }

    #[test]
    fn it_sorts_ordered_array() {
        let mut array = vec![1, 2, 3, 4];

        shell_sort(&mut array);

        assert_eq!(array[0], 1);
        assert_eq!(array[1], 2);
        assert_eq!(array[2], 3);
        assert_eq!(array[3], 4);
    }

    #[test]
    fn it_sorts_reversed_array() {
        let mut array = vec![4, 3, 2, 1];

        shell_sort(&mut array);

        assert_eq!(array[0], 1);
        assert_eq!(array[1], 2);
        assert_eq!(array[2], 3);
        assert_eq!(array[3], 4);
    }

    #[test]
    fn it_is_generic() {
        let mut array = vec!["abc", "cbd", "abd"];

        shell_sort(&mut array);

        println!("{:?}", array);

        assert_eq!(array[0], "abc");
        assert_eq!(array[1], "abd");
        assert_eq!(array[2], "cbd");
    }

    #[test]
    fn it_is_stable() {
        let a = 1;
        let b = 1;
        let c = 2;
        let d = 2;
        let mut array = vec![&d, &c, &b, &a, &3];

        shell_sort(&mut array);

        assert!(std::ptr::eq(array[0], &b));
        assert!(std::ptr::eq(array[1], &a));
        assert!(std::ptr::eq(array[2], &d));
        assert!(std::ptr::eq(array[3], &c));
    }

    #[test]
    fn it_sorts_example() {
        let mut array = vec![44, 55, 12, 42, 94, 18, 6, 67];

        shell_sort(&mut array);

        assert!(is_sorted(&array));
    }

    #[test]
    fn fuzzy_test() {
        extern crate rand;
        use rand::prelude::SliceRandom;

        let mut rng = rand::thread_rng();
        let mut numbers: Vec<u32> = (1..FUZZY_TEST_ITERATIONS).collect();

        for _ in 0..100 {
            numbers.shuffle(&mut rng);

            shell_sort(&mut numbers);

            assert!(is_sorted(&numbers));
        }
    }
}
