use std::io::{stdin, stdout, Write};
use std::process::exit;
use std::env;
use colored::Colorize;
use std::fs;
use rand::{Rng, rng};
use std::time::Instant;

fn main() {
    // Get command line args from the user to determine source and output file
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("{}\n{}", "No arguments specified!".bold().cyan(), "Usage: ./text_generator source_file output_file\nIf no input or output file is provided, the defaults will be used".italic().green());
    }

    // Assign default values if the user does not provide any
    let input_file = args.get(1).map(|s| s.as_str()).unwrap_or("./default_input");
    let output_file = args.get(2).map(|s| s.as_str()).unwrap_or("./default_output");

    let contents = match fs::read_to_string(input_file) {
        Ok(contents) => contents,
        Err(e) => {
            println!("{} {}", "Failed to open provided file!".bold().red(), e);
            exit(0);
        }
    };

    // Read contents into vector that can be accessed at random
    let words: Vec<&str> = contents.trim().split(" ").collect();

    let number_of_words = get_num_words();
    println!("{}", "Beginning generation".yellow());
    let start = Instant::now();
    generate_output(&output_file, number_of_words, words);
    let end = Instant::now();
    let time = end - start;
    println!("{}{}{}{}{}{}{}", "Generation completed. Wrote ".green(), number_of_words.to_string().green(), " words to ".green(), output_file.green(), " in ".green(), time.as_secs().to_string().green(), " seconds".green());
}

fn generate_output(file_name: &str, number_of_words: i32, words: Vec<&str>) {
    // Open output file
    let mut output = fs::File::create(file_name).expect("Failed to create output file!");
    let mut rng = rng();

    // Write words to the file
    let mut index = 0;
    while index < number_of_words {
        let word = words[rng.gen_range(0..words.len())].to_owned() + " ";
        output.write_all(&word.as_bytes()).expect("Failed to write to output file!");
        index += 1;
    }

}

fn get_num_words() -> i32 {
    // Get input from the user on the number of words they would like to generate
    let mut number_of_words = String::new();
    print!("{}", "How many words would you like to generate? ");
    let _ = stdout().flush();
    stdin().read_line(&mut number_of_words).expect(&format!("{}", "Could not read from STDIN".bold().red()));

    // Check if the user passed in a valid integer
    let num_words = match number_of_words.trim().parse::<i32>() {
        Ok(n) => n,
        Err(_) => {
            println!("{}", "Invalid input, please enter a valid integer".bold().red());
            exit(0);
        }
    };

    return num_words;
}

