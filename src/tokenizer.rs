#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    EOF,
    Invalid,
    Comment,
    BotInfo(Vec<String>),
    Ident(String),
    Number(u16),
    Register(u16),
    StackPointer,
    Instruction(InstructionType),
    Comma,
    Colon,
    OpenBracket,
    CloseBracket,
    OpenCurly,
    CloseCurly,
    Plus,
    Minus,
}

#[derive(Debug, PartialEq, Clone)]
#[repr(u16)]
pub enum InstructionType {
    NOP = 0,
    MOV = 1,
    PUSH = 2,
    POP = 3,
    CALL = 4,
    RET = 5,
    JMP = 6,
    JL = 7,
    JLE = 8,
    JG = 9,
    JGE = 10,
    JE = 11,
    JNE = 12,
    JS = 13,
    JNS = 14,
    ADD = 15,
    SUB = 16,
    MULT = 17,
    DIV = 18,
    MOD = 19,
    AND = 20,
    OR = 21,
    XOR = 22,
    CMP = 23,
    TEST = 24,
    GETXY = 25,
    ENERGY = 26,
    TRAVEL = 27,
    SHL = 28,
    SHR = 29,
    SENSE = 30,
    EAT = 31,
    RAND = 32,
    RELEASE = 33,
    CHARGE = 34,
    POKE = 35,
    PEEK = 36,
    CKSUM = 37,
}

impl From<u16> for InstructionType {
    fn from(instruction: u16) -> Self {
        match instruction {
            0 => InstructionType::NOP,
            1 => InstructionType::MOV,
            2 => InstructionType::PUSH,
            3 => InstructionType::POP,
            4 => InstructionType::CALL,
            5 => InstructionType::RET,
            6 => InstructionType::JMP,
            7 => InstructionType::JL,
            8 => InstructionType::JLE,
            9 => InstructionType::JG,
            10 => InstructionType::JGE,
            11 => InstructionType::JE,
            12 => InstructionType::JNE,
            13 => InstructionType::JS,
            14 => InstructionType::JNS,
            15 => InstructionType::ADD,
            16 => InstructionType::SUB,
            17 => InstructionType::MULT,
            18 => InstructionType::DIV,
            19 => InstructionType::MOD,
            20 => InstructionType::AND,
            21 => InstructionType::OR,
            22 => InstructionType::XOR,
            23 => InstructionType::CMP,
            24 => InstructionType::TEST,
            25 => InstructionType::GETXY,
            26 => InstructionType::ENERGY,
            27 => InstructionType::TRAVEL,
            28 => InstructionType::SHL,
            29 => InstructionType::SHR,
            30 => InstructionType::SENSE,
            31 => InstructionType::EAT,
            32 => InstructionType::RAND,
            33 => InstructionType::RELEASE,
            34 => InstructionType::CHARGE,
            35 => InstructionType::POKE,
            36 => InstructionType::PEEK,
            37 => InstructionType::CKSUM,
            _ => panic!("Not a valid instruction!")
        }
    }
}

impl From<InstructionType> for String {
    fn from(instruction_type: InstructionType) -> Self {
        match instruction_type {
            InstructionType::NOP => "nop",
            InstructionType::MOV => "mov",
            InstructionType::PUSH => "push",
            InstructionType::POP => "pop",
            InstructionType::CALL => "call",
            InstructionType::RET => "ret",
            InstructionType::JMP => "jmp",
            InstructionType::JL => "jl",
            InstructionType::JLE => "jle",
            InstructionType::JG => "jg",
            InstructionType::JGE => "jge",
            InstructionType::JE => "je",
            InstructionType::JNE => "jne",
            InstructionType::JS => "js",
            InstructionType::JNS => "jns",
            InstructionType::ADD => "add",
            InstructionType::SUB => "sub",
            InstructionType::MULT => "mult",
            InstructionType::DIV => "div",
            InstructionType::MOD => "mod",
            InstructionType::AND => "and",
            InstructionType::OR => "or",
            InstructionType::XOR => "xor",
            InstructionType::CMP => "cmp",
            InstructionType::TEST => "test",
            InstructionType::GETXY => "getxy",
            InstructionType::ENERGY => "energy",
            InstructionType::TRAVEL => "travel",
            InstructionType::SHL => "shl",
            InstructionType::SHR => "shr",
            InstructionType::SENSE => "sense",
            InstructionType::EAT => "eat",
            InstructionType::RAND => "rand",
            InstructionType::RELEASE => "release",
            InstructionType::CHARGE => "charge",
            InstructionType::POKE => "poke",
            InstructionType::PEEK => "peek",
            InstructionType::CKSUM => "cksum",
        }.to_string()
    }
}

