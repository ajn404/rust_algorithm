fn multiply_matrices(matrices: &[&Vec<Vec<f64>>]) -> Vec<Vec<f64>> {
    let len = matrices.len();
    if len == 1 {
        return matrices[0].clone();
    } else if len == 2 {
        return multiply_two_matrices(matrices[0], matrices[1]);
    } else {
        let mid = len / 2;
        let matrices1 = &matrices[0..mid];
        let matrices2 = &matrices[mid..len];
        let matrix1 = multiply_matrices(matrices1);
        let matrix2 = multiply_matrices(matrices2);
        return multiply_two_matrices(&matrix1, &matrix2);
    }
}

fn multiply_two_matrices(matrix1: &Vec<Vec<f64>>, matrix2: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let m1_rows = matrix1.len();
    let m2_cols = matrix2[0].len();
    let m2_rows = matrix2.len();
    let mut result = vec![vec![0.0; m2_cols]; m1_rows];
    for i in 0..m1_rows {
        for j in 0..m2_cols {
            for k in 0..m2_rows {
                result[i][j] += matrix1[i][k] * matrix2[k][j];
            }
        }
    }
    return result;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let a = vec![vec![1.0, 2.0], vec![2.0, 3.0]];
        let b = vec![vec![1.0, 2.0], vec![2.0, 3.0]];
        let c = multiply_matrices(&[&a, &b]);
        println!("{:?}", c);
    }
}
