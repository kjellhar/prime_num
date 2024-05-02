use std::time::Instant;

fn main() {
    let start = Instant::now();

    //const MAX_NUMBER: usize = 4096;
    const MAX_NUMBER: usize = 2_usize.pow(38);
    let size_of_word: usize = usize::BITS as usize;

    let mut primes: Vec<usize> = vec![0; MAX_NUMBER / (2 * size_of_word)];
    primes[0] = 1;

    //Calculate all the odd number non-primes
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

    let duration1 = Instant::now() - start;

    // // collect all the primes in a vector
    // let mut prime_list: Vec<usize> = Vec::new();
    // prime_list.push(2);
    // for (index, element) in primes.iter().enumerate() {
    //     let vi_num = index*128;
    //     for bit in 0..=63 {
    //         if (element >> bit) & 1 == 0 {
    //             prime_list.push(vi_num + (2 * bit + 1));
    //         }
    //     }
    // }

       // count all the primes
       let mut prime_count: usize = 1;
       for element in primes.iter() {
           for bit in 0..=63 {
               if (element >> bit) & 1 == 0 {
                  prime_count += 1;
               }
           }
       }

    let duration2 = Instant::now() - start;

    println!("Calc duration:  {} ms", duration1.as_millis());
    println!("Total duration:  {} ms", duration2.as_millis());
    println!("Number of primes:   {}", prime_count);
}
