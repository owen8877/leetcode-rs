pub fn min_distance(word1: String, word2: String) -> i32 {
    use std::cmp::min;
    let n = word1.len();
    let m = word2.len();
    let mut distance = vec![vec![None; m+1]; n+1];
    let c1: Vec<char> = word1.chars().collect();
    let c2: Vec<char> = word2.chars().collect();

    for i in 0..n+1 {
        distance[i][m] = Some((n-i) as i32);
    }
    for j in 0..m+1 {
        distance[n][j] = Some((m-j) as i32);
    }

    fn distance_dfs(i: usize, j: usize, c1: &[char], c2: &[char], distance: &mut Vec<Vec<Option<i32>>>) -> i32{
        match distance[i][j] {
            Some(d) => d,
            None => {
                let mut d = distance_dfs(i+1, j, c1, c2, distance) + 1;
                d = min(d, distance_dfs(i, j+1, c1, c2, distance) + 1);
                d = min(d, distance_dfs(i+1, j+1, c1, c2, distance) + if c1[i] == c2[j] { 0 } else { 1 });

                distance[i][j] = Some(d);
                d
            },
        }
    }

    distance_dfs(0, 0, c1.as_slice(), c2.as_slice(), &mut distance)
}

#[test]
fn test_min_distance() {
    assert_eq!(min_distance("horse".to_string(), "ros".to_string()), 3);
    assert_eq!(min_distance("intention".to_string(), "execution".to_string()), 5);
}