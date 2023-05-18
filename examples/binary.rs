use std::cmp::Ordering;

// recursive
pub fn binary_search_rec<T: Ord>(
    list_of_items: &[T],
    target: &T,
    left: &usize,
    right: &usize,
) -> Option<usize> {
    if left >= right {
        return None;
    }

    let is_asc = list_of_items[0] < list_of_items[list_of_items.len() - 1];

    let middle: usize = left + (right - left) / 2;

    if is_asc {
        match target.cmp(&list_of_items[middle]) {
            Ordering::Less => binary_search_rec(list_of_items, target, left, &middle),
            Ordering::Greater => binary_search_rec(list_of_items, target, &(middle + 1), right),
            Ordering::Equal => Some(middle),
        }
    } else {
        match target.cmp(&list_of_items[middle]) {
            Ordering::Less => binary_search_rec(list_of_items, target, &(middle + 1), right),
            Ordering::Greater => binary_search_rec(list_of_items, target, left, &middle),
            Ordering::Equal => Some(middle),
        }
    }
}

0 3 2


// normal
pub fn binary_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
    let mut is_asc = true;
    if arr.len() > 1 {
        is_asc = arr[0] < arr[(arr.len() - 1)];
    }
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;

        if is_asc {
            match item.cmp(&arr[mid]) {
                Ordering::Less => right = mid,
                Ordering::Equal => return Some(mid),
                Ordering::Greater => left = mid + 1,
            }
        } else {
            match item.cmp(&arr[mid]) {
                Ordering::Less => left = mid + 1,
                Ordering::Equal => return Some(mid),
                Ordering::Greater => right = mid,
            }
        }
    }
    None
}
