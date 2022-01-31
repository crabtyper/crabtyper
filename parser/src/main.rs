use std::fs;
use std::fs::File;
use std::io::Write;

fn main() {
    let data = fs::read_to_string("data.txt").expect("Unable to read file");

    let mut result = data.replace("\n", "\\n");
    result = result.replace("\t", "\\t");

    let mut file = File::create("output.txt").unwrap();
    file.write_all(result.as_bytes()).unwrap();
}
