pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let n = gas.len();
    let mut remaining = vec![0; n+1];
    for i in 0..n {
        remaining[i+1] = remaining[i] + gas[i] - cost[i];
    }
    if remaining[n] < 0 {
        return -1
    }
    remaining[0..n].iter().enumerate().fold((0, 0), |(j, rr), (i, &r)| {
        if r < rr {
            (i, r)
        } else {
            (j, rr)
        }
    }).0 as i32
}