impl InstructionType {
    pub fn get_operand_amount(&self) -> u16 {
        match self {
            InstructionType::NOP | InstructionType::RET | InstructionType::EAT => 0,

            InstructionType::PUSH
            | InstructionType::POP
            | InstructionType::CALL
            | InstructionType::JMP
            | InstructionType::JL
            | InstructionType::JLE
            | InstructionType::JG
            | InstructionType::JGE
            | InstructionType::JE
            | InstructionType::JNE
            | InstructionType::JS
            | InstructionType::JNS
            | InstructionType::ENERGY
            | InstructionType::TRAVEL
            | InstructionType::RELEASE
            | InstructionType::SENSE => 1,

            InstructionType::MOV
            | InstructionType::ADD
            | InstructionType::SUB
            | InstructionType::MULT
            | InstructionType::DIV
            | InstructionType::MOD
            | InstructionType::AND
            | InstructionType::OR
            | InstructionType::XOR
            | InstructionType::CMP
            | InstructionType::TEST
            | InstructionType::GETXY
            | InstructionType::SHL
            | InstructionType::SHR
            | InstructionType::RAND
            | InstructionType::CHARGE
            | InstructionType::POKE
            | InstructionType::PEEK
            | InstructionType::CKSUM => 2,
        }
    }

    pub fn is_positional(&self) -> bool {
        match self {
            InstructionType::CALL
            | InstructionType::JMP
            | InstructionType::JL
            | InstructionType::JLE
            | InstructionType::JG
            | InstructionType::JGE
            | InstructionType::JE
            | InstructionType::JNE
            | InstructionType::JS
            | InstructionType::JNS => true,
            _ => false,
        }
    }
}

pub struct Tokenizer {
    position: usize,
    read_position: usize,
    char: u8,
    input: Vec<u8>,
    preread: bool,
}

