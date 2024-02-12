use std::cmp::{max, Ordering};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Eq, PartialEq, PartialOrd)]
struct HandInfo{
    hand: String,
    bid: i32,
}
impl Ord for HandInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // todo
        println!("{}", self.hand);
        println!("{}", other.hand);
        std::cmp::Ordering::Equal
    }
}

// ordered from least to strongest, allow it to be used a hash index
#[derive(Hash, Debug, Eq, PartialEq)]
enum HandStrength {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}
fn get_hand_strength(hand: String) -> HandStrength {
    let mut card_counts: HashMap<char, i32> = HashMap::new();
    for letter in hand.chars(){
        if card_counts.contains_key(&letter){
            if let Some(x) = card_counts.get_mut(&letter) {
                *x += 1;
            }
        } else {
            card_counts.insert(letter, 1);
        }
    }
    let mut number_of_pairs = 0;
    let mut max_same_cards = 0;

    for &freq in card_counts.values() {
        if freq == 2{
            number_of_pairs += 1;
        }
        if freq > max_same_cards {
            max_same_cards = freq;
        }
    }
    match max_same_cards {
        1 => HandStrength::HighCard,
        2 => {
            return if number_of_pairs == 2 {
                HandStrength::TwoPair
            } else {
                HandStrength::OnePair
            }
        },
        3 => {
            return if number_of_pairs == 1 {
                HandStrength::FullHouse
            } else {
                HandStrength::ThreeOfAKind
            }
        },
        4 => HandStrength::FourOfAKind,
        5 => HandStrength::FiveOfAKind,
        _ => panic!("Unexpected amounts of same cards possible to determine hand strength")
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

fn main() {
    let results = parse_file("input.txt");
    let mut hands_bucketed_by_hand_strength: HashMap<HandStrength, Vec<HandInfo>> = HashMap::new();
    if let Ok(words) = results {
        for line in words {
            // I don't know how I can not avoid clone here, it doesn't let me move the value because its in the for
            // loop?
            let hand = line[0].clone();
            let bid: i32 = line[1].parse().unwrap();
            let hand_strength = get_hand_strength(hand.clone());
            let this_hand = HandInfo{
                hand: hand,
                bid: bid
            };
            // insert into hg
            let hands_that_share_this_hand_strength = hands_bucketed_by_hand_strength
                .entry(hand_strength).or_insert_with(|| vec![]);
            hands_that_share_this_hand_strength.push(this_hand);
        }
    } else {
        println!("Error reading file: {}", results.err().unwrap());
    }
    // go through the enum orders
    let hand_strengths_from_lowest_to_highest = vec!{HandStrength::HighCard, HandStrength::OnePair,
                                                     HandStrength::TwoPair, HandStrength::ThreeOfAKind, HandStrength::FullHouse,
                                                     HandStrength::FourOfAKind, HandStrength::FiveOfAKind};
    for one_hand_strength in hand_strengths_from_lowest_to_highest {
        let all_hands_of_this_type = hands_bucketed_by_hand_strength.get_mut(&one_hand_strength);
        if all_hands_of_this_type.is_some() {
            // please tell me if I can avoid using this clone()
            // I'm stuck here I dont know what to do to call sort on the vectors...
            all_hands_of_this_type.clone().unwrap().sort();
            println!("{:?}", all_hands_of_this_type);
        }
    }
    println!("Print statement so I pause the debugger here");
}


