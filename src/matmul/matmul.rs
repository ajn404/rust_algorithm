fn matmul(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let n = a.len();
    let p = b[0].len();
    let mut c: Vec<Vec<f64>> = vec![vec![0.; n]];

    return c;
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test() {
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
        let c = matmul(&a, &b);
        println!("{:?}", c);
    }
}
