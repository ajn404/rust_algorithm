// pub fn fib(n: i32) -> i32 {
//     if n == 1 {
//         return 1;
//     };
//     if n == 0 {
//         return 1;
//     };
//     let mut temp = vec![1, 1];
//     for i in 2..=n {
//         let end = temp[temp.len() - 1] + temp[temp.len() - 2];
//         temp.push(end % 1000000007)
//         //为了防止数据溢出1000000007是10位数最小的质数
//     }
//     return temp[temp.len() - 1];
// }

//优化
// pub fn fib(n: i32) -> i32 {
//     let mut v = vec![];
//     v.push(0);
//     v.push(1);
//     for i in 2..=n {
//         v.push((v[v.len() - 1] + v[v.len() - 2]) % (1e9 as i32 + 7));
//     }
//     v[n as usize]
// }

// i am so boring
pub fn fib(n: i32) -> i32 {
    let mut v = vec![];
    v.push(0);
    v.push(1);
    if n == 0 || n == 1 {
        return v[n as usize];
    }

    while v.len() <= n as usize {
        v.push((v[v.len() - 1] + v[v.len() - 2]) % (1e9 as i32 + 7));
    }
    v[v.len() - 1]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        for i in 0..46 {
            dbg!(i);
            dbg!(fib(i));
        }
    }
}
