use std::io::{self, Read, Write};
use std::fs::File;

fn filename_to_string(s: &str) -> io::Result<String> {
    let mut file = File::open(s)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

fn get_unique_letters(w: &str) -> i32 {
    let mut letters: [bool; 26] = [false; 26];
    for l in w.chars() {
        let val = (l.to_digit(36).unwrap() - 10) as usize;
        letters[val] = true;
        // println!("{:?}", val);
    }
    letters.iter().filter(|v| **v == true).count() as i32
}

fn main() {
    println!("Loading wordlist...");

    let whole_file = filename_to_string("../res/words_alpha.txt").unwrap();

    let words = whole_file.lines().collect::<Vec<&str>>();

    println!("Loaded wordlist, found {} words. Now scoring words...", words.len());

    let mut scores: Vec<f32> = vec![];

    let mut out = File::create("words.txt").expect("Couldn't create output file");
    let cool_words_score: f32 = 3.0;
    out.write_fmt(format_args!("Saving words that score at least {}...\n", cool_words_score)).expect("Couldn't write to output file");

    let mut best_word = 0;
    let mut best_fair_word = 0;
    
    let mut i = 0;
    while i < words.len() {
        let score: f32 = (words[i].len() as f32)/(get_unique_letters(words[i]) as f32);
        scores.push(score);

        if score >= cool_words_score {
            out.write_fmt(format_args!("{}, len = {:?}, ul = {:?}, score = {:?}\n", words[i], words[i].len(), get_unique_letters(words[i]), score)).expect("Couldn't write to output file");
        }

        if score > scores[best_word] {
            best_word = i
        }

        if score > scores[best_fair_word] && score != words[i].len() as f32 {
            best_fair_word = i;
        }

        i += 1;
    }

    println!("Done! The best word overall was {}. The best REAL word was {}, {:?} letters long, {:?} unique letters, giving a score of {:?}!", words[best_word], words[best_fair_word], words[best_fair_word].len(), get_unique_letters(words[best_fair_word]), scores[best_fair_word]);

    // for word in best_wordse {
    //     for i in 0..word.len() {
    //         for l in 0..word_len {
    //             // print!("{}", word[i] as char);
    //             out.write_fmt(format_args!("{}", trimmed_words[word[i]][l] as char)).expect("Couldn't write to output file");
    //         }
    //         out.write_all(b" ").expect("Couldn't write to output file");
    //     }
    //     // println!("")
    //     out.write_all(b"\n").expect("Couldn't write to output file");
    // }
}
