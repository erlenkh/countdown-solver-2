use std::{
    fs,
    io::BufReader,
    io::prelude::*,
    env,
    collections::HashSet
};
use itertools::Itertools;

fn main() {
    let args: Vec<String> = env::args().collect();

    let letter_sequence: Vec<char> = args[1].chars().collect();

    let dictionary = load_dictionary("src/sortedDic.txt");

    print_valid_words(&letter_sequence, &dictionary);
}

/// Prints out all valid n-letter words that can be made from letter sequence,
/// where n goes from 1 to length of letter sequence.
fn print_valid_words(letter_seq: &Vec<char>, dict: &HashSet<String>) {
    for n in 1..letter_seq.len()+1 {
        let permutations: Vec<String> = n_letter_perms(&letter_seq, n);
        
        let mut valid_words = dict_words(permutations, &dict);
        
        if valid_words.len() > 0 {
            println!("Valid {n}-letter words:");

            valid_words.sort();

            for word in valid_words {
                println!("{word}");
            }
        } else {
            println!("No valid {n}-letter words found");
        }
    
        println!("\n");
    }
}

/// Returns vector of all n-letter permutations of input letter sequence.
fn n_letter_perms(letter_seq: &Vec<char>, n: usize) -> Vec<String> {
    letter_seq.iter()
            .permutations(n)
            .unique()
            .map(|x| x.into_iter().collect()) // Converts Vec<char> to String
            .collect()
}

/// Filters out all non-dictionary words from vector of letter sequences.
fn dict_words(letter_seqs: Vec<String>, dict: &HashSet<String>) -> Vec<String> {
    letter_seqs.into_iter()
            .filter(|x| dict.contains(x))
            .collect()
}

/// Loads dictionary from filename into a HashSet of dictionary words.
fn load_dictionary(filename: &str) -> HashSet<String> {
    let file = fs::File::open(filename)
                        .expect("No such file");

    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}