impl Tokenizer {
    pub fn new(input: String) -> Tokenizer {
        let mut tokenizer = Tokenizer {
            position: 0,
            read_position: 0,
            char: 0,
            input: input.into_bytes(),
            preread: false,
        };

        tokenizer.read_char();

        return tokenizer;
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.char = 0;
        } else {
            self.char = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_bot_info(&mut self) -> Token {
        let mut bot_info = Vec::new();

        while !self.char.is_ascii_alphanumeric() {
            self.read_char();
        }
        while !self.char.is_ascii_control() {
            bot_info.push(self.char);
            self.read_char();
        }

        let info = String::from_utf8_lossy(&bot_info)
            .to_string()
            .split(", ")
            .map(String::from)
            .collect::<Vec<String>>();

        Token::BotInfo(info)
    }

    fn read_comment(&mut self) -> Token {
        while self.char != b'\n' {
            self.read_char();
        }

        Token::Comment
    }

    fn skip_whitespace(&mut self) {
        while self.char.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn read_int(&mut self) -> Token {
        let pos = self.position;

        while self.char.is_ascii_hexdigit() || (self.char.to_ascii_lowercase() == 'x' as u8) {
            self.read_char();
        }

        self.preread = true;

        //println!("FORTNITE BALLS: {:#?}", self.char as char);

        let num = String::from_utf8_lossy(&self.input[pos..self.position]).to_string();

        // !FIXME: fuck
        let num: Result<u16, _> = if num.starts_with("0x") {
            u16::from_str_radix(&num.replace("0x", ""), 16)
        } else {
            num.parse()
        };

        if num.is_err() {
            return Token::Invalid;
        }

        Token::Number(num.unwrap())
    }

    fn read_ident(&mut self) -> String {
        let mut ident = Vec::new();

        loop {
            if self.char.is_ascii_alphanumeric() || self.char == '_' as u8 {
                ident.push(self.char);
                self.read_char();
            } else {
                break;
            }
        }

        String::from_utf8_lossy(&ident).to_string()
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        self.preread = false;

        let token = match self.char {
            b'/' => self.read_comment(),
            b';' => self.read_comment(),
            b':' => Token::Colon,
            b',' => Token::Comma,
            b'[' => Token::OpenBracket,
            b']' => Token::CloseBracket,
            b'{' => Token::OpenCurly,
            b'}' => Token::CloseCurly,
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'0'..=b'9' => self.read_int(),
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_ident();

                if ident.to_lowercase().starts_with("r") {
                    // let num_str: String = *ident.split("").collect::<Vec<String>>()[1..].clone().collect();
                    let num_chars: Vec<char> = ident.chars().collect();
                    let num_str: String = num_chars[1..].iter().collect();
                    let num = num_str.parse::<u16>();

                    match num {
                        Ok(val) => match val {
                            0..=13 | 15 => return Token::Register(val),
                            _ => {}
                        },
                        _ => {}
                    }
                }

                match ident.to_lowercase().as_str() {
                    // "r" => {
                    //     if let Token::Number(num) = self.read_int() {
                    //         return Token::Register(num);
                    //     }
                    // }
                    "info" => return self.read_bot_info(),
                    "sp" => return Token::StackPointer,
                    _ => {}
                }

                let token = match ident.to_uppercase().as_str() {
                    "NOP" => Token::Instruction(InstructionType::NOP),
                    "MOV" => Token::Instruction(InstructionType::MOV),
                    "PUSH" => Token::Instruction(InstructionType::PUSH),
                    "POP" => Token::Instruction(InstructionType::POP),
                    "CALL" => Token::Instruction(InstructionType::CALL),
                    "RET" => Token::Instruction(InstructionType::RET),
                    "JMP" => Token::Instruction(InstructionType::JMP),
                    "JL" => Token::Instruction(InstructionType::JL),
                    "JLE" => Token::Instruction(InstructionType::JLE),
                    "JG" => Token::Instruction(InstructionType::JG),
                    "JGE" => Token::Instruction(InstructionType::JGE),
                    "JE" => Token::Instruction(InstructionType::JE),
                    "JNE" => Token::Instruction(InstructionType::JNE),
                    "JS" => Token::Instruction(InstructionType::JS),
                    "JNS" => Token::Instruction(InstructionType::JNS),
                    "ADD" => Token::Instruction(InstructionType::ADD),
                    "SUB" => Token::Instruction(InstructionType::SUB),
                    "MULT" => Token::Instruction(InstructionType::MULT),
                    "DIV" => Token::Instruction(InstructionType::DIV),
                    "MOD" => Token::Instruction(InstructionType::MOD),
                    "AND" => Token::Instruction(InstructionType::AND),
                    "OR" => Token::Instruction(InstructionType::OR),
                    "XOR" => Token::Instruction(InstructionType::XOR),
                    "CMP" => Token::Instruction(InstructionType::CMP),
                    "TEST" => Token::Instruction(InstructionType::TEST),
                    "GETXY" => Token::Instruction(InstructionType::GETXY),
                    "ENERGY" => Token::Instruction(InstructionType::ENERGY),
                    "TRAVEL" => Token::Instruction(InstructionType::TRAVEL),
                    "SHL" => Token::Instruction(InstructionType::SHL),
                    "SHR" => Token::Instruction(InstructionType::SHR),
                    "SENSE" => Token::Instruction(InstructionType::SENSE),
                    "EAT" => Token::Instruction(InstructionType::EAT),
                    "RAND" => Token::Instruction(InstructionType::RAND),
                    "RELEASE" => Token::Instruction(InstructionType::RELEASE),
                    "CHARGE" => Token::Instruction(InstructionType::CHARGE),
                    "POKE" => Token::Instruction(InstructionType::POKE),
                    "PEEK" => Token::Instruction(InstructionType::PEEK),
                    "CKSUM" => Token::Instruction(InstructionType::CKSUM),
                    _ => Token::Ident(ident),
                };

                return token;
            }
            0 => Token::EOF,
            _ => Token::Invalid,
        };

        if self.preread == false {
            self.read_char();
        }

        token
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.char != 0 {
            tokens.push(self.next_token());
        }

        // dirty hack, fixme?
        if tokens.last() != Some(&Token::EOF) {
            tokens.push(Token::EOF);
        }

        tokens
    }
}
