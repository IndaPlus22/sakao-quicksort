use rand::Rng;
use std::io::{stdin, Read};

fn main() {
    let mut arr = input();
    // let mut arr = test();

    // println!("arrf: {:?}", arr);
    qsort(&mut arr[..]);

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
    for _ in 0..1000000 {
        let eh: i32 = rng.gen();
        arr.push(eh % 30);
    }
    let mut right_arr: Vec<i32> = arr.clone();
    right_arr.sort_unstable();

    // println!("arrf: {:?}", arr);

    qsort(&mut arr[..]);

    assert_eq!(right_arr, arr);
}

fn qsort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    if arr.len() < 101 {
        ins_sort(arr);
        return;
    }

    if arr.len() > 0 {
        let pivot_i: usize = hpartition(&mut arr[..]);

        // println!("qarr: {:?}, pe: {}", arr, arr[pivot_i]);
        // println!(
        //     "p1: {:?}, p2: {:?}",
        //     &arr[..(pivot_i+1)],
        //     &arr[(pivot_i + 1)..]
        // );
        // println!("");

        qsort(&mut arr[..(pivot_i + 1)]);
        qsort(&mut arr[(pivot_i + 1)..]);
    }
}

// hoar
fn hpartition(arr: &mut [i32]) -> usize {
    fix_pivot(arr);
    let pivot_el = arr[0];
    // println!("bef par: {:?}, pe: {}", arr, pivot_el);

    // low pointer and high pointer
    let mut lp: isize = -1;
    let mut hp: isize = arr.len() as isize;

    loop {
        while {
            lp += 1;
            arr[lp as usize] < pivot_el
        } {}
        while {
            hp -= 1;
            arr[hp as usize] > pivot_el
        } {}

        if lp >= hp {
            return hp as usize;
        }

        arr.swap(lp as usize, hp as usize);
    }
}


fn fix_pivot(arr: &mut [i32]) {
    let b = arr.len() / 2;
    let c = arr.len() - 1;

    let mut median = 0;

    // a b c
    // a c b
    // c a b

    // b a c
    // b c a
    // c b a
    if arr[0] < arr[b] {
        if arr[b] < arr[c] {
            median = b;
        } else if arr[0] < arr[c] {
            median = c;
        }
    } else {
        if arr[c] < arr[b] {
            median = b;
        } else if arr[c] < arr[0] {
            median = c;
        }
    }

    arr.swap(median, 0);
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
