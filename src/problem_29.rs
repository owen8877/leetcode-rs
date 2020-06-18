pub fn divide(dividend: i32, divisor: i32) -> i32 {
    if divisor == 0 {
         return (((1 as i64) << 31) - 1) as i32;
    }
    if divisor == (-1) << 31 {
        return if dividend == (-1) << 31 {
            1
        } else {
            0
        }
    }

    fn core(dividend: i32, divisor: i32) -> i32 {
        let pdivisor = if divisor > 0 { divisor } else { -divisor };
        let pdividend = if dividend > 0 { dividend } else { -dividend };

        let ndivisor = {
            let mut m = 31;
            for n in (0..32).rev() {
                if pdivisor >> n != 0 {
                    m = n;
                    break
                }
            }
            m
        };

        let mut remainder = pdividend;
        let mut result = 0;
        for j in (0..(31 - ndivisor)).rev() {
            if remainder >= pdivisor << j {
                result += 1 << j;
                remainder -= pdivisor << j;
            }
        }

        if (dividend > 0) ^ (divisor > 0) {
            -result
        } else {
            result
        }
    }

    if dividend == (-1) << 31 {
        match divisor {
            1 => (-1) << 31,
            -1 => (((1 as i64) << 31) - 1) as i32,
            n => {
                if n > 0 {
                    core(dividend+n, n) - 1
                } else {
                    core(dividend-n, n) + 1
                }
            }
        }
    } else {
        core(dividend, divisor)
    }
}

#[test]
fn test_divide() {
    assert_eq!(divide(-2147483648, -1), 2147483647);
    assert_eq!(divide(-2147483648, 1), -2147483648);
    assert_eq!(divide(-2147483648, -10), 214748364);
    assert_eq!(divide(75678510, 45233), 1673);
    assert_eq!(divide(7, -3), -2);
    assert_eq!(divide(10, 3), 3);
}