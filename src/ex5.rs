use std::collections::HashSet;
use std::fmt::Display;
use std::{env, fs};

#[derive(Debug)]
struct Procedure {
    quantity: i32,
    from: i32,
    to: i32,
}

#[derive(Debug)]
struct Warehouse {
    layout: Vec<Vec<char>>,
    procedures: Vec<Procedure>,
}

impl Warehouse {
    pub fn new(layout: &str, procedure: &str) -> Self {
        // Parse input data into creates letters
        let mut parsed_layout: Vec<Vec<char>> = layout
            .lines()
            .map(|line| {
                line.as_bytes()
                    .chunks(4)
                    .map(|chunk| chunk[1] as char)
                    .collect()
            })
            .collect();
        // Discard last line has it contains the columns indexes
        parsed_layout.pop();

        let mut layout: Vec<Vec<char>> = vec![Vec::new(); parsed_layout[0].len()];

        for level in parsed_layout.into_iter().rev() {
            level.into_iter().enumerate().for_each(|(i, c)| {
                if c != ' ' {
                    layout[i].push(c);
                }
            });
        }

        //Parse instructions
        let procedures = procedure
            .lines()
            .map(|line| {
                let parsed_instructions = line
                    .split(' ')
                    .into_iter()
                    .filter_map(|word| word.parse::<i32>().ok())
                    .collect::<Vec<_>>();
                parsed_instructions
            })
            .map(|item| Procedure {
                quantity: *item.get(0).unwrap(),
                from: *item.get(1).unwrap(),
                to: *item.get(2).unwrap(),
            })
            .collect();
        Self {
            layout,
            procedures,
        }
    }

    fn get_crates<T>(layout: &mut Vec<T>, from: i32, to: i32) -> (&mut T, &mut T) {
        if from > to {
            let (left, right) = layout.split_at_mut(from as usize);
            (&mut right[0], &mut left[to as usize])
        } else {
            let (left, right) = layout.split_at_mut(to as usize);
            (&mut left[from as usize], &mut right[0])
        }
    }

    pub fn apply_procedures_a(&mut self) {
        for procedure in &self.procedures {
                let (stack_from, stack_to) = Self::get_crates(&mut self.layout, procedure.from - 1, procedure.to - 1);
                let moved_boxes = &stack_from[stack_from.len() - procedure.quantity as usize..];
                stack_to.extend(moved_boxes.into_iter().rev());
                stack_from.truncate(stack_from.len() - procedure.quantity as usize);
                println!("Layout after {:?}: {:?}", procedure, self.layout);
        }
    }
    pub fn apply_procedures_b(&mut self) {
        for procedure in &self.procedures {
                let (stack_from, stack_to) = Self::get_crates(&mut self.layout, procedure.from - 1, procedure.to - 1);
                let moved_boxes = &stack_from[stack_from.len() - procedure.quantity as usize..];
                stack_to.extend(moved_boxes);
                stack_from.truncate(stack_from.len() - procedure.quantity as usize);
                println!("Layout after {:?}: {:?}", procedure, self.layout);
        }
    }
}

pub fn run_a() {
    println!("Running program for exercise 5a of AOC-2022");

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    // Load supply map from file
    let supply_map = fs::read_to_string(file_path).expect("Could not read the provided file");
    let (layout, procedure) = supply_map.split_once("\n\n").unwrap();

    let mut warehouse = Warehouse::new(layout, procedure);
    println!("Board: {:?}", warehouse.layout);
    println!("Procedure: {:?}", warehouse.procedures);
    warehouse.apply_procedures_a();
    let final_layout = warehouse.layout.iter()
        .fold(String::new(), |acc, column| format!("{}{}", acc,column[column.len() - 1]));
    println!("Final layout: {}", final_layout);
}

pub fn run_b() {
    println!("Running program for exercise 5b of AOC-2022");

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    // Load supply map from file
    let supply_map = fs::read_to_string(file_path).expect("Could not read the provided file");
    let (layout, procedure) = supply_map.split_once("\n\n").unwrap();

    let mut warehouse = Warehouse::new(layout, procedure);
    println!("Board: {:?}", warehouse.layout);
    println!("Procedure: {:?}", warehouse.procedures);
    warehouse.apply_procedures_b();
    let final_layout = warehouse.layout.iter()
        .fold(String::new(), |acc, column| format!("{}{}", acc,column[column.len() - 1]));
    println!("Final layout: {}", final_layout);
}
