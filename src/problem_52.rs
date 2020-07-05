pub fn total_n_queens(n: i32) -> i32 {
    if n < 4 {
        if n == 1 {
            return 1
        }
        return 0
    }
    let n = n as usize;

    fn core(index: usize, v: &mut Vec<usize>) -> i32 {
        let n = v.len();
        if index == n {
            return 1
        }
        let mut sum = 0;
        'outer: for i in 0..n {
            for j in 0..index {
                if v[j] == i || v[j] + j == i + index || v[j] + index == i + j {
                    continue 'outer
                }
            }
            v[index] = i;
            sum += core(index+1, v);
        }
        sum
    }

    let mut sum = 0;
    for i in 0..n/2 {
        let mut v = vec![0; n];
        v[0] = i;
        sum += core(1, &mut v) * 2;
    }
    if n % 2  == 1 {
        let mut v = vec![0; n];
        v[0] = n/2;
        sum += core(1, &mut v);
    }

    sum
}

#[test]
fn test_solve_n_queens() {
    assert_eq!(total_n_queens(1), 1);
    assert_eq!(total_n_queens(2), 0);
    assert_eq!(total_n_queens(4), 2);
    assert_eq!(total_n_queens(8), 92);
    assert_eq!(total_n_queens(9), 352);
}
