pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
    use std::cmp::{min, max};
    let m = dungeon.len();
    let n = dungeon[0].len();

    let mut minimal_health = vec![0; n];
    minimal_health[n-1] = max(1, 1 - dungeon[m-1][n-1]);

    for i in (0..n-1).rev() {
        minimal_health[i] = max(1, minimal_health[i+1]-dungeon[m-1][i]);
    }
    // println!("{:?}", minimal_health);

    let mut top_health = vec![0; n];
    for j in (0..m-1).rev() {
        top_health[n-1] = max(1, minimal_health[n-1]-dungeon[j][n-1]);
        for i in (0..n-1).rev() {
            top_health[i] = max(1, min(top_health[i+1], minimal_health[i])-dungeon[j][i]);
        }
        minimal_health = top_health.clone();
        // println!("{:?}", minimal_health);
    }

    minimal_health[0]
}

#[test]
fn test_calculate_minimum_hp() {
    assert_eq!(calculate_minimum_hp(vec![
        vec![-2, -3, 3],
        vec![-5, -10, 1],
        vec![10, 30, -5],
    ]), 7);
}