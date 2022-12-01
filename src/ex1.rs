use std::{fs, env, cmp::Ordering, fmt::Display};

#[derive(Debug, Eq, PartialEq)]
struct Elf {
    id: i32,
    calories: i32,
}

impl Display for Elf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Elf {} has {} calories", self.id, self.calories)
    }
}

impl Ord for Elf {
    fn cmp(&self, other:&Self) -> Ordering {
        (self.calories).cmp(&(other.calories))
    }
}

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


pub fn run() {
    println!("Running program for exercise 1 of AOC-2022");

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    // Load elfs calories list from file and create a vector of each line
    let calories_list = fs::read_to_string(file_path).expect("Could not read the provided file");
    let lines: Vec<&str> = calories_list.split('\n').collect();

    // Aggregate elfs categories
    let mut elfs: Vec<Elf> = Vec::new();
    let mut elf_calories_sum = 0;
    for line in &lines {
        if line.len() == 0 {
            elfs.push(Elf {
                id: elfs.len() as i32 + 1,
                calories: elf_calories_sum
            });
            elf_calories_sum = 0;
        } else {
            elf_calories_sum += line
                .parse::<i32>()
                .expect("Found a non numeric calory value");
        }
    }

    // Sorting the elf vector and find the top 3 elfs
    elfs.sort();
    let elfs_count = elfs.len();
    println!("Top 3 Elfs calories are: {:?}", &elfs[elfs_count - 3..elfs_count]);
    let mut top_calories_sum = 0;
    let _ = &elfs[elfs_count - 3..elfs_count].iter().for_each(|elf| top_calories_sum+= elf.calories);
    println!("Top 3 Elfs calories total is: {}", top_calories_sum);
}
