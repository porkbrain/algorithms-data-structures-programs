//! # Problem
//! Given a sorted array **A**, mutate it in such a way that first **N**
//! elements are unique. All unique elements from **A** must be included in the
//! first **N** elements array slice. The tail is garbage and can contain any
//! values.
//!
//! ## Example
//! Given `A = [1, 1, 2, 3, 4, 4, 4, 5]`, mutate **A** to a state
//! `[1, 2, 3, 4, 5, _, _, _]` (`_` means the value is irrelevant) and output
//! **`5`**.

/// Solves the problem in space O(1) and time O(N).
///
/// It starts with the second element, of the array, marking the first element
/// as unique by default. It keeps track of how many unique elements have been
/// moved to the head of the array. This tracking counter is what this method
/// will output in the end. It visits each element once.
///
/// Example:
///
/// Rules:
///
/// `SHOULD_SWAP = A[I] != A[L - 1]`
///
/// `NEW_LEN += SHOULD_SWAP ? 1 : 0`
///
/// ```text
/// I = 1; L = 1;
///        +--------+
///        |        |
///        \/      \/
/// A = [   1   ,   1   ,   1   ,   2   ,   3   ,   3   ]
/// SHOULD_SWAP = (1 != 1) = FALSE
///
///
///
/// I = 2; L = 1;
///         +---------------+
///         |               |
///         \/             \/
/// A = [   1   ,   1   ,   1   ,   2   ,   3   ,   3   ]
/// SHOULD_SWAP = (1 != 1) = FALSE
///
///
///
/// I = 3; L = 1;
///         +------------------------+
///         |                        |
///         \/                      \/
/// A = [   1   ,   1   ,   1   ,   2   ,   3   ,   3   ]
/// SHOULD_SWAP = (2 != 1) = TRUE
/// L += 1;
///
///
///
/// I = 4; L = 2;
///                 +-----------------------+
///                 |                       |
///                 \/                     \/
/// A = [   1   ,   2   ,   1   ,   1   ,   3   ,   3   ]
/// SHOULD_SWAP = (3 != 2) = TRUE
/// L += 1;
///
///
///
/// I = 5; L = 3;
///                         +-----------------------+
///                         |                       |
///                         \/                     \/
/// A = [   1   ,   2   ,   3   ,   1   ,   1   ,   3   ]
/// SHOULD_SWAP = (3 != 3) = FALSE
///
///
///
/// OUTPUT L
/// ```
pub fn garbage_array_duplicates<T>(array: &mut [T]) -> usize
where
    T: PartialEq,
{
    // If the array has just one or zero elements, it already adheres to the
    // constrains and we can just return the length.
    if array.len() < 2 {
        return array.len();
    }

    // Keeps track of how many unique sorted elements have we found so far.
    // Starts at one because the first element is always unique.
    let mut new_len = 1;

    // For each element except the first one, do...
    for index in 1..array.len() {
        // If we don't have this element in the head yet, expand the head range
        // by one element.
        if array[index] != array[new_len - 1] {
            array.swap(new_len, index);
            new_len += 1;
        }
    }

    new_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_handles_empty_array() {
        let mut array: [u8; 0] = [];

        assert_eq!(0, garbage_array_duplicates(&mut array));
    }

    #[test]
    fn it_handles_array_of_one_element() {
        let mut array: [u8; 1] = [8];

        assert_eq!(1, garbage_array_duplicates(&mut array));
    }

    #[test]
    fn it_solves_array_of_one_unique_elements() {
        let mut array: [u8; 4] = [8, 8, 8, 8];

        assert_eq!(1, garbage_array_duplicates(&mut array));
        assert_eq!(array[0], 8);
    }

    #[test]
    fn it_solves_array_of_two_unique_elements() {
        let mut array: [u8; 4] = [1, 1, 2, 2];

        assert_eq!(2, garbage_array_duplicates(&mut array));
        assert_eq!(array[0], 1);
        assert_eq!(array[1], 2);
    }

    #[test]
    fn it_solves_array_in_example() {
        let mut array: [u8; 8] = [1, 1, 2, 3, 4, 4, 4, 5];

        assert_eq!(5, garbage_array_duplicates(&mut array));
        assert_eq!(array[0], 1);
        assert_eq!(array[1], 2);
        assert_eq!(array[2], 3);
        assert_eq!(array[3], 4);
        assert_eq!(array[4], 5);
    }

    #[test]
    fn it_solves_marcos_array() {
        let mut array: [u8; 8] = [1, 2, 2, 4, 6, 6, 6, 8];

        assert_eq!(5, garbage_array_duplicates(&mut array));
        assert_eq!(array[0], 1);
        assert_eq!(array[1], 2);
        assert_eq!(array[2], 4);
        assert_eq!(array[3], 6);
        assert_eq!(array[4], 8);
    }

    #[test]
    fn it_handles_array_with_all_values_unique() {
        let mut array: [u8; 4] = [1, 2, 3, 4];

        assert_eq!(4, garbage_array_duplicates(&mut array));
        assert_eq!(array[0], 1);
        assert_eq!(array[1], 2);
        assert_eq!(array[2], 3);
        assert_eq!(array[3], 4);
    }
}
