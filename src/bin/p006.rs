pub fn main() {
    let limit = 100_u64;
    let sum = limit * (limit + 1) / 2;
    let sum_sq = (2 * limit + 1) * (limit + 1) * limit / 6;

    println!("{}", (sum * sum) - sum_sq);
}
