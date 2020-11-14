pub fn intersection(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
    use std::cmp::Ordering::*;
    nums1.sort();
    nums2.sort();

    let mut p1 = 0;
    let mut p2 = 0;
    let mut result: Vec<i32> = vec![];
    while p1 < nums1.len() && p2 < nums2.len() {
        match nums1[p1].cmp(&nums2[p2]) {
            Less => {
                p1 += 1;
            }
            Equal => {
                if let Some(&a) = result.last() {
                    if a < nums1[p1] {
                        result.push(nums1[p1]);
                    }
                } else {
                    result.push(nums1[p1]);
                }
                p1 += 1;
                p2 += 1;
            }
            Greater => {
                p2 += 1;
            }
        }
    }
    result
}