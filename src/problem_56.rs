pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if intervals.len() == 0 {
        return vec![]
    }
    use std::cmp::*;
    let mut intervals = intervals;
    intervals.sort_by(|x, y| x[0].cmp(&y[0]));

    let mut merged_intervals = vec![];
    let mut start = intervals[0][0];
    let mut end = intervals[0][1];
    for i in 1..intervals.len() {
        let next_start = intervals[i][0];
        let next_end = intervals[i][1];
        if next_start > end {
            merged_intervals.push(vec![start, end]);
            start = next_start;
            end = next_end;
        } else {
            end = max(end, next_end);
        }
    }
    merged_intervals.push(vec![start, end]);

    merged_intervals
}