use rand::Rng;
use std::{io::{stdin, Read}, cmp::Ordering};


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
        let eh: i32 = rng.gen();
        arr.push(eh % 30);
    }
    let mut right_arr: Vec<i32> = arr.clone();
    right_arr.sort_unstable();

    // println!("arrf: {:?}", arr);

    q_sort(&mut arr[..]);

    assert_eq!(right_arr, arr);
}

fn q_sort(arr: &mut [i32]) {
    // if arr.len() < 101 {
    //     ins_sort(arr);
    //     return;
    // }

    let high = arr.len();
    if high > 0 {
        let pivot_low: usize = partition(&mut arr[..]).0;
        let pivot_high: usize = partition(&mut arr[..]).1;

        q_sort(&mut arr[..(pivot_low)]);
        q_sort(&mut arr[(pivot_high + 1)..high]);
    }
}

fn partition(arr: &mut [i32]) -> (usize, usize) {
    let mut pivot_i = pivot(arr);
    let pivot_el = arr[pivot_i];
    println!("pivot is {}", pivot_i);

    // start with up to pivot index
    let mut low = 0;
    let mut high = 0;
    let mut dupes = 0;
    while high < pivot_i {
        match arr[high].cmp(&pivot_el) {
            Ordering::Less => { // if arr[j] < pivot el
                let tmp = arr[low];
                arr[low] = arr[high];
                arr[high] = tmp;

                high += 1;
                println!("tl: {:?}, pi: {}", arr, pivot_i);
            },
            Ordering::Greater => {
                println!("tg: {:?}, pi: {}", arr, pivot_i);
            },
            _ => {
                // pivot_i -= 1;
                dupes += 1;
                println!("te: {:?}, pi: {}", arr, pivot_i);

            }
        }
        low += 1;
    }

    let tmp = arr[high];
    arr[high] = arr[pivot_i];
    arr[pivot_i-1] = tmp;

    println!("partitioned: {:?}, pi: {}", arr, pivot_i);
    (high, high + dupes)
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

    // println!("beg: {:?}", vals);
    vals
}
