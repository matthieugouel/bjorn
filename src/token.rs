#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    INT(String),
    FLOAT(String),
    BOOL(bool),
    ID(String),

    NEWLINE,
    INDENT,
    DEDENT,

    IF,
    ELSE,
    WHILE,

    DEF,
    RETURN,

    COLON,
    COMMA,

    OR,
    AND,
    NOT,

    EQ,
    NE,
    LE,
    GE,
    LT,
    GT,

    ASSIGN,

    PLUS,
    MINUS,
    MUL,
    DIV,

    LPAREN,
    RPAREN,

    EOF,
}

impl Token {
    pub fn integer(&self) -> Option<i32> {
        if let Token::INT(i) = self {
            return match i.parse::<i32>() {
                Ok(i) => Some(i),
                Err(_) => None
            }
        }
        None
    }

    pub fn float(&self) -> Option<f64> {
        if let Token::FLOAT(i) = self {
            return match i.parse::<f64>() {
                Ok(i) => Some(i),
                Err(_) => None
            }
        }
        None
    }

    pub fn boolean(&self) -> Option<bool> {
        if let Token::BOOL(i) = self {
            return Some(*i)
        }
        None
    }

    pub fn identifier(&self) -> Option<String> {
        if let Token::ID(i) = self {
            return match i.parse::<String>() {
                Ok(i) => Some(i),
                Err(_) => None
            }
        }
        None
    }
}
