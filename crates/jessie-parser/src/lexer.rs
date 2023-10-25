// just wrap the parser state to accumulate the tokens

use std::fmt::{Debug, Display};

use crate::parser::ParserState;

use utils::SharedString;

pub struct Lexer {
    state: ParserState<char>,
    parenthesize_stack: Vec<ParenthesisIndex>,
}

impl Lexer {
    pub fn new(input: Vec<char>) -> Self {
        Lexer {
            state: ParserState::new(input),
            parenthesize_stack: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.state.is_empty()
    }

    pub fn lookahead_1(&self) -> Option<char> {
        self.state.lookahead_1()
    }

    pub fn lookahead_2(&self) -> Option<char> {
        self.state.lookahead_2()
    }

    pub fn lookahead_3(&self) -> Option<char> {
        self.state.lookahead_3()
    }

    pub fn lookahead_4(&self) -> Option<char> {
        self.state.lookahead_4()
    }

    pub fn lookahead_n(&self, n: usize) -> Option<char> {
        self.state.lookahead_n(n)
    }

    pub fn proceed_with(&mut self, token: Token) -> Token {
        self.state.proceed_with(token)
    }

    pub fn proceed(&mut self) -> Option<char> {
        self.state.proceed()
    }

    pub fn consume(&mut self, token: Vec<char>) -> Result<(), String> {
        self.state.consume(token)
    }
    
    pub fn open_paren(&mut self, index: usize) {
        self.parenthesize_stack.push(ParenthesisIndex(index));
    }

    pub fn close_paren(&mut self) -> Result<usize, String> {
        match self.parenthesize_stack.pop() {
            Some(ParenthesisIndex(index)) => Ok(index),
            _ => Err("Parenthesis mismatch".to_string()),
        }
    }
}

pub struct ParenthesisIndex(usize);

// Valid jessie tokens
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    EOF,

    // Keyword
    Break,
    Case,
    Catch,
    Const,
    Continue,
    Debugger,
    Default,
    Else,
    Export,
    Finally,
    For,
    Function,
    If,
    Import,
    Let,
    Return,
    Switch,
    Throw,
    Try,
    // TypeOf,
    Void,
    While,

    // ReservedWord
    Null,
    False,
    True,
    Async,
    Arguments,
    Eval,
    Get,
    Set,

    // ReservedKeyword
    Class,
    Delete,
    Do,
    Extends,
    In,
    New,
    Super,
    This,
    Var,
    With,
    Yield,

    // FutureReservedWord

    Await,
    Enum,
    Implements,
    Package,
    Protected,
    Interface,
    Private,
    Public,
    
    // Operators
    // Binary
    BarBar, // ||
    QuestionQuestion, // ??
    AmpAmp, // &&
    Bar, // |
    Caret, // ^
    Ampersand, // &
    EqualEqualEqual, // ===
    BangEqualEqual, // !==
    LAngle, // <
    LAngleEqual, // <=
    RAngle, // >
    RAngleEqual, // >=
    LAngleLAngle, // <<
    RAngleRAngle, // >>
    RAngleRAngleRAngle, // >>>
    Plus, // +
    Minus, // -
    Asterisk, // *
    Slash, // /
    Percent, // %
    AsteriskAsterisk, // **
    // Assignment
    Equal, // =
    PlusEqual, // +=
    MinusEqual, // -=
    AsteriskEqual, // *=
    SlashEqual, // /=
    PercentEqual, // %=
    BarEqual, // |=
    AmpersandEqual, // &=
    CaretEqual, // ^=
    LAngleLAngleEqual, // <<=
    RAngleRAngleEqual, // >>=
    RAngleRAngleRAngleEqual, // >>>=
    // TODO: Add more assignment operators
    // Unary
    TypeOf, // typeof
    Tilde, // ~
    Bang, // !
    // Punctuation
    LeftParen, // (
    RightParen, // )
    ArrowLeftParen, // (, followed by =>
    ArrowRightParen, // ), followed by =>
    LeftBrace, // {
    RightBrace, // }
    LeftBracket, // [
    RightBracket, // ]
    Comma, // ,
    Dot, // .
    Colon, // :
    Semicolon, // ;
    DotDotDot, // ...
    FatArrow, // =>
    Question, // ?
    QuasiQuote, // `
    Dollar, // $
    DoubleSlash, // //
    SlashAsterisk, // /*
    AsteriskSlash, // */
    // Literals
    Undefined,
    Identifier(SharedString),
    String(SharedString),
    Integer(i64),
    Decimal(i64, u64),
    Bigint(bool, Box<[u64]>),

