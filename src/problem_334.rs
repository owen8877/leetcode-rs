pub fn increasing_triplet(nums: Vec<i32>) -> bool {
    if nums.len() < 3 {
        return false;
    }

    let mut a = None;
    let mut b = None;

    let mut aa = None;
    let mut bb = None;
    for num in nums {
        match (a, b) {
            (None, None) => {
                a = Some(num);
                b = None;
            }
            (Some(na), None) => {
                if num < na {
                    a = Some(num);
                    b = None;
                } else if num > na {
                    a = Some(na);
                    b = Some(num);
                } else {
                    a = Some(na);
                    b = None;
                }
            }
            (Some(na), Some(nb)) => {
                if num < na {
                    a = Some(num);
                    b = None;
                } else if num < nb && num > na {
                    a = Some(na);
                    b = Some(num);
                } else if num > nb {
                    return true;
                } else {
                    a = Some(na);
                    b = Some(nb);
                }
            }
            _ => {}
        }

        if let Some(&na) = a.as_ref() {
            if let Some(&nb) = b.as_ref() {
                if aa.is_some() & bb.is_some() {
                    if aa.unwrap() >= na && bb.unwrap() > nb {
                        aa = Some(na);
                        bb = Some(nb);
                    }
                } else {
                    aa = Some(na);
                    bb = Some(nb);
                }
            }
        }

        if aa.is_some() & bb.is_some() {
            if bb.unwrap() < num {
                return true;
            }
        }
    }
    false
}