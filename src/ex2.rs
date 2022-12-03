use std::{env, fs};

/*
 * Rock | paper | scissors
 * A    | B     | C
 * X    | Y     | Z
 * 1    | 2     | 3
 */
#[derive(Debug)]
struct Game1 {
    elf: i32,
    player: i32,
    score: i32,
}

impl Game1 {
    pub fn new(elf: i32, player: i32) -> Game1 {
        Game1 {
            elf,
            player,
            score: Self::get_challenge_result(elf, player),
        }
    }

    fn get_challenge_result(elf: i32, player: i32) -> i32 {
        let result = match (elf, player) {
            (1, 1) => 3,
            (2, 2) => 3,
            (3, 3) => 3,
            (1, 2) => 6,
            (2, 3) => 6,
            (3, 1) => 6,
            _ => 0,
        };

        player + result
    }
    pub fn decode_shape(shape: &str) -> i32 {
        match shape {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => panic!("Wrong shape provided: {}", shape),
        }
    }
}

#[derive(Debug)]
struct Game2 {
    elf: i32,
    result: i32,
    player: i32,
    score: i32,
}

impl Game2 {
    pub fn new(elf: i32, result: i32) -> Self {
        let (player, score) = Self::get_player_choice(elf, result);
        Game2 {
            elf,
            result,
            player,
            score,
        }
    }

    fn get_player_choice(elf: i32, result: i32) -> (i32, i32) {
        let player = match (elf, result) {
            (a, 3) => a,
            (1, 0) => 3,
            (2, 0) => 1,
            (3, 0) => 2,
            (1, 6) => 2,
            (2, 6) => 3,
            (3, 6) => 1,
            _ => 0,
        };

        (player, player + result)
    }
    pub fn decode_shape(shape: &str) -> i32 {
        match shape {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => panic!("Wrong shape provided: {}", shape),
        }
    }
}

pub fn run_a() {
    println!("Running program for exercise 2a of AOC-2022");

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    // Load strategy guide from file and create a vector of each line
    let strategy_guide = fs::read_to_string(file_path).expect("Could not read the provided file");
    let lines: Vec<&str> = strategy_guide.split('\n').collect();

    println!("Read lines: {:?}", lines);

    let mut game_history: Vec<Game1> = Vec::new();
    for line in &lines {
        if line.len() > 0 {
            let guesses: Vec<&str> = line.split(' ').collect();
            let elf_score = Game1::decode_shape(guesses[0]);
            let player_score = Game1::decode_shape(guesses[1]);
            game_history.push(Game1::new(elf_score, player_score))
        }
    }

    println!("Game results {:?}", game_history);

    let player_result = game_history.iter().fold(0, |acc, game| acc + game.score);

    println!("Player result {}", player_result);
}

pub fn run_b() {
    println!("Running program for exercise 2b of AOC-2022");

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    // Load strategy guide from file and create a vector of each line
    let strategy_guide = fs::read_to_string(file_path).expect("Could not read the provided file");
    let lines: Vec<&str> = strategy_guide.split('\n').collect();

    println!("Read lines: {:?}", lines);

    let mut game_history: Vec<Game2> = Vec::new();
    for line in &lines {
        if line.len() > 0 {
            let guesses: Vec<&str> = line.split(' ').collect();
            let elf_score = Game2::decode_shape(guesses[0]);
            let game_result = Game2::decode_shape(guesses[1]);
            game_history.push(Game2::new(elf_score, game_result))
        }
    }

    println!("Game results {:?}", game_history);

    let player_result = game_history.iter().fold(0, |acc, game| acc + game.score);

    println!("Player result {}", player_result);
}
