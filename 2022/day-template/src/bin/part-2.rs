use DAY_TEMPLATE_MOD::process_part2;
use std::fs;


fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process_part2(&file));
}
