// Using Lomuto partition scheme
// https://en.wikipedia.org/wiki/Quicksort

use std::cmp::{Ordering, PartialOrd};

// Let quicksort run only on a mutable fixed size array
// Should be trait bound to make sure comparison and copy is supported
// We use PartialOrd to support floating numbers, as they can be NaN which does not align with total ordering of Ord
pub fn quicksort<T>(arr: &mut [T], lo: usize, hi: usize)
where
    T: PartialOrd + Copy,
{
    if lo < hi {
        // Do nothing if only 1 element
        if arr.len() == 1 {
            return;
        }

        // Get a new partition
        let p = partition(arr, lo, hi);

        // Run quicksort on both partitions recursively
        // Because we are using usize, p-1 can lead to an underflow err, we check before if this would happen
        // This only happens when the last index is the smallest and thus partition returns 0
        if p > 0 {
            quicksort(arr, lo, p - 1); // sort left side
        }
        quicksort(arr, p + 1, hi); // sort right side
    }
}

fn partition<T>(arr: &mut [T], lo: usize, hi: usize) -> usize
where
    T: PartialOrd + Copy,
{
    let pivot = arr[hi];

    // Temporary pivot index
    let mut i = lo;

    for j in lo..=hi - 1 {
        match pivot.partial_cmp(&arr[j]) {
            Some(Ordering::Greater) => {
                arr.swap(i, j);
                i = i + 1;
            }
            _ => continue,
        }
    }
    arr.swap(i, hi);
    return i;
}
