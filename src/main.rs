use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

// Define the DFA structure
#[derive(Debug)]
struct DFA {
    states: Vec<String>,
    alphabet: Vec<String>,
    func: HashMap<String, HashMap<String, String>>,
    start: String,
    end: Vec<String>,
}

impl DFA {
    // Create a new DFA instance
    fn new(states: Vec<String>, alphabet: Vec<String>, func: HashMap<String, HashMap<String, String>>, start: String, end: Vec<String>) -> DFA {
        DFA { states, alphabet, func, start, end }
    }
}

// Function to load DFA from a JSON file
fn load_dfa(file_path: &str) -> DFA {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut json_data = String::new();
    while let Some(line) = lines.next() {
        json_data.push_str(&line.unwrap());
    }
    let json_data: serde_json::Value = serde_json::from_str(&json_data).expect("Failed to parse JSON");
    let states = json_data["states"].as_array().unwrap().iter().map(|s| s.as_str().unwrap().to_string()).collect();
    let alphabet = json_data["alphabet"].as_array().unwrap().iter().map(|s| s.as_str().unwrap().to_string()).collect();
    let func = json_data["func"].as_object().unwrap().iter().map(|(k, v)| {
        let state = k.clone();
        let transitions = v.as_object().unwrap().iter().map(|(k, v)| {
            let symbol = k.clone();
            let next_state = v.as_str().unwrap().to_string();
            (symbol, next_state)
        }).collect();
        (state, transitions)
    }).collect();
    let start = json_data["start"].as_str().unwrap().to_string();
    let end = json_data["end"].as_array().unwrap().iter().map(|s| s.as_str().unwrap().to_string()).collect();
    DFA::new(states, alphabet, func, start, end)
}

// Function to generate the transition table
fn generate_func_tab(dfa: &DFA) {
    println!("δ");
    for i in 0..dfa.alphabet.len() {
        print!("{} ", dfa.alphabet[i]);
    }
    println!();
    for i in 0..dfa.states.len() {
        print!("{}:", dfa.states[i]);
        for j in 0..dfa.alphabet.len() {
            let mut text = "λ";
            if let Some(transitions) = dfa.func.get(&dfa.states[i]) {
                if let Some(next_state) = transitions.get(&dfa.alphabet[j]) {
                    text = next_state;
                }
            }
            print!(" {} ", text);
        }
        println!();
    }
}

// Function to check if a word is accepted by the DFA
fn check_word(word: &str, dfa: &DFA) -> bool {
    let mut current_state = dfa.start.clone();
    for c in word.chars() {
        if let Some(transitions) = dfa.func.get(&current_state) {
            if let Some(next_state) = transitions.get(&c.to_string()) {
                current_state = next_state.clone();
            } else {
                println!("Error: No transition for state {} and symbol {}", current_state, c);
                return false;
            }
        } else {
            println!("Error: No transitions for state {}", current_state);
            return false;
        }
    }
    dfa.end.contains(&current_state)
}

fn main() {
    let file_path = "machine.json";
    let dfa = load_dfa(file_path);
    generate_func_tab(&dfa);
    println!("Enter a word to check:");
    let mut word = String::new();
    std::io::stdin().read_line(&mut word).expect("Failed to read input");
    let word = word.trim();
    if check_word(word, &dfa) {
        println!("The word is accepted by the DFA.");
    } else {
        println!("The word is not accepted by the DFA.");
    }
}
