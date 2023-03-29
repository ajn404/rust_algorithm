mod priority_queue;

mod heap_sort {

    pub fn _heap_sort<T: Ord>(arr: &mut [T]) {
        // 构建最大堆
        _build_max_heap(arr);

        // 逐个将最大值（堆顶元素）移到数组末尾
        for i in (1..arr.len()).rev() {
            arr.swap(0, i);
            _heapify(arr, 0, i);
        }
    }

    fn _build_max_heap<T: Ord>(arr: &mut [T]) {
        for i in (0..arr.len() / 2).rev() {
            _heapify(arr, i, arr.len());
        }
    }

    fn _heapify<T: Ord>(arr: &mut [T], mut i: usize, n: usize) {
        loop {
            let mut max = i;
            let left = 2 * i + 1;
            let right = 2 * i + 2;

            if left < n && arr[left] > arr[max] {
                max = left;
            }
            if right < n && arr[right] > arr[max] {
                max = right;
            }
            if max == i {
                break;
            }

            arr.swap(i, max);
            i = max;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut arr = [99, 22, 33, 11, 24, 65, 23, 12];
        heap_sort::_heap_sort(&mut arr);
        dbg!(arr);
    }
}
