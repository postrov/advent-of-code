use DAY_TEMPLATE_MOD::process_part1;
use std::fs;


fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process_part1(&file));
}
