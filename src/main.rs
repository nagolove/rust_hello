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
    loop {
        //i = i + 1;
        //i = i.checked_add(k);
    }
}

fn main() {
    //fun3();
    fun4();
}
