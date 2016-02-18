fn digits(mut num: i32) -> Vec<i32> {
    let mut digits = vec![0; 0];

    while num > 0 {
        digits.push(num % 10);
        num = num / 10;
    }

    digits.reverse();

    digits
}

fn is_palindrome(num: i32) -> bool {
    let digits = digits(num);

    for x in 0..(digits.len() / 2) {
        if digits[x] != digits[(digits.len() - 1) - x] {
            return false;
        }
    }

    true
}

pub fn main() {
    let mut largest_palindrome = 0;

    for x in 100..1000 {
        for y in 100..1000 {
            let product = x * y;
            if is_palindrome(product) && product > largest_palindrome {
                largest_palindrome = product;
            }
        }
    }

    println!("{}", largest_palindrome);
}
