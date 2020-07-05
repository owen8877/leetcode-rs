pub fn my_pow(x: f64, n: i32) -> f64 {
    fn core(x: f64, n: i64) -> f64 {
        if n == 1 {
            x
        } else if n == 2 {
            x*x
        } else if n % 2 == 0{
            let m = core(x, n/2);
            m*m
        } else {
            let m = core(x, n/2);
            m*m*x
        }
    }

    fn x_sign(x: f64, n: i64) -> f64 {
        if x < 0.0 {
            if n % 2 == 1{
                -core(-x, n)
            } else {
                core(-x, n)
            }
        } else {
            core(x, n)
        }
    }

    if n == 0 {
        1.0
    } else if n < 0 {
        1.0 / x_sign(x, -(n as i64))
    } else {
        x_sign(x, n as i64)
    }
}

#[test]
fn test_my_pow() {
    assert_eq!(my_pow(1.0, -2147483648), 1.0);
}