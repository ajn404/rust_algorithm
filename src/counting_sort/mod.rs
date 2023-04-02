//自己写的
fn count_sort(arr: &[i32]) -> Vec<i32> {
    let len = arr.len();
    if len <= 0 {
        return vec![];
    }
    let mut k = arr[0];
    for &item in arr.iter() {
        if item > k {
            k = item;
        }
    }
    let max = k;

    // let mut C = vec![];
    // for i in 0..=max as usize {
    //     C.push(0);
    // }

    let mut C = vec![0; (max + 1) as usize];
    let mut res = vec![0; arr.len()];

    for j in 0..len {
        let index = arr[j] as usize;
        C[index] += 1;
    }
    for i in 1..=max as usize {
        C[i] += C[i - 1];
    }

    let mut k: isize = (len - 1) as isize;
    dbg!(k);
    while k >= 0 {
        let index = C[arr[k as usize] as usize] as usize;
        res[index - 1] = arr[k as usize];
        C[arr[k as usize] as usize] -= 1;
        k -= 1;
    }

    return res;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let arr = [2, 5, 3, 0, 2, 3, 0, 3];
        let res = count_sort(&arr);
        dbg!(res);
    }
}

mod gpt;
