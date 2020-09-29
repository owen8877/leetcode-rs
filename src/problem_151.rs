pub fn reverse_words(s: String) -> String {
    let words: Vec<&str> = s.split(" ").collect();
    words.iter().filter(|x| x.len() > 0).rev().enumerate().fold("".to_string(), |acc, (i, x)| if i == 0 { x.to_string() } else { acc + " " + x })
}