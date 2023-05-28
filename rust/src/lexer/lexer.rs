use anyhow::Result;

#[derive(Debug, PartialEq)]
pub enum Token {
    Ident(String),
    Int(String),

    Illegal,
    Eof,
    Equal,
    Plus,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    LSquirly,
    RSquirly,
    Function,
    Let,

    Const,
    Static,
    Period,
    DQuote,
    SQuote,
    Colon,
    DoubleColon,
    Mod,
    Pub,
    Crate,
    Use,
    As,
    Extern,
    SelfType,
    Self_,
    Range,
    RangeInclusive,
    DefaultFields,
    For,
    In,
    If,
    Else,
    Match,
    While,
    Loop,
    Continue,
    Break,
    Return,
    PlusEqual,
    MinusEqual,
    Mut,
    Ampersand,
    AmpersandAmpersand,
    Pipe,
    PipePipe,
    Bang,
    BangEqual,
    EqualEqual,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    I128,
    U128,
    Isize,
    Usize,
    F32,
    F64,
    Bool,
    Char,
    Str,
    True,
    False,
    Default,
    Type,
    Struct,
    Enum,
    Trait,
    Impl,
    Arrow,
    Minus,
}

pub struct Lexer {
    position: usize,
    read_position: usize,
    ch: u8,
    input: Vec<u8>,
}

impl Lexer {
    fn new(input: String) -> Lexer {
        let mut lex = Lexer {
            position: 0,
            read_position: 0,
            ch: 0,
            input: input.into_bytes(),
        };
        lex.read_char();

        return lex;
    }

    pub fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespace();

        let tok = match self.ch {
            b'{' => Token::LSquirly,
            b'}' => Token::RSquirly,
            b'(' => Token::Lparen,
            b')' => Token::Rparen,
            b',' => Token::Comma,
            b';' => Token::Semicolon,
            b'+' => {
                let ident = self.read_match_any(&[b'+', b'=']);
                return Ok(match ident.as_str() {
                    "+=" => Token::PlusEqual,
                    _ => Token::Plus,
                });
            }
            b'=' => Token::Equal,
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_ident();
                return Ok(match ident.as_str() {
                    "fn" => Token::Function,
                    "let" => Token::Let,
                    "const" => Token::Const,
                    "static" => Token::Static,
                    "mod" => Token::Mod,
                    "pub" => Token::Pub,
                    "crate" => Token::Crate,
                    "use" => Token::Use,
                    "as" => Token::As,
                    "extern" => Token::Extern,
                    "Self" => Token::SelfType,
                    "self" => Token::Self_,
                    "for" => Token::For,
                    "in" => Token::In,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "match" => Token::Match,
                    "while" => Token::While,
                    "loop" => Token::Loop,
                    "continue" => Token::Continue,
                    "break" => Token::Break,
                    "return" => Token::Return,
                    "mut" => Token::Mut,
                    "i8" => Token::I8,
                    "u8" => Token::U8,
                    "i16" => Token::I16,
                    "u16" => Token::U16,
                    "i32" => Token::I32,
                    "u32" => Token::U32,
                    "i64" => Token::I64,
                    "u64" => Token::U64,
                    "i128" => Token::I128,
                    "u128" => Token::U128,
                    "isize" => Token::Isize,
                    "usize" => Token::Usize,
                    "f32" => Token::F32,
                    "f64" => Token::F64,
                    "bool" => Token::Bool,
                    "char" => Token::Char,
                    "str" => Token::Str,
                    "true" => Token::True,
                    "false" => Token::False,
                    "default" => Token::Default,
                    "type" => Token::Type,
                    "struct" => Token::Struct,
                    "enum" => Token::Enum,
                    "trait" => Token::Trait,
                    "impl" => Token::Impl,
                    "=>" => Token::Arrow,

                    _ => Token::Ident(ident),
                });
            }