    // ????
    Instanceof,
    Require,
    Static,
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Token::EOF => "EOF".to_string(),
            Token::Await => "await".to_string(),
            Token::Break => "break".to_string(),
            Token::Case => "case".to_string(),
            Token::Catch => "catch".to_string(),
            Token::Const => "const".to_string(),
            Token::Continue => "continue".to_string(),
            Token::Debugger => "debugger".to_string(),
            Token::Default => "default".to_string(),
            Token::Do => "do".to_string(),
            Token::Else => "else".to_string(),
            Token::Export => "export".to_string(),
            Token::Extends => "extends".to_string(),
            Token::Enum => "enum".to_string(),
            Token::Finally => "finally".to_string(),
            Token::For => "for".to_string(),
            Token::Function => "function".to_string(),
            Token::If => "if".to_string(),
            Token::Instanceof => "instanceof".to_string(),
            Token::In => "in".to_string(),
            Token::Implements => "implements".to_string(),
            Token::Import => "import".to_string(),
            Token::Interface => "interface".to_string(),
            Token::Let => "let".to_string(),
            Token::Package => "package".to_string(),
            Token::Protected => "protected".to_string(),
            Token::Private => "private".to_string(),
            Token::Public => "public".to_string(),
            Token::Return => "return".to_string(),
            Token::Require => "require".to_string(),
            Token::Switch => "switch".to_string(),
            Token::Super => "super".to_string(),
            Token::Static => "static".to_string(),
            Token::Throw => "throw".to_string(),
            Token::Try => "try".to_string(),
            Token::This => "this".to_string(),
            Token::Void => "void".to_string(),
            Token::Var => "var".to_string(),
            Token::With => "with".to_string(),
            Token::While => "while".to_string(),
            Token::Yield => "yield".to_string(),
            Token::Null => "null".to_string(),
            Token::New => "new".to_string(),
            Token::False => "false".to_string(),
            Token::True => "true".to_string(),
            Token::Async => "async".to_string(),
            Token::Arguments => "arguments".to_string(),
            Token::Eval => "eval".to_string(),
            Token::Get => "get".to_string(),
            Token::Set => "set".to_string(),
            Token::Class => "class".to_string(),
            Token::Delete => "delete".to_string(),
            Token::AmpAmp => "&&".to_string(),
            Token::BarBar => "||".to_string(),
            Token::QuestionQuestion => "??".to_string(),
            Token::Caret => "^".to_string(),
            Token::Ampersand => "&".to_string(),
            Token::EqualEqualEqual => "===".to_string(),
            Token::BangEqualEqual => "!==".to_string(),
            Token::LAngle => "<".to_string(),
            Token::LAngleEqual => "<=".to_string(),
            Token::RAngle => ">".to_string(),
            Token::RAngleEqual => ">=".to_string(),
            Token::LAngleLAngle => "<<".to_string(),
            Token::RAngleRAngle => ">>".to_string(),
            Token::RAngleRAngleRAngle => ">>>".to_string(),
            Token::Plus => "+".to_string(),
            Token::Minus => "-".to_string(),
            Token::Asterisk => "*".to_string(),
            Token::Slash => "/".to_string(),
            Token::Percent => "%".to_string(),
            Token::AsteriskAsterisk => "**".to_string(),
            Token::Bar => "|".to_string(),
            Token::Equal => "=".to_string(),
            Token::PlusEqual => "+=".to_string(),
            Token::MinusEqual => "-=".to_string(),
            Token::AsteriskEqual => "*=".to_string(),
            Token::SlashEqual => "/=".to_string(),
            Token::PercentEqual => "%=".to_string(),
            Token::BarEqual => "|=".to_string(),
            Token::AmpersandEqual => "&=".to_string(),
            Token::CaretEqual => "^=".to_string(),
            Token::LAngleLAngleEqual => "<<=".to_string(),
            Token::RAngleRAngleEqual => ">>=".to_string(),
            Token::RAngleRAngleRAngleEqual => ">>>=".to_string(),
            Token::TypeOf => "typeof".to_string(),
            Token::Tilde => "~".to_string(),
            Token::Bang => "!".to_string(),
            Token::LeftParen => "(".to_string(),
            Token::RightParen => ")".to_string(),
            Token::ArrowLeftParen => "(".to_string(),
            Token::ArrowRightParen => ")".to_string(),
            Token::LeftBrace => "{".to_string(),
            Token::RightBrace => "}".to_string(),
            Token::LeftBracket => "[".to_string(),
            Token::RightBracket => "]".to_string(),
            Token::Comma => ",".to_string(),
            Token::Dot => ".".to_string(),
            Token::Colon => ":".to_string(),
            Token::Semicolon => ";".to_string(),
            Token::DotDotDot => "...".to_string(),
            Token::FatArrow => "=>".to_string(),
            Token::Question => "?".to_string(),
            Token::QuasiQuote => "`".to_string(),
            Token::Dollar => "$".to_string(),
            Token::DoubleSlash => "//".to_string(),
            Token::SlashAsterisk => "/*".to_string(),
            Token::AsteriskSlash => "*/".to_string(),

