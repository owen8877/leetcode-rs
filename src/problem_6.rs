pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 || s.len() == 0 {
        return s;
    }

    let n = s.len();
    let v: Vec<char> = s.chars().collect();
    let mut r = Vec::<char>::with_capacity(n);

    let m = (2*num_rows - 2) as usize;
    let max_cycle = n / m + 1;

    fn safe_insert(r: &mut Vec<char>, i: usize, v: &Vec<char>, j: usize) -> usize {
        if j < v.len() {
            r.push(v[j]);
            i + 1
        } else {
            i
        }
    }

    let mut counter = 0 as usize;

    // First row
    for i in 0..max_cycle+1 {
        counter = safe_insert(&mut r, counter, &v, i*m+0);
    }

    // Second to last second row
    for j in 1..num_rows as usize-1 {
        for i in 0..max_cycle+1 {
            counter = safe_insert(&mut r, counter, &v, i*m+j);
            counter = safe_insert(&mut r, counter, &v, i*m+m-j);
        }
    }

    // Last row
    for i in 0..max_cycle+1 {
        counter = safe_insert(&mut r, counter, &v, i*m+num_rows as usize-1);
    }

    r.iter().collect()
}

#[test]
fn test_convert() {
    assert_eq!(convert(String::from("abcdefg"), 1), "abcdefg");
    assert_eq!(convert(String::from("abcdefg"), 2), "acegbdf");
    assert_eq!(convert(String::from("PAYPALISHIRING"), 3), "PAHNAPLSIIGYIR");
    assert_eq!(convert(String::from("PAYPALISHIRING"), 4), "PINALSIGYAHRPI");
}