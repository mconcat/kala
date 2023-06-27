// just wrap the parser state to accumulate the tokens

use std::fmt::{Debug, Display};

use crate::parser::{ParserState, ArrayLike, ParserError};

use utils::{OwnedString, OwnedSlice, SharedString};

#[derive(Debug, PartialEq, Clone)]
pub struct Str(pub String);

impl ArrayLike for Str {
    type Element = char;

    fn get(&self, index: usize) -> Option<Self::Element> {
        self.0.bytes().nth(index).map(|b| b as char)
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn slice(&self, start: usize, end: usize) -> Self {
        self.0.as_str()[start..end].to_string().into()
    }
}

impl Display for Str {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl From<String> for Str {
    fn from(s: String) -> Self {
        Str(s)
    }
}

#[derive(Debug, Clone)]
pub struct VecToken(pub Vec<Token>);

impl ToString for VecToken {
    fn to_string(&self) -> String {
        self.0.iter().map(|x| format!("{:?}", x)).collect::<Vec<String>>().join(" ")
    }
}

impl ArrayLike for VecToken {
    type Element = Token;

    fn get(&self, index: usize) -> Option<Self::Element> {
        self.0.get(index).cloned()
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn slice(&self, start: usize, end: usize) -> Self {
        VecToken(self.0[start..end].to_vec())
    }
}

type Lexer = ParserState<Str>;

// Valid jessie tokens
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
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
    InstanceOf,
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
    // TODO: Add more assignment operators
    // Unary
    TypeOf, // typeof
    Tilde, // ~
    Bang, // !
    // Punctuation
    LeftParen, // (
    RightParen, // )
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
    String(OwnedString),
    Integer(OwnedString),
    Decimal(OwnedString),
    Bigint(OwnedString),

