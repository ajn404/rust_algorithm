fn matmul(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let n = a.len();

    let mut c: Vec<Vec<f64>> = vec![vec![0.; n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                c[i][j] = c[i][j] + a[i][k] * b[k][j];
            }
        }
    }
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
