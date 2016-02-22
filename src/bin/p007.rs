extern crate primal;

pub fn main() {
    let nth = primal::Primes::all().nth(10001 - 1).unwrap();

    println!("{}", nth);
}
