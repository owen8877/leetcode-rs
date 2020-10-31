pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    if n == 1 {
        return vec![0];
    } else if n == 2 {
        return vec![0, 1];
    }

    use std::collections::*;
    let n = n as usize;
    let mut neighbours = vec![HashSet::new(); n];
    for edge in &edges {
        neighbours[edge[0] as usize].insert(edge[1]);
        neighbours[edge[1] as usize].insert(edge[0]);
    }

    let mut leaves = vec![];
    for i in 0..n {
        if neighbours[i].len() == 1 {
            leaves.push(i as i32);
        }
    }

    let mut remaining = n;
    while remaining > 2 {
        remaining -= leaves.len();

        let mut new_leaves = vec![];
        for &leaf in &leaves {
            let neighbour = *neighbours[leaf as usize].iter().next().unwrap();
            neighbours[leaf as usize].clear();
            neighbours[neighbour as usize].remove(&leaf);
            if neighbours[neighbour as usize].len() == 1 {
                new_leaves.push(neighbour);
            }
        }
        leaves = new_leaves;
    }
    leaves
}