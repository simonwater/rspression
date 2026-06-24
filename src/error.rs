use thiserror::Error;

#[derive(Error, Debug)]
pub enum RspError {
    #[error("Parse error at line {line}, position {position}: {message}")]
    ParseError {
        line: usize,
        position: usize,
        message: String,
    },

    #[error("Analyze error: {message}")]
    AnalyzeError { message: String },

    #[error("Runtime error: {message}")]
    RuntimeError { message: String },

    #[error("Compile error: {message}")]
    CompileError { message: String },

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Type error: {message}")]
    TypeError { message: String },
}

pub type RspResult<T> = Result<T, RspError>;