            Token::Undefined => "undefined".to_string(),
            Token::Identifier(s) => s.to_string(),
            Token::String(s) => s.to_string(),
            Token::Integer(i) => i.to_string(),
            Token::Decimal(i, f) => format!("{}.{}", i, f), // TODO
            Token::Bigint(s, v) => {
                let mut s = if *s { "-".to_string() } else { "".to_string() };
                for i in v.iter() {
                    s.push_str(&i.to_string()); // TODO
                }
                s
            },
        }
    }
}

#[inline]
fn given_keyword_or_ident(lexer: &mut Lexer, keyword: &'static str, token: Token) -> Result<Token, String> {
    // if the following character of the (supposed) keyword can form an identifier, namely, one of a-z, A-Z, $, _, or 0-9, then it is not a keyword.
    // proceed to identifier parsing
    if lexer.lookahead_n(keyword.len()+1).map(|x| x.is_alphabetic() || x == '_' || x.is_digit(10)).unwrap_or(false) {
        println!("fastpath ident given_keyword_or_ident");
        return ident(lexer);   
    }

    if lexer.consume(keyword.chars().collect()).is_ok() {
        println!("keyword given_keyword_or_ident: {:?}", token);
        Ok(token)
    } else {
        println!("ident given_keyword_or_ident: {:?}", Token::Identifier(SharedString::from_str(keyword)));
        ident(lexer)
    }
}

