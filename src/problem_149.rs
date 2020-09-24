use std::collections::*;
use Line::*;
use std::cmp::*;

#[derive(Hash, Eq, PartialEq)]
enum Line {
    V(VerticalLine),
    S(SlopeLine),
}

#[derive(Hash, Eq, PartialEq)]
struct VerticalLine {
    x0: i32,
}

#[derive(Hash, Eq, PartialEq)]
struct SlopeLine {
    slope: Fraction,
    intersect: Fraction,
}

#[derive(Hash, Eq, PartialEq)]
struct Fraction {
    n: i32,
    d: i32,
}

impl Fraction {
    fn from(a: i32, b: i32) -> Self {
        if a == 0 {
            Self {
                n: 0,
                d: 0,
            }
        } else {
            if b == 0 {
                panic!("b can not be 0!")
            }

            fn gcd(a: i32, b: i32) -> i32 {
                if b % a == 0 {
                    a
                } else {
                    gcd(b % a, a)
                }
            }
            let g = if a.abs() < b.abs() {
                gcd(a.abs(), b.abs())
            } else if a.abs() > b.abs() {
                gcd(b.abs(), a.abs())
            } else {
                a.abs()
            };
            Self {
                n: a / g * b.signum(),
                d: b.abs() / g,
            }
        }
    }
}

pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
    let mut vertices_c = HashMap::<(i32, i32), usize>::new();
    for p in points {
        *vertices_c.entry((p[0], p[1])).or_insert(0) += 1;
    }
    let n = vertices_c.len();
    if n == 1 {
        return *vertices_c.values().next().unwrap() as i32;
    }

    let vertices: Vec<&(i32, i32)> = vertices_c.keys().collect();
    // println!("vertices: {:?}", vertices);
    let mut m = 0;
    for i in 0..n {
        let mut lines = HashMap::<Line, usize>::new();
        for j in i + 1..n {
            let (xi, yi) = *vertices[i];
            let (xj, yj) = *vertices[j];
            let line = if xi == xj {
                V(VerticalLine {
                    x0: xi
                })
            } else {
                S(SlopeLine {
                    slope: Fraction::from(yj - yi, xj - xi),
                    intersect: Fraction::from(xj * yi - xi * yj, xj - xi),
                })
            };
            *lines.entry(line).or_insert(0) += vertices_c.get(vertices[j]).unwrap();
        }
        m = max(m, lines.values().fold(0, |acc, &v| max(acc, v)) + vertices_c.get(vertices[i]).unwrap());
    }

    m as i32
}