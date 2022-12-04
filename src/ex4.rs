use std::collections::HashSet;
use std::fmt::Display;
use std::{env, fs};

#[derive(Debug)]
struct CleanSection {
    elf1: HashSet<u32>,
    elf2: HashSet<u32>,
    has_fully_contained: bool,
    has_partial_contained: bool,
    elf_fully_contained: Option<u32>,
}

impl Display for CleanSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Cleaning sections: Elf1 ({:?}),  Elf2 ({:?})",
            self.elf1, self.elf2
        )
    }
}

impl CleanSection {
    pub fn new(range1: &str, range2: &str) -> Self {
        let (start1, end1) = Self::parse_range(range1);
        let (start2, end2) = Self::parse_range(range2);
        Self {
            elf1: (start1..end1 + 1).collect(),
            elf2: (start2..end2 + 1).collect(),
            has_fully_contained: false,
            has_partial_contained: false,
            elf_fully_contained: None
        }
    }

    fn parse_range(range: &str) -> (u32, u32) {
        let (start, end) = range.split_once("-").unwrap();
        (start.parse::<u32>().unwrap(), end.parse::<u32>().unwrap())
    }

    pub fn check_contained(&mut self) {
        let contained = &self.elf1 & &self.elf2;
        if contained.len() > 0 {
            self.has_partial_contained = true;
        }
        if contained.len() == self.elf1.len() {
            self.has_fully_contained = true;
            self.elf_fully_contained = Some(1);
        } else if contained.len() == self.elf2.len() {
            self.has_fully_contained = true;
            self.elf_fully_contained = Some(2);
        }
    }
}

pub fn run_a() {
    println!("Running program for exercise 4a of AOC-2022");

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    // Load rusacks items from file and create a vector of each line
    let rusacks_content = fs::read_to_string(file_path).expect("Could not read the provided file");
    let lines: Vec<&str> = rusacks_content.split('\n').collect();

    println!("Read lines: {:?}", lines);

    let mut cleaning_sections: Vec<CleanSection> = Vec::new();
    for line in &lines {
        if line.len() > 0 {
            let (region1, region2) = line.split_once(",").unwrap();
            let mut clean_section = CleanSection::new(region1, region2);
            clean_section.check_contained();
            cleaning_sections.push(clean_section);
        }
    }

    println!("Cleaning Sections results:");
    for section in &cleaning_sections {
        println!("{}", section);
    }

    let partial_contained_total : Vec<CleanSection>= cleaning_sections
        .into_iter()
        .filter(|section| section.has_partial_contained).collect();
    // Save length here to move owner bellow
    let partial_contained_count = partial_contained_total.len();

    let fully_contained_total : Vec<CleanSection>= partial_contained_total
        .into_iter()
        .filter(|section| section.has_fully_contained).collect();

    println!("Cleaning Sections fully overlapped {}, with partials overlapped at {}", fully_contained_total.len(), partial_contained_count);
}
