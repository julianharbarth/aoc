pub fn aoc1(path: &std::path::Path) -> (i32, i32) {
  let collected = std::fs::read_to_string(path)
    .unwrap()
    .lines()
    .map(|s| s.parse::<i32>().unwrap())
    .collect::<Vec<i32>>();

  let part_one = collected
    .windows(2)
    .fold(0, |acc, p| acc + ((p[0] < p[1]) as i32) );

  let part_two = collected
    .windows(3)
    .zip(collected.windows(3).skip(1))
    .fold(0, |acc, (p1, p2) |
      acc + ((p1.iter().sum::<i32>() < p2.iter().sum::<i32>()) as i32) );

  (part_one, part_two)
}