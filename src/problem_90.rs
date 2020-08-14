pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.is_empty() {
        return vec![vec![]]
    }
    nums.sort();
    let mut num_freq = vec![];
    let mut current_num = nums[0];
    let mut freq = 0;
    for num in nums {
        if current_num == num {
            freq += 1;
        } else {
            num_freq.push((current_num, freq));
            current_num = num;
            freq = 1;
        }
    }
    num_freq.push((current_num, freq));

    let mut power_set: Vec<Vec<i32>> = vec![vec![]];
    for (num, freq) in num_freq {
        let previous_set = power_set;
        power_set = vec![];
        for mut set in previous_set {
            for f in 0..=freq {
                power_set.push(set.clone());
                set.push(num);
            }
        }
    }
    power_set
}
