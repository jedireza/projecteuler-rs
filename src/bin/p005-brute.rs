pub fn main() {
    let mut smallest_multiple = 2;

    loop {
        let mut is_evenly_divided = true;

        for x in 1..21 {
            if smallest_multiple % x != 0 {
                is_evenly_divided = false;
                break;
            }
        }

        if is_evenly_divided {
            break;
        }

        smallest_multiple += 2;
    }

    println!("{}", smallest_multiple);
}
