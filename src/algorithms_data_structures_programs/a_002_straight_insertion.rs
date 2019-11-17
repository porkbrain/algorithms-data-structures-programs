//! # Sorting by Straight insertion
//!
//! > The items are divided into a destination sequence `a[0]..a[i]` and source
//!     sequence `a[i]..a[n]`. In each step, starting with `i - 1`, the `i`th
//!     element of the source sequence is picked and transferred into the
//!     destination sequence by inserting it at the appropriate place.
//!     \
//!     \
//!     Niklaus Wirth 1976, 60
//!
//! In this implementation, the index `i` is represented by variable `tracker`.
//! With each iteration, we increment the tracker by one. The we use it to track
//! the currently observed element. Tracker is being decremented until either
//!
//! a) **smaller number** is visited, or
//! b) **end of the array** is visited.
//!
//! By "visiting" an element we mean decrementing the tracker to index position
//! of that element in given slice we are sorting. By visiting end of array we
//! mean visiting element with index `0`.
//!
//! The unsorted tail of the array is called source sequence because we pick
//! elements _FROM_ it. The sorted head is called destination sequence because
//! each picked will be position on appropriate place _IN_ the sequence.
//!
//! Consider following slice from the algorithm after it has sorted first 4
//! elements and is about to sort 5th:
//!
//! ```text
//! 1) index = 4
//!                                     tracker = 4
//! .______________________________________||_________________________________.
//! |        destination sequence      |   \/          source sequence        |
//! |      3,      4,      6,      8,      5,     13,      1,      2,     12 |
//!                                /\ SWAP /\
//!                                 \______/
//!
//! tracker = index = 4
//! (array[4] = 5) < (array[4 - 1] = 8) => swap the two elements
//! ```
//!
//! ```text
//! 2) index = 4
//!                              tracker = 3
//! .___________________________ _||__________________________________________.
//! |    destination sequence     \/           |        source sequence       |
//! |      3,      4,      6,      5,      8,      13,      1,      2,     12 |
//!                        /\ SWAP /\
//!                         \______/
//!
//! tracker = tracker - 1 = 3
//! (array[3] = 5) < (array[3 - 1] = 6) => swap the two elements
//! ```
//! ```text
//! 3) index = 4
//!                      tracker = 2
//! .______________________||_________________________________________________.
//! |  destination         \/       sequence   |       source sequence        |
//! |      3,      4,      5,      6,      8,      13,      1,      2,     12 |
//!                /\ KEEP /\
//!                 \______/
//!
//! tracker = tracker - 1 = 2
//! (array[2] = 5) > (array[2 - 1] = 4) => index = index + 1, tracker = index
//! ```
//! ```text
//! 3) index = 5
//!                                             tracker = 5
//! .______________________________________________||_________________________.
//! |          destination sequence            |   \/   source sequence       |
//! |      3,      4,      5,      6,      8,      13,      1,      2,     12 |
//!                                        /\ SWAP /\
//!                                         \______/
//! ```
//!
//! And this process repeats as long as `index < length`.
//!
//! This algorithm uses two operations: **comparisons** and **moves**. Moves are
//! more expensive than comparisons.
//!
//! In the best case of already sorted array this algorithm does
//! `C[min] = 2(n - 1)` and `M[min] = 0`. That's because there are `2`
//! comparisons for each of `n` items except for the first one. Two comparisons
//! come from the implementation: we compare the tracker to be greater than zero
//! and then we compare the two neighbors. If the array is sorted, we don't need
//! to move any element.
//!
//! In worst case scenario we repeat the while cycle `(n(n + 1) / 2) - 1` times.
//! `C[max] = n^2 + n - 2` and `M[max] = (n(n + 1) / 2) - 1`.
//!
//! This puts the straight insertion into the `O(n^2)` family.
//!
//! > The least numbers occur if the items are originally in order; the worst
//!     case occurs if the items are originally in reverse order. In this sense,
//!     sorting by insertion exhibits a truly natural behavior. It is plain that
//!     the given algorithm also describes a **stable storting process**:
//!     it leaves the order of items with equal keys unchanged.
//!     \
//!     \
//!     Niklaus Wirth 1976, 61

/// Takes a mutable slice of comparable elements and sorts them in ASC order.
pub fn straight_insertion<T>(array: &mut [T])
where
    T: PartialEq + PartialOrd,
{
    // Guard for small arrays which are already sorted.
    if array.len() < 2 {
        return;
    }

    // Starts on seconds element and continues process until the last one.
    for index in 1..array.len() {
        // Defaults tracker to be index. Tracker marks currently sorted element.
        let mut tracker = index;

        // Repeat moves until
        // a) smallest element so far has been visited (on index 0);
        // b) an element smaller than tracker element has been visited.
        while tracker > 0 && array[tracker] < array[tracker - 1] {
            // Swaps two neighbours.
            array.swap(tracker, tracker - 1);

            // Decrement the tracker for next iteration.
            tracker -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::is_sorted;

    #[test]
    fn it_handles_empty_array() {
        let mut array: Vec<u8> = Vec::new();

        straight_insertion(&mut array);
    }

    #[test]
    fn it_handles_array_of_one_element() {
        let mut array = vec![4];

        straight_insertion(&mut array);

        assert_eq!(array[0], 4);
    }

    #[test]
    fn it_sorts_ordered_array() {
        let mut array = vec![1, 2, 3, 4];

        straight_insertion(&mut array);

        assert_eq!(array[0], 1);
        assert_eq!(array[1], 2);
        assert_eq!(array[2], 3);
        assert_eq!(array[3], 4);
    }

    #[test]
    fn it_sorts_reversed_array() {
        let mut array = vec![4, 3, 2, 1];

        straight_insertion(&mut array);

        assert_eq!(array[0], 1);
        assert_eq!(array[1], 2);
        assert_eq!(array[2], 3);
        assert_eq!(array[3], 4);
    }

    #[test]
    fn it_is_generic() {
        let mut array = vec!["abc", "cbd", "abd"];

        straight_insertion(&mut array);

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

        straight_insertion(&mut array);

        assert!(std::ptr::eq(array[0], &b));
        assert!(std::ptr::eq(array[1], &a));
        assert!(std::ptr::eq(array[2], &d));
        assert!(std::ptr::eq(array[3], &c));
    }

    #[test]
    fn it_sorts_example() {
        let mut array = vec![44, 55, 12, 42, 94, 18, 6, 67];

        straight_insertion(&mut array);

        assert!(is_sorted(&array));
    }

    #[test]
    fn fuzzy_test() {
        extern crate rand;
        use rand::prelude::SliceRandom;

        let mut rng = rand::thread_rng();
        let mut numbers: Vec<u16> = (1..100).collect();

        for _ in 0..100 {
            numbers.shuffle(&mut rng);

            straight_insertion(&mut numbers);

            assert!(is_sorted(&numbers));
        }
    }
}
