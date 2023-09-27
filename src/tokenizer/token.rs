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


