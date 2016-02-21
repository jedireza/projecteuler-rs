extern crate primal;

pub fn main() {
    let sieve = primal::Sieve::new(23);
    let k = 20_f32;
    let mut n = 1_f32;
    let mut i = 1;
    let mut a = vec![1_f32; 9];
    let mut check = true;
    let limit = k.sqrt() as f32;
    let mut p_i = sieve.nth_prime(i) as f32;

    while p_i <= k  {
        if check {
            if p_i <= limit {
                a[i] = (k.log(10_f32) / p_i.log(10_f32)).floor();
            }
            else {
                check = false;
            }
        }

        n = n * p_i.powf(a[i]);

        i += 1;
        p_i = sieve.nth_prime(i) as f32;
    }

    println!("{}", n);
}
