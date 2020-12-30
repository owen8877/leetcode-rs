pub fn can_measure_water(x: i32, y: i32, z: i32) -> bool {
    if z == 0 {
        return true;
    } else if x == 0 {
        return y == z;
    } else if y == 0 {
        return z == x;
    } else {
        if z > x + y {
            return false;
        }
        fn gcd(a: i32, b: i32) -> i32 {
            if a == b {
                a
            } else if a < b {
                gcd(b, a)
            } else {
                if a % b == 0 {
                    b
                } else {
                    gcd(b, a % b)
                }
            }
        }

        let g = gcd(x, y);
        z % g == 0
    }
}