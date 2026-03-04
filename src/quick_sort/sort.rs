use std::cmp::Ordering;

use super::core::partition;

pub fn quick_sort<T, F>(arr: &mut [T], compare: F)
where
    T: Send,
    F: Fn(&T, &T) -> Ordering + Copy + Sync,
{
    if arr.len() <= 1 {
        return;
    }

    let (lt, i) = partition(arr, compare);

    let (left, rest) = arr.split_at_mut(lt);
    let (_, right) = rest.split_at_mut(i - lt);

    quick_sort(left, compare);
    quick_sort(right, compare);
}
