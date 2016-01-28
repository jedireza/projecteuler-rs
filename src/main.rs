extern crate projecteuler_rs as problems;

fn help() {
    println!("usage:
cargo run <string>
    The 3 digit problem id. Ex: 001, 012, 142");
}

fn main() {
	let args: Vec<String> = std::env::args().collect();

	match args.len() {
        1 => help(),
        2 => {
            match &args[1][..] {
                "001" => println!("Solution 001: {}", problems::q001::answer()),
                "002" => println!("Solution 002: {}", problems::q002::answer()),
                "003" => println!("Solution 003: {}", problems::q003::answer()),
                "004" => println!("Solution 004: {}", problems::q004::answer()),
                _ => println!("Solution not found")
            }
        },
        _ => help()
	}
}