fn keyword_or_ident(lexer: &mut Lexer) -> Result<Token, String> {
    // keywords in this list may or may not be used in the actual script,
    // but need to be reserved for future use
    /*
    A: async, arguments, await
    B: break, bigint, bool(ts)
    C: case, catch, class, const, continue
    D: delete, do, debugger, default
    E: else, enum, extends, eval
    F: finally, for, function
    G: get
    I: if, implements, import, in, instanceof, interface
    L: let
    N: new, null, number(ts)
    P: package, private, protected, public
    R: return, require
    S: set, static, string(ts), super, switch, symbol(ts)
    T: this, throw, try, typeof, true
    U: undefined,
    V: var, void
    W: while, with, while
    Y: yield
    */

    println!("keyword_or_ident: {:?}", lexer.lookahead_1());

    match lexer.lookahead_1() {
        Some('a') => 
            match lexer.lookahead_2() {
                Some('s') => given_keyword_or_ident(lexer, &"async", Token::Async),
                Some('r') => given_keyword_or_ident(lexer, &"arguments", Token::Arguments),
                Some('w') => given_keyword_or_ident(lexer, &"await", Token::Await),
                _ => ident(lexer),
            },
        Some('b') => {
            match lexer.lookahead_2() {
                Some('r') => given_keyword_or_ident(lexer, &"break", Token::Break),
                // Some('i') => given_keyword_or_ident(lexer, &"bigint", Token::Bigint("".to_string())),
                // Some('o') => given_keyword_or_ident(lexer, &"bool", Token::Identifier("bool".to_string())),
                _ => ident(lexer),
            }
        },
        Some('c') => {
            match lexer.lookahead_2() {
                Some('a') => match lexer.lookahead_3() {
                    Some('s') => given_keyword_or_ident(lexer, &"case", Token::Case),
                    Some('t') => given_keyword_or_ident(lexer, &"catch", Token::Catch),
                    _ => ident(lexer),
                },
                Some('l') => given_keyword_or_ident(lexer, &"class", Token::Class),
                Some('o') => match lexer.lookahead_3() {
                    Some('n') => match lexer.lookahead_4() {
                        Some('s') => given_keyword_or_ident(lexer, &"const", Token::Const),
                        Some('t') => given_keyword_or_ident(lexer, &"continue", Token::Continue),
                        _ => ident(lexer),
                    },
                    _ => ident(lexer),
                },
                _ => ident(lexer),
            }
        },
        Some('d') => {
            match lexer.lookahead_2() {
                Some('e') => match lexer.lookahead_3() {
                    Some('l') => given_keyword_or_ident(lexer, &"delete", Token::Delete),
                    Some('b') => given_keyword_or_ident(lexer, &"debugger", Token::Debugger),
                    Some('f') => given_keyword_or_ident(lexer, &"default", Token::Default),
                    _ => ident(lexer),
                },
                Some('o') => given_keyword_or_ident(lexer, &"do", Token::Do),
                _ => ident(lexer),
            }
        },
        Some('e') => {
            match lexer.lookahead_2() {
                Some('l') => given_keyword_or_ident(lexer, &"else", Token::Else),
                Some('n') => given_keyword_or_ident(lexer, &"enum", Token::Enum),
                Some('x') => given_keyword_or_ident(lexer, &"extends", Token::Extends),
                Some('v') => given_keyword_or_ident(lexer, &"eval", Token::Eval),
                _ => ident(lexer),
            }
        },
        Some('f') => {
            match lexer.lookahead_2() {
                Some('i') => given_keyword_or_ident(lexer, &"finally", Token::Finally),
                Some('o') => given_keyword_or_ident(lexer, &"for", Token::For),
                Some('u') => given_keyword_or_ident(lexer, &"function", Token::Function),
                _ => ident(lexer),
            }
        },
        Some('g') => {
            match lexer.lookahead_2() {
                Some('e') => given_keyword_or_ident(lexer, &"get", Token::Get),
                _ => ident(lexer),
            }
        },
        Some('i') => {
            match lexer.lookahead_2() {
                Some('f') => given_keyword_or_ident(lexer, &"if", Token::If),
                Some('m') => match lexer.lookahead_3() {
                    Some('p') => match lexer.lookahead_4() {
                        Some('l') => given_keyword_or_ident(lexer, &"implements", Token::Implements),
                        Some('o') => given_keyword_or_ident(lexer, &"import", Token::Import),
                        _ => ident(lexer),
                    },
                    _ => ident(lexer),
                }
                Some('n') => match lexer.lookahead_3() {
                    Some('s') => given_keyword_or_ident(lexer, &"instanceof", Token::Instanceof),
                    Some('t') => given_keyword_or_ident(lexer, &"interface", Token::Interface),
                    _ => given_keyword_or_ident(lexer, &"in", Token::In),
                },
                _ => ident(lexer),
            }
        },
        Some('l') => given_keyword_or_ident(lexer, &"let", Token::Let),
        Some('n') => {
            match lexer.lookahead_2() {
                Some('e') => given_keyword_or_ident(lexer, &"new", Token::New),
                Some('u') => match lexer.lookahead_3() {
                    Some('l') => given_keyword_or_ident(lexer, &"null", Token::Null),
                    // Some('m') => given_keyword_or_ident(lexer, &"number", Token::Identifier("number".to_string())),
                    _ => ident(lexer),
                },
                _ => ident(lexer),
            }
        },
        Some('p') => {
            match lexer.lookahead_2() {
                Some('a') => given_keyword_or_ident(lexer, &"package", Token::Package),
                Some('r') => match lexer.lookahead_3() {
                    Some('i') => given_keyword_or_ident(lexer, &"private", Token::Private),
                    Some('o') => given_keyword_or_ident(lexer, &"protected", Token::Protected),
                    _ => ident(lexer),
                },
                Some('u') => given_keyword_or_ident(lexer, &"public", Token::Public),
                _ => ident(lexer),
            }
        },
        Some('r') => {
            match lexer.lookahead_2() {
                Some('e') => match lexer.lookahead_3() {
                    Some('t') => given_keyword_or_ident(lexer, &"return", Token::Return),
                    Some('q') => given_keyword_or_ident(lexer, &"require", Token::Require),
                    _ => ident(lexer),
                },
                _ => ident(lexer),
            }
        },
        Some('s') => {
            match lexer.lookahead_2() {
                Some('e') => given_keyword_or_ident(lexer, &"set", Token::Set),
                Some('t') => match lexer.lookahead_3() {
                    Some('a') => given_keyword_or_ident(lexer, &"static", Token::Static),
                    // Some('r') => given_keyword_or_ident(lexer, &"string", Token::String),
                    _ => ident(lexer),
                }
                Some('u') => given_keyword_or_ident(lexer, &"super", Token::Super),
                Some('w') => given_keyword_or_ident(lexer, &"switch", Token::Switch),
                _ => ident(lexer),
            }
        },
        Some('t') => {
            match lexer.lookahead_2() {
                Some('h') => given_keyword_or_ident(lexer, &"this", Token::This),
                Some('r') => match lexer.lookahead_3() {
                    Some('u') => given_keyword_or_ident(lexer, &"true", Token::True),
                    Some('h') => given_keyword_or_ident(lexer, &"throw", Token::Throw),
                    Some('y') => given_keyword_or_ident(lexer, &"try", Token::Try),
                    _ => ident(lexer),
                },
                Some('y') => given_keyword_or_ident(lexer, &"typeof", Token::TypeOf),
                _ => ident(lexer),
            }
        },
        Some('u') => given_keyword_or_ident(lexer, &"undefined", Token::Undefined),
        Some('v') => {
            match lexer.lookahead_2() {
                Some('a') => given_keyword_or_ident(lexer, &"var", Token::Var),
                Some('o') => given_keyword_or_ident(lexer, &"void", Token::Void),
                _ => ident(lexer),
            }
        },
        Some('w') => {
            match lexer.lookahead_2() {
                Some('h') => given_keyword_or_ident(lexer, &"while", Token::While),
                Some('i') => given_keyword_or_ident(lexer, &"with", Token::With),
                _ => ident(lexer),
            }
        },
        Some('y') => {
            match lexer.lookahead_2() {
                Some('i') => given_keyword_or_ident(lexer, &"yield", Token::Yield),
                _ => ident(lexer),
            }
        },
        Some(_) => ident(lexer),
        None => unreachable!("Lexer should not be empty"),
    }
}

