pub fn super_pow(mut a: i32, b: Vec<i32>) -> i32 {
    let m = 1337;
    a %= m;

    let mut table = vec![1; 10];
    for i in 1..10 {
        table[i] = (table[i - 1] * a) % m;
    }

    let mut result = 1i32;
    for num in b {
        let mut tmp = 1i32;
        for i in 0..10 {
            tmp *= result;
            tmp %= m;
        }
        result = tmp;
        result *= table[num as usize];
        result %= m;
    }

    result
}