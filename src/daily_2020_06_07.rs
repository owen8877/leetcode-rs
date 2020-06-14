pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut cache = HashMap::<(i32, i32), i32>::new();
    let mut coins = coins;
    coins.sort();

    fn core(cache: &mut HashMap<(i32, i32), i32>, amount: i32, coins: &[i32]) -> i32 {
        if amount == 0 {
            return 1
        }
        if coins.len() == 0 {
            return 0
        }
        if coins.len() == 1 {
            return if amount % coins[0] == 0 {
                1
            } else {
                0
            }
        }

        let key = (amount, coins.len() as i32);
        if cache.contains_key(&key) {
            return cache.get(&key).unwrap().clone()
        }

        let nc = coins.len();
        let largest = coins[nc-1];
        let max = amount / largest;
        let mut counter = 0;
        for i in 0..=max {
            counter += core(cache, amount-i*largest, &coins[0..nc-1]);
        }

        cache.insert(key, counter);
        counter
    }

    core(&mut cache, amount, &coins)
}

#[test]
fn test_change() {
    assert_eq!(change(5, vec![1, 2, 5]), 4);
    assert_eq!(change(3, vec![2]), 0);
    assert_eq!(change(10, vec![10]), 1);
    assert_eq!(change(500, vec![3, 5, 7, 8, 9, 10, 11]), 35502874);
}
