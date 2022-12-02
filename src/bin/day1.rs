use std::fs;

#[derive(Debug)]
#[allow(dead_code)]
#[derive(Eq)]
struct Elf {
    index: i32,
    calories: usize,
}
impl Ord for Elf {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.calories.cmp(&self.calories)
    }
}
impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.calories.cmp(&self.calories))
    }
}

impl PartialEq for Elf {
    fn eq(&self, other: &Self) -> bool {
        other.calories == self.calories
    }
}

impl Elf {
    fn new(index: i32, calories: usize) -> Self {
        Self { index, calories }
    }
}

fn main() {
    let input = fs::read_to_string("data/day1.txt").unwrap();

    let mut elfs = vec![];

    let mut index = 1;
    let mut calories = 0;

    for line in input.split("\n") {
        let line = line.trim();

        if line.is_empty() {
            elfs.push(Elf::new(index, calories));

            calories = 0;
            index += 1;
        } else {
            calories += line.parse::<usize>().unwrap();
        }
    }

    elfs.sort();
    let part1 = elfs.first();
    println!("Part 1: {:?}", part1.unwrap().calories);

    let part2 = elfs
        .iter()
        .take(3)
        .fold(
            Elf {
                index: 0,
                calories: 0,
            },
            |a, b| Elf {
                index: 0,
                calories: a.calories + b.calories,
            },
        )
        .calories;
    println!("Part 2: {:#?}", part2);
}
