
use std::time::Instant;

fn main() {

    let start = Instant::now();

    const MAX_NUMBER: usize = 4096;
    //const MAX_NUMBER: usize = 2_usize.pow(32) as usize;

    let size_of_word: usize = usize::BITS as usize;
    // Generate vector to store the prime number check
    // Store one bit for each odd number to check. Don't bother with 
    // even numbers since they are not prime.
    // All are marked as prime to start with, and the test is to see if they
    // are not
    //  let mut primes: Vec<usize> = vec![0; MAX_NUMBER/(2*size_of_word)];

    let mut primes: Vec<usize> = vec![0; MAX_NUMBER/(size_of_word)];
    primes[0] = 3;


    // Iterate only over odd numbers starting ith 3
    // The index is global bit index
    // Vector index is bit index divided by 64, rounded down.
    let mut p: usize = 2;

    while p*p < MAX_NUMBER {   
        if (primes[p>>6] >> (p&0x3F)) & 1 == 0 {
            let mut i = p*p;
            while i < MAX_NUMBER {
                primes[i>>6] |= 1 << (i & 0x3F);
                i += p;
            }
        }
        p += 1;   
    }
    let duration = Instant::now() - start;

    // Count prime numbers
    let mut prime_list: Vec<usize> = Vec::new();

    for (index, element) in primes.iter().enumerate() {
        for bit in 0..=63 {
            if (element >> bit) & 1 == 0 {
                prime_list.push(index*64 + bit);
            }
        }
    }

    
    println!("Total duration:  {}", duration.as_secs());
    println!("Number of primes:   {}", prime_list.len());
    //println!("{:?}", prime_list);





}

    