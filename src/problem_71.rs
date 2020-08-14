pub fn simplify_path(path: String) -> String {
    let mut directories = vec![];
    for s in path.split('/') {
        if s == "." || s == "" {
            continue
        } else if s == ".." {
            if directories.len() > 0 {
                directories.pop();
            }
        } else {
            directories.push(s);
        }
    }
    if directories.len() == 0 {
        "/".to_string()
    } else {
        directories.iter().fold("".to_string(), |acc, x| acc + "/" + x)
    }
}