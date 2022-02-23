use std::str::from_utf8;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use rayon::prelude::*; // par_iter_mut(), par_sort_unstable()

// Generate a random word, save it to the given string.
fn generate_random_word(word: &mut String) {
    let word_length = 5;
    let mut rng = thread_rng();

    // Generate 5 bytes in alphanumeric range.
    let mut alpha_bytes = vec![0; word_length];
    for i in 0 .. word_length {
        alpha_bytes[i] = rng.sample(&Alphanumeric);
    }
    *word = from_utf8(&alpha_bytes).unwrap().to_string();
}

// Generate random words in parallel, then sort them in parallel.
fn main() {
    let word_count = 10; // try increasing to 100000 or more
    let mut vec = vec![String::new(); word_count];

    // replace with vec.iter_mut() for single threaded
    vec.par_iter_mut().for_each(generate_random_word);

    // replace with vec.sort_unstable() for single-threaded
    vec.par_sort_unstable();

    for word in vec {
        println!("{word}");
    }
}
