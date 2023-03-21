fn kadane_max_subarray(arr: &[i32]) -> i32 {
    let mut max_sum = std::i32::MIN;
    let mut cur_sum = 0;
    for &num in arr {
        cur_sum = cur_sum.max(0) + num;
        max_sum = max_sum.max(cur_sum);
    }
    max_sum
}

/*
该实现使用了 Kadane 算法，其时间复杂度为 O(n)，其中 n 为数组的长度。具体来说，我们使用两个变量 max_sum 和 cur_sum，其中 max_sum 记录目前为止找到的最大子数组的和，cur_sum 记录以当前位置为结尾的子数组的和。

在遍历数组时，对于每个元素，我们更新 cur_sum 为 cur_sum.max(0) + num，这里的 max(0) 是因为如果当前元素 num 之前的子数组的和已经小于 0 了，那么我们直接抛弃之前的子数组，从当前位置重新开始。然后我们更新 max_sum 为 max_sum.max(cur_sum)，即当前找到的最大子数组和与已知的最大子数组和取较大值。
 */

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let targetArr = [1, 2, 3, 4, 5, -1, -2, -3, 4, 5, -1, -9, 10, 9];
        let max_num = kadane_max_subarray(&targetArr);
        dbg!(max_num);
    }
}
