extern crate primal;

pub fn answer() -> String {
    let sieve = primal::Sieve::new(10000);
    let factor_result = sieve.factor(600851475143);

    let answer = match factor_result {
        Ok(factors) => match factors.last() {
            Some(factor) => {
                let &(x, _) = factor;
                x.to_string()
            },
            None => "Error!".to_string(),
        },
        Err(_) => "Error!".to_string(),
    };

    answer
}
