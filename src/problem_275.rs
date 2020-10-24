pub fn h_index(citations: Vec<i32>) -> i32 {
    if citations.len() == 0 {
        return 0;
    }
    if citations.iter().fold(true, |acc, x| acc && x == &0) {
        return 0;
    }

    let mut min_index = 0;
    let mut max_index = citations.len() - 1;

    while max_index > min_index {
        let index = (min_index + max_index) / 2;
        let c = citations[index];
        if c as usize >= citations.len() - index {
            max_index = index;
        } else {
            min_index = index + 1;
        }
    }

    (citations.len() - min_index) as i32
}