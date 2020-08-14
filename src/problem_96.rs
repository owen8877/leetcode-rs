pub fn num_trees(n: i32) -> i32 {
    if n <= 0 {
        return 0
    } else if n == 1 {
        return 1
    } else if n == 2 {
        return 2
    }
    let mut cache: Vec<Option<i32>> = vec![None; n as usize];
    cache[0] = Some(1);
    cache[1] = Some(2);
    cache[2] = Some(5);

    fn core(i: usize, cache: &mut Vec<Option<i32>>) -> i32 {
        match cache[i-1] {
            None => {
                let mut result = 2 * core(i-1, cache);
                for j in 1..=i-2 {
                    result += core(j, cache) * core(i-1-j, cache);
                }
                cache[i-1] = Some(result);
                result
            },
            Some(n) => n,
        }
    }

    core(n as usize, &mut cache)
}