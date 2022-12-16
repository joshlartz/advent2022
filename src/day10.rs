use itertools::Itertools;

type Input = Vec<Instruction>;
type Crt = Vec<Vec<String>>;

#[derive(Clone)]
pub enum Instruction {
    Noop,
    Addx(i32),
}

struct Processor {
    cycle: i32,
    register: i32,
    blocking: u8,
    instruction: Option<Instruction>,
}
impl Processor {
    fn new() -> Processor {
        Processor {
            cycle: 1,
            register: 1,
            blocking: 0,
            instruction: None,
        }
    }

    fn start_processing(&mut self, instruction: Option<&Instruction>) {
        match instruction {
            Some(x) => {
                self.instruction = Some(x.clone());
                match x {
                    Instruction::Noop => self.blocking = 1,
                    Instruction::Addx(_) => self.blocking = 2,
                }
            }
            None => self.blocking = 0,
        }
    }

    fn finish_processing(&mut self) {
        if let Some(Instruction::Addx(x)) = self.instruction { self.register += x }
        self.instruction = None;
    }

    fn tick(&mut self) {
        self.cycle += 1;
        self.blocking -= 1;
    }
}

pub fn generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let instruction = line.split_once(' ');
            match instruction {
                None => Instruction::Noop,
                Some(v) => Instruction::Addx(v.1.parse().unwrap()),
            }
        })
        .collect()
}

pub fn part1(input: &Input) -> i32 {
    let mut processor = Processor::new();
    let mut signal_strengths: Vec<i32> = Vec::new();

    let mut instructions = input.iter();

    while instructions.len() > 0 || processor.instruction.is_some() {
        if processor.blocking == 0 {
            processor.start_processing(instructions.next());
        }

        if processor.cycle == 20 || ((processor.cycle - 20) % 40 == 0) {
            signal_strengths.push(processor.cycle * processor.register);
        }

        processor.tick();
        if processor.blocking == 0 {
            processor.finish_processing();
        }
    }
    signal_strengths.iter().sum()
}

pub fn part2(input: &Input) -> String {
    let mut processor = Processor::new();
    let mut crt: Crt = vec![vec![String::from(" "); 40]; 6];

    let mut instructions = input.iter();

    while instructions.len() > 0 || processor.instruction.is_some() {
        if processor.blocking == 0 {
            processor.start_processing(instructions.next());
        }

        let row = (processor.cycle - 1) / 40;
        let position = (processor.cycle - 1) % 40;
        let sprite = processor.register - 1..=processor.register + 1;
        if sprite.contains(&position) {
            crt[row as usize][position as usize] = "#".to_string();
        }

        processor.tick();
        if processor.blocking == 0 {
            processor.finish_processing();
        }
    }

    convert(&crt)
}

fn convert(crt: &Crt) -> String {
    crt.iter().map(|row| row.join("")).collect_vec().join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(SAMPLE)), 13140);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&generator(SAMPLE)),
            "##  ##  ##  ##  ##  ##  ##  ##  ##  ##  
###   ###   ###   ###   ###   ###   ### 
####    ####    ####    ####    ####    
#####     #####     #####     #####     
######      ######      ######      ####
#######       #######       #######     "
        );
    }
}
