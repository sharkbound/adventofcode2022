use rustutils::numbers::is_prime_u32;

fn main() {
    let nums = vec![23, 19, 13, 17];
    let cycle = dbg!(get_cycle_repeat(nums.as_slice()));
    dbg!(cycle * cycle);

    let mut n = 2;
    while nums.iter().any(|x| n % x != 0) {
        n += 1;
    }
    dbg!(n);
}

fn get_cycle_repeat(nums: &[u32]) -> u32 {
    let (mut primes, mut non_primes): (Vec<u32>, Vec<u32>) = nums
        .iter() /*.map(|x| *x)*/
        .partition(|x| is_prime_u32(**x));
    while !non_primes.is_empty() {
        let n = non_primes.pop().unwrap();
        match split_number(n) {
            Some((x, y)) => {
                if is_prime_u32(x) {
                    primes.push(x);
                } else {
                    non_primes.push(x);
                }
                if is_prime_u32(y) {
                    primes.push(y);
                } else {
                    non_primes.push(y);
                }
            }
            None => {
                primes.push(n);
            }
        }
    }
    return primes.iter().fold(1, |acc, x| acc * x);
}

fn split_number(n: u32) -> Option<(u32, u32)> {
    if is_prime_u32(n) {
        return None;
    }

    for x in (1..n).rev() {
        for y in (1..=x).rev() {
            if x * y == n {
                return Some((x, y));
            }
        }
    }
    None
}
