
use std::time::{Instant};

fn main() {

    let start = Instant::now();

    //const MAX_NUMBER: usize = 128*10000000;
    const MAX_NUMBER: usize = 2_usize.pow(32) as usize;

    let size_of_word: usize = usize::BITS as usize;
    // Generate vector to store the prime number check
    // Store one bit for each odd number to check. Don't bother with 
    // even numbers since they are not prime.
    // All are marked as prime to start with, and the test is to see if they
    // are not
    let mut prime_check: Vec<usize> = vec![0; MAX_NUMBER/(2*size_of_word)];


    // Iterate only over odd numbers starting ith 3
    // The index is global bit index
    // Vector index is bit index divided by 64, rounded down.
    let mut number_to_test: usize = 3;

    loop {       
        let mut number = number_to_test*number_to_test;  
        if number > MAX_NUMBER {
            break;
        }

        loop {
            let total_index = (number-1)>>1;
            let vec_index = total_index >> 6;
            let bit_index = total_index & 0x3F;

            // Identify the bit in question
            prime_check[vec_index] |= 1<<bit_index as usize;
            
            // We can skip every other since that will be an even number
            number += 2*number_to_test;

            if number > MAX_NUMBER {
                break;
            }
        }
        number_to_test += 2;
        
    }

    // Count prime numbers
    let mut counter  = 0 as usize;
    for element in prime_check {
        for bit in 0..63 {
            if (element >> bit) & 1 == 0 {
                counter += 1;
            }
        }
    }

    let duration = Instant::now() - start;
    println!("Total duration:  {}", duration.as_secs());
    println!("Number of primes:   {}", counter);

}

    