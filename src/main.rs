mod lib;

use lib::helpers;
use std::env::args;

#[allow(unused_must_use)]
fn main() {
    let args: Vec<String> = args().collect();
    let filename = args[1].clone();
    let search = args[2].clone();
    let lines = helpers::read_file(filename);

    helpers::run(search, lines);
}
