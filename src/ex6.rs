use std::collections::HashSet;
use std::fmt::Display;
use std::{env, fs};

pub fn run_a() {
    println!("Running program for exercise 6a of AOC-2022");

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    // Load communication buffer from file
    let communication_buffer =
        fs::read_to_string(file_path).expect("Could not read the provided file");

    let mut position = 4;
    for x in communication_buffer.into_bytes().windows(4) {
        println!("position: {} with {:?}", position, x);
        let chars: HashSet<&u8> = HashSet::from_iter(x.iter());
        if chars.len() == 4 {
            break;
        } else {
            position += 1;
        }
    };

    println!("Found position {}", position);
}

pub fn run_b() {
    println!("Running program for exercise 6b of AOC-2022");

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    // Load communication buffer from file
    let communication_buffer =
        fs::read_to_string(file_path).expect("Could not read the provided file");

    let mut position = 14;
    for x in communication_buffer.into_bytes().windows(14) {
        println!("position: {} with {:?}", position, x);
        let chars: HashSet<&u8> = HashSet::from_iter(x.iter());
        if chars.len() == 14 {
            break;
        } else {
            position += 1;
        }
    };

    println!("Found position {}", position);
}

pub fn run_c() {
    println!("Running program for exercise 6c of AOC-2022");

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    // Load communication buffer from file
    let communication_buffer =
        fs::read_to_string(file_path).expect("Could not read the provided file");

    let pos = communication_buffer.into_bytes().windows(4) 
        .position(|x| HashSet::<&u8>::from_iter(x.iter()).len() == 4).unwrap() + 4;

    println!("Found position {}", pos);
}

pub fn run_d() {
    println!("Running program for exercise 6d of AOC-2022");

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    // Load communication buffer from file
    let communication_buffer =
        fs::read_to_string(file_path).expect("Could not read the provided file");

    let pos = communication_buffer.into_bytes().windows(14) 
        .position(|x| HashSet::<&u8>::from_iter(x.iter()).len() == 14).unwrap() + 14;

    println!("Found position {}", pos);
}
