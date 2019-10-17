
/// Returns whether slice is sorted.
pub fn is_sorted<T>(array: &[T]) -> bool
    where
        T: PartialEq + PartialOrd + std::fmt::Debug,
{
    for index in 1..array.len() {
        if array[index] < array[index - 1] {
            return false
        }
    }

    true
}
