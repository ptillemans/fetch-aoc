#![feature(test)]
use aoc_{{year}}_{{day}}::{AocError, InputModel};

const INPUT: &str = include_str!("../data/input.txt");


fn part1(_input: &InputModel) -> Result<String,AocError> {
    return Ok("Not implemented".to_string())
}

fn part2(_input: &InputModel) -> Result<String, AocError> {
    return Ok("Not implemented".to_string())
}

fn main() -> Result<(), AocError> {
    let input:InputModel = INPUT.parse::<InputModel>()?;
    let part1_result = part1(&input)?;
    println!("Part1: {}", part1_result);
    println!("--------------");
    let part2_result = part2(&input)?;
    println!("Part2: {}", part2_result);
    Ok(())
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use test::Bencher;

    const TEST_INPUT: &str = "";

    pub fn input_data() -> InputModel {
        InputModel {
        }
    }

    #[test]
    fn test_parse() {
        let actual = TEST_INPUT.parse::<InputModel>().unwrap();
        let expected = input_data();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1() {
        let actual = part1(&input_data()).unwrap();
        let expected = "";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let actual = part2(&input_data()).unwrap();
        let expected = "";

        assert_eq!(actual, expected);
    }
    
    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| INPUT.parse::<InputModel>().unwrap())
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let data = INPUT.parse::<InputModel>().unwrap();
        b.iter(|| part1(&data))
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let data = INPUT.parse::<InputModel>().unwrap();
        b.iter(|| part2(&data))
    }

}
