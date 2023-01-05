use std::fmt;

use crate::{
    interpreter::Value,
    tokenizer::{Operator, Token, TokenValue},
};

/// A Tokenizer Error
#[derive(Debug, Clone)]
pub enum TokenizerError {
    /// This error occurs when the tokenizer reaches a set of characters that
    /// does not match to a known token.
    UnknownToken(String),
    /// This error occurs when the tokenizer tokenizes a string but never
    /// reaches a terminating double quote.
    UnterminatedStringLiteral,
    /// This error occurs when the tokenizer reaches an invalid escape sequence.
    InvalidEscapeSequence(char),
}

/// A Parse Error
#[derive(Debug, Clone)]
pub enum ParseError {
    /// This error occurs when there is no `main` procedure.
    NoMain,
    /// This error occurs when there are multiple top-level procedures with the same name.
    MultipleSameNamedProcs(String),
    /// This error occurs when there is an unexpected token consumed when parsing.
    UnexpectedToken(Token),
    /// This error occurs when the consume token differs from the token that was expected.
    Expected(TokenValue, Token),
    /// This error occurs when a keyword is used a variable name.
    KeywordAsVar(String),
    /// This error occurs when a keyword is used as the name of a top-level procedure.
    KeywordAsProc(String),
    /// This error occurs when a keyword is used as the name of a procedure parameter.
    KeywordAsParam(String),
    /// This error occurs when the parser expects to parse a statement but was unsuccessful.
    ExpectedStatement,
    /// This error occurs when the parser expects to parse a block statement but was unsuccessful.
    ExpectedBlock,
    Custom(String),
}

/// A Runtime Error
#[derive(Debug, Clone)]
pub enum RuntimeError {
    /// This error occurs when the interpreter encounters an variable unbound in the environment.
    UnknownVariable(String),
    /// This error occurs when a single argument to a procedure is incorrect.
    BadArg(Value),
    /// This error occurs when multiple arguments to a procedure are incorrect.
    BadArgs(Vec<Value>),
    /// This error occurs when the number of arguments passed to a procedure is different from the
    /// number of parameters defined for that procedure.
    ArgMismatch(String, usize, usize),
    /// This error occurs when a value is expected to be a boolean but is not.
    ExpectedBool(Value),
    /// This error occurs when a binary operator is used as a unary operator.
    BinaryAsUnary(Operator),
    /// This error occurs when a unary operator is used as a binary operator.
    UnaryAsBinary(Operator),
    /// This error occurs when a `break` statement occurs outside of a loop.
    BreakNotInLoop,
    /// This error occurs when a `continue` statement occurs outside of a loop.
    ContinueNotInLoop,
    InvalidAssignmentTarget,
}

/// A LingerError. This is a wrapper enum around all of [TokenizerError], [ParseError], and
/// [RuntimeError].
#[derive(Debug, Clone)]
pub enum LingerError {
    /// A [ParseError]
    ParseError(ParseError),
    /// A [TokenizerError]
    TokenizerError(TokenizerError),
    /// A [RuntimeError]
    RuntimeError(RuntimeError),
}

impl fmt::Display for LingerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LingerError::ParseError(err) => match err {
                ParseError::NoMain => write!(f, "main procedure not found"),
                ParseError::UnexpectedToken(token) => write!(
                    f,
                    "unexpected token {} @ ({}, {})",
                    token.0, token.1, token.2
                ),
                ParseError::Expected(target, token) => write!(
                    f,
                    "expected token {} @ ({}, {}), instead got {}",
                    target, token.1, token.2, token.0
                ),
                ParseError::Custom(s) => write!(f, "{}", s),
                ParseError::KeywordAsVar(keyword) => {
                    write!(f, "keyword \"{}\" used as variable", keyword)
                }
                ParseError::KeywordAsProc(keyword) => {
                    write!(f, "keyword \"{}\" used as procedure name", keyword)
                }
                ParseError::KeywordAsParam(keyword) => {
                    write!(f, "keyword \"{}\" used as parameter name", keyword)
                }
                ParseError::ExpectedStatement => write!(f, "expected statement"),
                ParseError::ExpectedBlock => write!(f, "expected block"),
                ParseError::MultipleSameNamedProcs(proc_name) => {
                    write!(f, "multiple procedures with name \"{proc_name}\"")
                }
            },
            LingerError::TokenizerError(err) => match err {
                TokenizerError::UnknownToken(s) => write!(f, "unknown token: {s}"),
                TokenizerError::UnterminatedStringLiteral => {
                    write!(f, "unterminated string literal")
                }
                TokenizerError::InvalidEscapeSequence(char) => {
                    write!(f, "invalid escape sequence \"\\{char}\"")
                }
            },
            LingerError::RuntimeError(err) => match err {
                RuntimeError::UnknownVariable(id) => write!(f, "unknown variable \"{}\"", id),
                RuntimeError::BadArg(v) => write!(f, "bad argument \"{}\"", v),
                RuntimeError::ArgMismatch(proc_name, actual, expected) => write!(
                    f,
                    "procedure \"{}\" expected {} args, instead got {}",
                    proc_name, expected, actual
                ),
                RuntimeError::ExpectedBool(v) => {
                    write!(f, "expected boolean value, instead got {}", v)
                }
                RuntimeError::BadArgs(args) => {
                    let arg_strings_vec: Vec<String> =
                        args.iter().map(|arg| arg.to_string()).collect();
                    let arg_string = arg_strings_vec.join(", ");
                    write!(f, "bad args: [{}]", arg_string)
                }
                RuntimeError::BinaryAsUnary(op) => {
                    write!(f, "binary operator \"{}\" used as unary operator", op)
                }
                RuntimeError::UnaryAsBinary(op) => {
                    write!(f, "unary operator \"{}\" used as binary operator", op)
                }
                RuntimeError::BreakNotInLoop => write!(f, "tried to break while not within a loop"),
                RuntimeError::ContinueNotInLoop => {
                    write!(f, "continue statement found outside of a loop")
                }
                RuntimeError::InvalidAssignmentTarget => write!(f, "invalid assignment target"),
            },
        }
    }
}
