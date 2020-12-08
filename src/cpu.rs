impl CPU {
    pub fn new(input: &Vec<Instruction>) -> Self {
        Self::new_owned(input.clone())
    }

    pub fn new_owned(input: Vec<Instruction>) -> Self {
        Self {
            pos: 0,
            acc: 0,
            input,
        }
    }

    pub fn next(&mut self) -> Option<usize> {
        if self.pos as usize >= self.input.len() {
            return None; // finished execution
        }

        self.pos += match self.input[self.pos as usize] {
            Instruction::Acc(a) => {
                self.acc += a;
                1
            }
            Instruction::Jmp(a) => a,
            Instruction::Nop(_) => 1,
        };

        Some(self.pos as usize)
    }

    pub fn get_acc_value(&self) -> i64 {
        self.acc
    }
}

impl From<&str> for Instruction {
    fn from(val: &str) -> Self {
        use regex::Regex;
        lazy_static! {
            static ref TO_INSTRUCTION: Regex = Regex::new(r"^(?P<ins>\w{3}) (?P<cnt>.*)$").unwrap();
        }

        let (ins, cnt) = TO_INSTRUCTION
            .captures_iter(val)
            .map(|cap| (cap["ins"].to_owned(), cap["cnt"].parse::<i64>().unwrap()))
            .last()
            .unwrap();

        match ins.as_str() {
            "acc" => Instruction::Acc(cnt),
            "jmp" => Instruction::Jmp(cnt),
            "nop" => Instruction::Nop(cnt),
            _ => panic!("Uncovered op {}", ins),
        }
    }
}

#[derive(Debug)]
pub struct CPU {
    pos: i64,
    acc: i64,
    input: Vec<Instruction>,
}

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}
