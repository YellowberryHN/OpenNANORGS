use crate::parser::{Instruction, Operand, Parser, ParserToken, PlusMinus, Value};
use crate::tokenizer::Tokenizer;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use crate::symbol_table::SymbolTable;

pub struct Compiler {
    position: usize,
    read_position: usize,
    byte_position: usize,
    instruction: ParserToken,
    input: Vec<ParserToken>,
    labels: Vec<(String, u16)>,
    label_index: Vec<(String, (u16, u16))>,
    symbol_table: HashMap<String, u16>,
    pub output: Vec<u16>,
}

impl Compiler {
    pub fn new(input: Vec<ParserToken>, symbol_table: HashMap<String, u16>) -> Compiler {
        let mut compiler = Compiler {
            position: 0,
            read_position: 0,
            byte_position: 0,
            instruction: ParserToken::Invalid,
            output: [0; 3600].to_vec(),
            label_index: Vec::new(),
            input: input.clone(),
            labels: Vec::new(),
            symbol_table,
        };

        compiler.read_instruction();
        compiler
    }

    pub fn new_from_file(path: &PathBuf, verbose: bool) -> Compiler {
        let input: String = fs::read_to_string(path).unwrap().parse().unwrap();

        let mut tokenizer = Tokenizer::new(input.clone());

        let tokens = tokenizer.tokenize();

        if verbose {
            for token in tokens.clone() {
                println!("{:?}", token);
            }

            println!("-------------------------------------------");
        }

        let mut parser = Parser::new(tokens);
        let mut parser_tokens = Vec::new();

        loop {
            let token = parser.next_token();

            parser_tokens.push(token.clone());

            if token == ParserToken::EOF {
                break;
            }
        }

        if verbose {
            for token in parser_tokens.clone() {
                println!("{:#?}", token);
            }
        }

        let symbol_table = SymbolTable::new(&parser_tokens);

        if verbose {
            println!("{:#?}", symbol_table.label_to_address);
        }

        let mut compiler = Compiler::new(parser_tokens, symbol_table.label_to_address);
        compiler.compile();

        if verbose {
            println!("{:?}", compiler.output);

            let mut bruh = 0;
            for word in &compiler.output {
                print!("{:04x} ", word);
                bruh += 1;
                if bruh == 3 {
                    bruh = 0;
                    print!("\n");
                }
            }
            if bruh != 3 {
                print!("\n");
            }
        }

        compiler
    }

