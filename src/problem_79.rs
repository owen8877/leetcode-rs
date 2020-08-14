pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let m = board.len();
    if m == 0 {
        return false
    }
    let n = board[0].len();
    if n == 0 {
        return false
    }

    fn visit(i: usize, j : usize, m: usize, n: usize, board: &Vec<Vec<char>>, words: &[char], visited: &mut Vec<Vec<bool>>) -> bool {
        if words.len() == 0 {
            return true
        }
        if board[i][j] != words[0] {
            return false
        }
        if words.len() == 1 {
            return true
        }

        visited[i][j] = true;
        for d in [[-1, 0], [0, -1], [1, 0], [0, 1]].iter() {
            let new_i = i as i32 + d[0];
            let new_j = j as i32 + d[1];
            if new_i >= 0 && new_i < m as i32 && new_j >= 0 && new_j < n as i32 {
                let new_i = new_i as usize;
                let new_j = new_j as usize;
                if !visited[new_i][new_j] && visit(new_i, new_j, m, n, board, &words[1..], visited) {
                    return true
                }
            }
        }
        visited[i][j] = false;
        false
    }

    let words: Vec<char> = word.chars().collect();
    let mut visited = vec![vec![false; n]; m];
    for i in 0..m {
        for j in 0..n {
            if visit(i, j, m, n, &board, words.as_slice(), &mut visited) {
                return true
            }
        }
    }
    false
}