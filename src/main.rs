use std::process::exit;
use std::fs;
use std::env;
use std::result;
use std::error::Error;
use std::io::Read;

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
    ip: usize,
    dat_ptr: usize,
}

impl VM {
    fn new() -> VM {
        VM { prog: Vec::new(), prog_meta: Vec::new(), mem: [0; MEM_SIZE], ip: 0, dat_ptr: 0 }
    }

    fn clear_all(&mut self) {
        self.prog.clear();
        self.prog_meta.clear();
        self.mem = [0; MEM_SIZE];
        self.ip = 0;
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
                // println!("Unknown instruction {} encountered", c);
                continue;
            }            
            else if instr==Instruction::Begin {
                brk.push(self.prog.len());
            }
            else if instr==Instruction::End {
                if let Some(b) = brk.pop() {
                    pmeta = b as u32;
                    // TOOD: Error checking if it goes out of program length
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

    fn run(&mut self) -> Result<()> {
        while self.ip != self.prog.len() {
            match self.prog[self.ip] {
                Instruction::Right  => { self.dat_ptr += 1; 
                                        if self.dat_ptr>=MEM_SIZE { return err!("Memory overflow"); } },
                Instruction::Left   => { if self.dat_ptr==0 { return err!("Memory underflow") }
                                        self.dat_ptr -= 1; },
                Instruction::Incr   => { if self.mem[self.dat_ptr]==255 { 
                                            self.mem[self.dat_ptr]=0; 
                                        }
                                        else {
                                            self.mem[self.dat_ptr] += 1; 
                                        } },
                Instruction::Decr   => { if self.mem[self.dat_ptr]==0 {
                                            self.mem[self.dat_ptr]=255;
                                         } 
                                         else {
                                            self.mem[self.dat_ptr] -= 1; 
                                        } },
                Instruction::Output => print!("{}", self.mem[self.dat_ptr] as char),
                Instruction::Read   => self.mem[self.dat_ptr] = std::io::stdin().bytes().next().unwrap().unwrap(),
                Instruction::Begin  => if self.mem[self.dat_ptr]==0 { 
                                            self.ip = self.prog_meta[self.ip] as usize;
                                        }
                Instruction::End    => if self.mem[self.dat_ptr]!=0 {
                                            self.ip = self.prog_meta[self.ip] as usize;
                                        }
                Instruction::Unknown => { return err!("Unknown instruction encountered in execution"); }, 
            }
            self.ip += 1;
        }

        Ok(())
    }
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
    vm.run()?;
    Ok(())
}