            b'0'..=b'9' => return Ok(Token::Int(self.read_int())),
            0 => Token::Eof,
            b'.' => {
                let ident = self.read_match_any(&[b'.', b'=']);
                return Ok(match ident.as_str() {
                    "." => Token::Period,
                    ".." => Token::Range,
                    "..=" => Token::RangeInclusive,
                    "..." => Token::DefaultFields,
                    _ => todo!("we need to implement this....1"),
                });
            }
            b':' => {
                let ident = self.read_match(b':');
                return Ok(match ident.as_str() {
                    "::" => Token::DoubleColon,
                    _ => Token::Colon,
                });
            }
            b'-' => {
                let ident = self.read_match_any(&[b'-', b'=', b'>']);
                return Ok(match ident.as_str() {
                    "-=" => Token::MinusEqual,
                    "->" => Token::Arrow,
                    _ => Token::Minus,
                });
            }
            _ => todo!("we need to implement this....2"),
        };

        self.read_char();
        return Ok(tok);
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn read_ident(&mut self) -> String {
        let pos = self.position;
        // fist char must be a letter or underscore
        if self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read_char();
            // the rest of the ident can be alphanumeric or underscore
            while self.ch.is_ascii_alphanumeric() || self.ch == b'_' {
                self.read_char();
            }
        }

        return String::from_utf8_lossy(&self.input[pos..self.position]).to_string();
    }

    fn read_match(&mut self, match_byte: u8) -> String {
        let pos = self.position;
        while self.ch == match_byte {
            self.read_char();
        }

        return String::from_utf8_lossy(&self.input[pos..self.position]).to_string();
    }

    fn read_match_any(&mut self, match_bytes: &[u8]) -> String {
        let pos = self.position;
        while match_bytes.contains(&self.ch) {
            self.read_char();
        }

        return String::from_utf8_lossy(&self.input[pos..self.position]).to_string();
    }

    fn read_int(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }

        return String::from_utf8_lossy(&self.input[pos..self.position]).to_string();
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;

    use super::{Lexer, Token};

    #[test]
    fn get_next_token() -> Result<()> {
        let input = "=+(){},;";
        let mut lexer = Lexer::new(input.into());

        let tokens = vec![
            Token::Equal,
            Token::Plus,
            Token::Lparen,
            Token::Rparen,
            Token::LSquirly,
            Token::RSquirly,
            Token::Comma,
            Token::Semicolon,
        ];

        for token in tokens {
            let next_token = lexer.next_token()?;
            println!("expected: {:?}, received {:?}", token, next_token);
            assert_eq!(token, next_token);
        }

        return Ok(());
    }

    #[test]
    fn get_next_complete() -> Result<()> {
        let input = r#"let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            };
            let result = add(five, ten);"#;

        let mut lex = Lexer::new(input.into());

        let tokens = vec![
            Token::Let,
            Token::Ident(String::from("five")),
            Token::Equal,
            Token::Int(String::from("5")),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("ten")),
            Token::Equal,
            Token::Int(String::from("10")),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("add")),
            Token::Equal,
            Token::Function,
            Token::Lparen,
            Token::Ident(String::from("x")),
            Token::Comma,
            Token::Ident(String::from("y")),
            Token::Rparen,
            Token::LSquirly,
            Token::Ident(String::from("x")),
            Token::Plus,
            Token::Ident(String::from("y")),
            Token::Semicolon,
            Token::RSquirly,
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("result")),
            Token::Equal,
            Token::Ident(String::from("add")),
            Token::Lparen,
            Token::Ident(String::from("five")),
            Token::Comma,
            Token::Ident(String::from("ten")),
            Token::Rparen,
            Token::Semicolon,
            Token::Eof,
        ];

        for token in tokens {
            let next_token = lex.next_token()?;
            println!("expected: {:?}, received {:?}", token, next_token);
            assert_eq!(token, next_token);
        }

        return Ok(());
    }

    #[test]
    fn get_nutz() -> Result<()> {
        let input = r#"
            pub mod nutz {
                use crate::both_nutz;
                pub const NUTZ: u32 = 5;
                pub static NUTZ2: u32 = 10;
                
                fn bunch_o_nutz() -> u32 {
                    let rng1 = NUTZ..NUTZ2;
                    let rng2 = NUTZ..=NUTZ2;
                    
                    let mut x = 0;
                    for i in rng1 {
                        x += i;
                    }
                    
                    for i in rng2 {
                        x += i;
                    }
                    i
                }   
            }            
            "#;

        let mut lex = Lexer::new(input.into());

        let tokens = vec![
            Token::Pub,
            Token::Mod,
            Token::Ident(String::from("nutz")),
            Token::LSquirly,
            Token::Use,
            Token::Crate,
            Token::DoubleColon,
            Token::Ident(String::from("both_nutz")),
            Token::Semicolon,
            Token::Pub,
            Token::Const,
            Token::Ident(String::from("NUTZ")),
            Token::Colon,
            Token::U32,
            Token::Equal,
            Token::Int(String::from("5")),
            Token::Semicolon,
            Token::Pub,
            Token::Static,
            Token::Ident(String::from("NUTZ2")),
            Token::Colon,
            Token::U32,
            Token::Equal,
            Token::Int(String::from("10")),
            Token::Semicolon,
            Token::Function,
            Token::Ident(String::from("bunch_o_nutz")),
            Token::Lparen,
            Token::Rparen,
            Token::Arrow,
            Token::U32,
            Token::LSquirly,
            Token::Let,
            Token::Ident(String::from("rng1")),
            Token::Equal,
            Token::Ident(String::from("NUTZ")),
            Token::Range,
            Token::Ident(String::from("NUTZ2")),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("rng2")),
            Token::Equal,
            Token::Ident(String::from("NUTZ")),
            Token::RangeInclusive,
            Token::Ident(String::from("NUTZ2")),
            Token::Semicolon,
            Token::Let,
            Token::Mut,
            Token::Ident(String::from("x")),
            Token::Equal,
            Token::Int(String::from("0")),
            Token::Semicolon,
            Token::For,
            Token::Ident(String::from("i")),
            Token::In,
            Token::Ident(String::from("rng1")),
            Token::LSquirly,
            Token::Ident(String::from("x")),
            Token::PlusEqual,
            Token::Ident(String::from("i")),
            Token::Semicolon,
            Token::RSquirly,
            Token::For,
            Token::Ident(String::from("i")),
            Token::In,
            Token::Ident(String::from("rng2")),
            Token::LSquirly,
            Token::Ident(String::from("x")),
            Token::PlusEqual,
            Token::Ident(String::from("i")),
            Token::Semicolon,
            Token::RSquirly,
            Token::Ident(String::from("i")),
            Token::RSquirly,
            Token::RSquirly,
            Token::Eof,
        ];

        for token in tokens {
            let next_token = lex.next_token()?;
            println!("expected: {:?}, received {:?}", token, next_token);
            assert_eq!(token, next_token);
        }

        return Ok(());
    }
}
