// use std::collections::HashSet;

// pub fn _three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
//     let mut nums: Vec<i32> = nums;
//     nums.sort();
//     let mut nums[i] = 0;
//     let mut _left = 0;
//     let mut _right = 0;
//     let mut set = HashSet::new();

//     for i in 0..len {
//         nums[i] = nums[i];
//         _left = i + 1;
//         _right = len - 1;

//         if nums[i] <= 0 {
//             for _j in i + 1..len {
//                 if _left >= _right {
//                     break;
//                 }

//                 let sum = nums[i] + nums[_left] + nums[_right];
//                 if sum == 0 {
//                     set.insert(vec![nums[i], nums[_left], nums[_right]]);
//                     _left += 1;
//                     _right -= 1;
//                 } else if sum < 0 {
//                     _left += 1;
//                 } else {
//                     _right -= 1;
//                 }
//             }
//         }
//     }

//     let unique_arr: Vec<_> = set.into_iter().collect();

//     unique_arr
// }

//改进
// use std::collections::HashSet;

pub fn _three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort_unstable();
    let mut _left = 0;
    let mut _right = 0;
    let len = nums.len();
    let mut res: Vec<_> = vec![];
    for i in 0..len - 2 {
        if nums[i] > 0 {
            break;
        }
        _left = i + 1;
        _right = len - 1;

        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        for _j in i + 1..len {
            if _left >= _right {
                break;
            }

            if _left > i + 1 && nums[_left] == nums[_left - 1] {
                _left += 1;
                continue;
            }

            let sum = nums[i] + nums[_left] + nums[_right];
            if sum == 0 {
                res.push(vec![nums[i], nums[_left], nums[_right]]);
                _left += 1;
                _right -= 1;
            } else if sum < 0 {
                _left += 1;
            } else {
                _right -= 1;
            }
        }
    }
    res
}

//大神写的
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let len = nums.len();
    if len < 3 {
        return vec![];
    }
    let mut nums = nums.clone();
    nums.sort_unstable();
    let mut res = vec![];
    let mut i = 0;
    loop {
        if i == len - 2 || nums[i] > 0 {
            break;
        }
        if i > 0 && nums[i] == nums[i - 1] {
            i += 1;
            continue;
        }
        let mut left = i + 1;
        let mut right = len - 1;
        loop {
            if left >= right {
                break;
            }
            let sum = nums[i] + nums[left] + nums[right];
            if sum == 0 {
                res.push(vec![nums[i], nums[left], nums[right]]);
                while left < right && nums[left] == nums[left + 1] {
                    left += 1;
                }
                while left < right && nums[right] == nums[right - 1] {
                    right -= 1;
                }
                left += 1;
                right -= 1;
                continue;
            } else if sum < 0 {
                left += 1;
                continue;
            } else {
                right -= 1;
                continue;
            }
        }

        i += 1;
    }
    res
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let mut nums = vec![-2, 0, 0, 2, 2];
        let res = _three_sum(nums);
        dbg!(res);
    }

    #[test]
    fn testss() {
        let mut nums = vec![-2, 0, 0, 2, 2];
        let res = three_sum(nums);
        dbg!(res);
    }
}
