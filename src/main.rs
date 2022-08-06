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

    let letter_seq: Vec<char> = args[1].chars().collect();

    let dict = load_dictionary("src/sortedDic.txt");

    // Print out all n letter words that can be made from letter sequence
    for n in 1..letter_seq.len()+1 {
        let permutations: Vec<String> = n_letter_perms(&letter_seq, n);
        
        let valid_words = dict_words(permutations, &dict);

        println!("{n} letter words:");
        print_words(&valid_words); 
    }
}

fn print_words(words: &Vec<String>) {
    if words.len() > 0 {
        for word in words {
            println!("{word}");
        }
    } else {
        println!("(No words found)")
    }

    println!("\n");
}

fn n_letter_perms(letter_seq: &Vec<char>, n: usize) -> Vec<String> {
    letter_seq.iter()
            .permutations(n)
            .unique()
            .map(|x| x.into_iter().collect()) // converts Vec<char> to String
            .collect()
}

fn dict_words(letter_seqs: Vec<String>, dict: &HashSet<String>) -> Vec<String> {
    letter_seqs.into_iter()
            .filter(|x| dict.contains(x))
            .collect()
}


fn load_dictionary(filename: &str) -> HashSet<String> {
    let file = fs::File::open(filename)
                        .expect("No such file");

    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}