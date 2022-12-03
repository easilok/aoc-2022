use std::{env, fs};

#[derive(Debug)]
struct Rusack {
    compartment_1: String,
    compartment_2: String,
    duplicated: Option<char>,
    duplicated_u32: u32,
}

impl Rusack {
    pub fn new(compartment_1: &str, compartment_2: &str) -> Self {
        Self {
            compartment_1: String::from(compartment_1),
            compartment_2: String::from(compartment_2),
            duplicated: None,
            duplicated_u32: 0,
        }
    }

    pub fn find_duplicated(&mut self) {
        self.compartment_1.chars().for_each(|c| {
            if let Some(_) = self.compartment_2.find(c) {
                self.duplicated = Some(c);
                self.duplicated_u32 = get_badge_priority(c);
                return;
            }
        });
    }
}

#[derive(Debug)]
struct GroupRusack {
    elf1: String,
    elf2: String,
    elf3: String,
    badge: Option<char>,
    badge_priority: u32,
}

impl GroupRusack {
    pub fn new() -> GroupRusack {
        GroupRusack {
            elf1: String::new(),
            elf2: String::new(),
            elf3: String::new(),
            badge: None,
            badge_priority: 0,
        }
    }

    pub fn add_elf(&mut self, content: &str, elf: u32) {
        match elf {
            1 => self.elf1 = String::from(content),
            2 => self.elf2 = String::from(content),
            3 => self.elf3 = String::from(content),
            _ => return,
        }
    }

    pub fn find_badge(&mut self) {
        self.elf1.chars().for_each(|c| {
            if let Some(_) = self.elf2.find(c) {
                if let Some(_) = self.elf3.find(c) {
                    self.badge = Some(c);
                    return;
                }
            }
        });
        if let Some(badge) = self.badge {
            self.badge_priority = get_badge_priority(badge);
        }
    }
}

fn get_badge_priority(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        c as u32 - 97 + 1
    } else {
        c as u32 - 65 + 27
    }
}

pub fn run_a() {
    println!("Running program for exercise 3a of AOC-2022");

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    // Load rusacks items from file and create a vector of each line
    let rusacks_content = fs::read_to_string(file_path).expect("Could not read the provided file");
    let lines: Vec<&str> = rusacks_content.split('\n').collect();

    println!("Read lines: {:?}", lines);

    let mut rusacks: Vec<Rusack> = Vec::new();
    for line in &lines {
        if line.len() > 0 {
            let mut rusack = Rusack::new(&line[..line.len() / 2], &line[line.len() / 2..]);
            rusack.find_duplicated();
            rusacks.push(rusack);
        }
    }

    println!("Rusacks results {:?}", rusacks);

    let priorities_total = rusacks
        .iter()
        .fold(0, |acc, rusack| acc + rusack.duplicated_u32);

    println!("Rusacks priorities {}", priorities_total);
}

pub fn run_b() {
    println!("Running program for exercise 3b of AOC-2022");

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    // Load rusacks items from file and create a vector of each line
    let rusacks_content = fs::read_to_string(file_path).expect("Could not read the provided file");
    let lines: Vec<&str> = rusacks_content.split('\n').collect();

    println!("Read lines: {:?}", lines);

    let mut group_rusacks: Vec<GroupRusack> = Vec::new();
    let mut group = GroupRusack::new();
    let mut elf: u32 = 1;
    for line in &lines {
        if line.len() > 0 {
            group.add_elf(line, elf);
            if elf == 3 {
                group.find_badge();
                group_rusacks.push(group);
                group = GroupRusack::new();
                elf = 0;
            }
            elf += 1;
        }
    }

    println!("Elf Groups results {:?}", group_rusacks);

    let priorities_total = group_rusacks
        .iter()
        .fold(0, |acc, rusack| acc + rusack.badge_priority);

    println!("Elf Groups priorities {}", priorities_total);
}
