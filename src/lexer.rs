use super::token::*;

pub struct Lexer {
    input: String,
    ch: char,
    length: usize,
    position: usize,
    read_position: usize,
}

impl Lexer {
    fn new(input: String) -> Lexer {
        let mut length = input.len();
        length = if length == 0 { 0 } else { length - 1 };
        let mut lexer = Lexer {
            input,
            ch: '\0',
            length,
            position: 0,
            read_position: 0,
        };
        lexer.read();
        lexer
    }

    fn next_token(&mut self) -> Token {
        while self.ch.is_whitespace() {
            self.read();
        }
        let token = match self.ch {
            '(' => Token::Lparen,
            ')' => Token::Rparen,
            '{' => Token::Lbrace,
            '}' => Token::Rbrace,
            '[' => Token::Lbracket,
            ']' => Token::Rbracket,
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            ':' => Token::Colon,
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            '%' => Token::REMAINDER,
            '+' => match self.peek() {
                '0'..='9' => self.read_number(),
                _ => Token::Plus,
            },
            '-' => match self.peek() {
                '0'..='9' => self.read_number(),
                _ => Token::Minus,
            },
            '=' => {
                if self.peek() == '=' {
                    self.read();
                    if self.peek() == '=' {
                        self.read();
                        Token::StrictEq
                    } else {
                        Token::Eq
                    }
                } else {
                    Token::Assign
                }
            }
            '`' => {
                // TODO: read literal body
                Token::TemplateLiteral("".into())
            }
            '!' => {
                if self.peek() == '=' {
                    self.read();
                    if self.peek() == '=' {
                        self.read();
                        Token::NotStrictEq
                    } else {
                        Token::NotEq
                    }
                } else {
                    Token::Bang
                }
            }
            '|' => {
                if self.peek() == '|' {
                    self.read();
                    Token::Or
                } else {
                    Token::BitOr
                }
            }
            '&' => {
                if self.peek() == '&' {
                    self.read();
                    Token::And
                } else {
                    Token::BitAnd
                }
            }
            '.' => {
                if self.peek() == '.' {
                    self.read();
                    if self.peek() == '.' {
                        self.read();
                        Token::Spread
                    } else {
                        Token::Illegal(self.ch.into())
                    }
                } else {
                    Token::Dot
                }
            }
            '<' => {
                if self.peek() == '=' {
                    self.read();
                    if self.peek() == '=' {
                        self.read();
                        Token::LtStrictEq
                    } else {
                        Token::LtEq
                    }
                } else if self.peek() == '<' {
                    self.read();
                    Token::BitShiftLeft
                } else {
                    Token::Lt
                }
            }
            '>' => {
                if self.peek() == '=' {
                    self.read();
                    if self.peek() == '=' {
                        self.read();
                        Token::GtStrictEq
                    } else {
                        Token::GtEq
                    }
                } else if self.peek() == '>' {
                    self.read();
                    Token::BitShiftRight
                } else {
                    Token::Gt
                }
            }
            '~' => Token::BitNot,
            '^' => Token::BitXor,
            '0'..='9' => self.read_number(),
            '"' | '\'' => self.read_string(),
            'a'..='z' | 'A'..='Z' => self.read_ident(),
            '\0' => Token::Eof,
            _ => Token::Illegal(self.ch.to_string()),
        };
        self.read();

        token
    }

