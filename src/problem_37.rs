pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    let mut b = vec![];
    for _ in 0..9 {
        b.push(vec![None; 9]);
    }
    let mut row_occ = vec![0 as u16; 9];
    let mut col_occ = vec![0 as u16; 9];
    let mut blo_occ = vec![0 as u16; 9];

    fn coor2blo(i: usize, j: usize) -> usize {
        j / 3 + (i / 3) * 3
    }

    for i in 0..9 {
        for j in 0..9 {
            if board[i][j] != '.' {
                let x = board[i][j] as u16 - '0' as u16;
                b[i][j] = Some(x);
                row_occ[i] |= 1 << x;
                col_occ[j] |= 1 << x;
                blo_occ[coor2blo(i, j)] |= 1 << x;
            }
        }
    }

    fn core(b: &mut Vec<Vec<Option<u16>>>, row_occ: &mut Vec<u16>, col_occ: &mut Vec<u16>, blo_occ: &mut Vec<u16>, hint: (usize, usize)) -> bool {
        // First search which position is vacant
        let mut i = hint.0;
        let mut j = hint.1;
        loop {
            if b[i][j].is_none() {
                break
            }
            j += 1;
            if j == 9 {
                j = 0;
                i += 1;
            }
            if i == 9 {
                return true
            }
        }

        for x in 1..=9 {
            // Check if row, col and blo allows
            if row_occ[i] & 1 << x != 0 || col_occ[j] & 1 << x != 0 || blo_occ[coor2blo(i, j)] & 1 << x != 0 {
                continue
            }

            // Then we try to fill x in this box
            b[i][j] = Some(x);
            row_occ[i] |= 1 << x;
            col_occ[j] |= 1 << x;
            blo_occ[coor2blo(i, j)] |= 1 << x;

            if core(b, row_occ, col_occ, blo_occ, (i, j)) {
                return true
            } else {
                b[i][j] = None;
                row_occ[i] &= !(1 << x);
                col_occ[j] &= !(1 << x);
                blo_occ[coor2blo(i, j)] &= !(1 << x);
            }
        }

        false
    }

    core(&mut b, &mut row_occ, &mut col_occ, &mut blo_occ, (0, 0));

    for i in 0..9 {
        for j in 0..9 {
            board[i][j] = ('0' as u8 + b[i][j].unwrap() as u8) as char;
        }
    }
}

#[test]
fn test_is_valid_sudoku() {
    let mut board = vec![
        vec!['5','3','.','.','7','.','.','.','.'],
        vec!['6','.','.','1','9','5','.','.','.'],
        vec!['.','9','8','.','.','.','.','6','.'],
        vec!['8','.','.','.','6','.','.','.','3'],
        vec!['4','.','.','8','.','3','.','.','1'],
        vec!['7','.','.','.','2','.','.','.','6'],
        vec!['.','6','.','.','.','.','2','8','.'],
        vec!['.','.','.','4','1','9','.','.','5'],
        vec!['.','.','.','.','8','.','.','7','9'],
    ];
    solve_sudoku(&mut board);
    for line in board {
        println!("{}", line.into_iter().collect::<String>());
    }
}