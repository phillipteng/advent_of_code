use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_file(filename: &str) -> Result<Vec<Vec<i32>>, std::io::Error> {
    let mut words = Vec::new();
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let line_words: Vec<_>= line.split_whitespace()
            .map(|word| word.parse::<i32>().expect("Cannot convert input to i32")).collect();
        if !line_words.is_empty() {
            words.push(line_words);
        }
    }

    Ok(words)
}


fn main() {
    let results = parse_file("inputs.txt");
    let mut answer = 0;

    if let Ok(inputs) = results {
        for current_input in inputs{
            // create two iterators to zip together to find the difference
            let mut number_to_add = 0;
            let mut new_vec = current_input;
            while new_vec.iter().any(|&x| x != 0){
                if let Some(tsar) = new_vec.last() {
                    number_to_add += *tsar;
                }
                new_vec = new_vec.windows(2)
                    .map(|window| window[1] - window[0])
                    .collect::<Vec<i32>>();
            }
            answer += number_to_add;
        }
    } else {
        println!("Error reading file: {}", results.err().unwrap());
    }
    println!("answer {}", answer);
}


