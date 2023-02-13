use std::fs;

fn main() {
    let input: String = read_input("input.txt");
    let vec_input: Vec<&str> = input.split("\n").collect();
    let mut score: i32 = 0;

    for i in vec_input.iter() {
	    if !i.is_empty() {
	        match i {
		        &"A X" => score += 4,
		        &"A Y" => score += 8,
		        &"A Z" => score += 3,
		        &"B X" => score += 1,
		        &"B Y" => score += 5,
		        &"B Z" => score += 9,
		        &"C X" => score += 7,
		        &"C Y" => score += 2,
		        &"C Z" => score += 6,
		        &_ => score += 0,
	        }
	    }
    }

    println!("Score: {0}", score);
}

fn read_input(file_path: &str) -> String {
    let input: String = fs::read_to_string(file_path)
	.expect("File not found!");

    input
}
