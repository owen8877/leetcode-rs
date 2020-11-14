pub fn intersect(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
    use std::cmp::Ordering::*;
    nums1.sort();
    nums2.sort();

    let mut p1 = 0;
    let mut p2 = 0;
    let mut result = vec![];
    while p1 < nums1.len() && p2 < nums2.len() {
        match nums1[p1].cmp(&nums2[p2]) {
            Less => {
                p1 += 1;
            }
            Equal => {
                result.push(nums1[p1]);
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