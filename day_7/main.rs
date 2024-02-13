use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;

#[derive(Debug, Eq)]

struct HandInfo{
    hand: String,
    bid: i32,
}
impl Ord for HandInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // todo
        let map: HashMap<char, u32> = HashMap::from([
            ('2', 2),
            ('3', 3),
            ('4', 4),
            ('5', 5),
            ('6', 6),
            ('7', 7),
            ('8', 8),
            ('9', 9),
            ('T', 10),
            ('J', 11),
            ('Q', 12),
            ('K', 13),
            ('A', 14),
        ]);
        for (first_char, second_char) in zip(self.hand.chars(), other.hand.chars()){
            let first_numerical_rep = map.get(&first_char).unwrap();
            let second_numerical_rep = map.get(&second_char).unwrap();
            if first_numerical_rep < second_numerical_rep {
                return std::cmp::Ordering::Less;
            } else if first_numerical_rep > second_numerical_rep {
                return std::cmp::Ordering::Greater;
            }
        }
        println!("cmp called");
        std::cmp::Ordering::Equal
    }
}

impl PartialOrd for HandInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HandInfo {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
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
    let results = parse_file("inputs.txt");
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
    let mut final_answer = 0;
    let mut current_rank  = 1;
    for one_hand_strength in hand_strengths_from_lowest_to_highest {
        let mut all_hands_of_this_type = hands_bucketed_by_hand_strength.get_mut(&one_hand_strength);
        if let Some(ref mut all_hands) = all_hands_of_this_type {
            all_hands.sort();
            for one_hand in all_hands.iter() {
                final_answer += one_hand.bid * current_rank;
                current_rank += 1;
            }
        }
    }
    println!("Final score is {}", final_answer);
}
