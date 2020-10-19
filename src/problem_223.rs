pub fn compute_area(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> i32 {
    use std::cmp::*;
    let a = a as i64;
    let b = b as i64;
    let c = c as i64;
    let d = d as i64;
    let e = e as i64;
    let f = f as i64;
    let g = g as i64;
    let h = h as i64;
    ((c - a) * (d - b) + (g - e) * (h - f) - max(0, min(c, g) - max(a, e)) * max(0, min(d, h) - max(b, f))) as i32
}