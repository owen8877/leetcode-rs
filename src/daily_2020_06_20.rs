pub fn get_permutation(n: i32, k: i32) -> String {
    let n = n as usize;
    let mut result = vec![0; n];
    let mut took = vec![false; n];


    fn find_l_untaken(n: usize, took: &mut Vec<bool>, l: usize) -> usize {
        let mut counter = 0;
        for j in 0..n {
            if !took[j] {
                if counter == l {
                    took[j] = true;
                    return j
                }
                counter += 1;
            }
        }
        unreachable!()
    }

    let mut k = k as usize;
    let mut fact = (1..n).fold(1, |acc, x| acc * x) as usize;
    for i in (0..n).rev() {
        let l = (k-1) / fact;
        result[n-1-i] = find_l_untaken(n, &mut took, l);
        k -= l * fact;
        fact /= std::cmp::max(i, 1);
    }

    result.into_iter().map(|x| std::char::from_digit(x as u32 + 1, 10).unwrap()).collect()
}

#[test]
fn test_get_permutation() {
    assert_eq!(get_permutation(3, 3), "213".to_string());
    assert_eq!(get_permutation(4, 9), "2314".to_string());
}