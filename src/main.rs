use std::process::exit;
use std::fs;
use std::env;
use std::result;
use std::error::Error;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Instruction {
    Left,
    Right,
    Incr,
    Decr,
    Output,
    Read,
    Begin,
    End,
    Unknown,
}

const MEM_SIZE: usize = 30_000;

struct VM {
    prog: Vec<Instruction>,
    prog_meta: Vec<u32>,
    mem: [u8; MEM_SIZE],
    instr_ptr: usize,
    dat_ptr: usize,
}

impl VM {
    fn new() -> VM {
        VM { prog: Vec::new(), prog_meta: Vec::new(), mem: [0; MEM_SIZE], instr_ptr: 0, dat_ptr: 0 }
    }

    fn clear_all(&mut self) {
        self.prog.clear();
        self.prog_meta.clear();
        self.mem = [0; MEM_SIZE];
        self.instr_ptr = 0;
        self.dat_ptr = 0;
    }

    fn load_program(&mut self, src: &str) -> Result<()> {
        self.clear_all();
        let mut brk: Vec<usize> = Vec::new();

        for c in src.chars() {
            let mut pmeta: u32 = 0;
            let instr = match c {
                '>' => Instruction::Right,
                '<' => Instruction::Left,
                '+' => Instruction::Incr,
                '-' => Instruction::Decr,
                '.' => Instruction::Output,
                ',' => Instruction::Read,
                '[' => Instruction::Begin,
                ']' => Instruction::End,
                _   => Instruction::Unknown,
            };
            if instr==Instruction::Unknown {
                println!("Unknown instruction {} encountered", c);
                continue;
            }            
            else if instr==Instruction::Begin {
                brk.push(self.prog.len());
            }
            else if instr==Instruction::End {
                if let Some(b) = brk.pop() {
                    pmeta = b as u32;
                    self.prog_meta[b] = self.prog.len() as u32;
                }
                else {             
                    return err!("Brackets do not match in program!");
                }
            }
            self.prog.push(instr);
            self.prog_meta.push(pmeta);
        }

        Ok(())
    }

    // fn execute(&mut self) {}
}

type Result<T> = result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let args = env::args().collect::<Vec<_>>();
    if args.len()!=2 {
        println!("Usage: {} <bf file>", args[0]);
        exit(1);
    }

    let src =  fs::read_to_string(&args[1])?;
    let mut vm = VM::new();
    vm.load_program(&src)?;
    println!("{:?} {:?}", vm.prog, vm.prog_meta);

    Ok(())
}
