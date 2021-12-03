mod aoc;

use std::path::Path;

fn main() {

  { // Day 1
   let path = Path::new("files/input1");
   let (part_one, part_two) = aoc::aoc1(path);
   println!("AoC1:\n Part One: {}\n Part Two: {}\n", part_one, part_two);
  }

  { // Day 2
    let path = Path::new("files/input2");
    let (part_one, part_two) = aoc::aoc2(path);
    println!("AoC2:\n Part One: {}\n Part Two: {}\n", part_one, part_two);
  }

}
