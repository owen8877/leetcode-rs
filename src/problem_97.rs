pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
    let s1: Vec<char> = s1.chars().collect();
    let s2: Vec<char> = s2.chars().collect();
    let s3: Vec<char> = s3.chars().collect();
    let n1 = s1.len();
    let n2 = s2.len();

    if n1 + n2 != s3.len() {
        return false
    }

    let mut partial_interleaving = vec![vec![None; n2+1]; n1+1];
    partial_interleaving[0][0] = Some(true);

    let mut match_partial_s1 = true;
    for i in 0..n1 {
        if match_partial_s1 && s1[i] != s3[i] {
            match_partial_s1 = false;
        }
        partial_interleaving[i+1][0] = Some(match_partial_s1);
    }

    let mut match_partial_s2 = true;
    for j in 0..n2 {
        if match_partial_s2 && s2[j] != s3[j] {
            match_partial_s2 = false;
        }
        partial_interleaving[0][j+1] = Some(match_partial_s2);
    }

    fn core(i: usize, j: usize, s1: &[char], s2: &[char], s3: &[char], partial_interleaving: &mut Vec<Vec<Option<bool>>>) -> bool {
        match partial_interleaving[i][j] {
            Some(b) => b,
            None => {
                let b = (s1[i-1] == s3[i+j-1] && core(i-1, j, s1, s2, s3, partial_interleaving)) || (s2[j-1] == s3[i+j-1] && core(i, j-1, s1, s2, s3, partial_interleaving));
                partial_interleaving[i][j] = Some(b);
                b
            }
        }
    }

    core(n1, n2, s1.as_slice(), s2.as_slice(), s3.as_slice(), &mut partial_interleaving)
}