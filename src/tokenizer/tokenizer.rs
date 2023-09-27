use super::token::Token;

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
        println!("{:?}", self.source);
        while !self.at_end() {
            match self.current() {
                '+' => self.push_and_advance(Token::Plus),
                '-' => self.push_and_advance(Token::Minus),
                '*' => self.push_and_advance(Token::Star),
                '/' => self.push_and_advance(Token::Slash),
                '!' => self.push_and_advance(Token::Bang),
                '(' => self.push_and_advance(Token::LParen),
                ')' => self.push_and_advance(Token::RParen),
                '{' => self.push_and_advance(Token::LCurly),
                '}' => self.push_and_advance(Token::RCurly),
                '[' => self.push_and_advance(Token::LSquare),
                ']' => self.push_and_advance(Token::RSquare),
                '0'..='9' => {
                    let num = self.tokenize_number();
                    self.tokens.push(Token::Number(num))
                },
                'a'..='z' | 'A'..='Z' | '_' => {

                },
                '\n' => { 
                    self.push_and_advance(Token::Newline);
                    self.line += 1;
                },
                ' ' => {
                    while !self.at_end() && self.current() == ' ' {
                        self.advance()
                    }
                },
                _ => panic!("Invalid character")
            }
        }
        self.tokens.push(Token::EOF);
        return &self.tokens;
    }


    /// Pushes a single token and advances by 1 unit.
    fn push_and_advance(&mut self, token: Token) {
        println!("Pushed a token!");
        self.advance();
        self.tokens.push(token);
    }

    fn tokenize_number(&mut self) -> i128 {
        0
    }

    fn tokenize_identifier(&mut self) {
        
    }

    /// Gets the current token
    #[inline]
    pub fn current(&self) -> char {
        *self.source.get(self.pos).unwrap()
    }

    /// Advances by one position.
    #[inline]
    fn advance(&mut self) {
        self.pos += 1;
    }

    #[inline]
    fn at_end(&self) -> bool {
        self.pos >= self.source.len()
    }

    #[inline]
    fn peek(&self) -> char {
        self.source[self.pos + 1]
    } 
}

#[test]
fn test_tokenizer() {
    let tokens = "
    !+-{}()[]
    ";
    let mut tokenizer  = Tokenizer::new(tokens.into());
    println!("{:?}", tokenizer.tokenize());
}