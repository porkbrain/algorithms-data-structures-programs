//! # Sorting by straight exchange: Bubblesort
//!
//! > We make repeated passes over the array, each time sifting the least item
//!     of the remaining set to the left end of the array.
//!     ...
//!     This algorithm easily lends itself to some improvements. An obvious
//!     technique for improving this algorithm is to remember whether or not any
//!     exchange had take place during a pass.
//!     ...
//!     A single misplaced bubble in the "heavy" end of an otherwise sorted
//!     array will sift into order in a single pass, but a misplaced item in the
//!     "light" end will sink toward its correct position only one step in each
//!     pass. For example, the array
//!     12 18 42 44 55 67 94 06
//!     will be sorted by the improved Bubblesort in a single pass, but the array
//!     94 06 12 18 42 44 55 67
//!     will require seven passes for sorting. This unnatural asymmetry suggests
//!     an improvement: alternating the direction of consecutive passes.
//!     \
//!     \
//!     Niklaus Wirth 1976, 65-66
//!
//! Enough said about Bubblesort. Niklaus Wirth suggests few improvements to be
//! implemented. Implementing these improvements promotes Bubblesort to
//! Shakersort.

pub fn bubble_sort<T>(array: &mut [T])
    where
        T: PartialEq + PartialOrd,
{
    // Guard for small arrays which are already "sorted".
    if array.len() < 2 {
        return;
    }

    // Starts from second element and visits each.
    for index in 1..array.len() {
        // Iterates each element from the last one up to currently visited.
        // To achieve this, we use range that goes from visited (inclusive) to
        // last element (exclusive) and then reverse it.
        for bubble in (index..array.len()).rev() {
            // If the neighbour on the right is smaller than the neighbour on
            // the left, we swap them. The comparison operator here suggests
            // that this sorting is stable.
            if array[bubble - 1] > array[bubble] {
                array.swap(bubble, bubble - 1);
            }
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

        bubble_sort(&mut array);
    }

    #[test]
    fn it_handles_array_of_one_element() {
        let mut array = vec![4];

        bubble_sort(&mut array);

        assert_eq!(array[0], 4);
    }

    #[test]
    fn it_sorts_ordered_array() {
        let mut array = vec![1, 2, 3, 4];

        bubble_sort(&mut array);

        assert_eq!(array[0], 1);
        assert_eq!(array[1], 2);
        assert_eq!(array[2], 3);
        assert_eq!(array[3], 4);
    }

    #[test]
    fn it_sorts_reversed_array() {
        let mut array = vec![4, 3, 2, 1];

        bubble_sort(&mut array);

        assert_eq!(array[0], 1);
        assert_eq!(array[1], 2);
        assert_eq!(array[2], 3);
        assert_eq!(array[3], 4);
    }

    #[test]
    fn it_is_generic() {
        let mut array = vec!["abc", "cbd", "abd"];

        bubble_sort(&mut array);

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

        bubble_sort(&mut array);

        assert!(std::ptr::eq(array[0], &b));
        assert!(std::ptr::eq(array[1], &a));
        assert!(std::ptr::eq(array[2], &d));
        assert!(std::ptr::eq(array[3], &c));
    }

    #[test]
    fn it_sorts_example() {
        let mut array = vec![44, 55, 12, 42, 94, 18, 6, 67];

        bubble_sort(&mut array);

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

            bubble_sort(&mut numbers);

            assert!(is_sorted(&numbers));
        }
    }
}
