#[derive(Clone, Copy)]
enum State {
    Unvisited,
    Temporary,
    Perminant,
}

pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    use State::*;

    let n = num_courses as usize;
    if n == 1 {
        return true;
    } else if n == 2 {
        return prerequisites.len() < 2;
    } else if prerequisites.len() < 2 {
        return true;
    }
    let mut states = vec![Unvisited; n];
    let mut neighbours = vec![vec![]; n];
    for p in prerequisites {
        neighbours[p[0] as usize].push(p[1] as usize);
    }

    fn visit(i: usize, states: &mut Vec<State>, neighbours: &Vec<Vec<usize>>) -> bool {
        match states[i] {
            Unvisited => {
                states[i] = Temporary;
                for &j in &neighbours[i] {
                    if !visit(j, states, neighbours) {
                        return false;
                    }
                }
                states[i] = Perminant;
                true
            }
            Temporary => false,
            Perminant => true,
        }
    }

    for i in 0..n {
        match states[i] {
            Unvisited => {
                if !visit(i, &mut states, &neighbours) {
                    return false;
                }
            }
            _ => continue,
        }
    }
    true
}