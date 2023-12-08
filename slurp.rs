use std::fs;

fn slurp_file(file_path: String) -> Vec<String> {
    let contents = fs::read_to_string(file_path).expect("ERROR: Unable to read file");
    let parts = contents.split("\n").map(str::to_string).collect::<Vec<String>>();
    parts
}

fn main() {
    let lines = slurp_file("Hello.txt".to_string());
    for line in lines.iter() {
     	println!("{}", line);
    }

    // let contents = fs::read_to_string("Hello.txt".to_string()).expect("ERROR: unable to read file");
    // let parts = contents.split("\n");
    // let lines = parts.collect::<Vec<&str>>();

    // for line in lines.iter() {
    //     println!("{}", line);
    // }
}
