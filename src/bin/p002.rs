pub fn main() {
    let mut the_sum: i32 = 0;
    let mut prev = (0, 1);

    loop {
        let (x, y) = prev;
        let fibo = x + y;

        if fibo > 4000000 {
            break;
        }

        if fibo % 2 == 0 {
            the_sum += fibo;
        }

        prev = (y, fibo);
    }

    println!("{}", the_sum);
}
