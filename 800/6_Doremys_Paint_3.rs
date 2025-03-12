/*
problem link: https://codeforces.com/problemset/problem/1890/A
test input:
5
2
8 9
3
1 1 2
4
1 1 4 5
5
2 3 3 3 3
4
100000 100000 100000 100000

expected output:
Yes
Yes
No
No
Yes

*/

use std::io;
use std::collections::HashMap;
 
fn take_input() -> String {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("");
    return input.trim().to_string();
}

fn solution(arr: Vec<i32>) -> String {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for a in &arr {
        // let count: &mut i32 =  map.entry(*a).or_insert(0);
        // *count += 1;
        map.entry(*a).and_modify(|counter| *counter += 1).or_insert(1);
    }
    if map.len() > 2 {
        return String::from("No");
    } else if map.len() == 1 {
        return String::from("Yes");
    }
    let values: Vec<i32> = map.into_values().collect();
    if (values[0] - values[1]).abs() > 1 {
        return String::from("No");
    } else {
        return String::from("Yes");
    }
}
 
fn main() {
    let t: i32 = take_input().parse().expect("");
    for _ in 0..t {
        let _n: i32 = take_input().parse().expect("");
        let mut arr: Vec<i32> = Vec::new();
        for x in take_input().split(" ") {
            arr.push(x.parse().expect(""));
        }
        let ans: String = solution(arr);
        println!("{ans}");

    }
}