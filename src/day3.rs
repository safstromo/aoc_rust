use crate::common;

fn day3_part1() -> u32 {
    let input = common::get_file("src/input/day3.txt").expect("failed to read file");
    let mut result = 0;
    let backpacks: Vec<(&str, &str)> =
        input.lines()
            .map(|lines| lines.split_at(lines.len() / 2)).collect();

    let mut doubles = vec![];
    for backpack in backpacks {
        let chars = backpack.0.chars();
        for char in chars {
            if backpack.1.contains(char) {
                doubles.push(char);
                break;
            }
        }
    }
    for c in doubles {
        if c.is_ascii_lowercase() {
            result += 1 + c as u32 - 'a' as u32;
        } else if c.is_ascii_uppercase() {
            result += 27 + c as u32 - 'A' as u32;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::day3::day3_part1;

    #[test]
    fn day3_part1_test() {
        assert_eq!(day3_part1(), 7727)
    }
}
