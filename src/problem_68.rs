pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
    use std::cmp::*;
    let max_width = max_width as usize;

    let mut groups = vec![];
    let mut length_counter = 0;
    let mut group_slice_begin = 0;
    for (i, word) in words.iter().enumerate() {
        let new_length = if length_counter == 0 { word.len() } else { length_counter + 1 + word.len() };
        if new_length <= max_width {
            length_counter = new_length;
        } else {
            groups.push((&words[group_slice_begin..i], length_counter));
            length_counter = word.len();
            group_slice_begin = i;
        }
    }
    groups.push((&words[group_slice_begin..], length_counter));

    groups.iter().enumerate().map(|(i, (slice, length))| {
        let full_justify = i < groups.len() - 1;
        let base = (max_width - *length) / max(slice.len() - 1, 1);
        let remainder = (max_width - *length) % max(slice.len() - 1, 1);
        let mut result = vec![' '; max_width];
        let mut ptr = 0;
        result[ptr..ptr+slice[0].len()].copy_from_slice(slice[0].chars().collect::<Vec<char>>().as_slice());
        for i in 1..slice.len() {
            ptr += slice[i-1].len() + if full_justify {
                base + 1 + if i <= remainder { 1 } else { 0 }
            } else { 1 };
            result[ptr..ptr+slice[i].len()].copy_from_slice(&slice[i].chars().collect::<Vec<char>>().as_slice());
        }
        result.into_iter().collect()
    }).collect()
}

#[test]
fn test_full_justify() {
    fn core(arr: &[&str], max_width: i32, result: &[&str]) {
        assert_eq!(full_justify(arr.iter().map(|s| s.to_string()).collect(), max_width), result.iter().map(|s| s.to_string()).collect::<Vec<String>>())
    }

    core(&["This", "is", "an", "example", "of", "text", "justification."], 16, &[
        "This    is    an",
        "example  of text",
        "justification.  "
    ]);
    core(&["What","must","be","acknowledgment","shall","be"], 16, &[
        "What   must   be",
        "acknowledgment  ",
        "shall be        "
    ]);
    core(&["Science","is","what","we","understand","well","enough","to","explain","to","a","computer.","Art","is","everything","else","we","do"], 20, &[
        "Science  is  what we",
        "understand      well",
        "enough to explain to",
        "a  computer.  Art is",
        "everything  else  we",
        "do                  "
    ]);
}