use std::collections::HashMap;
use std::env;
use std::fs;

/**
 * 1. Read in a text file.
 * 2. Count the number of time each word occurs.
 * 3. Print a message with the most common words and how many times they appeared.
 */

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut map = HashMap::new();
    if args.len() == 2 {
        let file_name = &args[1];
        let content = fs::read_to_string(file_name).expect("error: file cannot be opened");
        let lines = content.split("\n");
        for l in lines {
            for w in l.split_whitespace() {
                if map.get(w) == None {
                    map.insert(w, 1);
                } else {
                    let v: i32 = map.get(w).unwrap() + 1;
                    map.insert(w, v);
                }
            }
        }
        let mut maximum = 0;
        let mut val = "";
        for k in map.keys() {
            if maximum < *map.get(k).unwrap() {
                maximum = *map.get(k).unwrap();
                val = k;
            }
        }
        println!("The maximum (word, count): ({}, {})", val, maximum);
    } else {
        println!("usage: count_words.exe <file_name>");
    }
}
