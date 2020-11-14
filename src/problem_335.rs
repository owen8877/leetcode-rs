pub fn is_self_crossing(x: Vec<i32>) -> bool {
    for i in 3..x.len() {
        if x[i - 3] >= x[i - 1] && x[i] >= x[i - 2] {
            return true;
        }
    }
    for i in 4..x.len() {
        if x[i - 3] == x[i - 1] && x[i] + x[i - 4] >= x[i - 2] {
            return true;
        }
    }
    for i in 5..x.len() {
        if x[i - 2] >= x[i - 4] && x[i - 3] >= x[i - 1] && x[i - 5] + x[i - 1] >= x[i - 3] && x[i - 4] + x[i] >= x[i - 2] {
            return true;
        }
    }
    false
}