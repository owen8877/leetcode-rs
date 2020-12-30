pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
    envelopes.sort();

    use std::cmp::max;
    let n = envelopes.len();
    let mut chain = vec![1; n];
    let mut longest = 0;
    for i in 0..n {
        for j in 0..i {
            if envelopes[j][0] < envelopes[i][0] && envelopes[j][1] < envelopes[i][1] {
                chain[i] = max(chain[j] + 1, chain[i]);
            }
        }
        longest = max(longest, chain[i]);
    }
    longest as i32
}