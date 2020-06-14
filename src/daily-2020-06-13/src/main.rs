pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
    if nums.len() == 0 {
        return vec![];
    }
    let mut nums = nums;
    nums.sort();
    let n = nums.len();

    let mut pointer: Vec<usize> = (0..n).collect();
    let mut longest = vec![1; n];

    let mut largest = 0;
    let mut largest_index = n as usize;

    for i in (0..n).rev() {
        for j in i+1..n {
            if nums[j] % nums[i] == 0 {
                if longest[j] + 1 > longest[i] {
                    longest[i] = longest[j] + 1;
                    pointer[i] = j;
                }
            }
        }
        if longest[i] > largest {
            largest = longest[i];
            largest_index = i;
        }
    }

    let mut result = Vec::with_capacity(largest);
    let mut p = largest_index;
    for _ in 0..largest {
        result.push(nums[p]);
        p = pointer[p];
    }
    result
}

#[test]
fn test_largest_divisible_subset() {
    assert_eq!(largest_divisible_subset(vec![1, 2, 3]), vec![1, 2]);
    assert_eq!(largest_divisible_subset(vec![1, 2, 4, 8]), vec![1, 2, 4, 8]);
    assert_eq!(largest_divisible_subset(vec![3, 16, 4, 8]), vec![4, 8, 16]);
    assert_eq!(largest_divisible_subset(vec![1,2,4,8,16,32,64,128,256,512,1024,2048,4096,8192,16384,32768,65536,131072,262144,524288,1048576,2097152,4194304,8388608,16777216,33554432,67108864,134217728,268435456,536870912,1073741824]), vec![1,2,4,8,16,32,64,128,256,512,1024,2048,4096,8192,16384,32768,65536,131072,262144,524288,1048576,2097152,4194304,8388608,16777216,33554432,67108864,134217728,268435456,536870912,1073741824]);
}