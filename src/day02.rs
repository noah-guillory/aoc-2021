struct ShipPosition {
  horizontal: u64,
  depth: u64,
}

pub fn part1(inp: String) {
  let position = ShipPosition {
    horizontal: 0,
    depth: 0,
  };

  let instructions: Vec<&str> = inp.split("\n").collect();

  for instruction in instructions {
    println!("{}", instruction)
  }
}

fn evalInstruction(instruction: &str, currentPosition: ShipPosition) -> ShipPosition {
  let parts: Vec<&str> = instruction.split(" ").collect();
  let direction: &str = parts[0];
  let amount: u64 = parts[1].parse().unwrap();

  match direction {
    "forward" => currentPosition.horizontal += amount,
  }
}

pub fn part2(inp: String) {}
