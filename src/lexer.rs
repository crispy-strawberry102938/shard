/// A token in the source code
#[derive(Debug)]
pub enum Token {
    /// #[a..z | A..=Z]
    Label(String),
    /// ~[a..z | A..Z] 
    Jump(String),

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

    /// =
    Equal,

    Newline,

    EOF
}

#[derive(Debug)]
pub struct Register(u8, RegisterType);

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

/// Does not include `=` as it depends on context.
#[derive(Debug)]
pub enum Conditional {
    NotEqual,

    LessThan,
    GreaterThan,

    LessOrEqual,
    GreaterOrEquals,
}

#[derive(Debug)]
pub struct Tokenizer {
    source: String,
    pos: usize,
    line: usize,
}

impl Tokenizer {

    pub fn new(source: String) -> Self {
        return Self {
            source,
            pos: 0,
            line: 1,
        }
    }

    pub fn tokenize(&mut self, source: String) -> Vec<Token> {
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
                b'\n' => { 
                    tokens.push(Token::Newline);
                    self.line += 1;
                },
                b' ' => continue,
                _ => panic!("Invalid character")
            }
            self.advance()
        }
        return tokens;
    }

    fn tokenize_identifier(&mut self) {

    }

    #[inline]
    fn advance(&mut self) {
        self.pos += 1;
    }
}