use std::env;
use std::fs;

fn main() {
    // file name
    // name of the person
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("\n\nusage: check_roster.exe <file_name> <person_name>\n");
        return;
    }
    let file_name = &args[1];
    let person_name = &args[2];
    let mut found = 0;
    let content = fs::read_to_string(file_name).expect("error: cannot read the input file");
    for l in content.lines() {
        if l.to_lowercase() == person_name.to_lowercase() {
            println!("Person - {} was found on the list", l);
            found = 1;
            break;
        }
    }
    if found == 0 {
        println!("Person - {} was not found on the list", person_name);
    }
    println!("Done.");
}
