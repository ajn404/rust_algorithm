fn binary_addition(a: &[u32], b: &[u32]) -> Vec<u32> {
    let n = a.len();
    let mut c = vec![0; n + 1];
    let mut carry = 0;
    for i in (0..n).rev() {
        let sum = a[i] + b[i] + carry;
        c[i + 1] = sum % 2;
        carry = sum / 2;
    }
    c[0] = carry;
    c
}

#[cfg(test)]
mod texts {
    use super::*;
    #[test]
    fn text_binary_addition() {
        let a = [1, 0, 1];
        let b = [0, 1, 1];
        let c = binary_addition(&a, &b);
        dbg!(c);
    }
}