    pub fn compile(&mut self) {
        let mut bytecode: Vec<u16> = vec![];
        let mut instruction_pointer = 0;

        for token in &self.input {
            match token {
                ParserToken::Instruction(instruction) => {
                    while (instruction_pointer % 3) != 0 {
                        bytecode.push(0);
                        instruction_pointer += 1;
                    }

                    let op1 = &instruction.operand1;
                    let op2 = &instruction.operand2;

                    let mut op1_value: u16 = 0;
                    let mut op2_value: u16 = 0;

                    let mut op1_offset: u16 = 0;
                    let mut op2_offset: u16 = 0;

                    let mut op1_carry: bool = false;
                    let mut op2_carry: bool = false;

                    let positional = instruction.instruction_type.is_positional();

                    match op1 {
                        Operand::None => {}
                        Operand::Direct(value) => match value {
                            Value::Number(num) => op1_value = *num,
                            Value::Label(label) => {
                                op1_value = *self.symbol_table.get(&label.to_lowercase()).unwrap();
                            }
                        },
                        Operand::ImmediateValue(value) => match value {
                            Value::Number(num) => {
                                op1_value = *num;

                                if positional {
                                    op1_value = op1_value.wrapping_sub(instruction_pointer);
                                }
                            }
                            Value::Label(label) => {
                                op1_value = *self.symbol_table.get(&label.to_lowercase()).unwrap();

                                if positional {
                                    op1_value = op1_value.wrapping_sub(instruction_pointer);
                                }
                            }
                        },
                        Operand::Register(register) => {
                            op1_value = register.to_owned() as u16;
                        }
                        Operand::RegisterIndexedDirect(base, operator, offset) => {
                            match base.as_ref() {
                                Operand::ImmediateValue(value) => match value {
                                    Value::Label(label) => {
                                        op1_offset =
                                            *self.symbol_table.get(&label.to_lowercase()).unwrap();
                                    }
                                    _ => {}
                                },
                                Operand::Register(register) => {
                                    op1_value = (register.to_owned() as u16) << 12;
                                }
                                _ => {}
                            }
                            match offset.as_ref() {
                                Operand::ImmediateValue(value) => {
                                    op1_offset = match value {
                                        Value::Number(num) => *num,
                                        Value::Label(_) =>
                                            panic!("Label cannot be used as offset")
                                    };
                                }
                                Operand::Register(register) => {
                                    match base.as_ref() {
                                        Operand::Register(_) => panic!("Register cannot be used as offset"), // invalid
                                        Operand::ImmediateValue(value) => match value {
                                            Value::Label(_) => {
                                                op1_value = (register.to_owned() as u16) << 12
                                            }
                                            _ => {}
                                        },
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                            match operator {
                                PlusMinus::Minus => {
                                    if op1_offset > 0 {
                                        op1_offset = 0u16.wrapping_sub(op1_offset);
                                        op1_carry = true;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }

                    match op2 {
                        Operand::None => {}
                        Operand::Direct(value) => match value {
                            Value::Number(num) => op2_value = *num,
                            Value::Label(label) => {
                                op2_value = *self.symbol_table.get(&label.to_lowercase()).unwrap();
                            }
                        },
                        Operand::ImmediateValue(value) => match value {
                            Value::Number(num) => {
                                op2_value = *num;

                                if positional  {
                                    op2_value = op2_value.wrapping_sub(instruction_pointer);
                                }
                            }
                            Value::Label(label) => {
                                op2_value = *self.symbol_table.get(&label.to_lowercase()).unwrap();
                                if positional {
                                    op2_value = op2_value.wrapping_sub(instruction_pointer);
                                }
                            }
                        },
                        Operand::Register(register) => {
                            op2_value = register.to_owned() as u16;
                        }
                        Operand::RegisterIndexedDirect(base, operator, offset) => {
                            match base.as_ref() {
                                Operand::ImmediateValue(value) => match value {
                                    Value::Label(label) => {
                                        op2_offset =
                                            *self.symbol_table.get(&label.to_lowercase()).unwrap();
                                    }
                                    _ => {}
                                },
                                Operand::Register(register) => {
                                    op2_value = (register.to_owned() as u16) << 12;
                                }
                                _ => {}
                            }
                            match offset.as_ref() {
                                Operand::ImmediateValue(value) => {
                                    op2_offset = match value {
                                        Value::Number(num) => *num,
                                        Value::Label(_) => panic!("Label cannot be used as offset"),
                                    };
                                }
                                Operand::Register(register) => {
                                    match base.as_ref() {
                                        Operand::Register(_) => panic!("Register cannot be used as offset"),
                                        Operand::ImmediateValue(value) => match value {
                                            Value::Label(_) => {
                                                op2_value = (register.to_owned() as u16) << 12
                                            }
                                            _ => {}
                                        },
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                            match operator {
                                PlusMinus::Minus => {
                                    if op2_offset > 0 {
                                        op2_offset = 0u16.wrapping_sub(op2_offset);
                                        op2_carry = true;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }

                    let inst = instruction.to_owned().instruction_type as u16
                        | Compiler::get_modes(instruction, op1_carry, op2_carry);

                    //println!("inst: {:04X}, op1: {} ({}), op2: {} ({})", inst, op1_value, op1_offset, op2_value, op2_offset);

                    bytecode.push(inst);
                    bytecode.push(op1_value | (op1_offset & 0xFFF));
                    bytecode.push(op2_value | (op2_offset & 0xFFF));
                    instruction_pointer += 3;
                }
                ParserToken::Data(data) => {
                    for value in data {
                        bytecode.push(match value {
                            Value::Number(num) => *num,
                            Value::Label(label) => {
                                *self.symbol_table.get(&label.to_lowercase()).unwrap()
                            }
                        });
                        instruction_pointer += 1;
                    }
                }
                _ => {}
            }
        }

        for (pos, word) in bytecode.iter().enumerate() {
            self.output[pos] = *word;
        }
    }

    fn get_modes(instruction: &Instruction, op1_carry: bool, op2_carry: bool) -> u16 {
        let mut value = u16::from(instruction.to_owned().operand1) << 2;
        value |= u16::from(instruction.to_owned().operand2);

        value <<= 12;

        if op1_carry {
            value |= 0x800
        }
        if op2_carry {
            value |= 0x400
        }

        value
    }

    fn read_instruction(&mut self) {
        if !(self.instruction == ParserToken::EOF) {
            self.instruction = self.input[self.read_position].clone();
        }

        self.position = self.read_position;
        self.read_position += 1;
    }
}
