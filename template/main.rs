
const INPUT: &str = include_str!("../data/input.txt");


#[derive(Debug)]
struct InputModel  {
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Error parsing the input")]
    ParseError,
}
        
impl TryFrom<String> for InputModel {
    type Error = AocError;

    fn try_from(_value: String) -> Result<Self, Self::Error> {
        todo!()
    }
}

fn part1(_input: &InputModel) -> Result<String,AocError> {
    return Ok("Not implemented".to_string())
}

fn part2(_input: &InputModel) -> Result<String, AocError> {
    return Ok("Not implemented".to_string())
}

fn main() -> Result<(), AocError> {
    let input:InputModel = InputModel::try_from(INPUT.to_string())?;
    let part1_result = part1(&input)?;
    println!("Part1: {}", part1_result);
    println!("--------------");
    let part2_result = part2(&input)?;
    println!("Part2: {}", part2_result);
    Ok(())
}
