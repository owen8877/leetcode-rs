pub fn fraction_to_decimal(mut numerator: i32, mut denominator: i32) -> String {
    if numerator == 0 {
        return "0".to_string()
    }
    let mut result = vec![];
    if (numerator as i64) * (denominator as i64) < 0 {
        result.push("-".to_string());
    }
    let mut numerator = (numerator as i64).abs();
    let mut denominator = (denominator as i64).abs();

    result.push(format!("{}", numerator / denominator));
    let mut occ = std::collections::HashMap::<i64, usize>::new();
    if numerator % denominator != 0 {
        result.push(".".to_string());
        numerator %= denominator;
        while !occ.contains_key(&numerator) {
            occ.insert(numerator, result.len());
            numerator *= 10;
            result.push(format!("{}", numerator / denominator));
            numerator %= denominator;
            if numerator == 0 {
                break
            }
        }
        if numerator != 0 {
            let index = occ.get(&numerator).unwrap();
            result.insert(*index, "(".to_string());
            result.push(")".to_string());
        }
    }

    result.iter().fold("".to_string(), |acc, x| acc + x.as_str())
}