pub fn lex_jessie(input: String) -> Result<Vec<Token>, String> {
    let mut result = Vec::new();
    let mut lexer = Lexer::new(input.chars().collect());
    lex(&mut lexer, &mut result)?;
    Ok(result)
}

fn table(lexer: &mut Lexer, result: &mut Vec<Token>, token: Token) -> Result<(), String> {
    match token {
        Token::LeftParen => {
            lexer.open_paren(result.len()-1);
        },
        Token::RightParen => {
            let open_index = lexer.close_paren()?;
            let next_token = tokenize(lexer, result)?;
            match next_token {
                Token::FatArrow => {
                    let close_index = result.len()-2; // one before the fat arrow 
                    result[open_index] = Token::ArrowLeftParen;
                    result[close_index] = Token::ArrowRightParen;
                },
                _ => table(lexer, result, next_token)?,
            }
        },
        _ => {},
    }

    Ok(())
}

fn lex(lexer: &mut Lexer, result: &mut Vec<Token>) -> Result<(), String> {
    loop {
        let token = tokenize(lexer, result)?;
        if token == Token::EOF {
            return Ok(())
        }
        table(lexer, result, token)?;
    }
}

// Function lex consumes the input string, returns a single Token, and modifies the lexer state if needed
fn tokenize(lexer: &mut Lexer, result: &mut Vec<Token>) -> Result<Token, String> {
    consume_whitespace(lexer);
    let res = match lexer.lookahead_1() {
        Some('a'..='z') => keyword_or_ident(lexer)?,
        Some('A'..='Z'|'_') => ident(lexer)?,
        Some('0'..='9') => parse_number_or_bigint(lexer, false)?,
        Some('"'|'\'') => parse_string(lexer)?,
        // Punctuation
        Some('(') => lexer.proceed_with(Token::LeftParen),
        Some(')') => lexer.proceed_with(Token::RightParen),
        Some('{') => lexer.proceed_with(Token::LeftBrace),
        Some('}') => lexer.proceed_with(Token::RightBrace),
        Some('[') => lexer.proceed_with(Token::LeftBracket),
        Some(']') => lexer.proceed_with(Token::RightBracket),
        Some(',') => lexer.proceed_with(Token::Comma),
        Some('.') => {
            if lexer.lookahead_2() == Some('.') && lexer.lookahead_3() == Some('.') {
                lexer.proceed();
                lexer.proceed();
                lexer.proceed();
                Token::DotDotDot
            } else {
                lexer.proceed_with(Token::Dot)
            }
        },
        Some(':') => lexer.proceed_with(Token::Colon),
        Some(';') => lexer.proceed_with(Token::Semicolon),
        Some('?') => lexer.proceed_with(Token::Question),
        Some('`') => lexer.proceed_with(Token::QuasiQuote),
        Some('$') => lexer.proceed_with(Token::Dollar),
        Some('/') => {
            if lexer.lookahead_2() == Some('/') {
                unreachable!("line comment should have been handled by consume_whitespace")
            } else if lexer.lookahead_2() == Some('*') {
                unreachable!("block comment should have been handled by consume_whitespace")
            } else {
                if lexer.lookahead_2() == Some('=') {
                    lexer.proceed();
                    lexer.proceed();
                    Token::SlashEqual
                } else {
                    lexer.proceed_with(Token::Slash)
                }
            }
        },
        // Operators
        Some('+') => {
            if lexer.lookahead_2() == Some('+') {
                return Err("Increment operator not supported yet".to_string());
            } else if lexer.lookahead_2() == Some('=') {
                lexer.proceed();
                lexer.proceed();
                Token::PlusEqual
            } else {
                lexer.proceed_with(Token::Plus)
            }
        },            
        Some('-') => {
            match lexer.lookahead_2() {
                Some('-') => {
                    return Err("Decrement operator not supported yet".to_string());
                },
                Some('=') => {
                    lexer.proceed();
                    lexer.proceed();
                    Token::MinusEqual
                },
                Some('0'..='9') => parse_number_or_bigint(lexer, true)?,
                _ => lexer.proceed_with(Token::Minus)
            }
        },
        Some('*') => {
            if lexer.lookahead_2() == Some('=') {
                lexer.proceed();
                lexer.proceed();
                Token::AsteriskEqual
            } else if lexer.lookahead_2() == Some('*') {
                lexer.proceed();
                if lexer.lookahead_3() == Some('=') {
                    return Err("Exponentiation assignment operator not supported yet".to_string())
                    // lexer.proceed();
                    // result.push(lexer.proceed_with(Token::AsteriskAsteriskEqual);
                } else {
                    lexer.proceed_with(Token::AsteriskAsterisk)
                }
            } else if lexer.lookahead_2() == Some('/') {
                lexer.proceed();
                lexer.proceed();
                Token::AsteriskSlash
            } else {
                lexer.proceed_with(Token::Asterisk)
            }
        },
        Some('%') => {
            if lexer.lookahead_2() == Some('=') {
                lexer.proceed();
                lexer.proceed();
                Token::PercentEqual
            } else {
                lexer.proceed_with(Token::Percent)
            }
        },
        Some('&') => {
            if lexer.lookahead_2() == Some('&') {
                lexer.proceed();
                lexer.proceed();
                Token::AmpAmp
            } else if lexer.lookahead_2() == Some('=') {
                lexer.proceed();
                lexer.proceed();
                Token::AmpersandEqual
            } else {
                lexer.proceed_with(Token::Ampersand)
            }
        },
        Some('|') => {
            if lexer.lookahead_2() == Some('|') {
                lexer.proceed();
                lexer.proceed();
                Token::BarBar
            } else if lexer.lookahead_2() == Some('=') {
                lexer.proceed();
                lexer.proceed();
                Token::BarEqual
            } else {
                lexer.proceed_with(Token::Bar)
            }
        },
        Some('^') => {
            if lexer.lookahead_2() == Some('=') {
                lexer.proceed();
                lexer.proceed();
                Token::CaretEqual
            } else {
                lexer.proceed_with(Token::Caret)
            }
        },
        Some('~') => lexer.proceed_with(Token::Tilde),
        Some('!') => {
            if lexer.lookahead_2() == Some('=') {
                if lexer.lookahead_3() == Some('=') {
                    lexer.proceed();
                    lexer.proceed();
                    lexer.proceed();
                    Token::BangEqualEqual
                } else {
                    return Err("!= operator not supported".to_string());
                }
            } else {
                lexer.proceed_with(Token::Bang)
            }
        },
        Some('=') => {
            if lexer.lookahead_2() == Some('=') {
                if lexer.lookahead_3() == Some('=') {
                    lexer.proceed();
                    lexer.proceed();
                    lexer.proceed();
                    Token::EqualEqualEqual
                } else {
                    return Err("== operator not supported".to_string());
                }
            } else {
                lexer.proceed_with(Token::Equal)
            }
        },
        Some('<') => { // TODO: type annotations
            if lexer.lookahead_2() == Some('<') {
                if lexer.lookahead_3() == Some('=') {
                    lexer.proceed();
                    lexer.proceed();
                    lexer.proceed();
                    Token::LAngleLAngleEqual
                } else {
                    lexer.proceed();
                    lexer.proceed();
                    Token::LAngleLAngle
                }
            } else if lexer.lookahead_2() == Some('=') {
                lexer.proceed();
                lexer.proceed();
                Token::LAngleEqual
            } else {
                lexer.proceed_with(Token::LAngle)
            }
        },
        Some('>') => {
            if lexer.lookahead_2() == Some('>') {
                if lexer.lookahead_3() == Some('=') {
                    lexer.proceed();
                    lexer.proceed();
                    lexer.proceed();
                    Token::RAngleRAngleEqual
                } else if lexer.lookahead_3() == Some('>') {
                    lexer.proceed();
                    lexer.proceed();
                    lexer.proceed();
                    Token::RAngleRAngleRAngle
                } else {
                    lexer.proceed();
                    lexer.proceed();
                    Token::RAngleRAngle
                }
            } else if lexer.lookahead_2() == Some('=') {
                lexer.proceed();
                lexer.proceed();
                Token::RAngleEqual
            } else {
                lexer.proceed_with(Token::RAngle)
            } 
        } 
        None => Token::EOF,
        Some(c) => return Err(format!("Unexpected character {}", c)),
    };

    result.push(res.clone());

    Ok(res)
}