    // ????
    Instanceof,
    Require,
    Static,
}

#[inline]
fn given_keyword_or_ident(lexer: &mut Lexer, keyword: &'static str, token: Token) -> Result<Token, String> {
    // if the following character of the (supposed) keyword can form an identifier, namely, one of a-z, A-Z, $, _, or 0-9, then it is not a keyword.
    // proceed to identifier parsing
    if lexer.lookahead_n(keyword.len()+1).map(|x| x.is_alphabetic() || x == '_' || x.is_digit(10)).unwrap_or(false) {
        println!("fastpath ident given_keyword_or_ident");
        return ident(lexer);
    }

    if lexer.consume(Str(keyword.to_string())).is_ok() {
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
                    Some('n') => given_keyword_or_ident(lexer, &"const", Token::Const),
                    Some('n') => given_keyword_or_ident(lexer, &"continue", Token::Continue),
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

pub fn lex(lexer: &mut Lexer) -> Result<Vec<Token>, String> {
    let mut result = vec![];

    while !lexer.is_empty() {
        match lexer.lookahead_1() {
            Some('a'..='z') => result.push(keyword_or_ident(lexer)?),
            Some('A'..='Z'|'_') => result.push(ident(lexer)?),
            Some('0'..='9') => result.push(parse_number_or_bigint(lexer)?),
            Some('"'|'\'') => result.push(parse_string(lexer)?),
            // Punctuation
            Some('(') => result.push(lexer.proceed_with(Token::LeftParen)),
            Some(')') => result.push(lexer.proceed_with(Token::RightParen)),
            Some('{') => result.push(lexer.proceed_with(Token::LeftBrace)),
            Some('}') => result.push(lexer.proceed_with(Token::RightBrace)),
            Some('[') => result.push(lexer.proceed_with(Token::LeftBracket)),
            Some(']') => result.push(lexer.proceed_with(Token::RightBracket)),
            Some(',') => result.push(lexer.proceed_with(Token::Comma)),
            Some('.') => {
                if lexer.lookahead_2() == Some('.') && lexer.lookahead_3() == Some('.') {
                    lexer.proceed();
                    lexer.proceed();
                    lexer.proceed();
                    result.push(Token::DotDotDot);
                } else {
                    result.push(lexer.proceed_with(Token::Dot));
                }
            },
            Some(':') => result.push(lexer.proceed_with(Token::Colon)),
            Some(';') => result.push(lexer.proceed_with(Token::Semicolon)),
            Some('?') => result.push(lexer.proceed_with(Token::Question)),
            Some('`') => result.push(lexer.proceed_with(Token::QuasiQuote)),
            Some('$') => result.push(lexer.proceed_with(Token::Dollar)),
            Some('/') => {
                if lexer.lookahead_2() == Some('/') {
                    lexer.proceed();
                    lexer.proceed();
                    result.push(Token::DoubleSlash);
                } else if lexer.lookahead_2() == Some('*') {
                    lexer.proceed();
                    lexer.proceed();
                    result.push(lexer.proceed_with(Token::SlashAsterisk));
                    enter_block_comment(lexer, &mut result);
                } else {
                    if lexer.lookahead_2() == Some('=') {
                        lexer.proceed();
                        lexer.proceed();
                        result.push(Token::SlashEqual);
                    } else {
                        result.push(lexer.proceed_with(Token::Slash));
                    }
                }
            },
            // Operators
            Some('+') => {
                if lexer.lookahead_2() == Some('+') {
                    return Err("Increment operator not supported yet".to_string());
                } else if lexer.lookahead_2() == Some('=') {
                    lexer.proceed();
                    result.push(lexer.proceed_with(Token::PlusEqual));
                } else {
                    result.push(lexer.proceed_with(Token::Plus));
                }
            },
            Some('-') => {
                if lexer.lookahead_2() == Some('-') {
                    return Err("Decrement operator not supported yet".to_string());
                } else if lexer.lookahead_2() == Some('=') {
                    lexer.proceed();
                    result.push(lexer.proceed_with(Token::MinusEqual));
                } else {
                    result.push(lexer.proceed_with(Token::Minus));
                }
            },
            Some('*') => {
                if lexer.lookahead_2() == Some('=') {
                    lexer.proceed();
                    result.push(lexer.proceed_with(Token::AsteriskEqual));
                } else if lexer.lookahead_2() == Some('*') {
                    lexer.proceed();
                    if lexer.lookahead_3() == Some('=') {
                        return Err("Exponentiation assignment operator not supported yet".to_string())
                        // lexer.proceed();
                        // result.push(lexer.proceed_with(Token::AsteriskAsteriskEqual);
                    } else {
                        result.push(lexer.proceed_with(Token::AsteriskAsterisk));
                    }
                } else {
                    result.push(lexer.proceed_with(Token::Asterisk));
                }
            },
            Some('%') => {
                if lexer.lookahead_2() == Some('=') {
                    lexer.proceed();
                    result.push(lexer.proceed_with(Token::PercentEqual));
                } else {
                    result.push(lexer.proceed_with(Token::Percent));
                }
            },
            Some('&') => {
                if lexer.lookahead_2() == Some('&') {
                    lexer.proceed();
                    result.push(lexer.proceed_with(Token::AmpAmp));
                } else if lexer.lookahead_2() == Some('=') {
                    return Err("BitwiseAnd assignment operator not supported yet".to_string());
                    // lexer.proceed();
                    // result.push(lexer.proceed_with(Token::AmpersandEqual);
                } else {
                    result.push(lexer.proceed_with(Token::Ampersand));
                }
            },
            Some('|') => {
                if lexer.lookahead_2() == Some('|') {
                    lexer.proceed();
                    result.push(lexer.proceed_with(Token::BarBar));
                } else if lexer.lookahead_2() == Some('=') {
                    return Err("BitwiseOr assignment operator not supported yet".to_string());
                    // lexer.proceed();
                    // result.push(lexer.proceed_with(Token::AssignOp(AssignOp::AssignBitOr));
                } else {
                    result.push(lexer.proceed_with(Token::Bar));
                }
            },
            Some('^') => {
                if lexer.lookahead_2() == Some('=') {
                    return Err("BitwiseXor assignment operator not supported yet".to_string());
                    // lexer.proceed();
                    // result.push(lexer.proceed_with(Token::CaretEqual);
                } else {
                    result.push(lexer.proceed_with(Token::Caret));
                }
            },
            Some('~') => result.push(lexer.proceed_with(Token::Tilde)),
            Some('!') => {
                if lexer.lookahead_2() == Some('=') {
                    if lexer.lookahead_3() == Some('=') {
                        lexer.proceed();
                        lexer.proceed();
                        result.push(lexer.proceed_with(Token::BangEqualEqual));
                    } else {
                        return Err("!= operator not supported".to_string());
                    }
                } else {
                    result.push(lexer.proceed_with(Token::Bang));
                }
            },
            Some('=') => {
                if lexer.lookahead_2() == Some('=') {
                    if lexer.lookahead_3() == Some('=') {
                        lexer.proceed();
                        lexer.proceed();
                        result.push(lexer.proceed_with(Token::EqualEqualEqual));
                    } else {
                        return Err("== operator not supported".to_string());
                    }
                } else {
                    result.push(lexer.proceed_with(Token::Equal));
                }
            },
            Some('<') => { // TODO: type annotations
                if lexer.lookahead_2() == Some('<') {
                    if lexer.lookahead_3() == Some('=') {
                        return Err("<<= operator not supported yet".to_string());
 //                        lexer.proceed();
 //                        lexer.proceed();
//                         result.push(lexer.proceed_with(Token::AssignOp(AssignOp::AssignLShift));
                    } else {
                        lexer.proceed();
                        result.push(lexer.proceed_with(Token::LAngleLAngle));
                    }
                } else if lexer.lookahead_2() == Some('=') {
                    lexer.proceed();
                    result.push(lexer.proceed_with(Token::LAngleEqual));
                } else {
                    result.push(lexer.proceed_with(Token::LAngle));
                }
            },
            Some('>') => unimplemented!(),
            None => unreachable!("Lookahead returned None when it should have returned Some"),
            Some(c) => return Err(format!("Unexpected character {}", c)),
        }
        consume_whitespace(lexer); // TODO: exclude comment cases from whitespace
    }

    Ok(result)
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

fn enter_block_comment(state: &mut Lexer, buf: &mut Vec<Token>) -> Result<(), String> {
    unimplemented!("enter_block_comment")
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



/////////
///
/// // comma seperated list of elements, with optional trailing comma
pub fn repeated_elements<Data: Debug>(
    state: &mut ParserState<VecToken>,
    open: Option<Token>,
    close: Token,
    element: &impl Fn(&mut ParserState<VecToken>) -> Result<Data, ParserError<Option<Token>>>,
    trailing: bool
) -> Result<Vec<Data>, ParserError<Option<Token>>> {
    let mut elements = Vec::new();
    if let Some(some_open) = open.clone() {
        state.consume_1(some_open)?;
    }
    loop { // I don't like having loop here
        println!("loop {:?}", elements);
        // consume_whitespace(state);
        if state.lookahead_1() == Some(close.clone()) {
            state.proceed();
            break;
        }
        println!("element start");
        println!("{:?}{:?}", state.lookahead_1(), state.lookahead_2());
        elements.push(element(state)?);
        println!("element end");
        // consume_whitespace(state);
        if state.try_proceed(Token::Comma) {
            if state.lookahead_1() == Some(close.clone()) {
                if trailing {
                    state.proceed();
                    break;
                } else {
                    return state.err_expected("no trailing comma", Some(Token::Comma))
                }
            }
        } else if state.try_proceed(close.clone()) {
            break
        } else {
            return state.err_expected("comma or close", state.lookahead_1())
        }
    }

    Ok(elements)
}

pub fn enclosed_element<Data: Debug>(
    state: &mut ParserState<VecToken>,
    open: Token,
    close: Token,
    element: &impl Fn(&mut ParserState<VecToken>) -> Result<Data, ParserError<Option<Token>>>
) -> Result<Data, ParserError<Option<Token>>> {
    state.consume_1(open)?;
    let result = element(state)?;
    state.consume_1(close)?;
    Ok(result)
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
pub fn parse_number_or_bigint(state: &mut Lexer) -> Result<Token, String> {
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
        return Ok(Token::Decimal(OwnedString::from_string(number)))
    } else if state.lookahead_1() == Some('n') {
        state.proceed();
        return Ok(Token::Bigint(OwnedString::from_string(number)))
    }

    Ok(Token::Integer(OwnedString::from_string(number)))
}

// TODO: Detect ` string to use variable / Song127
pub fn parse_string(state: &mut Lexer) -> Result<Token, String> {
    let mut string = String::new();
    let enclosing = state.lookahead_1().filter(|c| *c == '"' || *c == '\'' || *c == '`').ok_or("Expected string".to_string())?;
    state.proceed();
    while let Some(c) = state.lookahead_1() {
        if c == enclosing {
            state.proceed();
            break;
        } else {
            if(c == '$') {
                let is_interpolation = state.lookahead_2() == Some('{');
                if is_interpolation {
                    string.push(lex(state)?);

                    state.proceed();
                    state.proceed();
                    break;
                }
            }
            before = c;
            string.push(c); // TODO: optimize, i think we can just slice the string
            state.proceed();
        }
    }
    Ok(Token::String(OwnedString::from_string(string)))
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