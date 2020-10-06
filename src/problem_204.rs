pub fn count_primes(n: i32) -> i32 {
    if n <= 2 {
        0
    } else {
        let mut primes = vec![2];
        'outer: for i in 3..n {
            for &prime in &primes {
                if i % prime == 0 {
                    continue 'outer
                }
                if prime * prime > i {
                    break
                }
            }
            primes.push(i);
        }
        primes.len() as i32
    }
}