pub fn check_whitespace_nonident(c1: Option<char>, c2: Option<char>) -> Result<(), String> {
    println!("c1: {:?}, c2: {:?}", c1, c2);
    if let Some(c) = c1 {
        match c {
            ' ' | '\t' | '\r' | '\n' => {
                Ok(())
            }
            '/' => {
                match c2 {
                    Some('/') | Some('*') => Ok(()),
                    _ => Err(format!("Expected whitespace, but got {}", c)),
                }
            }
            'A'..='Z' | 'a'..='z' | '_' => Err(format!("Expected whitespace, but got {}", c)),
            _ => Ok(()),
        }
    } else {
        Ok(())
        
    }
}

pub fn lookahead_whitespace_nonident(state: &mut Lexer) -> Result<(), String> {
    check_whitespace_nonident(state.lookahead_1(), state.lookahead_2())
}

pub fn consume_whitespace(state: &mut Lexer) {
    while let Some(c) = state.lookahead_1() {
        match c {
            ' ' | '\t' | '\r' | '\n' => { state.proceed(); },
            '/' => {
                match state.lookahead_2() {
                    Some('/') => {
                        state.proceed();
                        state.proceed();
                        while let Some(c) = state.lookahead_1() {
                            if c == '\r' || c == '\n' {
                                break;
                            } else {
                                state.proceed();
                            }
                        }
                    }
                    Some('*') => {
                        state.proceed();
                        state.proceed();
                        while let Some(c) = state.lookahead_1() {
                            if c == '*' {
                                state.proceed();
                                if let Some(c) = state.lookahead_1() {
                                    if c == '/' {
                                        state.proceed();
                                        break;
                                    }
                                }
                            } else {
                                state.proceed();
                            }
                        }
                    }
                    _ => break,
                }
            },
            _ => break,
        }
    }
}

