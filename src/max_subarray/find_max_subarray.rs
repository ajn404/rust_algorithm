// 定义一个包含最大子数组的元素位置和子数组的结构体
#[derive(Debug)]
struct MaxSubarray {
    start: usize,
    end: usize,
    sum: i32,
}

// 求解最大子数组问题
fn find_max_subarray(nums: &[i32], start: usize, end: usize) -> MaxSubarray {
    // 递归终止条件
    if start == end {
        return MaxSubarray {
            start,
            end,
            sum: nums[start],
        };
    }

    // 递归求解左右两个子数组的最大子数组
    let mid = (start + end) / 2;

    let left = find_max_subarray(nums, start, mid);
    let right = find_max_subarray(nums, mid + 1, end);

    // 合并两个子数组的最大子数组
    let cross = find_max_crossing_subarray(nums, start, mid, end);

    if left.sum >= right.sum && left.sum >= cross.sum {
        left
    } else if right.sum >= left.sum && right.sum >= cross.sum {
        right
    } else {
        cross
    }
}

// 求解跨越中点的最大子数组
fn find_max_crossing_subarray(nums: &[i32], start: usize, mid: usize, end: usize) -> MaxSubarray {
    let mut left_sum = i32::MIN;
    let mut right_sum = i32::MIN;
    let mut max_left = mid;
    let mut max_right = mid + 1;
    let mut sum = 0;

    for i in (start..=mid).rev() {
        sum += nums[i];
        if sum > left_sum {
            left_sum = sum;
            max_left = i;
        }
    }

    sum = 0;
    for i in mid + 1..=end {
        sum += nums[i];
        if sum > right_sum {
            right_sum = sum;
            max_right = i;
        }
    }

    MaxSubarray {
        start: max_left,
        end: max_right,
        sum: left_sum + right_sum,
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        let max_subarray = find_max_subarray(&nums, 0, nums.len() - 1);
        println!("Max subarray: {:?}", max_subarray);
    }
}
