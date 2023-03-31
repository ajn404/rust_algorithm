pub fn _quick_sort(arr: &mut [u64], start: usize, end: usize) {
    // dbg!(end, start);
    if end <= start {
        return;
    }

    let pivot = _partition(arr, start, end);

    //end usize是无符号，o shit
    if pivot == 0 {
        return;
    }
    _quick_sort(arr, start, pivot - 1);
    _quick_sort(arr, pivot + 1, end);
}

fn _partition(arr: &mut [u64], start: usize, end: usize) -> usize {
    let pivot = arr[end];
    let mut i = start;

    for j in start..=end - 1 {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, end);
    dbg!(i, end, arr);

    i
}

mod sort;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut arr = [2, 8, 7, 1, 3, 5, 6, 4];
        let len = arr.len();
        _quick_sort(&mut arr, 0, len - 1);
        dbg!(arr);
    }

    #[test]
    fn fbg() {
        let mut arr = [2, 8, 7, 1, 3, 5, 6, 4];
        let len = arr.len();
        let x = _partition(&mut arr, 0, len - 1);
        dbg!(x);
        dbg!(arr);
    }

    #[test]
    fn panic_test() {
        let a: isize = 0;
        dbg!(a - 1);
    }
}