fn ident(state: &mut Lexer) -> Result<Token, String> {
    // [a-zA-Z_][a-zA-Z0-9_]*
    let mut ident = String::new();
    match state.lookahead_1() {
        Some('a'..='z') | Some('A'..='Z') | Some('_') => {
            while let Some('a'..='z') | Some('A'..='Z') | Some('_') | Some('0'..='9') = state.lookahead_1() {
                ident.push(state.lookahead_1().unwrap());
                state.proceed();
            }
        }
        _ => return Err(format!("Expected identifier, but got {:?}", state.lookahead_1())),
    }

    Ok(Token::Identifier(SharedString::from_string(ident)))
}






/*
pub fn parse_number(state: &mut Lexer) -> Result<DataLiteral, String> {
    // [1-9][0-9]*(\.[0-9]*|n)?
    let mut number = String::new();
    if state.lookahead_1().map(|x| x.is_ascii_digit()) != Some(true) {
        return Err("not a number".to_string())
    }
    while let Some(c) = state.lookahead_1() {
        if c.is_ascii_digit() {
            number.push(c);
            state.proceed();
        } else {
            break;
        }
    }
    if state.lookahead_1() == Some('.') {
        state.proceed();
        number.push('.');
        while let Some(c) = state.lookahead_1() {
            if c.is_ascii_digit() {
                number.push(c);
                state.proceed();
            } else {
                break;
            }
        } 
    } 
 
    Ok(DataLiteral::Number(number)) 
}
 */
