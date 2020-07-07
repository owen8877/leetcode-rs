pub fn climb_stairs(n: i32) -> i32 {
    if n == 1 {
        return 1
    }
    let n = n as usize;
    let mut ways_to_climb = vec![1; n];
    ways_to_climb[1] = 2;
    for i in 2..n {
        ways_to_climb[i] = ways_to_climb[i-1] + ways_to_climb[i-2];
    }
    ways_to_climb[n-1]
}