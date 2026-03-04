use std::cmp::Ordering;

use super::core::partition;
use super::simple;

pub fn quick_sort<T, F>(arr: &mut [T], compare: F)
where
    T: Send,
    F: Fn(&T, &T) -> Ordering + Copy + Sync,
{
    if arr.len() <= 1 {
        return;
    }
    if arr.len() < 512 {
        simple::quick_sort(arr, compare);
        return;
    }

    let (lt, i) = partition(arr, compare);

    let (left, rest) = arr.split_at_mut(lt);
    let (_, right) = rest.split_at_mut(i - lt);

    rayon::join(|| quick_sort(left, compare), || quick_sort(right, compare));
}
