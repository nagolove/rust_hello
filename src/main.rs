#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::env;
use std::io;

include!("lib.rs");

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
        Ordering::Less => {
            println!("small");
            fn sub() {}
        }
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
        return;
    }
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
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
        break;
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
    for i in 0..5 {
        for j in -1..0b1111 {
            println!("i * j = {}", i * j);
            let even_odd = if (i * j) % 2 == 0 { "even" } else { "odd" };
            println!("{} {}", even_odd, i);
        }
    }
}

fn fun9() {
    let mut sum = 0.;
    for i in 0..5 {
        sum += i as f32;
    }
    println!("sum is {}", sum);
}

/*
 *use std::fmt::Display;
 *impl<T: Display> ToString for T {
 *    let i: T;
 *    // snip
 *}
 */

fn main() {
    fun1();
    fun2();
    fun3();
    fun4();
    fun5();
    fun6();
    fun7();
    fun8();
    fun10();
}

fn abs(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        -x
    }
}

fn clamp(x: f64, x1: f64, x2: f64) -> f64 {
    if x < x1 {
        x1
    } else if x > x2 {
        x2
    } else {
        x
    }
}

fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn by_ref(x: &i32) -> i32 {
    *x + 1
}

fn modifies(x: &mut f64) {
    *x = 1.0;
}

fn fun10() {
    let i = 10;
    let res1 = by_ref(&i);
    let res2 = by_ref(&41);
    println!("{} {}", res1, res2);

    let mut res = 0.0;
    modifies(&mut res);
    println!("res is {}", res);
}

fn fun11() {
    let mut line = String::new();
    //line[1] = '1';
    io::stdin().read_line(&mut line).expect("Error");
    let secret = rand::thread_rng().gen_range(1..11);
    let line: u32 = line.trim().parse().expect("Should be a number");

    let args: Vec<String> = env::args().collect();

    //match line.cmp(&args) {
    //Ordering::Less => {
    //println!("small"); fn sub(){}
    //}
    //Ordering::Greater => println!("big"),
    //Ordering::Equal => println!("win"),
    //}

    let args: Vec<String> = env::args().collect();
    //for &i in args {
    //for i in args {
    //match i {
    //"--help" => println!("Fuckstringdoc");
    //}
    //}
}

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Node {
    name: String,
    left: usize, // usize == size_t ??
    //mutex: Mutex<()>,
    mutex: Mutex<()>,
}

impl Node {
    //fn new(name: &str) -> Node {
    fn new(name: &String) -> Node {
        Node {
            name: name.to_string(),
            left: 0,
            //mutex: Mutex::new(()),
            // пустые скобки - unit value
            mutex: Mutex::new(()),
        }
    }

    fn bar(&self) -> i8 {
        let tuple = (1, 2);
        self.mutex.lock();
        thread::sleep(Duration::from_millis(100));
        self.mutex.lock();
        tuple.1
    }
}

trait Printer {
    fn about2(&self, n: i8);
    fn about(&self);
}

struct Person {
    //name: str,
    name: String,
}

impl Person {
    fn new() -> Person {
        Person {
            name: "default".to_string(),
        }
    }
    fn сделать_чтото() -> () {}
}

impl Printer for Person {
    fn about2(&self, n: i8) {
        println!("name is {}", self.name);
    }
    fn about(&self) {
        println!("name is {}", self.name);
    }
}

fn fun12() {
    let p = Person::new();
    let p: Person = Person::new();
    p.about();
}

fn fun13() {
    init();
}

fn fun14() {
    //for arg in env::args().skip(1) { =>
    let str3 = "str3";
    let args = vec!["str1", "str2", str3];
    |x: i8| x * x;
    |x: u64, y: u64| (x * x, y);
    for arg in args {
        match arg {
            "--table" | "-t" => println!("1\n\n"),
            _ => println!("трололо пьет мед и молоко"),
        };
    }
}

type Functor = fn(i8, i8) -> fn() -> fn() -> fn() -> i8;

fn fun15(f: Functor) -> Functor {
    f(0, 0);
    f
}

use std::result;

enum ConcreteError {
    Foo,
    Bar,
}

type Result<T> = result::Result<T, ConcreteError>;
