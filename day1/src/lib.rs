pub fn get_input() -> Vec<u32> {
    include_str!("../../inputs/day1/input.txt")
        .split('\n')
        .map(str::parse::<u32>)
        .filter_map(Result::ok)
        .collect::<Vec<_>>()
}

pub fn p1() -> u32 {
    let input = get_input();

    let mut increases = 0;
    let mut prev = None;

    for curr_val in input {
        if let Some(prev_val) = prev {
            if curr_val > prev_val {
                increases += 1;
            }
        }

        prev = Some(curr_val);
    }

    increases
}

pub fn p2() -> u32 {
    let input = get_input();

    let mut increases = 0;
    let mut prev = None;

    for curr_val in input.windows(3) {
        let curr_sum = curr_val.iter().sum::<u32>();

        if let Some(prev_sum) = prev {
            if curr_sum > prev_sum {
                increases += 1;
            }
        }

        prev = Some(curr_sum);
    }

    increases
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
    fn sample_input_works() {
        let answer = p1();
        assert_eq!(answer, 1557);
    }
}
