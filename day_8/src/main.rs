use std::collections::HashMap;
use std::ffi::IntoStringError;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_file(filename: &str) -> Result<Vec<Vec<String>>, std::io::Error> {
    let mut words = Vec::new();
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let line_words: Vec<_>= line.split_whitespace().map(|word| word.to_string()).collect();
        if !line_words.is_empty() {
            words.push(line_words);
        }
    }

    Ok(words)
}

// I don't really understand the function signature that I should use here
// in regards to &Vec<String> and &[<Vec<String>], is it valid to pass
// by reference like this? Is there a better way?
fn get_answer(instruction: String, information_to_construct_graph: Vec<Vec<String>>) -> i32 {
    // construct graph by using hash maps
    let mut my_map: HashMap<String, Vec<String>> = HashMap::new();

    for line in information_to_construct_graph {
        // having trouble with stripping prefix because
        //^^^^^^ the trait `From<Option<&str>>` is not implemented for `String`
        my_map.insert(line[0].clone(), vec!(line[2].clone(),
                                            line[3].clone()));
    }

    // now that I have the map, ic can go through the instructions and find the answer
    let mut answer = 0;
    let mut current_node = String::from("AAA");
    let mut current_instruction_index = 0;
    while current_node != String::from("ZZZ") {
        if let Some(current_instruction) = instruction.chars().nth(current_instruction_index){
            if current_instruction == 'L'{
                current_node = my_map[&current_node][0].to_owned();
            } else if current_instruction == 'R'{
                current_node = my_map[&current_node][1].to_owned();
            } else {
                panic!("unknown instruction")
            }
            current_instruction_index += 1;
            if current_instruction_index == instruction.len(){
                current_instruction_index = 0;
            }
            answer += 1;
        }
    }
    answer
}

fn sanitize_input(p0: &mut Vec<Vec<String>>) {
    for line in &mut *p0{
        line[2] = line[2].clone()[1..4].to_owned();
        line[3] = line[3].clone()[..3].to_owned();
    }
}

fn main() {
    let results = parse_file("inputs.txt");
    if let Ok(mut words) = results {
        let instruction = words[0][0].clone();
        words = words[1..].to_owned();
        sanitize_input(&mut words);
        println!("answer is {}", get_answer(instruction, words));

    } else {
        println!("Error reading file: {}", results.err().unwrap());
    }
}


fn parse_file(filename: &str) -> Result<Vec<Vec<String>>, std::io::Error> {
    let mut words = Vec::new();
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let line_words: Vec<_>= line.split_whitespace().map(|word| word.to_string()).collect();
        if !line_words.is_empty() {
            words.push(line_words);
        }
    }

    Ok(words)
}

// I don't really understand the function signature that I should use here
// in regards to &Vec<String> and &[<Vec<String>], is it valid to pass
// by reference like this? Is there a better way?
fn get_answer(instruction: Vec<String>, information_to_construct_graph: Vec<Vec<String>>) -> i32 {
    println!("{:?}", instruction);
    println!("{:?}", information_to_construct_graph);
    let current_node = String::from("AAA");
    // construct graph by using hash maps
    let mut my_map: HashMap<String, Vec<String>> = HashMap::new();

    for line in information_to_construct_graph {
        // having trouble with stripping prefix because
        //^^^^^^ the trait `From<Option<&str>>` is not implemented for `String`
        my_map.insert(line[0].clone(), vec!(String::from(line[2].clone()),
                                            String::from(line[3].clone())));
        println!("{:?}", my_map);
    }

    0
}
fn sanitize_input(p0: &mut Vec<Vec<String>>) {
    p0[2] = p0[2];
}
fn main() {
    let results = parse_file("inputs.txt");
    if let Ok(mut words) = results {
        let instruction = words[0].clone();
        words = words[1..].to_owned();
        sanitize_input(&mut words);
        println!("{}", get_answer(instruction, words));

        // for line in words {
        //     println!("{:?}", line);
        //     line.
        //     instructions
        // }
        //
    } else {
        println!("Error reading file: {}", results.err().unwrap());
    }

    println!("Final score statement so I pause the debugger here");
}



