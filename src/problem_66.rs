pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut digits = digits;
    let mut carrier = 1;
    for j in (0..digits.len()).rev() {
        let sum = digits[j] + carrier;
        digits[j] = sum % 10;
        carrier = sum / 10;
        if carrier == 0 {
            return digits
        }
    }
    if carrier ==  1 {
        digits.insert(0, 1);
    }
    digits
}