pub fn multiply(num1: String, num2: String) -> String {
    if num1 == "0".to_string() || num2 == "0".to_string() {
        return "0".to_string()
    }

    let n1 = num1.chars().map(|c| (c as u8 - '0' as u8)).collect::<Vec<u8>>();
    let n2 = num2.chars().map(|c| (c as u8 - '0' as u8)).collect::<Vec<u8>>();
    let d1 = n1.len();
    let d2 = n2.len();

    let mut result = vec![0; d1+d2];
    for j in (0..d2).rev() {
        for i in (0..d1).rev() {
            let mut pointer = i+j+1;
            let mut carrier = n1[i] * n2[j];
            loop {
                let sum = result[pointer] + carrier;
                result[pointer] = sum % 10;
                carrier = sum / 10;
                if carrier == 0 {
                    break
                }
                pointer -= 1;
            }
        }
    }

    let start = if result[0] == 0 { 1 } else { 0 };
    result[start..].iter().map(|x| ('0' as u8 + *x) as char).collect()
}

#[test]
fn test_multiply() {
    assert_eq!(multiply("123".to_string(), "456".to_string()), "56088".to_string());
    assert_eq!(multiply("0".to_string(), "456".to_string()), "0".to_string());
    assert_eq!(multiply("2".to_string(), "456".to_string()), "912".to_string());
    assert_eq!(multiply("12".to_string(), "456".to_string()), "5472".to_string());
    assert_eq!(multiply("1231124123".to_string(), "456".to_string()), "561392600088".to_string());
}