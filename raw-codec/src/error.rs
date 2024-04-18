use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display};
use std::io::ErrorKind;

#[repr(u16)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CodecErrorCode {
    Failed,
    OutOfLimit,
    InvalidFormat,
    NotSupport,
    CryptoError,
    NotFound,
    PermissionDenied,
    ConnectionRefused,
    ConnectionReset,
    ConnectionAborted,
    NotConnected,
    AddrInUse,
    AddrNotAvailable,
    BrokenPipe,
    AlreadyExists,
    WouldBlock,
    InvalidInput,
    InvalidData,
    Timeout,
    WriteZero,
    Interrupted,
    UnexpectedEof,
    UnknownIOError,
}

#[derive(Clone)]
pub struct CodecError {
    code: CodecErrorCode,
    msg: String,
}

impl CodecError {
    pub fn new(code: impl Into<CodecErrorCode>, msg: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            msg: msg.into(),
        }
    }

    pub fn set_code(&mut self, code: impl Into<CodecErrorCode>) {
        self.code = code.into();
    }

    pub fn code(&self) -> CodecErrorCode {
        self.code
    }

    pub fn with_code(mut self, code: impl Into<CodecErrorCode>) -> Self {
        self.code = code.into();
        self
    }

    pub fn set_msg(&mut self, msg: impl Into<String>) {
        self.msg = msg.into();
    }

    pub fn msg(&self) -> &str {
        self.msg.as_ref()
    }

    pub fn with_msg(mut self, msg: impl Into<String>) -> Self {
        self.msg = msg.into();
        self
    }

    fn format(&self) -> String {
        format!("err: ({:?}, {})", self.code, self.msg)
    }

    pub fn error_with_log<T>(msg: impl Into<String> + std::fmt::Display) -> CodecResult<T> {
        error!("{}", msg);

        Err(CodecError::new(CodecErrorCode::Failed, msg))
    }
    fn io_error_kind_to_code(kind: std::io::ErrorKind) -> CodecErrorCode {
        match kind {
            ErrorKind::NotFound => CodecErrorCode::NotFound,
            ErrorKind::PermissionDenied => CodecErrorCode::PermissionDenied,
            ErrorKind::ConnectionRefused => CodecErrorCode::ConnectionRefused,
            ErrorKind::ConnectionReset => CodecErrorCode::ConnectionReset,
            ErrorKind::ConnectionAborted => CodecErrorCode::ConnectionAborted,
            ErrorKind::NotConnected => CodecErrorCode::NotConnected,
            ErrorKind::AddrInUse => CodecErrorCode::AddrInUse,
            ErrorKind::AddrNotAvailable => CodecErrorCode::AddrNotAvailable,
            ErrorKind::BrokenPipe => CodecErrorCode::BrokenPipe,
            ErrorKind::AlreadyExists => CodecErrorCode::AlreadyExists,
            ErrorKind::WouldBlock => CodecErrorCode::WouldBlock,
            ErrorKind::InvalidInput => CodecErrorCode::InvalidInput,
            ErrorKind::InvalidData => CodecErrorCode::InvalidData,
            ErrorKind::TimedOut => CodecErrorCode::Timeout,
            ErrorKind::WriteZero => CodecErrorCode::WriteZero,
            ErrorKind::Interrupted => CodecErrorCode::Interrupted,
            ErrorKind::UnexpectedEof => CodecErrorCode::UnexpectedEof,
            _ => CodecErrorCode::UnknownIOError,
        }
    }

    fn io_error_to_bucky_error(e: std::io::Error) -> CodecError {
        let kind = e.kind();
        if kind == std::io::ErrorKind::Other && e.get_ref().is_some() {
            match e.into_inner().unwrap().downcast::<CodecError>() {
                Ok(e) => {
                    e.as_ref().clone()
                }
                Err(e) => {
                    CodecError {
                        code: Self::io_error_kind_to_code(kind),
                        msg: format!("io_error: {}", e),
                    }
                }
            }
        } else {
            CodecError {
                code: Self::io_error_kind_to_code(e.kind()),
                msg: format!("io_error: {}", e),
            }
        }
    }
}

impl From<std::io::Error> for CodecError {
    fn from(err: std::io::Error) -> CodecError {
        CodecError::io_error_to_bucky_error(err)
    }
}

impl Error for CodecError {}

impl Display for CodecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.format(), f)
    }
}

impl Debug for CodecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.format(), f)
    }
}

pub type CodecResult<T> = Result<T, CodecError>;
