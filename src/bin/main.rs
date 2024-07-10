#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused)]

fn main() {
    match fp::run() {
        Ok(_) => println!("all good"),
        Err(err) => println!("{}", err),
    }
}
