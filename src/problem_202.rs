pub fn is_happy(mut n: i32) -> bool {
    let mut set = std::collections::HashSet::new();
    while n != 1 {
        if set.contains(&n) {
            return false
        }
        set.insert(n);

        let mut sum = 0;
        while n > 0 {
            sum += (n%10).pow(2);
            n /= 10;
        }
        n = sum;
    }
    true
}