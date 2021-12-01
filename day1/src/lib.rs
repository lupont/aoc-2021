fn get_input() -> Vec<u32> {
    include_str!("../../inputs/day1/input.txt")
        .split('\n')
        .map(str::parse::<u32>)
        .filter_map(Result::ok)
        .collect::<Vec<_>>()
}

pub fn p1() -> u32 {
    get_input()
        .windows(2)
        .filter(|w| w[1] > w[0])
        .count() as u32
}

pub fn p2() -> u32 {
    get_input()
        .windows(3)
        .map(|w| w.iter().sum::<u32>())
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|w| w[1] > w[0])
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_is_correct() {
        let input = get_input();
        assert_eq!(input.len(), 2000);
    }

    #[test]
    fn first_problem_is_correct() {
        let answer = p1();
        assert_eq!(answer, 1557);
    }

    #[test]
    fn second_problem_is_correct() {
        let answer = p2();
        assert_eq!(answer, 1608);
    }
}
