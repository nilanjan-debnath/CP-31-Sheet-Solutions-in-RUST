/*
problem link: https://codeforces.com/problemset/problem/1896/A
test input:
6
3
1 2 3
5
1 3 2 5 4
5
5 4 3 2 1
3
3 1 2
4
2 3 1 4
5
5 1 2 3 4

expected output:
YES
YES
NO
NO
NO
NO

*/

use std::io;
 
fn take_input() -> String {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("");
    return input.trim().to_string();
}
 
fn main() {
    let t: i32 = take_input().parse().expect("");
    for _ in 0..t {
        let _n: i32 = take_input().parse().expect("");
        let mut arr: Vec<i32> = Vec::new();
        for x in take_input().split(" ") {
            arr.push(x.parse().expect(""));
        }
        if arr[0] == 1 {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}