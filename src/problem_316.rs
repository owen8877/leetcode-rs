pub fn remove_duplicate_letters(s: String) -> String {
    let mut remaining = vec![0; 26];
    let mut visited = vec![false; 26];
    let mut st = vec![];

    let ind = |c| (c as u8 - 'a' as u8) as usize;
    for c in s.chars() {
        remaining[ind(c)] += 1;
    }

    for c in s.chars() {
        remaining[ind(c)] -= 1;
        if visited[ind(c)] {
            continue;
        }
        while let Some(&l) = st.last() {
            if l > c && remaining[ind(l)] > 0 {
                st.pop();
                visited[ind(l)] = false;
            } else {
                break;
            }
        }
        st.push(c);
        visited[ind(c)] = true;
    }
    st.into_iter().collect()
}