pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    use std::cmp::*;
    if intervals.is_empty() {
        return vec![new_interval]
    }
    let start = new_interval[0];
    let end = new_interval[1];
    let n = intervals.len();
    if start > intervals[n-1][1] {
        let mut result = intervals.clone();
        result.push(new_interval);
        return result
    }

    fn locate_new_interval(intervals: &Vec<Vec<i32>>, start: i32, end: i32, n: usize) -> (bool, usize) {
        if start < intervals[0][0] {
            (true, 0)
        } else {
            for i in 0..n-1 {
                if intervals[i][0] <= start && start <= intervals[i][1] {
                    return (false, i)
                }
                if intervals[i][1] < start && start < intervals[i+1][0] {
                    return (true, i+1)
                }
            }
            (false, n-1)
        }
    }

    let (is_out_of_intervals, neighbour_index) = locate_new_interval(&intervals, start, end, n);

    let mut result = vec![];
    for i in 0..neighbour_index {
        result.push(intervals[i].clone());
    }
    let mut merged_start = if is_out_of_intervals { start } else { intervals[neighbour_index][0] };
    let mut merged_end = end;
    let mut stopping_index = neighbour_index;
    let mut merged = false;
    while stopping_index < n {
        if merged_end < intervals[stopping_index][0] {
            result.push(vec![merged_start, merged_end]);
            merged = true;
            break
        }
        merged_end = max(merged_end, intervals[stopping_index][1]);

        stopping_index += 1;
    }
    if !merged {
        result.push(vec![merged_start, merged_end]);
    } else {
        for j in stopping_index..n {
            result.push(intervals[j].clone());
        }
    }

    result
}

#[test]
fn test_insert() {
    assert_eq!(insert(vec![vec![1, 5]], vec![0, 0]), vec![vec![0, 0], vec![1, 5]]);
}