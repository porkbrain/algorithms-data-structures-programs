//! # Insertion Sort by Diminishing Increment: Shellsort

/// Takes a mutable slice of comparable elements and sorts them in ASC order.
pub fn shell_sort<T>(array: &mut [T])
    where
        T: PartialEq + PartialOrd,
{
    // Guard for small arrays which are already "sorted".
    if array.len() < 2 {
        return;
    }

    let sort_gaps_len = ((array.len() as f64).log2().floor() as usize - 1).max(1);
    let sort_gaps: Vec<_> = (0..sort_gaps_len).map(|x| 3 * x + 1).collect();

    for gap_index in (0..sort_gaps_len).rev() {
        let gap = sort_gaps[gap_index];

        for index in gap..array.len() {
            let mut tracker = index;

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
