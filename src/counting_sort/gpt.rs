fn counting_sort(arr: &mut [i32]) {
    let max_value = arr.iter().max().unwrap_or(&0) + 1; //6
    let mut count = vec![0; max_value as usize]; //[0,0,0,0,0,0]
    let mut output = vec![0; arr.len()];

    for &value in arr.iter() {
        count[value as usize] += 1;
    }

    for i in 1..max_value {
        count[i as usize] += count[(i - 1) as usize];
    }

    for &value in arr.iter().rev() {
        output[count[value as usize] - 1] = value;
        count[value as usize] -= 1;
    }

    //Copies all elements from src into self, using a memcpy.
    arr.copy_from_slice(&output);
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let mut arr = [2, 5, 3, 0, 2, 3, 0, 3];
        counting_sort(&mut arr);
        dbg!(arr);

        let mut buffer = [0; 4];
        buffer = [2, 1, 4, 3];
        let data = [1, 2, 3, 4];

        buffer.copy_from_slice(&data);
        // dbg!(buffer);
    }
}
