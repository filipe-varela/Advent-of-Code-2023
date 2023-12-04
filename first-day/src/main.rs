use std::fs;

fn main() {
    let contents = fs::read_to_string("../res/first-day/input.txt").expect("The path is wrong!");
    let mut a: u32;
    let mut i: u32 = 1;
    let mut sum: u32 = 0;
    let mut parsed_line: String;
    for line in contents.trim().split("\n") {
        parsed_line = clear_spelled_digits(line);
        a = 10 * find_digit(&parsed_line.chars().collect()) + find_digit(&parsed_line.chars().rev().collect());
        println!("Line in {i} ({line}) becames {parsed_line} and results in {a}");
        sum += a;
        i += 1;
    }
    println!("Sum: \n{sum}");
}

fn clear_spelled_digits(line: &str) -> String {
    let mut line_with_numbers = String::new();
    for c in line.chars() {
        line_with_numbers.push(c);
        line_with_numbers = line_with_numbers
            .replace("one", "1ne")
            .replace("two", "2wo")
            .replace("three", "3hree")
            .replace("four", "4our")
            .replace("five", "5ive")
            .replace("six", "6ix")
            .replace("seven", "7even")
            .replace("eight", "8ight")
            .replace("nine", "9ine");
    }
    line_with_numbers
}

fn find_digit(line: &Vec<char>) -> u32 {
    return line.iter()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap_or(0))
        .collect::<Vec<_>>()[0];
}