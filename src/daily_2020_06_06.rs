pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    use std::cmp::Ordering::*;
    if people.len() <= 1 {
        return people;
    }

    let n = people.len();
    let mut result = Vec::<Vec<i32>>::with_capacity(n);
    let mut sorted = people.clone();
    sorted.sort_by(|a, b| {
        match a[0].cmp(&b[0]) {
            Less => Greater,
            Greater => Less,
            Equal => a[1].cmp(&b[1]),
        }
    });

    for person in sorted {
        result.insert(person[1] as usize, person);
    }

    result
}

#[test]
fn test_reconstruct_queue() {
    let input = vec![[7,0], [4,4], [7,1], [5,0], [6,1], [5,2]];
    let output = vec![[5,0], [7,0], [5,2], [6,1], [4,4], [7,1]];
    let arr2vec = |b: Vec<[i32; 2]>| b.into_iter().map(|a: [i32; 2]| vec![a[0], a[1]]).collect();
    assert_eq!(reconstruct_queue(arr2vec(input)), arr2vec(output));
}