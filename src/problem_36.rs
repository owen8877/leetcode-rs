pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    for i in 0..9 {
        // row
        let mut occ = vec![0; 9];
        for j in 0..9 {
            match board[i][j] {
                '.' => continue,
                c => {
                    let n = (c as u8 - '1' as u8) as usize;
                    if occ[n] >= 1 {
                        return false
                    } else {
                        occ[n] = 1;
                    }
                }
            }
        }
    }
    for i in 0..9 {
        // column
        let mut occ = vec![0; 9];
        for j in 0..9 {
            match board[j][i] {
                '.' => continue,
                c => {
                    let n = (c as u8 - '1' as u8) as usize;
                    if occ[n] >= 1 {
                        return false
                    } else {
                        occ[n] = 1;
                    }
                }
            }
        }
    }
    for i in 0..3 {
        for j in 0..3 {
            // square
            let mut occ = vec![0; 9];
            for k in 0..3 {
                for l in 0..3 {
                    match board[3*i+k][3*j+l] {
                        '.' => continue,
                        c => {
                            let n = (c as u8 - '1' as u8) as usize;
                            if occ[n] >= 1 {
                                return false
                            } else {
                                occ[n] = 1;
                            }
                        }
                    }
                }
            }
        }
    }
    true
}

#[test]
fn test_is_valid_sudoku() {
    assert_eq!(is_valid_sudoku(vec![
        vec!['8','3','.','.','7','.','.','.','.'],
        vec!['6','.','.','1','9','5','.','.','.'],
        vec!['.','9','8','.','.','.','.','6','.'],
        vec!['8','.','.','.','6','.','.','.','3'],
        vec!['4','.','.','8','.','3','.','.','1'],
        vec!['7','.','.','.','2','.','.','.','6'],
        vec!['.','6','.','.','.','.','2','8','.'],
        vec!['.','.','.','4','1','9','.','.','5'],
        vec!['.','.','.','.','8','.','.','7','9'],
    ]), false);
}