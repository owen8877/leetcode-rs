pub fn is_valid_serialization(preorder: String) -> bool {
    if preorder == "#" {
        return true;
    }
    let mut v = vec![];
    let splits: Vec<&str> = preorder.split(',').collect();
    for i in 0..splits.len() {
        if splits[i] == "#" {
            match v.last_mut() {
                None => return false,
                Some(k) => *k += 1,
            }
        } else {
            v.push(0);
        }

        while !v.is_empty() && v[v.len() - 1] == 2 {
            v.pop();
            if v.is_empty() {
                return i == splits.len() - 1;
            }
            let n = v.len();
            v[n - 1] += 1;
        }
    }
    v.is_empty()
}