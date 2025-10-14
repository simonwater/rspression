use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoxError {
    #[error("Parse error at line {line}: {message}")]
    ParseError { line: usize, message: String },

    #[error("Analyze error: {message}")]
    AnalyzeError { message: String },

    #[error("Runtime error: {message}")]
    RuntimeError { message: String },

    #[error("Compile error: {message}")]
    CompileError { message: String },

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type LoxResult<T> = Result<T, LoxError>;
