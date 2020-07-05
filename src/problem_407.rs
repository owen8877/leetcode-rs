// Reference: https://leetcode.com/problems/trapping-rain-water-ii/discuss/688184/C%2B%2B-Solution-Using-Min-Heap
// The idea is to maintain the wall around the possible raining area
pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
    let m = height_map.len();
    if m <= 2 {
        return 0
    }
    let n = height_map[0].len();
    if n <= 2 {
        return 0
    }

    use std::cmp::max;
    use std::cmp::Ordering;
    use std::collections::binary_heap::BinaryHeap;

    #[derive(Copy, Clone, Eq, PartialEq)]
    struct State {
        height: i32,
        coor: (usize, usize),
    }

    impl Ord for State {
        fn cmp(&self, other: &State) -> Ordering {
            other.height.cmp(&self.height).then_with(|| self.coor.cmp(&other.coor))
        }
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &State) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut visited = vec![];
    let mut queue = BinaryHeap::new();
    for i in 0..m {
        visited.push(vec![false; n]);
        visited[i][0] = true;
        visited[i][n-1] = true;
        queue.push(State { height: height_map[i][0], coor: (i, 0) });
        queue.push(State { height: height_map[i][n-1], coor: (i, n-1) });
    }
    for j in 1..n-1 {
        visited[0][j] = true;
        visited[m-1][j] = true;
        queue.push(State { height: height_map[0][j], coor: (0, j) });
        queue.push(State { height: height_map[m-1][j], coor: (m-1, j) });
    }

    let mut min_height = 0;
    let mut sum = 0;
    while !queue.is_empty() {
        let node = queue.pop().unwrap();
        min_height = max(min_height, node.height);
        for d in [[0, -1], [-1, 0], [0, 1], [1, 0]].iter() {
            let new = (node.coor.0 as isize + d[0], node.coor.1 as isize + d[1]);
            if new.0 < 0 || new.1 < 0 || new.0 >= m as isize || new.1 >= n as isize {
                continue
            }
            let new = (new.0 as usize, new.1 as usize);
            if visited[new.0][new.1] {
                continue
            }
            queue.push(State { height: height_map[new.0][new.1], coor: (new.0, new.1) });
            visited[new.0][new.1] = true;
            if min_height >= height_map[new.0][new.1] {
                sum += min_height - height_map[new.0][new.1];
            }
        }
    }

    sum
}

#[test]
fn test_trap_rain_water() {
    assert_eq!(trap_rain_water(vec![
        vec![9,9,9,9,9],
        vec![9,2,1,2,9],
        vec![9,2,8,2,9],
        vec![9,2,3,2,9],
        vec![9,9,9,9,9],
    ]), 57);
    assert_eq!(trap_rain_water(vec![
        vec![1,4,3,1,3,2],
        vec![3,2,1,3,2,4],
        vec![2,3,3,2,3,1]
    ]), 4);
    assert_eq!(trap_rain_water(vec![
        vec![12,13,1,12],
        vec![13,4,13,12],
        vec![13,8,10,12],
        vec![12,13,12,12],
        vec![13,13,13,13],
    ]), 14);
    assert_eq!(trap_rain_water(vec![
        vec![2,3,4],
        vec![5,6,7],
        vec![8,9,10],
        vec![11,12,13],
        vec![14,15,16],
    ]), 0)
}