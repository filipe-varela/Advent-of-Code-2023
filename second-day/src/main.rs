use std::fs;
use std::io;

struct Game {
    id: u32,
    blue: u32,
    green: u32,
    red: u32
}

fn main() {
    let contents = fs::read_to_string("../res/second-day/input.txt").expect("The path is wrong!");
    first_task(&contents);
    second_task(&contents);
}

fn second_task(contents: &String) {
    let minimum_power: u32 = retrieve_games(contents).iter()
        .map(|g| g.red * g.blue * g.green)
        .sum();
    println!("Minimum power is: {minimum_power}");
}

fn first_task(contents: &String) {
    let mut input = String::from("");
    println!("How much red: ");
    io::stdin().read_line(&mut input).expect("It was expected a string.");
    let max_red: u32 = input.trim().parse().expect("It must be a valid number > 0");
    println!("How much green: ");
    input.clear();
    io::stdin().read_line(&mut input).expect("It was expected a string.");
    let max_green: u32 = input.trim().parse().expect("It must be a valid number > 0");
    println!("How much blue: ");
    input.clear();
    io::stdin().read_line(&mut input).expect("It was expected a string.");
    let max_blue: u32 = input.trim().parse().expect("It must be a valid number > 0");
    let max_game = Game { id: 0, blue: max_blue, green: max_green, red: max_red };
    let games = retrieve_possible_games(&contents, max_game);
    println!("Games' id added up gives: {games}");
}

fn retrieve_possible_games(contents: &String, max_game: Game) -> u32 {
    retrieve_games(contents).iter()
        .filter(|g| {
            g.red <= max_game.red && g.green <= max_game.green && g.blue <= max_game.blue
        }).map(|g| g.id)
        .sum()
}

fn retrieve_games(contents: &String) -> Vec<Game> {
    contents.trim().split('\n')
        .map(|l| {
            let mut expected_split = String::from("The line wasn't split well: ");
            expected_split.push_str(l);
            let id: u32 = l.split(' ')
                .collect::<Vec<&str>>()
                .get(1).expect(&expected_split.as_str())
                .strip_suffix(":")
                .unwrap_or("0")
                .to_string()
                .parse::<u32>()
                .unwrap_or(0);
            let blue: u32 = l.split(":").collect::<Vec<&str>>()[1].split(" blue").map(|b| {
                b.split(" ")
                    .last()
                    .unwrap_or("0")
                    .parse::<u32>()
                    .unwrap_or(0)
            }).max().unwrap_or(0);
            let green: u32 = l.split(":").collect::<Vec<&str>>()[1].split(" green").map(|b| {
                b.split(" ")
                    .last()
                    .unwrap_or("0")
                    .parse::<u32>()
                    .unwrap_or(0)
            }).max().unwrap_or(0);
            let red: u32 = l.split(":").collect::<Vec<&str>>()[1].split(" red").map(|b| {
                b.split(" ")
                    .last()
                    .unwrap_or("0")
                    .parse::<u32>()
                    .unwrap_or(0)
            }).max().unwrap_or(0);
            Game { id, blue, green, red }
        }).collect()
}
