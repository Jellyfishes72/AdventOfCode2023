use std::fs;

fn slurp_file(file_path: String) -> Vec<String> {
    let contents = fs::read_to_string(file_path).expect("ERROR: Unable to read file");
    let parts = contents.split("\n").map(str::to_string).collect::<Vec<String>>();
    parts
}

fn part_1() {
    let lines = slurp_file("input.txt".to_string());
    let mut total = 0;
    let mut first: String = "".to_string();
    let mut last: String = "".to_string();
    let mut found_first = false;

    for line in lines {
        for ch in line.chars() {
            if ch.is_digit(10) && !found_first {
		first = ch.to_string();
		found_first = true;
	    }
	    if ch.is_digit(10) && found_first {
                last = ch.to_string();
	    }
	}
	if line.len() > 0 {
	    found_first = false;
	    let calibration = first.clone() + &last;
	    println!("Calibration value = {calibration}");
	    total += calibration.parse::<i32>().expect("Unable to parse number");
	}
    }

    println!("Total: {total}");
}

fn starts_with(string: &String, pattern: &String, pos: usize) -> bool {
    if string.len() < pattern.len() { return false; }
    if pos + pattern.len() > string.len() {return false; }

    for i in 0..pattern.len() {
	if string.as_bytes()[pos + i] != pattern.as_bytes()[i] {
	    return false;
	}
    }

    return true;
}

fn main () {
    let lines = slurp_file("example2.txt".to_string());
    let mut total = 0;
    let mut first: String = "".to_string();
    let mut last: String = "".to_string();
    let mut found_first = false;
    let numbers = ["zero".to_string(), "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), "eight".to_string(), "nine".to_string()];
	

    for line in lines {
	for i in 0..line.len() {
            if line.as_bytes()[i].is_ascii_digit() && !found_first {
		first = line.as_bytes()[i].to_string();
		found_first = true;
	    }
	    if line.as_bytes()[i].is_ascii_digit() && found_first {
                last = line.as_bytes()[i].to_string();
	    }

	    let mut j = 0;
            while j < line.len() {
		if starts_with(&line, &numbers[j], i) && !found_first {
                    first = j.to_string();
		    found_first = true;
		}
		if starts_with(&line, &numbers[j], i) && found_first {
                    last = j.to_string();
		}
		j += 1;
            }
	}
	if line.len() > 0 {
	    found_first = false;
	    let calibration = first.clone() + &last;
	    println!("Calibration value = {calibration}");
	    total += calibration.parse::<i32>().expect("Unable to parse number");
	}
    }

    println!("Total: {total}");
}
