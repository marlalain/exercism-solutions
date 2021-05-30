pub fn nth(n: u32) -> u32 {
    let mut last_prime: u32 = 0;
    let mut current_number: u32 = 0;
    while current_number <= n {
        last_prime += 1;
        if is_prime(last_prime) {
            current_number += 1;
        }
    }
    last_prime
}

fn is_prime(number: u32) -> bool {
    if number == 1 {
        return false;
    }

    let mut limit: u32 = 0;
    for x in 2..number {
        if number % x == 0 {
            limit += 1;
        }
    }
    if limit < 1 {
        return true;
    }

    false
}
