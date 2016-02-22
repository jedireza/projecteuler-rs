pub fn main() {
    let mut sum_of_squares = 0_u64;
    let mut square_of_sums = 0_u64;

    for x in 1..101 {
        sum_of_squares += x * x;
        square_of_sums += x;
    }

    square_of_sums *= square_of_sums;

    println!("{}", square_of_sums - sum_of_squares);
}
