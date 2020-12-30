pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    use std::collections::BinaryHeap;

    let mut heap = BinaryHeap::new();
    for row in matrix {
        for num in row {
            heap.push(num);
            if heap.len() == k as usize + 1 {
                heap.pop();
            }
        }
    }
    heap.pop().unwrap()
}