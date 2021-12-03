enum Direction {
  Forward,
  Down,
  Up,
}

impl TryFrom<&str> for Direction {
  type Error = String;
  fn try_from(str: &str) -> Result<Self, Self::Error> {
    match str {
      "forward" => Ok(Direction::Forward),
      "down" => Ok(Direction::Down),
      "up" => Ok(Direction::Up),
      _ => Err(format!("Could not determine Direction from string {}", str))
    }
  }
}

pub fn aoc2(path: &std::path::Path) -> (i32, i32) {
  let collected = std::fs::read_to_string(path)
    .unwrap()
    .lines()
    .map(|s| s.split(' ').collect::<Vec<&str>>())
    .map(|split| (split[0].try_into().unwrap(), split[1].parse::<i32>().unwrap()))
    .collect::<Vec<(Direction, i32)>>();

  let (x, y) = collected
    .iter()
    .fold((0, 0), |(x, y), (dir, amount)| match dir {
      Direction::Forward => (x + amount, y),
      Direction::Down => (x, y + amount),
      Direction::Up => (x, y - amount),
    });
  let part_one = x * y;

  let (x, y, _) = collected.iter()
    .fold((0, 0, 0), |(x, y, aim), (dir, amount)| match dir {
      Direction::Forward => (x + amount, y + (aim * amount), aim),
      Direction::Down => (x, y, aim + amount),
      Direction::Up => (x, y, aim - amount),
    });
  let part_two = x * y;

  (part_one, part_two)
}
