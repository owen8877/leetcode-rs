pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    if nums1.len() > nums2.len() {
        return find_median_sorted_arrays(nums2, nums1);
    }

    let m = nums1.len();
    let n = nums2.len();

    if m == 0 {
        return if n % 2 == 0 {
            (nums2[n / 2 - 1] + nums2[n / 2]) as f64 / 2.0
        } else {
            nums2[(n - 1) / 2] as f64
        }
    }

    // Here we can safely assume m <= n
    // Edge cases
    if nums1[m-1] <= nums2[0] {
        return match (m + n) % 2 {
            0 => {
                if m == n {
                    (nums2[0] + nums1[m-1]) as f64 / 2.0
                } else {
                    let mid = (m + n) / 2 - m;
                    (nums2[mid - 1] + nums2[mid]) as f64 / 2.0
                }
            },
            _ => {
                let mid = (m + n - 1) / 2 - m;
                nums2[mid] as f64
            }
        }
    }
    if nums1[0] >= nums2[n-1] {
        return match (m + n) % 2 {
            0 => {
                if m == n {
                    (nums1[0] + nums2[n-1]) as f64 / 2.0
                } else {
                    let mid = (m + n) / 2;
                    (nums2[mid - 1] + nums2[mid]) as f64 / 2.0
                }
            },
            _ => {
                let mid = (m + n - 1) / 2;
                nums2[mid] as f64
            }
        }
    }
    // Now we can assume no edge cases occur
    let mut imin = 0;
    let mut imax = m;
    let mut i;
    let mut j;
    loop {
        if imin == imax {
            i = imin;
            j = match (m + n) % 2 {
                0 => (m + n) / 2 - i,
                _ => (m + n + 1) / 2 - i,
            };
        } else {
            i = (imin + imax) / 2;
            j = match (m + n) % 2 {
                0 => (m + n) / 2 - i,
                _ => (m + n + 1) / 2 - i,
            };

            if nums2[j - 1] > nums1[i] {
                imin = i + 1;
                continue;
            }
            if i > 0 && nums2[j] < nums1[i - 1] {
                imax = i;
                continue;
            }
        }
        return match (m + n) % 2 {
            0 => {
                let max = if i == m {
                    nums2[j]
                } else {
                    std::cmp::min(nums1[i], nums2[j])
                };
                let min = if i == 0 {
                    nums2[j - 1]
                } else {
                    std::cmp::max(nums1[i - 1], nums2[j - 1])
                };
                (min + max) as f64 / 2.0
            },
            _ => {
                if i == 0 {
                    nums2[j - 1] as f64
                } else { std::cmp::max(nums1[i - 1], nums2[j - 1]) as f64 }
            },
        }
    }

    0.0
}

#[test]
fn test_find_median_sorted_arrays() {
    // assert_eq!(find_median_sorted_arrays(vec![1, 2, 3], vec![4, 5]), 3.0);
    // assert_eq!(find_median_sorted_arrays(vec![1, 3], vec![2, 4]), 2.5);
    // assert_eq!(find_median_sorted_arrays(vec![2], vec![1, 4]), 2.0);
    // assert_eq!(find_median_sorted_arrays(vec![], vec![1, 4]), 2.5);
    // assert_eq!(find_median_sorted_arrays(vec![], vec![1]), 1.0);
    // assert_eq!(find_median_sorted_arrays(vec![2], vec![1, 3, 4]), 2.5);
    // assert_eq!(find_median_sorted_arrays(vec![3], vec![1, 2, 4]), 2.5);
    assert_eq!(find_median_sorted_arrays(vec![4], vec![1, 2, 3, 5]), 3.0);
}
