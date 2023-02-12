use std::fs;

fn main() {
    let input: String = read_input("input.txt");
    let vec_input: Vec<&str> = input.split("\n").collect();
    
    let mut elfs: Vec<i32> = Vec::new();
    let mut sum: i32 = 0;

    for value in vec_input.iter() {
	if value.is_empty() {
	    elfs.push(sum);
	    sum = 0;
	} else {
	    sum += match value.trim().parse::<i32>() {
		Ok(n) => n,
		Err(_) => 0,
	    };
	}
    }

    elfs.sort();

    println!("{0}", elfs[elfs.len()-1]);
}


fn read_input(file_path: &str) -> String {
    let input: String = fs::read_to_string(file_path)
	.expect("File not found!");

    input
}
