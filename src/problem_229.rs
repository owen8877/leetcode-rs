pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    if n < 2 {
        return nums.clone();
    }
    if n == 2 {
        return if nums[0] == nums[1] { vec![nums[0]] } else { nums.clone() };
    }

    let mut ele1 = None;
    let mut ele2 = None;
    let mut cnt1 = 0;
    let mut cnt2 = 0;
    for (i, &num) in nums.iter().enumerate() {
        if let Some(ele1) = ele1 {
            if num == ele1 {
                cnt1 += 1;
            } else {
                if let Some(ele2) = ele2 {
                    if num == ele2 {
                        cnt2 += 1;
                    } else {
                        cnt1 -= 1;
                        cnt2 -= 1;
                    }
                } else {
                    ele2 = Some(num);
                    cnt2 = 1;
                }
            }
        } else {
            ele1 = Some(num);
            cnt1 = 1;
        }
        if cnt1 == 0 && i + 1 < n && ele1.map_or(true, |e| e != nums[i + 1]) && ele2.map_or(true, |e| e != nums[i + 1]) {
            ele1 = Some(nums[i + 1]);
        } else if cnt2 == 0 && i + 1 < n && ele1.map_or(true, |e| e != nums[i + 1]) && ele2.map_or(true, |e| e != nums[i + 1]) {
            ele2 = Some(nums[i + 1]);
        }
    }

    if let Some(ele2) = ele2 {
        cnt1 = 0;
        cnt2 = 0;
        for num in nums {
            if num == ele1.unwrap() {
                cnt1 += 1;
            }
            if num == ele2 {
                cnt2 += 1;
            }
        }
        let mut result = vec![];
        if cnt1 * 3 > n {
            result.push(ele1.unwrap());
        }
        if cnt2 * 3 > n {
            result.push(ele2);
        }
        result
    } else {
        cnt1 = 0;
        for num in nums {
            if num == ele1.unwrap() {
                cnt1 += 1;
            }
        }
        if cnt1 * 3 > n {
            vec![ele1.unwrap()]
        } else {
            vec![]
        }
    }
}