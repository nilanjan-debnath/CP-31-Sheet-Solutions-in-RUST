/*
problem link: 
test input:
3
3 7
1 2 5
3 6
1 2 5
1 10
7

expected output:
4
3
7
 */

use std::io;

// function to take numeric input and return a vector of number
fn take_input() -> Vec<i32> {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("");
    
    let mut vec: Vec<i32> = Vec::new();
    for i in input.trim().split(" ") {
        let tmp: i32 = i.parse().expect("num error");
        vec.push(tmp);
    }
    return vec;
}

fn main() {
    let mut t: i32 = take_input()[0];
    while t > 0 {
        t -= 1;
        let tmp_arr: Vec<i32> = take_input();
        let n: usize = tmp_arr[0] as usize;
        let x: i32 = tmp_arr[1];
        let arr: Vec<i32> = take_input();
        
        let ans: i32 = min_gas_tank(n, x, arr);
        println!("{ans}");
    }
}

fn min_gas_tank(n: usize, x: i32, arr: Vec<i32>) -> i32 {
    if x < arr[0] {
        return x;
    }
    let mut ans: i32 = arr[0];
    let mut dif: i32;
    for i in 1..n {
        if x < arr[i] {
            dif = (x - arr[n-1])*2;
        } else {
            dif = arr[i] - arr[i-1];
        }
        if dif > ans {
            ans = dif;
        }
        if x < arr[i] {
            return ans;
        }
    }
    dif = (x - arr[n-1])*2;
    if dif > ans {
        ans = dif;
    }
    return ans;
}
