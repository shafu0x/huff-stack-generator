use crate::opcodes::{Opcode, UNKNOWN};
use crate::stack::Stack;

const CONSTANT_START: &str = "0x";
const REFERENCE_START: &str = "[";
const VARIABLE_START: &str = "<";
const FUNCTION_START: &str = "_";
pub const JUMP_LABEL_END: &str = ":";

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Constant,
    Opcode,
    Reference,
    Variable,
    Function,
    Return,
    JumpLabel,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub value: String,
    pub token_type: TokenType,
    pub opcode: Option<Opcode>, // Only has an opcode if token_type is Opcode
    pub operands: Vec<Token>,
}

impl Token {
    pub fn new() -> Token {
        Token {
            token_type: TokenType::Unknown,
            value: String::new(),
            opcode: Some(UNKNOWN),
            operands: Vec::new(),
        }
    }

    pub fn from_string(word: &str) -> Token {
        let word = word.trim();
        let mut token = Token::new();
        let token_type = match word {
            _ if word.starts_with(CONSTANT_START) => TokenType::Constant,
            _ if word.starts_with(REFERENCE_START) => TokenType::Reference,
            _ if word.starts_with(VARIABLE_START) => TokenType::Variable,
            _ if word.starts_with(FUNCTION_START) => TokenType::Function,
            _ if word.ends_with(JUMP_LABEL_END) => TokenType::JumpLabel,
            _ => TokenType::Opcode,
        };

        token.token_type = token_type;
        token.value = word.to_string();

        if token.token_type == TokenType::Opcode {
            token.opcode = Some(Opcode::from_string(word));
        }

        token
    }

    pub fn set_operands(&mut self, stack: &Stack) {
        let mut operands = Vec::new();
        for i in 0..self.opcode.as_ref().unwrap().pops {
            operands.push(stack.get(stack.len() - 1 - i).unwrap().clone());
        }
        self.operands = operands;
    }

    pub fn to_str(&self, show_stack_output: bool) -> String {
        match &self.token_type {
            TokenType::Opcode => {
                let opcode = self.opcode.as_ref().unwrap();
                let sign = match opcode.sign {
                    Some(sign) => format!(" {} ", sign).to_string(),
                    None => ",".to_string(),
                };
                let operands = self
                    .operands
                    .iter()
                    .map(|operand| operand.to_str(show_stack_output))
                    .collect::<Vec<String>>()
                    .join(&sign);
                let name = match opcode.sign {
                    Some(_) => String::new(),
                    None => opcode.name.to_lowercase(),
                };
                if show_stack_output && opcode.output != "" {
                    opcode.output.to_string()
                } else {
                    format!("{}({})", name, operands)
                }
            }
            _ => self.value.clone(),
        }
    }
}
