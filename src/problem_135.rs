// TODO: Needs Improvement
pub fn candy(ratings: Vec<i32>) -> i32 {
    use std::cmp::max;
    let n = ratings.len();
    if n <= 1 {
        return n as i32
    }
    let mut candys = vec![0; n];
    let mut index_ratings: Vec<(usize, &i32)> = ratings.iter().enumerate().collect();
    index_ratings.sort_by_key(|ir| ir.1);
    for (i, &r) in index_ratings {
        let mut c = 1;
        if i >= 1 && ratings[i-1] < ratings[i] {
            c = max(c, candys[i-1]+1);
        }
        if i <= n-2 && ratings[i+1] < ratings[i] {
            c = max(c, candys[i+1]+1);
        }
        candys[i] = c;
    }
    candys.iter().fold(0, |acc, &x| acc+x)
}