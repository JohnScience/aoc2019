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

#[derive(Debug, Clone)]
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

    // the names of args come from the problem description
    fn pre_run(&mut self, noun: usize, verb: usize) {
        self.0[1] = noun;
        self.0[2] = verb;
    }

    fn run_at_cur(&mut self, cur: usize) -> Option<()> {
        let instruction = Instruction::from_slice(&self.0[cur..]);
        self.run_instruction(instruction)
    }

    fn run_instruction(&mut self, instruction: Instruction) -> Option<()> {
        // println!("{:?}", self);
        // println!("{:?}", instruction);
        match instruction {
            Instruction::Add {
                a_pos,
                b_pos,
                sum_pos,
            } => {
                let sum = self.0[a_pos] + self.0[b_pos];
                // println!("program[{sum_pos}]: {} -> {sum}", self.0[sum_pos]);
                self.0[sum_pos] = sum;
                Some(())
            }
            Instruction::Mul {
                a_pos,
                b_pos,
                product_pos,
            } => {
                let product = self.0[a_pos] * self.0[b_pos];
                // println!(
                //     "program[{product_pos}]: {} -> {product}",
                //     self.0[product_pos]
                // );
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
    let program: Program = input
        .trim_end()
        .split(',')
        .map(|x| x.parse::<u8>().unwrap() as usize)
        .collect();
    for noun in 0..=99 {
        for verb in 0..=99 {
            println!("noun: {noun}, verb: {verb}");
            let mut program = program.clone();
            program.pre_run(noun, verb);
            program.run();
            if program.pos0() == 19690720 {
                println!("noun: {}, verb: {}", noun, verb);
                println!("100 * noun + verb = {}", 100 * noun + verb);
                return;
            }
        }
    }
}
