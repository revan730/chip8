use std::collections::HashMap;
use crate::instructions::Instructions;

#[derive(Debug)]
pub enum ArgumentType {
    Reg,
    Byte,
    Addr,
}

#[derive(Debug)]
pub struct ArgumentDecoder {
    pub mask: u16,
    pub shift: u8,
    pub kind: ArgumentType,
}

#[derive(Debug)]
pub struct OpcodeDecoder<'a> {
    pub name: &'a str,
    pub instruction: Instructions,
    pub pattern: u16,
    pub mask: u16,
    pub argument_decoders: Vec<ArgumentDecoder>,
}

lazy_static! {
    pub static ref OPCODE_DECODERS: HashMap<Instructions, OpcodeDecoder<'static>> = {
        let mut m = HashMap::new();
        m.insert(Instructions::Ret, OpcodeDecoder{
            name: "RET",
            instruction: Instructions::Ret,
            pattern: 0x00EE,
            mask: 0x00FF,
            argument_decoders: vec![],
        });
        m.insert(Instructions::Jp, OpcodeDecoder{
            name: "JP addr",
            instruction: Instructions::Jp,
            pattern: 0x1000,
            mask: 0xF000,
            argument_decoders: vec![ArgumentDecoder{ mask: 0x0f00, shift: 8, kind: ArgumentType::Addr }, ArgumentDecoder{ mask: 0x0ff, shift: 0, kind: ArgumentType::Addr }],
        });
        m.insert(Instructions::Call, OpcodeDecoder{
            name: "CALL addr",
            instruction: Instructions::Call,
            pattern: 0x2000,
            mask: 0xf000,
            argument_decoders: vec![ArgumentDecoder{ mask: 0x0f00, shift: 8, kind: ArgumentType::Addr }, ArgumentDecoder{ mask: 0x0ff, shift: 0, kind: ArgumentType::Addr }],
        });
        m.insert(Instructions::SeVxByte, OpcodeDecoder{
            name: "SE Vx, byte",
            instruction: Instructions::SeVxByte,
            pattern: 0x3000,
            mask: 0xf000,
            argument_decoders: vec![ArgumentDecoder{ mask: 0x0f00, shift: 8, kind: ArgumentType::Reg }, ArgumentDecoder{ mask: 0x0ff, shift: 0, kind: ArgumentType::Byte }],
        });
        m.insert(Instructions::SneVxByte, OpcodeDecoder{
            name: "SNE Vx, byte",
            instruction: Instructions::SneVxByte,
            pattern: 0x4000,
            mask: 0xf000,
            argument_decoders: vec![ArgumentDecoder{ mask: 0x0f00, shift: 8, kind: ArgumentType::Reg }, ArgumentDecoder{ mask: 0x0ff, shift: 0, kind: ArgumentType::Byte }],
        });
        m.insert(Instructions::LdVxByte, OpcodeDecoder{
            name: "LD Vx, byte",
            instruction: Instructions::LdVxByte,
            pattern: 0x6000,
            mask: 0xf000,
            argument_decoders: vec![ArgumentDecoder{ mask: 0x0f00, shift: 8, kind: ArgumentType::Reg }, ArgumentDecoder{ mask: 0x0ff, shift: 0, kind: ArgumentType::Byte }],
        });
        m.insert(Instructions::AddVxByte, OpcodeDecoder{
            name: "ADD Vx, byte",
            instruction: Instructions::AddVxByte,
            pattern: 0x7000,
            mask: 0xf000,
            argument_decoders: vec![ArgumentDecoder{ mask: 0x0f00, shift: 8, kind: ArgumentType::Reg }, ArgumentDecoder{ mask: 0x0ff, shift: 0, kind: ArgumentType::Byte }],
        });
        m.insert(Instructions::LdVxVy, OpcodeDecoder{
            name: "LD Vx, Vy",
            instruction: Instructions::LdVxVy,
            pattern: 0x8000,
            mask: 0xf00f,
            argument_decoders: vec![ArgumentDecoder{ mask: 0x0f00, shift: 8, kind: ArgumentType::Reg }, ArgumentDecoder{ mask: 0x0f0, shift: 4, kind: ArgumentType::Reg }],
        });
        m.insert(Instructions::AndVxVy, OpcodeDecoder{
            name: "AND Vx, Vy",
            instruction: Instructions::AndVxVy,
            pattern: 0x8002,
            mask: 0xf00f,
            argument_decoders: vec![ArgumentDecoder{ mask: 0x0f00, shift: 8, kind: ArgumentType::Reg }, ArgumentDecoder{ mask: 0x0f0, shift: 4, kind: ArgumentType::Reg }],
        });
        m.insert(Instructions::AddVxVy, OpcodeDecoder{
            name: "ADD Vx, Vy",
            instruction: Instructions::AddVxVy,
            pattern: 0x8004,
            mask: 0xf00f,
            argument_decoders: vec![ArgumentDecoder{ mask: 0x0f00, shift: 8, kind: ArgumentType::Reg }, ArgumentDecoder{ mask: 0x0f0, shift: 4, kind: ArgumentType::Reg }],
        });
        m.insert(Instructions::SubVxVy, OpcodeDecoder{
            name: "SUB Vx, Vy",
            instruction: Instructions::SubVxVy,
            pattern: 0x8005,
            mask: 0xf00f,
            argument_decoders: vec![ArgumentDecoder{ mask: 0x0f00, shift: 8, kind: ArgumentType::Reg }, ArgumentDecoder{ mask: 0x0f0, shift: 4, kind: ArgumentType::Reg }],
        });
        m.insert(Instructions::LdIAddr, OpcodeDecoder{
            name: "LD I, addr",
            instruction: Instructions::LdIAddr,
            pattern: 0xA000,
            mask: 0xf000,
            argument_decoders: vec![ArgumentDecoder{ mask: 0x0f00, shift: 8, kind: ArgumentType::Addr  }, ArgumentDecoder{ mask: 0x0ff, shift: 0, kind: ArgumentType::Addr }],
        });
        m.insert(Instructions::RndVxByte, OpcodeDecoder{
            name: "RND Vx, byte",
            instruction: Instructions::RndVxByte,
            pattern: 0xC000,
            mask: 0xf000,
            argument_decoders: vec![ArgumentDecoder{ mask: 0x0f00, shift: 8, kind: ArgumentType::Reg }, ArgumentDecoder{ mask: 0x0ff, shift: 0, kind: ArgumentType::Byte }],
        });
        m.insert(Instructions::DrwVxVyNib, OpcodeDecoder{
            name: "DRW VX, VY, nibble",
            instruction: Instructions::DrwVxVyNib,
            pattern: 0xD000,
            mask: 0xf000,
            argument_decoders: vec![ArgumentDecoder{ mask: 0x0f00, shift: 8, kind: ArgumentType::Reg }, ArgumentDecoder{ mask: 0x0f0, shift: 4, kind: ArgumentType::Reg }, ArgumentDecoder{ mask: 0xf, shift: 0, kind: ArgumentType::Byte }],
        });
        m.insert(Instructions::SknpVx, OpcodeDecoder{
            name: "SKNP Vx",
            instruction: Instructions::SknpVx,
            pattern: 0xE0A1,
            mask: 0xf0ff,
            argument_decoders: vec![ArgumentDecoder{ mask: 0x0f00, shift: 8, kind: ArgumentType::Reg }],
        });
        m.insert(Instructions::LdVxDt, OpcodeDecoder{
            name: "LD Vx, DT",
            instruction: Instructions::LdVxDt,
            pattern: 0xF007,
            mask: 0xf0ff,
            argument_decoders: vec![ArgumentDecoder{ mask: 0x0f00, shift: 8, kind: ArgumentType::Reg }],
        });
        m.insert(Instructions::LdDtVx, OpcodeDecoder{
            name: "LD DT, Vx",
            instruction: Instructions::LdDtVx,
            pattern: 0xF015,
            mask: 0xf0ff,
            argument_decoders: vec![ArgumentDecoder{ mask: 0x0f00, shift: 8, kind: ArgumentType::Reg }],
        });
        m.insert(Instructions::LdStVx, OpcodeDecoder{
            name: "LD ST, Vx",
            instruction: Instructions::LdStVx,
            pattern: 0xF018,
            mask: 0xf0ff,
            argument_decoders: vec![ArgumentDecoder{ mask: 0x0f00, shift: 8, kind: ArgumentType::Reg }],
        });
        m.insert(Instructions::LdFVx, OpcodeDecoder{
            name: "LD F, Vx",
            instruction: Instructions::LdFVx,
            pattern: 0xF029,
            mask: 0xf0ff,
            argument_decoders: vec![ArgumentDecoder{ mask: 0x0f00, shift: 8, kind: ArgumentType::Reg }],
        });
        m.insert(Instructions::LdBVx, OpcodeDecoder{
            name: "LD B, Vx",
            instruction: Instructions::LdBVx,
            pattern: 0xF033,
            mask: 0xf0ff,
            argument_decoders: vec![ArgumentDecoder{ mask: 0x0f00, shift: 8, kind: ArgumentType::Reg }],
        });
        m.insert(Instructions::LdVxI, OpcodeDecoder{
            name: "LD VX, [I]",
            instruction: Instructions::LdVxI,
            pattern: 0xF065,
            mask: 0xf0ff,
            argument_decoders: vec![ArgumentDecoder{ mask: 0x0f00, shift: 8, kind: ArgumentType::Reg }],
        });

        m
    };
}