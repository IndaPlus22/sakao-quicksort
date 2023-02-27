use std::io::{stdin, Read};
use rand::Rng;

fn main() {
    // let mut arr = input();
    let mut arr = test();

    q_sort(&mut arr[..]);

    // output
    for _v in arr {
        print!("{} ", _v);
    }
    println!("");
}

#[test]
fn tester() {
    let mut arr = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        arr.push(rng.gen());
    }
    let mut right_arr: Vec<i32> = arr.clone();
    right_arr.sort_unstable();
    q_sort(&mut arr[..]);

    assert_eq!(right_arr, arr);
    // for i in 0..arr.len() {
    //     assert_eq!(right_arr[i], arr[i]);
    // }
}

fn q_sort(arr: &mut [i32]) {
    // if arr.len() < 101 {
    //     ins_sort(arr);
    //     return;
    // }

    if arr.len() > 0 {
        let high = arr.len();
        let pivot_i: usize = partition(&mut arr[..high]) as usize;
        q_sort(&mut arr[..(pivot_i)]);
        q_sort(&mut arr[(pivot_i + 1)..high]);
    }
}

fn partition(arr: &mut [i32]) -> i32 {
    let pivot = arr[pivot(arr)];

    let mut j: i32 = 0;
    for i in 0..arr.len() {
        if arr[i] < pivot {
            let tmp = arr[i];
            arr[i] = arr[j as usize];
            arr[j as usize] = tmp;
            j += 1;
        }
    }
    let tmp = arr[j as usize];
    arr[j as usize] = arr[arr.len() - 1];
    arr[arr.len() - 1] = tmp;
    return j;
}

fn pivot(arr: &[i32]) -> usize {
    let a = 0;
    let b = arr.len() / 2;
    let c = arr.len() - 1;

    // a b c
    // a c b
    // c a b

    // b a c
    // b c a
    // c b a
    if arr[a] < arr[b] {
        if arr[b] < arr[c] {
            b
        } else if arr[c] < arr[a] {
            c
        } else {
            a
        }
    } else {
        if arr[a] < arr[c] {
            a
        } else if arr[c] < arr[a] {
            c
        } else {
            b
        }
    }
}

// if less than 100 just insertion sort
fn ins_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let old = arr[i];
        let mut j: i32 = i as i32 - 1;
        while j >= 0 && arr[j as usize] > old {
            arr[(j + 1) as usize] = arr[j as usize];
            j -= 1;
        }
        arr[(j + 1) as usize] = old;
    }
}

// thank you jblomlof for letting me know how this kattis deals with input
fn input() -> Vec<i32> {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input);

    let mut vals: Vec<i32> = input
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();

    vals
}

fn test() -> Vec<i32> {
    let input = std::fs::read_to_string("src/tests.txt").unwrap();

    let mut vals: Vec<i32> = input
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();

    vals
}
