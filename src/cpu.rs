use crate::constants::{SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::drawable::Drawable;
use crate::instructions::Instructions;
use crate::opcode_decoders::{OPCODE_DECODERS};
use std::fmt;
use std::num::Wrapping;
use rand::Rng;

pub struct Cpu {
    pub ram: [u8; 4096],
    pub registers: [u8; 16],
    pub stack: [u16; 16],
    pub st: u8,
    pub dt: u8,
    pub i: u16,
    pub sp: u8,
    pub pc: u16,
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Registers:\n").unwrap();
        for (i, reg) in self.registers.iter().enumerate() {
            write!(f, "V{:X}: {:#X}\n", i, reg).unwrap();
        }
        write!(f, "DT: {:#X} ST: {:#X}\n", self.dt, self.st).unwrap();
        write!(f, "I: {:#X}\n", self.i).unwrap();

        if self.sp < 17 {
            write!(f, "Last stack addr: {:#X}\n", self.stack[self.sp as usize]).unwrap();
        }

        write!(f, "SP: {}\n", self.sp)
    }
}

#[derive(Debug)]
struct Instruction {
    pub int: Instructions,
    pub args: Vec<u8>,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu{
            ram: [0; 4096],
            registers: [0; 16],
            stack: [0; 16],
            st: 0x0,
            dt: 0x0,
            i: 0x0,
            sp: 0xFF,
            pc: 0x200,
        }
    }

    pub fn load_rom(&mut self, rom: [u8; 4096]) {
        self.ram = rom;
    }

    pub fn tick(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            self.st -= 1;
            // TODO: Stop sound if st is now 0
        }
    }

    pub fn reset(&mut self) {
        self.ram.fill(0x0);
        self.registers.fill(0x0);
        self.st = 0x0;
        self.dt = 0x0;
        self.i = 0x0;
        self.pc = 0x200;
        self.sp = 0xff;
    }

    pub fn step(&mut self, screen: &mut Box<dyn Drawable>, pressed_keys: &[u8; 16]) {
        let opcode = self.fetch();
        let instruction = self.decode(opcode);

        self.execute(instruction, screen, pressed_keys);
    }

    fn fetch(&self) -> u16 {
        match self.pc {
            0..=4096 => return u16::from(self.ram[self.pc as usize]) << 8 | u16::from(self.ram[self.pc as usize + 1]),
            _ => panic!("PC address out of ram bounds"),
        }
    }

    fn decode(&self, opcode: u16) -> Instruction {
        //println!("opcode: {:#X}", opcode);
        for (_, decoder) in OPCODE_DECODERS.iter() {
            let masking_result = opcode & decoder.mask;
            if masking_result == decoder.pattern {
                //println!("Got {} instruction", decoder.name);
                let args: Vec<u8> = decoder.argument_decoders.iter().map(|arg_decoder| {
                    let arg: u8 = ((opcode & arg_decoder.mask) >> arg_decoder.shift) as u8;
                    //println!("Arg with mask {:#X} and shift {}", arg_decoder.mask, arg_decoder.shift);

                    arg
                }).collect();

                return Instruction{
                    int: decoder.instruction,
                    args,
                }
            }
        }

        panic!("Unknown instruction");
    }

    fn execute(&mut self, instr: Instruction, screen: &mut Box<dyn Drawable>, pressed_keys: &[u8; 16]) {
        //println!("Instruction {:?}", instr.int);
        match instr.int {
            Instructions::Ret => {
                if self.sp > 0xf {
                    panic!("Trying to return with empty stack");
                }

                self.pc = self.stack[self.sp as usize];
                self.sp -= 1;
            },
            Instructions::Jp => {
                self.pc = u16::from(instr.args[0]) << 8 | u16::from(instr.args[1]);
            },
            Instructions::Call => {
                if self.sp > 0xf { // Using value larger than stack size to indicate that stack is empty
                    self.sp = 1;
                } else {
                    self.sp += 1;
                }

                if self.sp > 0xf {
                    panic!("stack pointer out of bounds");
                }


                self.stack[self.sp as usize] = self.pc + 2;
                self.pc = u16::from(instr.args[0]) << 8 | u16::from(instr.args[1]);
            },
            Instructions::SeVxByte => {
                if self.registers[instr.args[0] as usize] == instr.args[1] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            Instructions::SneVxByte => {
                if self.registers[instr.args[0] as usize] != instr.args[1] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            Instructions::LdVxByte => {
                self.registers[instr.args[0] as usize] = instr.args[1];
                self.pc += 2;
            },
            Instructions::AddVxByte => {
                let result = Wrapping(self.registers[instr.args[0] as usize]) + Wrapping(instr.args[1]);

                self.registers[instr.args[0] as usize] = result.0;
                self.pc += 2;
            },
            Instructions::LdVxVy => {
                self.registers[instr.args[0] as usize] = self.registers[instr.args[1] as usize];
                self.pc += 2;
            },
            Instructions::AndVxVy => {
                self.registers[instr.args[0] as usize] &= self.registers[instr.args[1] as usize];
                self.pc += 2;
            },
            Instructions::AddVxVy => {
                let result = u16::from(self.registers[instr.args[0] as usize]) + u16::from(self.registers[instr.args[1] as usize]);
                if result > 255 {
                    self.registers[0xf] = 0x1;
                } else {
                    self.registers[0xf] = 0x0;
                }

                self.registers[instr.args[0] as usize] = result as u8;

                self.pc += 2;
            },
            Instructions::LdIAddr => {
                self.i = u16::from(instr.args[0]) << 8 | u16::from(instr.args[1]);
                self.pc += 2;
            },
            Instructions::RndVxByte => {
                let mut rng = rand::thread_rng();
                let rnd_value: u8 = rng.gen_range(0..255) & instr.args[1];
                self.registers[instr.args[0] as usize] = rnd_value;

                self.pc += 2;
            },
            Instructions::SknpVx => {
                let key_hex = self.registers[instr.args[0] as usize];
                if pressed_keys[key_hex as usize] == 0 {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            Instructions::DrwVxVyNib => {
                self.registers[0xf] = 0;

                for i in 0..instr.args[2] {
                    let line = self.ram[self.i as usize + i as usize];

                    for pos in 0..8 {
                        let mut value = line & (1 << (7 - pos));
                        if value > 0 {
                            value = 1;
                        }

                        let x = (self.registers[instr.args[0] as usize] + pos) % SCREEN_WIDTH as u8;
                        let y = (self.registers[instr.args[1] as usize] + i) % SCREEN_HEIGHT as u8;

                        let set_vf = screen.draw(x, y, value);
                        if set_vf == true {
                            self.registers[0xf] = 1;
                        }
                    }
                }

                self.pc += 2;
            },
            Instructions::LdVxDt => {
                self.registers[instr.args[0] as usize] = self.dt;

                self.pc += 2;
            },
            Instructions::LdDtVx => {
                self.dt = self.registers[instr.args[0] as usize];

                self.pc += 2;
            },
            Instructions::LdFVx => {
                self.i = u16::from(self.registers[instr.args[0] as usize] * 5);

                self.pc += 2;
            },
            Instructions::LdBVx => {
                let mut x = self.registers[instr.args[0] as usize];

                let a = x.div_floor(100);
                x = x - a * 100;
                let b = x.div_floor(10);
                x = x - b * 10;
                let c = x;

                self.ram[self.i as usize] = a;
                self.ram[self.i as usize + 1] = b;
                self.ram[self.i as usize + 2] = c;

                self.pc += 2;
            },
            Instructions::LdVxI => {
                let last_reg = instr.args[0] as usize;
                for x in 0..last_reg {
                    self.registers[x] = self.ram[self.i as usize + x];
                }

                self.pc += 2;
            },
            _ => panic!("Unknown instruction {:?}", instr.int),
        }
    }
}