    fn read(&mut self) {
        self.ch = if self.read_position > self.length || self.length == 0 {
            '\0'
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        };
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek(&mut self) -> char {
        if self.read_position > self.length {
            '\0'
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        }
    }

    fn read_ident(&mut self) -> Token {
        let mut s = String::from("");
        loop {
            s.push(self.ch);
            match self.peek() {
                'a'..='z' | 'A'..='Z' => self.read(),
                _ => break,
            }
        }

        match s.as_str() {
            "var" => Token::Var,
            "let" => Token::Let,
            "const" => Token::Const,
            "true" => Token::True,
            "false" => Token::False,
            "null" => Token::Null,
            "undefined" => Token::Undefined,
            "this" => Token::This,
            "function" => Token::Function,
            "class" => Token::Class,
            "super" => Token::Super,
            "new" => Token::New,
            "async" => Token::Async,
            "await" => Token::Await,
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            "switch" => Token::Switch,
            "case" => Token::Case,
            "default" => Token::Default,
            "break" => Token::Break,
            "for" => Token::For,
            "while" => Token::While,
            "continue" => Token::Continue,
            "in" => Token::In,
            "of" => Token::Of,
            "instanceof" => Token::Instanceof,
            "delete" => Token::Delete,
            "void" => Token::Void,
            "typeof" => Token::Typeof,
            "try" => Token::Try,
            "catch" => Token::Catch,
            "throw" => Token::Throw,
            "finally" => Token::Finally,
            "interface" => Token::Interface,
            "type" => Token::Type,
            _ => Token::Ident(s),
        }
    }

    fn read_string(&mut self) -> Token {
        let mut s = String::from("");
        loop {
            self.read();
            s.push(self.ch);
            match self.peek() {
                '"' | '\'' => {
                    self.read();
                    break;
                }
                _ => continue,
            }
        }
        Token::String(s)
    }

    fn read_number(&mut self) -> Token {
        let mut chars = Vec::<char>::new();

        loop {
            chars.push(self.ch);
            match self.peek() {
                '0'..='9' | '.' => {
                    self.read();
                }
                _ => {
                    break;
                }
            }
        }

        let s: String = chars.iter().collect();
        if s.contains(".") {
            Token::Float(s.parse::<f64>().unwrap())
        } else {
            Token::Int(s.parse::<isize>().unwrap())
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_token())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_valid_token() {
        let tests: Vec<(&str, Vec<Token>)> = vec![
            ("", vec![Token::Eof]),
            ("abc", vec![Token::Ident("abc".into())]),
            ("101", vec![Token::Int(101)]),
            ("10.1", vec![Token::Float(10.1)]),
            ("\"hello world\"", vec![Token::String("hello world".into())]),
            (
                "=+-!*/(){}[]!=,!==;==:===&&||&|.<...><=>=>==^~<<>>",
                vec![
                    Token::Assign,
                    Token::Plus,
                    Token::Minus,
                    Token::Bang,
                    Token::Asterisk,
                    Token::Slash,
                    Token::Lparen,
                    Token::Rparen,
                    Token::Lbrace,
                    Token::Rbrace,
                    Token::Lbracket,
                    Token::Rbracket,
                    Token::NotEq,
                    Token::Comma,
                    Token::NotStrictEq,
                    Token::Semicolon,
                    Token::Eq,
                    Token::Colon,
                    Token::StrictEq,
                    Token::And,
                    Token::Or,
                    Token::BitAnd,
                    Token::BitOr,
                    Token::Dot,
                    Token::Lt,
                    Token::Spread,
                    Token::Gt,
                    Token::LtEq,
                    Token::GtEq,
                    Token::GtStrictEq,
                    Token::BitXor,
                    Token::BitNot,
                    Token::BitShiftLeft,
                    Token::BitShiftRight,
                ],
            ),
            (
                "var foo = 1;let bar=1.5;const baz=\"hello\"
                true == false
                null === undefined
                this.foo
                function f() { return 1 }
                new class Hoge { super() }
                if (true) { 1 } else if { 2 } else { 3 }
                switch some {
                    case 'foo':
                      break;
                    default:
                      break;
                }
                for (let el in array) {  }
                obj instanceof class
                try {
                  something
                } catch (e) {
                  throw e
                } finally {
                  dosomething
                }
                type Foo {}
                interface I {}
                ",
                vec![
                    Token::Var,
                    Token::Ident("foo".into()),
                    Token::Assign,
                    Token::Int(1),
                    Token::Semicolon,
                    Token::Let,
                    Token::Ident("bar".into()),
                    Token::Assign,
                    Token::Float(1.5),
                    Token::Semicolon,
                    Token::Const,
                    Token::Ident("baz".into()),
                    Token::Assign,
                    Token::String("hello".into()),
                    Token::True,
                    Token::Eq,
                    Token::False,
                    Token::Null,
                    Token::StrictEq,
                    Token::Undefined,
                    Token::This,
                    Token::Dot,
                    Token::Ident("foo".into()),
                    Token::Function,
                    Token::Ident("f".into()),
                    Token::Lparen,
                    Token::Rparen,
                    Token::Lbrace,
                    Token::Return,
                    Token::Int(1),
                    Token::Rbrace,
                    Token::New,
                    Token::Class,
                    Token::Ident("Hoge".into()),
                    Token::Lbrace,
                    Token::Super,
                    Token::Lparen,
                    Token::Rparen,
                    Token::Rbrace,
                    Token::If,
                    Token::Lparen,
                    Token::True,
                    Token::Rparen,
                    Token::Lbrace,
                    Token::Int(1),
                    Token::Rbrace,
                    Token::Else,
                    Token::If,
                    Token::Lbrace,
                    Token::Int(2),
                    Token::Rbrace,
                    Token::Else,
                    Token::Lbrace,
                    Token::Int(3),
                    Token::Rbrace,
                    Token::Switch,
                    Token::Ident("some".into()),
                    Token::Lbrace,
                    Token::Case,
                    Token::String("foo".into()),
                    Token::Colon,
                    Token::Break,
                    Token::Semicolon,
                    Token::Default,
                    Token::Colon,
                    Token::Break,
                    Token::Semicolon,
                    Token::Rbrace,
                    Token::For,
                    Token::Lparen,
                    Token::Let,
                    Token::Ident("el".into()),
                    Token::In,
                    Token::Ident("array".into()),
                    Token::Rparen,
                    Token::Lbrace,
                    Token::Rbrace,
                    Token::Ident("obj".into()),
                    Token::Instanceof,
                    Token::Class,
                    Token::Try,
                    Token::Lbrace,
                    Token::Ident("something".into()),
                    Token::Rbrace,
                    Token::Catch,
                    Token::Lparen,
                    Token::Ident("e".into()),
                    Token::Rparen,
                    Token::Lbrace,
                    Token::Throw,
                    Token::Ident("e".into()),
                    Token::Rbrace,
                    Token::Finally,
                    Token::Lbrace,
                    Token::Ident("dosomething".into()),
                    Token::Rbrace,
                    Token::Type,
                    Token::Ident("Foo".into()),
                    Token::Lbrace,
                    Token::Rbrace,
                    Token::Interface,
                    Token::Ident("I".into()),
                    Token::Lbrace,
                    Token::Rbrace,
                ],
            ),
        ];
        for (i, test) in tests.iter().enumerate() {
            let mut l = Lexer::new(test.0.into());
            for want in test.1.iter() {
                let got = l.next().unwrap();
                assert_eq!(got, *want, "[{}] got={:?}, want={:?}", i, got, want);
            }
        }
    }
}
