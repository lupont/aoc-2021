fn get_input() -> Vec<&'static str> {
    include_str!("../../inputs/day2/input.txt")
        .split('\n')
        .filter(|s| !s.is_empty())
        .collect()
}

pub fn p1_first() -> u32 {
    let input = get_input();
    let mut initial: (i32, i32) = (0, 0);

    for instruction in input {
        let (dir, amt) = instruction.split_once(' ').unwrap();
        let amt = amt.parse::<i32>().unwrap();

        match dir {
            "forward" => initial.0 += amt,
            "up" => initial.1 -= amt,
            "down" => initial.1 += amt,
            _ => (),
        }
    }

    (initial.0 * initial.1) as u32
}

pub fn p1_cooler() -> u32 {
    let (hor, depth) = get_input()
        .iter()
        .filter_map(|instruction| instruction.split_once(' '))
        .map(|(dir, delta)| (dir, delta.parse::<i32>().unwrap()))
        .fold((0, 0), |(x, y), (dir, delta)| match dir {
            "forward" => (x + delta, y),
            "up" => (x, y - delta),
            "down" => (x, y + delta),
            _ => (x, y),
        });
    (hor * depth) as u32
}

pub fn p1_cooler_2() -> u32 {
    let f = get_input()
        .iter()
        .filter_map(|instruction| instruction.split_once(' '))
        .map(|(dir, delta)| (dir, delta.parse::<i32>().unwrap()))
        .map(|(dir, delta)| match dir {
            "forward" => (delta, 0),
            "up" => (0, -delta),
            "down" => (0, delta),
            _ => (0, 0),
        })
        .fold((0, 0), |(x, y), (dx, dy)| (x + dx, y + dy));
        
    (f.0 * f.1) as u32
}

pub fn p2_first() -> u32 {
    let input = get_input();

    let (mut hor, mut depth, mut aim): (i32, i32, i32) = (0, 0, 0);

    for instruction in input {
        let (dir, amt) = instruction.split_once(' ').unwrap();
        let amt = amt.parse::<i32>();
        if let Ok(amt) = amt {
            match dir {
                "forward" => {
                    hor += amt;
                    depth += amt * aim;
                }

                "up" => aim -= amt,
                "down" => aim += amt,
                _ => (),
            }
        }
    }

    (hor * depth) as u32
}

pub fn p2_cooler() -> u32 {
    let (hor, depth, _) = get_input()
        .iter()
        .filter_map(|instruction| instruction.split_once(' '))
        .map(|(dir, delta)| (dir, delta.parse::<i32>().unwrap(), 0))
        .fold((0, 0, 0), |(hor, depth, aim), (dir, delta, _)| match dir {
            "forward" => (hor + delta, depth + aim * delta, aim),
            "up" => (hor, depth, aim - delta),
            "down" => (hor, depth, aim + delta),
            _ => (hor, depth, aim),
        });

    (hor * depth) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_is_correct() {
        let input = get_input();
        assert_eq!(input[0], "forward 8");
        assert_eq!(input.len(), 1000);
        assert_eq!(input.last(), Some(&"forward 4"));
    }

    #[test]
    fn first_problem_is_correct() {
        assert_eq!(p1_first(), 1855814);
        assert_eq!(p1_cooler(), 1855814);
        assert_eq!(p1_cooler_2(), 1855814);
    }

    #[test]
    fn second_problem_is_correct() {
        assert_eq!(p2_first(), 1845455714);
        assert_eq!(p2_cooler(), 1845455714);
    }
}
