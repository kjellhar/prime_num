
use std::time::{Instant};

fn main() {

    let mut prime_numbers: Vec<u32> = Vec::new();
    let mut numbers_to_test: Vec<u32> = (4..u32::MAX).rev().filter(|n| n%2 == 1).collect();

    //Add first prime number
    prime_numbers.push(3);

    let mut test_progress: usize = 0;
    let mut max_number_to_test: u32 = prime_numbers[test_progress].pow(2);

    let start = Instant::now();
    while let Some(num_to_test) = numbers_to_test.pop() {
        if max_number_to_test <= num_to_test {test_progress += 1};
        max_number_to_test = prime_numbers[test_progress].pow(2);

        if prime_numbers[0..test_progress].iter().all(|p|num_to_test%p > 0) {
            prime_numbers.push(num_to_test);
        }
    }

    let duration = start.elapsed();
    //prime_numbers.iter().for_each(|p|println!("{}", p));
    println!("Number of found primes: {}", prime_numbers.len());
    println!("Time elapsed: {}", duration.as_secs());
}
