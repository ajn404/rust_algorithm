use std::collections::BinaryHeap;

fn max_heap() {
    let mut max_heap = BinaryHeap::new();
    max_heap.push(3);
    max_heap.push(1);
    max_heap.push(4);
    max_heap.push(1);
    max_heap.push(5);
    while let Some(num) = max_heap.pop() {
        println!("{}", num);
    }
}

fn min_heap() {
    let mut min_heap = BinaryHeap::new();
    min_heap.push(3);
    min_heap.push(1);
    min_heap.push(4);
    min_heap.push(1);
    min_heap.push(5);
    let mut reversed_heap = min_heap.into_iter().rev().collect::<BinaryHeap<_>>();
    while let Some(num) = reversed_heap.pop() {
        println!("{}", num);
    }
}
