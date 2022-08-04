#[derive(Debug, PartialEq)]
pub enum Token {
    Eof,

    // operators
    Assign,                  // =
    Plus,                    // +
    Minus,                   // -
    Bang,                    // !
    Asterisk,                // *
    Slash,                   // /
    REMAINDER,               // %
    Lparen,                  // (
    Rparen,                  // )
    Lbrace,                  // {
    Rbrace,                  // }
    Lbracket,                // [
    Rbracket,                // ]
    Comma,                   // ,
    Semicolon,               // ;
    Colon,                   // :
    Dot,                     // .
    Spread,                  // ...
    Lt,                      // <
    LtEq,                    // <=
    LtStrictEq,              // <==
    Gt,                      // >
    GtEq,                    // >=
    GtStrictEq,              // >==
    Eq,                      // ==
    StrictEq,                // ===
    NotEq,                   // !=
    NotStrictEq,             // !==
    And,                     // &&
    BitAnd,                  // &
    Or,                      // ||
    BitOr,                   // |
    BitXor,                  // ^
    BitNot,                  // ~
    BitShiftLeft,            // <<
    BitShiftRight,           // >>
    TemplateLiteral(String), // ``

    Illegal(String),
    Ident(String),  // abc
    Int(isize),     // 10
    Float(f64),     // 1.1
    String(String), // "hoge"

    // keywords
    Var,        // var
    Let,        // let
    Const,      // const
    True,       // true
    False,      // false
    Null,       // null
    Undefined,  // undefined
    This,       // this
    Function,   // function
    Class,      // class
    Super,      // super
    New,        // new
    Async,      // async
    Await,      // await
    If,         // if
    Else,       // else
    Return,     // return
    Switch,     // switch
    Case,       // case
    Default,    // default
    Break,      // break
    For,        // for
    While,      // while
    Continue,   // continue
    In,         // in
    Of,         // of
    Instanceof, // instanceof
    Delete,     // delete
    Void,       // void
    Typeof,     // typeof
    Try,        // try
    Catch,      // catch
    Throw,      // throw
    Finally,    // finally
    Interface,  // interface
    Type,       // type
}
