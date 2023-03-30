pub fn _quick_sort(arr: &mut [u64], start: usize, end: usize) {
    if end <= start {
        return;
    }

    let pivot = _partition(arr, start, end);
    _quick_sort(arr, start, pivot - 1);
    _quick_sort(arr, pivot + 1, end);
}

fn _partition(arr: &mut [u64], start: usize, end: usize) -> usize {
    let pivot = arr[end];
    let mut i = start;

    for j in start..=end - 1 {
        // if let Some(v) = arr.get(j) {
        //     dbg!(v);
        //     if *v <= pivot {
        //         dbg!(pivot);
        //         i += 1;
        //         arr.swap(i, j);
        //     }
        // }
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    dbg!(i);
    arr.swap(i, end);
    i + 1
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
        // let mut arr = [2, 1, 3];
        let len = arr.len();

        let x = _partition(&mut arr, 0, len - 1);
        dbg!(x);
        //21347568
        dbg!(arr);
    }
}
