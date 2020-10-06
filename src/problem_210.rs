pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::HashSet;

    let n = num_courses as usize;
    if n == 0 {
        return vec![]
    } else if n == 1 {
        return vec![0]
    }

    let mut nodes_without_out = HashSet::<usize>::new();
    let mut out_edges = vec![HashSet::<usize>::new(); n];
    let mut in_edges = vec![HashSet::<usize>::new(); n];

    for p in prerequisites {
        let i = p[0] as usize;
        let j = p[1] as usize;
        out_edges[i].insert(j);
        in_edges[j].insert(i);
    }

    for i in 0..n {
        if out_edges[i].len() == 0 {
            nodes_without_out.insert(i);
        }
    }

    let mut result = vec![];
    while nodes_without_out.len() > 0 {
        let node = nodes_without_out.iter().next().unwrap().clone();
        result.push(node as i32);
        nodes_without_out.take(&node);

        for parent in in_edges[node].iter() {
            out_edges[*parent].take(&node);
            if out_edges[*parent].len() == 0 {
                nodes_without_out.insert(*parent);
            }
        }
    }

    for i in 0..n {
        if out_edges[i].len() != 0 {
            return vec![]
        }
    }
    result
}