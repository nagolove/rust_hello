use std::io;
use rand::Rng;
use std::cmp::Ordering;

//#[warn(dead_code)] 
//fn foo() -> u64 {
    //2
//}

fn fun1() {
    let mut line = String::new();
    //let line2 = line;
    //line[1] = '1';
    //let line2 = String::new();
    io::stdin().read_line(&mut line).expect("Error");
    //io::stdin().read_line(&mut line).expect("Error");
    println!("guess {}", line);

    let x = 1;
    //x.rem_euclid
}

fn fun2() {
    let secret_num = rand::thread_rng().gen_range(1..11);
    println!("secret is {}", secret_num);
}

fn fun3() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Error");
    let secret = rand::thread_rng().gen_range(1..11);
    let line: u32 = line.trim().parse().expect("Should be a number");

    match line.cmp(&secret) {
        Ordering::Less => println!("small"),
        Ordering::Greater => println!("big"),
        Ordering::Equal => println!("win"),
    }
}

fn fun4() {
    //let mut i: u64 = 1;
    let mut i: i16 = 1;
    let k: i16 = 1;
    let ok: bool = false;
    let ok1: bool = false;
    let ch: char = 'a';
    let ch: i8;
    //ch.div_euclidj
    // char utf8  от 1 байта до 4х. В зависимости от полноты поддержки.
    if ok == true {
        println!("ok is {}", ok)
    }
    loop {
        //i = i + 1;
        //i = i.checked_add(k);
        return
    }
}

fn gcd(mut n:u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    debug_assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

//Oneliner
/*
   Multiline
 */
#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    //assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * * 13 * 19), 3 * 11);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}

fn fun5() {
    let _a: i8 = 0b10_000_00;
    let _a_unsigned: u8 = 0b10_00_00_01;
    //use std::opts;
    use std::ops::Shl;
    let _a = _a.shl(3);
    //println!("size_of(_a) {}", _a.size_of _a);
    let _b: u8;
    let _c: i16;
    let _d: u16;
    let _e: i32;
    let _f: u32;
    let _g: i64;
    let _k: u64;
    let _l: i128;
    let _m: u128;
    let _n: f32;
    let _o: f64;
    //_a.cmp
    //_o.ceil().le
    //_a.cmp
    //_o.asin(0.).asin(1.).hypot(
    loop {
        break
    }
}

fn fun6() -> u32 {
    {
        let v = 100;
        println!("{}", v);
        v
    }
}

use std::io::Write;
use std::str::FromStr;


fn fun7() {
    let mut numbers = Vec::new();
    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("parsing argument error"));
    }
    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "Usage: gcd NUMBER").unwrap();
        std::process::exit(1);
    }
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("greatest gcd of {:?} is {}", numbers, d);
}

fn fun8() {
}

fn main() {
    //fun3();
    fun4();
}
