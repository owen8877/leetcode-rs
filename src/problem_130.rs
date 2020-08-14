pub fn solve(board: &mut Vec<Vec<char>>) {
    let m = board.len();
    if m <= 2 {
        return
    }
    let n = board[0].len();
    if n <= 2 {
        return
    }

    let mut result = Vec::<Vec<char>>::new();
    for _ in 0..m {
        result.push(vec!['X'; n]);
    }

    fn valid(coord: &(usize, usize), direction: &(isize, isize), range: &(usize, usize)) -> bool {
        match direction {
            (0, 1)  => coord.1 + 1 < range.1,
            (0, -1) => coord.1 > 0,
            (1, 0)  => coord.0 + 1 < range.0,
            (-1, 0) => coord.0 > 0,
            _ => panic!(""),
        }
    }

    fn plant(board: &Vec<Vec<char>>, result: &mut Vec<Vec<char>>, coord: &(usize, usize), range: &(usize, usize)) {
        let (i, j) = coord;
        result[*i][*j] = 'O';

        for direction in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if valid(coord, &direction, range) {
                let new_i = (coord.0 as isize+direction.0) as usize;
                let new_j = (coord.1 as isize+direction.1) as usize;
                if result[new_i][new_j] == 'O' || board[new_i][new_j] == 'X' {
                    continue
                }
                plant(board, result, &(new_i, new_j), range);
            }
        }

    }

    for i in 1..m-1 {
        if board[i][0] == 'O' {
            plant(&board, &mut result, &(i, 0), &(m, n));
        }
        if board[i][n-1] == 'O' {
            plant(&board, &mut result, &(i, n-1), &(m, n));
        }
    }

    for j in 0..n {
        if board[0][j] == 'O' {
            plant(&board, &mut result, &(0, j), &(m, n));
        }
        if board[m-1][j] == 'O' {
            plant(&board, &mut result, &(m-1, j), &(m, n));
        }
    }

    for i in 0..m {
        for j in 0..n {
            board[i][j] = result[i][j];
        }
    }
}

#[test]
fn test_solve() {
    let mut board = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'O', 'X'],
        vec!['X', 'X', 'O', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ];
    solve(&mut board);
    assert_eq!(board, vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ]);
    board = vec![
        vec!['O', 'X', 'O'],
        vec!['X', 'O', 'X'],
        vec!['O', 'X', 'O'],
    ];
    solve(&mut board);
    assert_eq!(board, vec![
        vec!['O', 'X', 'O'],
        vec!['X', 'X', 'X'],
        vec!['O', 'X', 'O'],
    ]);
}