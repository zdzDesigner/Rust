use std::fmt::Debug;

fn duplicate<T: Clone>(a: &T, b: &T) -> (T, T) {
    (a.clone(), b.clone())
}

pub fn run_dupli() {
    println!(
        "{:?}",
        duplicate(&String::from("zdz"), &String::from("lmy")),
    );
}

fn addto(x: impl Into<i32>) -> i32 {
    x.into() + 3
}

pub fn run_addto() {
    let v = 89_i8;
    {
        let vv: i32 = v.into();
        println!("{}", vv);
    }
    println!("{v}");
    println!("{}", addto(v));
}

fn debug_show(v: i32) -> impl Debug {
    (v, v)
}

pub fn run_debug_show() {
    println!("{:?}", debug_show(8));
}

fn min<'a, T: PartialOrd>(a: &'a T, b: &'a T) -> &'a T {
    if a.lt(b) {
        a
    } else {
        b
    }
}

use std::cmp::Ordering::{Equal, Greater, Less};
fn min_common<'a, T: Ord>(a: &'a T, b: &'a T) -> &'a T {
    match &a.cmp(&b) {
        Less | Equal => &a,
        Greater => &b,
    }
}

fn min2<T: Ord>(l: T, r: T) -> T {
    match l.cmp(&r) {
        Less | Equal => l,
        Greater => r,
    }
}

pub fn run_min() {
    println!("{}", min(&4, &8));
    println!("{}", min(&48, &8));
    println!("{}", min_common(&2, &4));
    // println!("{}", min_common("aa", "bb"));
    let aa = "aa";
    let bb = "bb";
    println!("{}", min2(&aa, &bb));
    println!("{aa},{bb}");

    let a = String::from("aa");
    let b = String::from("bb");
    println!("{}", min2(&a, &b));
    println!("{a},{b}");
}
