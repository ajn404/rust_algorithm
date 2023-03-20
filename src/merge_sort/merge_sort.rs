fn merge_sort(mut arr: &mut [i32]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }

    let mid = n / 2;
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    let mut temp = vec![0; n];
    let mut i = 0;
    let mut j = mid;
    let mut k = 0;

    while i < mid && j < n {
        if arr[i] <= arr[j] {
            temp[k] = arr[i];
            i += 1;
        } else {
            temp[k] = arr[j];
            j += 1;
        }
        k += 1;
    }

    while i < mid {
        temp[k] = arr[i];
        i += 1;
        k += 1;
    }

    while j < n {
        temp[k] = arr[j];
        j += 1;
        k += 1;
    }

    for i in 0..n {
        arr[i] = temp[i];
    }
}
