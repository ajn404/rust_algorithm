///插入排序
/// O(n^2)
/// 
/// 
fn insertion_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 1..len {
        let key = arr[i];
        let mut j = i - 1;
        while j >= 0 && arr[j] > key { // 非降序
            arr[j + 1] = arr[j];
            j -= 1;
        }
        arr[j + 1] = key;
    }
}

fn insertion_sort_inverse(arr:&mut [i32]){
    let len = arr.len();
    for i in 1..len {
        let key = arr[i];
        let mut j = i - 1;
        while j >= 0 && arr[j] < key { //非升
            arr[j + 1] = arr[j];
            j -= 1;
        }
        arr[j + 1] = key;
    }

}

fn insertion_sort_vec(arr: &mut Vec<i32>) {
    let n = arr.len();

    for i in 1..n {
        let mut j = i;

        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}





#[cfg(test)]
mod test_insertion_sort{
    use super::*;
    #[test]
    fn text(){
        let mut arr = [1,2,4,5,3,6];
        insertion_sort(&mut arr);
        dbg!(arr);

    }
}

