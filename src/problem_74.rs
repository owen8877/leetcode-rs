pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let m = matrix.len();
    if m == 0 {
        return false
    }
    let n = matrix[0].len();
    if n == 0 {
        return false
    }

    if target < matrix[0][0] || target > matrix[m-1][n-1] {
        return false
    }
    let mut rmin = 0;
    let mut rmax = m;
    while rmax > rmin+1 {
        let rmid = (rmin+rmax)/2;
        if target == matrix[rmid][0] {
            return true
        } else if target > matrix[rmid][0] {
            rmin = rmid;
        } else {
            rmax = rmid;
        }
    }

    let mut cmin = 0;
    let mut cmax = n;
    while cmax > cmin {
        let cmid = (cmin+cmax)/2;
        if target == matrix[rmin][cmid] {
            return true
        } else if target > matrix[rmin][cmid] {
            cmin = cmid + 1;
        } else {
            cmax = cmid;
        }
    }
    false
}

#[test]
fn test_search_matrix() {
    assert_eq!(search_matrix(vec![
        vec![-8,-6,-6,-4,-2,-1,-1,-1,0,1,2,4,6,7,7],
        vec![10,10,12,13,13,14,14,16,17,17,17,17,17,18,19],
        vec![22,24,26,28,29,31,32,34,34,34,34,36,38,39,39],
        vec![40,40,41,43,43,44,46,47,49,49,50,52,53,55,55],
        vec![56,57,59,61,62,64,65,65,66,67,68,68,69,70,71],
        vec![74,75,77,77,79,79,79,80,81,83,85,87,88,89,89],
        vec![91,93,94,96,97,98,99,99,100,100,102,104,105,107,107],
        vec![110,112,114,114,115,117,117,118,118,120,120,121,123,124,124],
        vec![127,127,129,131,133,134,134,135,137,139,139,140,142,143,144],
        vec![145,146,147,149,151,151,153,155,156,156,158,160,162,162,163],
    ],107), true);
}