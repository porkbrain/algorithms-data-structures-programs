//! # Sorting by straight exchange: Shakersort
//!
//! An improved version of [`bubble_sort`]. Shakersort implements:
//! - alternating between directions (left, right);
//! - `last_exchange` pointer for eliminating redundant comparisons.
//!
//! > All improvements mentioned above do in no way affect the number of
//!     exchanges; they only reduce the number of redundant double checks.
//!     Unfortunately, an exchange of two items is generally a much more costly
//!     operation than a comparison of keys; our clever improvements therefore
//!     have a much less profound effect than one would intuitively expect.
//!     \
//!     \
//!     Niklaus Wirth 1976, 68
//!
//! [`bubble_sort`]: ../a_003_bubble_sort/fn.bubble_sort.html

/// Takes a mutable slice of comparable elements and sorts them in ASC order.
pub fn shaker_sort<T>(array: &mut [T])
where
    T: PartialEq + PartialOrd,
{
    // Guard for small arrays which are already "sorted".
    if array.len() < 2 {
        return;
    }

    // Alternates directions when sorting. While Bubblesort had only a single
    // `index` variable, we have to have a separate index like variable for each
    // direction. Left starts at the beginning (2nd element) while right starts
    // at the last element.
    let mut right = array.len() - 1;
    let mut left = 1;

    // One improvement over classical Bubblesort is to eliminate extra
    // comparisons on parts of array which have already been sorted. Shakersort
    // does this by remembering the position of lastly swapped elements. This
    // pointer then accelerates decrementing of the right bound and incrementing
    // of the left bound.
    let mut last_exchange = array.len() - 1;

    // The algorithm runs until both bounds sort their part of the array.
    while left <= right {
        // Starts from the rightmost element which hasn't been sorted yet and
        // continues down to the left bound. The right index starts at n - 1 and
        // decrements. We reverse the sequence so that we iterate from the right
        // index downwards. We can be positive that it won't overflow. Since the
        // lowest we can go is left bound, which starts at 1 and increments, we
        // can be positive that we don't try to decrement usize 0.
        for bubble in (left..=right).rev() {
            if array[bubble - 1] > array[bubble] {
                array.swap(bubble, bubble - 1);
                last_exchange = bubble;
            }
        }

        // Left index is accelerated to the point where right bound did its last
        // exchange. We know that anything to the left of that point of last
        // exchange is sorted (otherwise right bound would swap elements and
        // pushed the pointer closer to left).
        left = last_exchange + 1;

        // Starts from the leftmost element which hasn't been sorted yet. The
        // index can equal at most n - 1. We can therefore be positive that the
        // index won't overflow.
        for bubble in left..=right {
            if array[bubble - 1] > array[bubble] {
                array.swap(bubble, bubble - 1);
                last_exchange = bubble;
            }
        }

        // In the same way we pushed the left bound, we can now accelerate the
        // right bound.
        right = last_exchange - 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::*;

    #[test]
    fn it_handles_empty_array() {
        let mut array: Vec<u8> = Vec::new();

        shaker_sort(&mut array);
    }

    #[test]
    fn it_handles_array_of_one_element() {
        let mut array = vec![4];

        shaker_sort(&mut array);

        assert_eq!(array[0], 4);
    }

    #[test]
    fn it_sorts_ordered_array() {
        let mut array = vec![1, 2, 3, 4];

        shaker_sort(&mut array);

        assert_eq!(array[0], 1);
        assert_eq!(array[1], 2);
        assert_eq!(array[2], 3);
        assert_eq!(array[3], 4);
    }

    #[test]
    fn it_sorts_reversed_array() {
        let mut array = vec![4, 3, 2, 1];

        shaker_sort(&mut array);

        assert_eq!(array[0], 1);
        assert_eq!(array[1], 2);
        assert_eq!(array[2], 3);
        assert_eq!(array[3], 4);
    }

    #[test]
    fn it_is_generic() {
        let mut array = vec!["abc", "cbd", "abd"];

        shaker_sort(&mut array);

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

        shaker_sort(&mut array);

        assert!(std::ptr::eq(array[0], &b));
        assert!(std::ptr::eq(array[1], &a));
        assert!(std::ptr::eq(array[2], &d));
        assert!(std::ptr::eq(array[3], &c));
    }

    #[test]
    fn it_sorts_example() {
        let mut array = vec![44, 55, 12, 42, 94, 18, 6, 67];

        shaker_sort(&mut array);

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

            shaker_sort(&mut numbers);

            assert!(is_sorted(&numbers));
        }
    }
}
