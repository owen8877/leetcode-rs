pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
    fn core(m: i32, depth: usize, remaining: i32, history: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if remaining < m || depth < 0 {
            return
        }
        history.push(m);
        if remaining == m && depth == 0 {
            result.push(history.clone());
        }
        for i in m+1..10 {
            core(i, depth-1, remaining-m, history, result);
        }
        history.pop();
    }

    let mut history = vec![];
    let mut result = vec![];
    for i in 1..10 {
        core(i, k as usize-1, n, &mut history, &mut result);
    }
    result
}