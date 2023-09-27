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
    source: Vec<char>,
    pos: usize,
    line: usize,
}

impl Tokenizer {

    pub fn new(source: String) -> Self {
        return Self {
            source: source.chars().collect(),
            pos: 0,
            line: 1,
        }
    }

    pub fn tokenize(&mut self, source: String) -> Vec<Token> {
        let mut tokens  = Vec::new();

        while !self.at_end() {
            match self.current() {
                '+' => tokens.push(Token::Plus),
                '-' => tokens.push(Token::Minus),
                '*' => tokens.push(Token::Star),
                '/' => tokens.push(Token::Slash),
                '!' => tokens.push(Token::Bang),
                '(' => tokens.push(Token::LParen),
                ')' => tokens.push(Token::RParen),
                '{' => tokens.push(Token::LCurly),
                '}' => tokens.push(Token::RCurly),
                '[' => tokens.push(Token::LSquare),
                ']' => tokens.push(Token::RSquare),
                

                'a'..='z' | 'A'..='Z' | '_' => {

                },
                '\n' => { 
                    tokens.push(Token::Newline);
                    self.line += 1;
                },
                ' ' => continue,
                _ => panic!("Invalid character")
            }
            self.advance();
        }
        tokens.push(Token::EOF);
        return tokens;
    }

    fn tokenize_identifier(&mut self) {
        
    }


    #[inline]
    pub fn current(&self) -> char {
        self.source[self.pos]
    }

    #[inline]
    fn advance(&mut self) {
        self.pos += 1;
    }

    fn at_end(&self) -> bool {
        self.pos <= self.source.len()
    }

    fn peek(&self) -> char {
        self.source[self.pos + 1]
    } 
}