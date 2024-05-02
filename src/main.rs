use std::time::Instant;

fn main() {
    let start = Instant::now();

    //const MAX_NUMBER: usize = 4096;
    const MAX_NUMBER: usize = 2_usize.pow(32);
    let size_of_word: usize = usize::BITS as usize;

    let mut primes: Vec<usize> = vec![0; MAX_NUMBER / (2 * size_of_word)];
    primes[0] = 1;

    let mut p: usize = 3;
    while p * p < MAX_NUMBER {
        if (primes[p >> 7] >> ((p >> 1) & 0x3F)) & 1 == 0 {
            let mut i = p * p;
            while i < MAX_NUMBER {
                primes[i >> 7] |= 1 << ((i >> 1) & 0x3F);
                i += p + p;
            }
        }
        p += 2;
    }

    let mut prime_list: Vec<usize> = Vec::new();
    prime_list.push(2);
    for (index, element) in primes.iter().enumerate() {
        for bit in 0..=63 {
            if (element >> bit) & 1 == 0 {
                prime_list.push(index * 128 + (2 * bit + 1));
            }
        }
    }

    let duration = Instant::now() - start;

    println!("Total duration:  {} ms", duration.as_millis());
    println!("Number of primes:   {}", prime_list.len());
}
