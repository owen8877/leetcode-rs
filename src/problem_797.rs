pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = graph.len();
    let mut path_cache = vec![None; n];
    
    fn build_paths(i: usize, graph: &Vec<Vec<i32>>, path_cache: &mut Vec<Option<Vec<Vec<i32>>>>) {
        let n = graph.len();
        if i == n-1 {
            path_cache[i] = Some(vec![vec![i as i32]]);
            return
        }
        let mut paths = vec![];
        for j in graph[i].iter() {
            let j = *j as usize;
            if path_cache[j].is_none() {
                build_paths(j, graph, path_cache);
            }
            let jpaths = path_cache[j].as_ref().unwrap();
            for jpath in jpaths {
                let mut jp = jpath.clone();
                // Insert from head (Faster)
                jp.insert(0, i as i32);
                // Insert from tail (Slower)
                // jp.push(i as i32);
                paths.push(jp);
            }
        }
        path_cache[i] = Some(paths);
    }
    
    build_paths(0, &graph, &mut path_cache);
    // Insert from head (Faster)
    path_cache.swap_remove(0).unwrap()
    // Insert from tail (Slower)
    // path_cache.swap_remove(0).unwrap().into_iter().map(|mut x| {
    //     x.reverse();
    //     x
    // }).collect()
}
