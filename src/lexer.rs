/// A token in the source code
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Copy)]
pub struct Register(u8, RegisterType);

#[derive(Debug, Clone, Copy)]
enum RegisterType {
    Low,
    High,
    Word,
    Double,
    Quad,
}

#[derive(Debug, Clone, Copy)]
pub enum Directive {
    Entry,
    Include,
    Text,
    Arch,
    Def
}

/// Does not include `=` as it depends on context.
#[derive(Debug, Clone, Copy)]
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
    tokens: Vec<Token>,
    pos: usize,
    line: usize,
}

impl Tokenizer {

    pub fn new(source: String) -> Self {
        return Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            pos: 0,
            line: 1,
        }
    }

    pub fn tokenize(&mut self) -> &Vec<Token> {
        while !self.at_end() {
            match self.current() {
                '+' => self.push_single_token(Token::Plus),
                '-' => self.push_single_token(Token::Minus),
                '*' => self.push_single_token(Token::Star),
                '/' => self.push_single_token(Token::Slash),
                '!' => self.push_single_token(Token::Bang),
                '(' => self.push_single_token(Token::LParen),
                ')' => self.push_single_token(Token::RParen),
                '{' => self.push_single_token(Token::LCurly),
                '}' => self.push_single_token(Token::RCurly),
                '[' => self.push_single_token(Token::LSquare),
                ']' => self.push_single_token(Token::RSquare),

                'a'..='z' | 'A'..='Z' | '_' => {

                },
                '\n' => { 
                    self.push_single_token(Token::Newline);
                    self.line += 1;
                },
                ' ' => continue,
                _ => panic!("Invalid character")
            }
        }
        self.tokens.push(Token::EOF);
        return &self.tokens;
    }


    /// Pushes a single token and advances by 1 unit.
    fn push_single_token(&mut self, token: Token) {
        self.advance();
        self.tokens.push(token);
    }

    fn tokenize_identifier(&mut self) {
        
    }


    /// Gets the current token
    #[inline]
    pub fn current(&self) -> char {
        self.source[self.pos]
    }

    /// Advances by one position.
    #[inline]
    fn advance(&mut self) {
        self.pos += 1;
    }

    #[inline]
    fn at_end(&self) -> bool {
        self.pos <= self.source.len()
    }

    #[inline]
    fn peek(&self) -> char {
        self.source[self.pos + 1]
    } 
}