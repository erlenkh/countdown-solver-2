use std::fs;
use std::io::BufReader;
use std::io::prelude::*;
use std::env;
use std::collections::HashSet;
use itertools::Itertools;

fn main() {
    let args: Vec<String> = env::args().collect();

    let letter_seq: Vec<char> = args[1].chars().collect();

    let dict = load_dict();

    // Print out all n letter words that can be made from letter sequence
    for n in 1..letter_seq.len()+1 {
        let permutations: Vec<String> = get_all_n_letter_perms(&letter_seq, n);
        
        let valid_words = filter_dict_words(permutations, &dict);

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


fn get_all_n_letter_perms(letter_seq: &Vec<char>, n: usize) -> Vec<String> {
    letter_seq.iter()
            .permutations(n)
            .unique()
            .map(|x| x.into_iter().collect()) // converts Vec<char> to String
            .collect()
}

fn filter_dict_words(perms: Vec<String>, dict: &HashSet<String>) -> Vec<String> {
    perms.into_iter()
        .filter(|x| dict.contains(x))
        .collect()
}


fn load_dict() -> HashSet<String> {
    let file = fs::File::open("src/sortedDic.txt")
                        .expect("No such file");

    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}