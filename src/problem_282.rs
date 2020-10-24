pub fn add_operators(num: String, target: i32) -> Vec<String> {
    fn core(index: usize, previous_num: i64, mut current_num: i64, value: i64, ops: &mut Vec<String>, result: &mut Vec<String>, digits: &[i64], target: i64) {
        if index == digits.len() {
            if value == target && current_num == 0 {
                result.push(ops[1..].iter().fold("".to_string(), |acc, x| acc + x));
            }
            return;
        }

        current_num *= 10;
        current_num += digits[index];
        let custr = format!("{}", current_num);

        if current_num > 0 {
            core(index + 1, previous_num, current_num, value, ops, result, digits, target);
        }

        ops.push("+".to_string());
        ops.push(custr.clone());
        core(index + 1, current_num, 0, value + current_num, ops, result, digits, target);
        ops.pop();
        ops.pop();

        if !ops.is_empty() {
            ops.push("-".to_string());
            ops.push(custr.clone());
            core(index + 1, -current_num, 0, value - current_num, ops, result, digits, target);
            ops.pop();
            ops.pop();

            ops.push("*".to_string());
            ops.push(custr);
            core(index + 1, current_num * previous_num, 0, value - previous_num + current_num * previous_num, ops, result, digits, target);
            ops.pop();
            ops.pop();
        }
    }

    if num.len() == 0 {
        vec![]
    } else {
        let mut result = vec![];
        let mut ops = vec![];
        let digits: Vec<i64> = num.chars().map(|c| (c as u8 - '0' as u8) as i64).collect();

        core(0, 0, 0, 0, &mut ops, &mut result, &digits, target as i64);
        result
    }
}