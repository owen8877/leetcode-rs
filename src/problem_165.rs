pub fn compare_version(version1: String, version2: String) -> i32 {
    use std::cmp::*;
    let v1: Vec<&str> = version1.split('.').collect();
    let v2: Vec<&str> = version2.split('.').collect();
    for i in 0..max(v1.len(), v2.len()) {
        let n1 = if i < v1.len() {
            v1[i].parse::<i32>().unwrap()
        } else { 0 };
        let n2 = if i < v2.len() {
            v2[i].parse::<i32>().unwrap()
        } else { 0 };
        if n1 < n2 {
            return -1
        } else if n1 > n2 {
            return 1
        }
    }
    return 0
}