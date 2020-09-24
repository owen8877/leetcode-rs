pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut one = 0;
    let mut two = 0;
    for i in nums {
        two |= i & one;
        one ^= i;
        let nt = !(one & two);
        one &= nt;
        two &= nt;
    }
    one
}