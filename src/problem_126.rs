// TODO: Needs Improvement
pub fn find_ladders_core(begin_word: String, end_word: String, word_list: Vec<String>) -> Option<Vec<Vec<String>>> {
    fn cmp(a: &String, b: &String) -> bool {
        let mut counter = 0;
        for (c, d) in a.chars().zip(b.chars()) {
            if c != d {
                counter += 1;
            }
            if counter > 1 {
                return false
            }
        }
        true
    }

    use std::collections::*;
    let n = word_list.len();
    let mut neighbours = vec![HashSet::new(); n];
    let end_id = (0..n).filter(|&i| word_list[i] == end_word).next()?;

    for i in 0..n {
        for j in i+1..n {
            if cmp(&word_list[i], &word_list[j]) {
                neighbours[i].insert(j);
                neighbours[j].insert(i);
            }
        }
    }

    let mut distance = vec![n+2; n];
    let mut visited = HashSet::new();
    let mut active: HashSet<usize> = (0..n).filter(|&i| cmp(&word_list[i], &begin_word)).collect();
    let mut steps = 1;
    let mut found = false;
    while !active.is_empty() {
        for &node in &active {
            distance[node] = steps;
        }
        if active.contains(&end_id) {
            found = true;
            break
        }
        steps += 1;
        let mut frontier = HashSet::new();
        for &node in &active {
            for neighbour in &neighbours[node] {
                if !visited.contains(neighbour) && !active.contains(neighbour) {
                    frontier.insert(*neighbour);
                }
            }
        }
        for node in active {
            visited.insert(node);
        }
        active = frontier;
    }
    if !found {
        return None
    }

    fn core(counter: usize, steps: usize, node: usize, end_id: usize, neighbours: &Vec<HashSet<usize>>, hv: &mut Vec<usize>, result: &mut Vec<Vec<usize>>, distance: &Vec<usize>) {
        if counter == steps {
            if node == end_id {
                result.push(hv.clone());
            }
            return
        }
        for &neighbour in &neighbours[node] {
            if distance[neighbour] > counter {
                hv[counter] = neighbour;
                core(counter+1, steps, neighbour, end_id, neighbours, hv, result, distance);
            }
        }
    }

    let mut hv = vec![0; steps];
    let mut result = vec![];
    for node in (0..n).filter(|&i| cmp(&word_list[i], &begin_word)) {
        hv[0] = node;
        core(1, steps, node, end_id, &neighbours, &mut hv, &mut result, &distance);
    }
    Some(result.into_iter().map(|v| {
        let mut result = vec![begin_word.clone()];
        for i in v {
            result.push(word_list[i].clone());
        }
        result
    }).collect::<Vec<Vec<String>>>())
}

pub fn find_ladders(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<Vec<String>> {
    find_ladders_core(begin_word, end_word, word_list).unwrap_or(vec![])
}