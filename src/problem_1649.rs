pub fn create_sorted_array(instructions: Vec<i32>) -> i32 {
    let n = 100005;
    let mut d = vec![0; n * 4];
    let mut b = vec![0; n * 4];

    fn update(l: usize, r: usize, c: i32, s: usize, t: usize, p: usize, d: &mut Vec<i32>, b: &mut Vec<i32>) {
        if l > r {
            return;
        }
        if l <= s && t <= r {
            d[p] += (t - s + 1) as i32 * c;
            b[p] += c;
            return;
        }
        let m = (s + t) >> 1;
        if b[p] != 0 {
            d[p << 1] += b[p] * (m - s + 1) as i32;
            d[(p << 1) | 1] += b[p] * (t - m) as i32;
            b[p << 1] += b[p];
            b[(p << 1) | 1] += b[p];
        }
        b[p] = 0;
        if l <= m {
            update(l, r, c, s, m, p << 1, d, b);
        }
        if r > m {
            update(l, r, c, m + 1, t, (p << 1) | 1, d, b);
        }
        d[p] = d[p << 1] + d[(p << 1) | 1];
    }

    fn getsum(l: usize, r: usize, s: usize, t: usize, p: usize, d: &mut Vec<i32>, b: &mut Vec<i32>) -> i32 {
        if l > r {
            return 0;
        }
        if l <= s && t <= r {
            return d[p];
        }
        let m = (s + t) >> 1;
        if b[p] != 0 {
            d[p << 1] += b[p] * (m - s + 1) as i32;
            d[(p << 1) | 1] += b[p] * (t - m) as i32;
            b[p << 1] += b[p];
            b[(p << 1) | 1] += b[p];
        }
        b[p] = 0;
        let mut sum = 0;
        if l <= m {
            sum = getsum(l, r, s, m, p << 1, d, b);
        }
        if r > m {
            sum += getsum(l, r, m + 1, t, (p << 1) | 1, d, b);
        }
        sum
    }

    let mut cost = 0;
    for ins in instructions {
        cost += std::cmp::min(getsum(1, ins as usize - 1, 1, n, 1, &mut d, &mut b), getsum(ins as usize + 1, n, 1, n, 1, &mut d, &mut b));
        cost %= 1000000007;
        update(ins as usize, ins as usize, 1, 1, n, 1, &mut d, &mut b);
    }

    cost
}

#[test]
fn test_create_sorted_array() {
    assert_eq!(create_sorted_array(vec![1, 5, 6, 2]), 1);
}