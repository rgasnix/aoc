use std::fs;

fn main() {
    let input: String = read_input("input.txt");
    let vec_input: Vec<&str> = input.split("\n").collect();
    let mut score_part_one: i32 = 0;
    let mut score_part_two: i32 = 0;

    for i in vec_input.iter() {
	    if !i.is_empty() {
	        match i {
		        &"A X" => score_part_one += 4,
		        &"A Y" => score_part_one += 8,
		        &"A Z" => score_part_one += 3,
		        &"B X" => score_part_one += 1,
		        &"B Y" => score_part_one += 5,
		        &"B Z" => score_part_one += 9,
		        &"C X" => score_part_one += 7,
		        &"C Y" => score_part_one += 2,
		        &"C Z" => score_part_one += 6,
		        &_ => score_part_one += 0,
	        }
            match i {   
                &"A X" => score_part_two += 3,
		        &"A Y" => score_part_two += 4,
		        &"A Z" => score_part_two += 8,
		        &"B X" => score_part_two += 1,
		        &"B Y" => score_part_two += 5,
		        &"B Z" => score_part_two += 9,
		        &"C X" => score_part_two += 2,
		        &"C Y" => score_part_two+= 6,
		        &"C Z" => score_part_two += 7,
		        &_ => score_part_two += 0,
            }
	    }
    }

    println!("Score Part One: {0}", score_part_one);
    println!("Score Part Two: {0}", score_part_two);
}

fn read_input(file_path: &str) -> String {
    let input: String = fs::read_to_string(file_path)
	.expect("File not found!");

    input
}
