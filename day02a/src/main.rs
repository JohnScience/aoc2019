#[derive(Debug)]
enum Instruction {
    Add {
        a_pos: usize,
        b_pos: usize,
        sum_pos: usize,
    },
    Mul {
        a_pos: usize,
        b_pos: usize,
        product_pos: usize,
    },
    Hcf,
}

#[derive(Debug)]
struct Program(Vec<usize>);

impl Instruction {
    fn from_slice(slice: &[usize]) -> Self {
        match slice {
            [1, a_pos, b_pos, sum_pos, ..] => Instruction::Add {
                a_pos: *a_pos,
                b_pos: *b_pos,
                sum_pos: *sum_pos,
            },
            [2, a_pos, b_pos, product_pos, ..] => Instruction::Mul {
                a_pos: *a_pos,
                b_pos: *b_pos,
                product_pos: *product_pos,
            },
            [99, ..] => Instruction::Hcf,
            _ => panic!("Invalid opcode or slice too short"),
        }
    }
}

impl FromIterator<usize> for Program {
    fn from_iter<I: IntoIterator<Item = usize>>(iter: I) -> Self {
        Program(iter.into_iter().collect())
    }
}

impl Program {
    fn pos0(&self) -> usize {
        self.0[0]
    }

    fn pre_run(&mut self) {
        self.0[1] = 12;
        self.0[2] = 2;
    }

    fn run_at_cur(&mut self, cur: usize) -> Option<()> {
        let instruction = Instruction::from_slice(&self.0[cur..]);
        self.run_instruction(instruction)
    }

    fn run_instruction(&mut self, instruction: Instruction) -> Option<()> {
        println!("{:?}", self);
        println!("{:?}", instruction);
        match instruction {
            Instruction::Add {
                a_pos,
                b_pos,
                sum_pos,
            } => {
                let sum = self.0[a_pos] + self.0[b_pos];
                println!("program[{sum_pos}]: {} -> {sum}", self.0[sum_pos]);
                self.0[sum_pos] = sum;
                Some(())
            }
            Instruction::Mul {
                a_pos,
                b_pos,
                product_pos,
            } => {
                let product = self.0[a_pos] * self.0[b_pos];
                println!(
                    "program[{product_pos}]: {} -> {product}",
                    self.0[product_pos]
                );
                self.0[product_pos] = product;
                Some(())
            }
            Instruction::Hcf => None,
        }
    }

    fn run(&mut self) {
        for instr_pos in (0..self.0.len()).step_by(4) {
            if let Some(()) = self.run_at_cur(instr_pos) {
                continue;
            } else {
                break;
            }
        }
    }
}

fn main() {
    let input = std::fs::read_to_string(r"C:\Users\USER\Documents\github\aoc2019\day02a\input.txt")
        .unwrap();
    let mut program: Program = input
        .trim_end()
        .split(',')
        .map(|x| x.parse::<u8>().unwrap() as usize)
        .collect();
    program.pre_run();
    program.run();
    let pos0 = program.pos0();
    println!("{pos0}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program() {
        let mut program: Program = vec![1, 0, 0, 0, 99].into_iter().collect();
        program.run();
        assert_eq!(program.0, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_program2() {
        let mut program: Program = vec![2, 3, 0, 3, 99].into_iter().collect();
        program.run();
        assert_eq!(program.0, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_program3() {
        let mut program: Program = vec![2, 4, 4, 5, 99, 0].into_iter().collect();
        program.run();
        assert_eq!(program.0, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_program4() {
        let mut program: Program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99].into_iter().collect();
        program.run();
        assert_eq!(program.0, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn test_program5() {
        let mut program: Program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]
            .into_iter()
            .collect();
        program.run();
        assert_eq!(
            program.0,
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
    }
}