pub fn parse_number_or_bigint(state: &mut Lexer, sign: bool) -> Result<Token, String> {
    // integer: -?[1-9][0-9]* within i64 range
    // decimal: -?[1-9][0-9]*\.[0-9]* within i128 range
    // bigint: -?[1-9][0-9]*n without limit

    // the sign is pre-parsed by the caller and passed via sign parameter

    // the immediate following symbol is guaranteed to be a digit 1..=9 by the caller

    // integer part, [1-9][0-9]*
    let mut slice = String::new();
    while let Some(c) = state.lookahead_1() {
        if let Some(digit) = c.to_digit(10) {
            slice.push(c);
            state.proceed();
        } else {
            break;
        }
    }

    match state.lookahead_1() {
        Some('n') => {
            // bigint
            state.proceed();
            if slice.len() >= 20 {
                unimplemented!("bigint too large")
            }
            Ok(Token::Bigint(sign, Box::new([slice.parse::<u64>().unwrap()])))
        },
        Some('.') => {
            // decimal
            state.proceed();
            let mut decimal = String::new();
            while let Some(c) = state.lookahead_1() {
                if let Some(digit) = c.to_digit(10) {
                    decimal.push(c);
                    state.proceed();
                } else {
                    break;
                }
            }
            for _ in 0..(20 - decimal.len()) {
                decimal.push('0');
            }
            Ok(Token::Decimal(slice.parse::<i64>().unwrap() * if sign { -1 } else { 1 }, decimal.parse::<u64>().unwrap()))
        },
        _ => {
            // integer
            Ok(Token::Integer(slice.parse::<i64>().unwrap() * if sign { -1 } else { 1 }))
        }
    }
}

pub fn parse_string(state: &mut Lexer) -> Result<Token, String> {
    let mut string = String::new();
    let enclosing = state.lookahead_1().filter(|c| *c == '"' || *c == '\'').ok_or("Expected string".to_string())?;
    state.proceed();
    while let Some(c) = state.lookahead_1() {
        if c == enclosing {
            state.proceed();
            break;
        } else {
            string.push(c); // TODO: optimize, i think we can just slice the string
            state.proceed();
        }
    }
    Ok(Token::String(string.into()))
}

fn parse_ident(state: &mut Lexer) -> Result<Token, String> {
    // [a-zA-Z_][a-zA-Z0-9_]*
    let mut ident = String::new();

    match state.lookahead_1() {
        Some(x) if x.is_ascii_alphabetic() || x == '_' => {
            ident.push(x);
            state.proceed();
        }
        _ => return Err("Expected identifier".to_string()),
    }

    while let Some(x) = state.lookahead_1() {
        if x.is_ascii_alphanumeric() || x == '_' {
            ident.push(x);
            state.proceed();
        } else {
            break;
        }
    }
    Ok(Token::Identifier(SharedString::from_string(ident)))
}

