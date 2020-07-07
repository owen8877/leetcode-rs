pub fn add_binary(a: String, b: String) -> String {
    use std::cmp::*;
    let num_a: Vec<u8> = a.chars().rev().map(|c| c as u8 - '0' as u8).collect();
    let num_b: Vec<u8> = b.chars().rev().map(|c| c as u8 - '0' as u8).collect();

    let n_a = num_a.len();
    let n_b = num_b.len();
    let N = max(n_a, n_b)+1;
    let mut result = vec![0; N];

    let mut carrier = 0;
    let mut sum = 0;
    for i in 0..min(n_a, n_b) {
        sum = num_a[i] + num_b[i] + carrier;
        carrier = sum / 2;
        result[i] = sum % 2;
    }
    if n_a < n_b {
        for i in n_a..n_b {
            sum = num_b[i] + carrier;
            carrier = sum / 2;
            result[i] = sum % 2;
        }
    } else if n_a > n_b {
        for i in n_b..n_a {
            sum = num_a[i] + carrier;
            carrier = sum / 2;
            result[i] = sum % 2;
        }
    }
    result[N-1] = carrier;

    let end_pos = if carrier == 1 { N } else { N - 1 };
    result[0..end_pos].iter().rev().map(|x| (x + '0' as u8) as char).collect()
}