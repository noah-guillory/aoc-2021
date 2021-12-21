pub fn part1(inp: String) {
  let mut changes = 0;
  let mut last_measurement: i32 = -1;
  let measurements: Vec<i32> = inp
    .lines()
    .into_iter()
    .map(|s| s.parse::<i32>().unwrap())
    .collect();

  for measurement in measurements.iter() {
    if measurement > &last_measurement && last_measurement != -1 {
      changes += 1;
    }
    last_measurement = *measurement;
  }

  println!("{}", changes);
}

pub fn part2(inp: String) {
  let measurements: Vec<i32> = inp
    .lines()
    .into_iter()
    .map(|s| s.parse::<i32>().unwrap())
    .collect();
}
