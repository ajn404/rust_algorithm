fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot = partition(arr);
    let (left, right) = arr.split_at_mut(pivot);
    quick_sort(left);
    quick_sort(&mut right[1..]);
}

fn partition(arr: &mut [i32]) -> usize {
    let len = arr.len();
    let pivot_index = len / 2;
    arr.swap(pivot_index, len - 1);
    let mut store_index = 0;
    for i in 0..len - 1 {
        if arr[i] < arr[len - 1] {
            arr.swap(i, store_index);
            store_index += 1;
        }
    }
    arr.swap(store_index, len - 1);
    store_index
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut arr = [2, 8, 7, 1, 3, 5, 6, 4];
        quick_sort(&mut arr);
        dbg!(arr);
    }
}
