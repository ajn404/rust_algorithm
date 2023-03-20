/// 这段代码使用了递归来实现归并排序，其中 merge_sort 函数用来递归地将数组分成左右两部分并对其进行排序，
/// 而 merge 函数则用来合并两个已经排好序的子数组。
/// 在 merge 函数中，我们使用了一个临时的数组 temp 来存放归并后的结果，然后遍历左右两个子数组，将它们按照从小到大的顺序逐个取出，放入 temp 数组中。最后，我们将 temp 数组中的元素复制回原数组中。
/// 值得注意的是，在 merge_sort 函数中，我们使用了 Rust 的切片（slice）功能，这使得我们可以直接对原数组进行操作，而不需要通过传递起始和结束索引等方式来传递子数组的信息。

fn merge_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len < 2 {
        return;
    }
    let mid = len / 2;
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    let mut i = 0;
    let mut j = mid;
    let mut k = 0;
    let mut template = vec![0; len];

    while i < mid && j < len {
        if (arr[i] <= arr[j]) {
            template[k] = arr[i];
            i += 1;
        } else {
            template[k] = arr[j];
            j += 1;
        }
        k += 1;
    }

    while i < mid {
        template[k] = arr[i];
        k += 1;
        i += 1;
    }

    while j < len {
        template[k] = arr[j];
        j += 1;
        k += 1;
    }

    for m in 0..len {
        arr[m] = template[m];
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn text_merge_sort() {
        let mut arr = [2, 4, 5, 7, 1, 2, 3, 6];
        dbg!(&arr);
        merge_sort(&mut arr);
        dbg!(&arr);
    }
}
