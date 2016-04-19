extern crate english_lint;

use std::env;
use std::io::Read;
use std::fs::File;
use english_lint::lint;

fn main() {
    let mut input = String::new();
    let arg = env::args().skip(1).next();

    if let Some(file_name) = arg {
        let mut file = File::open(file_name).unwrap();
        file.read_to_string(&mut input).unwrap();
        drop(file);
    } else {
        std::io::stdin().read_to_string(&mut input).unwrap();
    }

    let results = lint(&input);
    for hint in &results {
        println!("{}", hint);
    }
}
