pub fn main() {
    let mut the_sum: i32 = 0;

    for x in 1..1000 {
        if x % 3 == 0  || x % 5 == 0 {
            the_sum += x;
        }
    }

    println!("{}", the_sum);
}
