pub fn coin_change(mut coins: Vec<i32>, amount: i32) -> i32 {
    if amount == 0 {
        return 0;
    }
    coins.sort_by_key(|x| -x);

    let mut counts = vec![None; amount as usize];

    fn search(coins: &Vec<i32>, amount: i32, counts: &mut Vec<Option<i32>>) -> Option<i32> {
        if amount < 0 {
            return Some(-1);
        } else if amount == 0 {
            return Some(0);
        }
        if counts[amount as usize - 1].is_none() {
            let mut m = None;
            for &coin in coins {
                let s = search(coins, amount - coin, counts);
                if let Some(s) = s {
                    if s >= 0 {
                        m = Some(std::cmp::min(m.unwrap_or(s), s));
                    }
                }
            }
            counts[amount as usize - 1] = Some(m.unwrap_or(-2) + 1);
        }
        return counts[amount as usize - 1].clone();
    }

    search(&coins, amount, &mut counts).unwrap()
}