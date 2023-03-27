fn unique_paths(m: i32, n: i32) -> i32 {
    let (m, n) = (m as u64, n as u64);
    let mut result = 1;
    for i in 1..m {
        result = result * (n + i - 1) / i;
    }
    result as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let res = unique_paths(12, 12);
        assert_eq!(res, 705432);
    }
}
