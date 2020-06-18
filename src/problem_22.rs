pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let v1 = vec!["()".to_string()];
    let v2 = vec!["()()".to_string(), "(())".to_string()];
    match n {
        0 => vec!["".to_string()],
        1 => v1,
        2 => v2,
        _ => {
            let mut comb = Vec::<Vec<String>>::new();
            comb.push(v1);
            comb.push(v2);
            for i in 3..=n {
                let mut result = Vec::<String>::new();
                let m = i as usize-2;

                for s in comb[m].iter() {
                    result.push("()".to_string() + s);
                    result.push("(".to_string() + s + ")");
                }

                for s in comb[m-1].iter() {
                    result.push("(".to_string() + s + ")()");
                }

                for j in 1..m {
                    for s1 in comb[j-1].iter() {
                        for s2 in comb[m-j].iter() {
                            result.push("(".to_string()+s1+")"+s2);
                        }
                    }
                }

                comb.push(result);
            }
            comb[n as usize-1].to_vec()
        }
    }
}

#[test]
fn test_generate_parenthesis() {
    // assert_eq!(generate_parenthesis(3), vec![
    //     "((()))",
    //     "(()())",
    //     "(())()",
    //     "()(())",
    //     "()()()",
    // ]);
    assert_eq!(generate_parenthesis(4), vec!["(((())))","((()()))","((())())","((()))()","(()(()))","(()()())","(()())()","(())(())","(())()()","()((()))","()(()())","()(())()","()()(())","()()()()"])
}