pub fn length_of_last_word(s: String) -> i32 {
    if s.len() == 0 {
        return 0
    }
    let mut alphabet_seen = false;
    let mut counter = 0;
    for c in s.chars().rev() {
        match c {
            ' ' => {
                if alphabet_seen {
                    return counter
                }
            },
            _ => {
                alphabet_seen = true;
                counter += 1;
            }
        }
    }
    counter
}