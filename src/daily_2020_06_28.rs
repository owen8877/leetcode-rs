// Needs improvement
pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
    use std::collections::HashMap;

    let n = tickets.len();
    if n == 1 {
        return tickets[0].clone()
    }

    let mut map = HashMap::<String, Vec<(String, bool)>>::new();
    for v in tickets {
        match map.get_mut(&v[0]) {
            None => {
                map.insert(v[0].clone(), vec![(v[1].clone(), true)]);
            },
            Some(list) => {
                list.push((v[1].clone(), true));
            },
        }
    }

    for v in map.values_mut() {
        v.sort_by(|r, l| r.0.cmp(&l.0));
    }

    struct Map {
        map: HashMap::<String, Vec<(String, bool)>>,
    }

    impl Map {
        fn get_next_moves(&self, key: &String) -> Option<Vec<(&String, usize)>> {
            match self.map.get(key) {
                None => None,
                Some(list) => Some(list.iter().enumerate().filter(|i| (i.1).1).map(|i| (&(i.1).0, i.0)).collect())
            }
        }

        fn apply(&mut self, key: &String, operation: usize) {
            self.map.get_mut(key).unwrap()[operation].1 = false;
        }

        fn revert(&mut self, key: &String, operation: usize) {
            self.map.get_mut(key).unwrap()[operation].1 = true;
        }

        fn core(&mut self, depth: usize, prefix: Vec<String>) -> Option<Vec<String>> {
            if depth == 0 {
                return Some(prefix)
            }
            let key = prefix.last().unwrap();
            let next_moves = self.get_next_moves(key);
            if next_moves.is_none() {
                return None
            }
            let mut next_moves_copied = vec![];
            for a in next_moves.unwrap() {
                next_moves_copied.push((a.0.clone(), a.1));
            }
            for next in next_moves_copied {
                let mut p = prefix.clone();
                p.push(next.0);
                self.apply(key, next.1);
                match self.core(depth-1, p) {
                    None => {
                        self.revert(key, next.1);
                        continue;
                    },
                    Some(a) => return Some(a),
                }
            }
            None
        }
    }

    let mut map = Map { map };
    map.core(n, vec!["JFK".to_string()]).unwrap()
}

#[test]
fn test_find_itinerary() {
    assert_eq!(find_itinerary(vec![["JFK","SFO"],["JFK","ATL"],["SFO","ATL"],["ATL","JFK"],["ATL","SFO"]].into_iter().map(|arr| vec![arr[0].to_string(), arr[1].to_string()]).collect()),
               vec!["JFK".to_string(),
                    "ATL".to_string(),
                    "JFK".to_string(),
                    "SFO".to_string(),
                    "ATL".to_string(),
                    "SFO".to_string()]);
    assert_eq!(find_itinerary(vec![["JFK","KUL"],["JFK","NRT"],["NRT","JFK"]].into_iter().map(|arr| vec![arr[0].to_string(), arr[1].to_string()]).collect()),
               vec!["JFK".to_string(),
                    "NRT".to_string(),
                    "JFK".to_string(),
                    "KUL".to_string()]);
}