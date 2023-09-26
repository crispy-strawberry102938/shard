use std::error::Error;

/// A token in the source code
#[derive(Debug)]
pub enum Token {
    Label(String),

    /// A stack variable
    StackVar(String),

    /// Didn't really know which size to go for...
    Number(i128),

    Marker(String),
    Register(Register),
    Directive(Directive),
    Conditional(Conditional),

    Ret,
    FuncCall,
    Jump(String),
    Mutate(String),

    /// (
    LParen,
    /// )
    RParen,
    /// {
    LCurly,
    /// }
    RCurly,
    /// [
    LSquare,
    /// ]
    RSquare,

    /// !
    Bang,
    /// +
    Plus,
    /// -
    Minus,
    /// *
    Star,
    /// /
    Slash,

    Newline
}

#[derive(Debug)]
pub struct Register {
    val: u8,
    r#type: RegisterType
}

#[derive(Debug)]
enum RegisterType {
    Low,
    High,
    Word,
    Double,
    Quad,
}

#[derive(Debug)]
pub enum Directive {
    Entry,
    Include,
    Text,
    Arch,
    Def
}

#[derive(Debug)]
pub enum Conditional {
    Equal,
    NotEqual,

    LessThan,
    GreaterThan,

    LessOrEqual,
    GreaterOrEquasl,
}

#[derive(Debug)]
pub struct Tokenizer {
    source: String,
    pos: usize,
}

impl Tokenizer {

    pub fn new(source: String) -> Self {
        return Self {
            source,
            pos: 0,
        }
    }

    pub fn tokenise(self: &Self, source: String) -> Vec<Token> {
        let mut tokens  = Vec::new();

        let bytes = self.source.as_bytes();
        while self.pos < bytes.len() {
            match bytes[self.pos] {
                b'+' => tokens.push(Token::Plus),
                b'-' => tokens.push(Token::Minus),
                b'*' => tokens.push(Token::Star),
                b'/' => tokens.push(Token::Slash),
                b'!' => tokens.push(Token::Bang),
                b'(' => tokens.push(Token::LParen),
                b')' => tokens.push(Token::RParen),
                b'{' => tokens.push(Token::LCurly),
                b'}' => tokens.push(Token::RCurly),
                b'[' => tokens.push(Token::LSquare),
                b']' => tokens.push(Token::RSquare),
                

                b'a'..=b'z' | b'A'..=b'Z' | b'_' => {

                },
                b'\n' => tokens.push(Token::Newline),
                b' ' => continue,
                _ => panic!("Invalid character")
            }
        }
        return tokens;
    }

    fn parse_indent() {

    }
}