pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
    let m = board.len();
    let n = board[0].len();

    let mut helper = vec![vec![0; n]; m];
    let ds = [[-1, 0], [-1, -1], [0, -1], [1, -1], [1, 0], [1, 1], [0, 1], [-1, 1]];
    for i in 0..m {
        for j in 0..n {
            let mut neighbours = 0;
            for d in &ds {
                let new_i = i as i32 + d[0];
                let new_j = j as i32 + d[1];
                if new_i >= 0 && new_j >= 0 && new_i < m as i32 && new_j < n as i32 {
                    neighbours += board[new_i as usize][new_j as usize];
                }
            }
            helper[i][j] = if board[i][j] == 1 {
                if 2 <= neighbours && neighbours <= 3 {
                    1
                } else {
                    0
                }
            } else {
                if neighbours == 3 {
                    1
                } else {
                    0
                }
            };
        }
    }
    for i in 0..m {
        for j in 0..n {
            board[i][j] = helper[i][j];
        }
    }
}