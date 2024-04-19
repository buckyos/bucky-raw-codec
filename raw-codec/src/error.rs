use std::error::Error;
use std::{alloc, fmt};
use std::fmt::{Debug, Display};
use std::io::ErrorKind;
use bucky_error::{BuckyError, BuckyErrorCode, BuckyOriginError, BuckyResult};
use crate::{RawDecode, RawEncode, RawEncodePurpose, RawFixedBytes, USize};

pub type CodecErrorCode = BuckyErrorCode;
pub type CodecError = BuckyError;
pub type CodecResult<T> = BuckyResult<T>;

impl RawEncode for BuckyErrorCode {
    fn raw_measure(&self, __purpose__: &Option<RawEncodePurpose>) -> CodecResult<usize> {
        match self {
            BuckyErrorCode::Ok => Ok(USize(0usize).raw_measure(__purpose__)?),
            BuckyErrorCode::Failed => Ok(USize(1usize).raw_measure(__purpose__)?),
            BuckyErrorCode::InvalidParam => {
                Ok(USize(2usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::Timeout => Ok(USize(3usize).raw_measure(__purpose__)?),
            BuckyErrorCode::NotFound => {
                Ok(USize(4usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::AlreadyExists => {
                Ok(USize(5usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::NotSupport => {
                Ok(USize(6usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::ErrorState => {
                Ok(USize(7usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::InvalidFormat => {
                Ok(USize(8usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::Expired => Ok(USize(9usize).raw_measure(__purpose__)?),
            BuckyErrorCode::OutOfLimit => {
                Ok(USize(10usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::InternalError => {
                Ok(USize(11usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::PermissionDenied => {
                Ok(USize(12usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::ConnectionRefused => {
                Ok(USize(13usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::ConnectionReset => {
                Ok(USize(14usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::ConnectionAborted => {
                Ok(USize(15usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::NotConnected => {
                Ok(USize(16usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::AddrInUse => {
                Ok(USize(17usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::AddrNotAvailable => {
                Ok(USize(18usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::Interrupted => {
                Ok(USize(19usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::InvalidInput => {
                Ok(USize(20usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::InvalidData => {
                Ok(USize(21usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::WriteZero => {
                Ok(USize(22usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::UnexpectedEof => {
                Ok(USize(23usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::BrokenPipe => {
                Ok(USize(24usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::WouldBlock => {
                Ok(USize(25usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::UnSupport => {
                Ok(USize(26usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::Unmatch => {
                Ok(USize(27usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::ExecuteError => {
                Ok(USize(28usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::Reject => Ok(USize(29usize).raw_measure(__purpose__)?),
            BuckyErrorCode::Ignored => {
                Ok(USize(30usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::InvalidSignature => {
                Ok(USize(31usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::AlreadyExistsAndSignatureMerged => {
                Ok(USize(32usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::TargetNotFound => {
                Ok(USize(33usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::Aborted => {
                Ok(USize(34usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::ConnectFailed => {
                Ok(USize(35usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::ConnectInterZoneFailed => {
                Ok(USize(36usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::InnerPathNotFound => {
                Ok(USize(37usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::RangeNotSatisfiable => {
                Ok(USize(38usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::UserCanceled => {
                Ok(USize(39usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::Conflict => {
                Ok(USize(40usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::OutofSessionLimit => {
                Ok(USize(41usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::Redirect => {
                Ok(USize(42usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::MongoDBError => {
                Ok(USize(43usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::SqliteError => {
                Ok(USize(44usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::UrlError => {
                Ok(USize(45usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::ZipError => {
                Ok(USize(46usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::HttpError => {
                Ok(USize(47usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::JsonError => {
                Ok(USize(48usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::HexError => {
                Ok(USize(49usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::RsaError => {
                Ok(USize(50usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::CryptoError => {
                Ok(USize(51usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::MpscSendError => {
                Ok(USize(52usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::MpscRecvError => {
                Ok(USize(53usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::IoError => {
                Ok(USize(54usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::NetworkError => {
                Ok(USize(55usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::CodeError => {
                Ok(USize(56usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::UnknownBdtError => {
                Ok(USize(57usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::UnknownIOError => {
                Ok(USize(58usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::Unknown => {
                Ok(USize(59usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::Pending => {
                Ok(USize(60usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::NotChange => {
                Ok(USize(61usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::NotMatch => {
                Ok(USize(62usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::NotImplement => {
                Ok(USize(63usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::NotInit => {
                Ok(USize(64usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::ParseError => {
                Ok(USize(65usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::NotHandled => {
                Ok(USize(66usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::InvalidTarget => {
                Ok(USize(67usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::ErrorTimestamp => {
                Ok(USize(68usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::DecNotRunning => {
                Ok(USize(69usize).raw_measure(__purpose__)?)
            }
            BuckyErrorCode::MetaError(ref __field0) => Ok(USize(70usize)
                .raw_measure(__purpose__)?
                + 0
                + __field0.raw_measure(__purpose__)?),
            BuckyErrorCode::DecError(ref __field0) => Ok(USize(71usize)
                .raw_measure(__purpose__)?
                + 0
                + __field0.raw_measure(__purpose__)?),
        }
    }
    fn raw_encode<'__de__>(
        &self,
        __buf__: &'__de__ mut [u8],
        __purpose__: &Option<RawEncodePurpose>,
    ) -> CodecResult<&'__de__ mut [u8]> {
        match self {
            BuckyErrorCode::Ok => {
                let __buf__ = USize(0usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::Failed => {
                let __buf__ = USize(1usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::InvalidParam => {
                let __buf__ = USize(2usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::Timeout => {
                let __buf__ = USize(3usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::NotFound => {
                let __buf__ = USize(4usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::AlreadyExists => {
                let __buf__ = USize(5usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::NotSupport => {
                let __buf__ = USize(6usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::ErrorState => {
                let __buf__ = USize(7usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::InvalidFormat => {
                let __buf__ = USize(8usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::Expired => {
                let __buf__ = USize(9usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::OutOfLimit => {
                let __buf__ = USize(10usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::InternalError => {
                let __buf__ = USize(11usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::PermissionDenied => {
                let __buf__ = USize(12usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::ConnectionRefused => {
                let __buf__ = USize(13usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::ConnectionReset => {
                let __buf__ = USize(14usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::ConnectionAborted => {
                let __buf__ = USize(15usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::NotConnected => {
                let __buf__ = USize(16usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::AddrInUse => {
                let __buf__ = USize(17usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::AddrNotAvailable => {
                let __buf__ = USize(18usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::Interrupted => {
                let __buf__ = USize(19usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::InvalidInput => {
                let __buf__ = USize(20usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::InvalidData => {
                let __buf__ = USize(21usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::WriteZero => {
                let __buf__ = USize(22usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::UnexpectedEof => {
                let __buf__ = USize(23usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::BrokenPipe => {
                let __buf__ = USize(24usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::WouldBlock => {
                let __buf__ = USize(25usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::UnSupport => {
                let __buf__ = USize(26usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::Unmatch => {
                let __buf__ = USize(27usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::ExecuteError => {
                let __buf__ = USize(28usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::Reject => {
                let __buf__ = USize(29usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::Ignored => {
                let __buf__ = USize(30usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::InvalidSignature => {
                let __buf__ = USize(31usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::AlreadyExistsAndSignatureMerged => {
                let __buf__ = USize(32usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::TargetNotFound => {
                let __buf__ = USize(33usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::Aborted => {
                let __buf__ = USize(34usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::ConnectFailed => {
                let __buf__ = USize(35usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::ConnectInterZoneFailed => {
                let __buf__ = USize(36usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::InnerPathNotFound => {
                let __buf__ = USize(37usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::RangeNotSatisfiable => {
                let __buf__ = USize(38usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::UserCanceled => {
                let __buf__ = USize(39usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::Conflict => {
                let __buf__ = USize(40usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::OutofSessionLimit => {
                let __buf__ = USize(41usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::Redirect => {
                let __buf__ = USize(42usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::MongoDBError => {
                let __buf__ = USize(43usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::SqliteError => {
                let __buf__ = USize(44usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::UrlError => {
                let __buf__ = USize(45usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::ZipError => {
                let __buf__ = USize(46usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::HttpError => {
                let __buf__ = USize(47usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::JsonError => {
                let __buf__ = USize(48usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::HexError => {
                let __buf__ = USize(49usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::RsaError => {
                let __buf__ = USize(50usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::CryptoError => {
                let __buf__ = USize(51usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::MpscSendError => {
                let __buf__ = USize(52usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::MpscRecvError => {
                let __buf__ = USize(53usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::IoError => {
                let __buf__ = USize(54usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::NetworkError => {
                let __buf__ = USize(55usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::CodeError => {
                let __buf__ = USize(56usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::UnknownBdtError => {
                let __buf__ = USize(57usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::UnknownIOError => {
                let __buf__ = USize(58usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::Unknown => {
                let __buf__ = USize(59usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::Pending => {
                let __buf__ = USize(60usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::NotChange => {
                let __buf__ = USize(61usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::NotMatch => {
                let __buf__ = USize(62usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::NotImplement => {
                let __buf__ = USize(63usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::NotInit => {
                let __buf__ = USize(64usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::ParseError => {
                let __buf__ = USize(65usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::NotHandled => {
                let __buf__ = USize(66usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::InvalidTarget => {
                let __buf__ = USize(67usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::ErrorTimestamp => {
                let __buf__ = USize(68usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::DecNotRunning => {
                let __buf__ = USize(69usize).raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::MetaError(ref __field0) => {
                let __buf__ = USize(70usize).raw_encode(__buf__, __purpose__)?;
                let __buf__ = __field0.raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
            BuckyErrorCode::DecError(ref __field0) => {
                let __buf__ = USize(71usize).raw_encode(__buf__, __purpose__)?;
                let __buf__ = __field0.raw_encode(__buf__, __purpose__)?;
                Ok(__buf__)
            }
        }
    }
}
#[automatically_derived]
#[allow(non_snake_case)]
fn BuckyErrorCode_call<'a, T: RawDecode<'a>>(__buf__: &'a [u8]) -> CodecResult<(T, &'a [u8])> {
    T::raw_decode(__buf__)
}
#[automatically_derived]
#[allow(non_snake_case)]
impl<'__de__> RawDecode<'__de__> for BuckyErrorCode {
    fn raw_decode(__buf__: &'__de__ [u8]) -> CodecResult<(Self, &'__de__ [u8])> {
        {
            let (element_type, __buf__) = USize::raw_decode(__buf__)?;
            match element_type.value() {
                0usize => Ok((BuckyErrorCode::Ok, __buf__)),
                1usize => Ok((BuckyErrorCode::Failed, __buf__)),
                2usize => Ok((BuckyErrorCode::InvalidParam, __buf__)),
                3usize => Ok((BuckyErrorCode::Timeout, __buf__)),
                4usize => Ok((BuckyErrorCode::NotFound, __buf__)),
                5usize => Ok((BuckyErrorCode::AlreadyExists, __buf__)),
                6usize => Ok((BuckyErrorCode::NotSupport, __buf__)),
                7usize => Ok((BuckyErrorCode::ErrorState, __buf__)),
                8usize => Ok((BuckyErrorCode::InvalidFormat, __buf__)),
                9usize => Ok((BuckyErrorCode::Expired, __buf__)),
                10usize => Ok((BuckyErrorCode::OutOfLimit, __buf__)),
                11usize => Ok((BuckyErrorCode::InternalError, __buf__)),
                12usize => Ok((BuckyErrorCode::PermissionDenied, __buf__)),
                13usize => Ok((BuckyErrorCode::ConnectionRefused, __buf__)),
                14usize => Ok((BuckyErrorCode::ConnectionReset, __buf__)),
                15usize => Ok((BuckyErrorCode::ConnectionAborted, __buf__)),
                16usize => Ok((BuckyErrorCode::NotConnected, __buf__)),
                17usize => Ok((BuckyErrorCode::AddrInUse, __buf__)),
                18usize => Ok((BuckyErrorCode::AddrNotAvailable, __buf__)),
                19usize => Ok((BuckyErrorCode::Interrupted, __buf__)),
                20usize => Ok((BuckyErrorCode::InvalidInput, __buf__)),
                21usize => Ok((BuckyErrorCode::InvalidData, __buf__)),
                22usize => Ok((BuckyErrorCode::WriteZero, __buf__)),
                23usize => Ok((BuckyErrorCode::UnexpectedEof, __buf__)),
                24usize => Ok((BuckyErrorCode::BrokenPipe, __buf__)),
                25usize => Ok((BuckyErrorCode::WouldBlock, __buf__)),
                26usize => Ok((BuckyErrorCode::UnSupport, __buf__)),
                27usize => Ok((BuckyErrorCode::Unmatch, __buf__)),
                28usize => Ok((BuckyErrorCode::ExecuteError, __buf__)),
                29usize => Ok((BuckyErrorCode::Reject, __buf__)),
                30usize => Ok((BuckyErrorCode::Ignored, __buf__)),
                31usize => Ok((BuckyErrorCode::InvalidSignature, __buf__)),
                32usize => Ok((BuckyErrorCode::AlreadyExistsAndSignatureMerged, __buf__)),
                33usize => Ok((BuckyErrorCode::TargetNotFound, __buf__)),
                34usize => Ok((BuckyErrorCode::Aborted, __buf__)),
                35usize => Ok((BuckyErrorCode::ConnectFailed, __buf__)),
                36usize => Ok((BuckyErrorCode::ConnectInterZoneFailed, __buf__)),
                37usize => Ok((BuckyErrorCode::InnerPathNotFound, __buf__)),
                38usize => Ok((BuckyErrorCode::RangeNotSatisfiable, __buf__)),
                39usize => Ok((BuckyErrorCode::UserCanceled, __buf__)),
                40usize => Ok((BuckyErrorCode::Conflict, __buf__)),
                41usize => Ok((BuckyErrorCode::OutofSessionLimit, __buf__)),
                42usize => Ok((BuckyErrorCode::Redirect, __buf__)),
                43usize => Ok((BuckyErrorCode::MongoDBError, __buf__)),
                44usize => Ok((BuckyErrorCode::SqliteError, __buf__)),
                45usize => Ok((BuckyErrorCode::UrlError, __buf__)),
                46usize => Ok((BuckyErrorCode::ZipError, __buf__)),
                47usize => Ok((BuckyErrorCode::HttpError, __buf__)),
                48usize => Ok((BuckyErrorCode::JsonError, __buf__)),
                49usize => Ok((BuckyErrorCode::HexError, __buf__)),
                50usize => Ok((BuckyErrorCode::RsaError, __buf__)),
                51usize => Ok((BuckyErrorCode::CryptoError, __buf__)),
                52usize => Ok((BuckyErrorCode::MpscSendError, __buf__)),
                53usize => Ok((BuckyErrorCode::MpscRecvError, __buf__)),
                54usize => Ok((BuckyErrorCode::IoError, __buf__)),
                55usize => Ok((BuckyErrorCode::NetworkError, __buf__)),
                56usize => Ok((BuckyErrorCode::CodeError, __buf__)),
                57usize => Ok((BuckyErrorCode::UnknownBdtError, __buf__)),
                58usize => Ok((BuckyErrorCode::UnknownIOError, __buf__)),
                59usize => Ok((BuckyErrorCode::Unknown, __buf__)),
                60usize => Ok((BuckyErrorCode::Pending, __buf__)),
                61usize => Ok((BuckyErrorCode::NotChange, __buf__)),
                62usize => Ok((BuckyErrorCode::NotMatch, __buf__)),
                63usize => Ok((BuckyErrorCode::NotImplement, __buf__)),
                64usize => Ok((BuckyErrorCode::NotInit, __buf__)),
                65usize => Ok((BuckyErrorCode::ParseError, __buf__)),
                66usize => Ok((BuckyErrorCode::NotHandled, __buf__)),
                67usize => Ok((BuckyErrorCode::InvalidTarget, __buf__)),
                68usize => Ok((BuckyErrorCode::ErrorTimestamp, __buf__)),
                69usize => Ok((BuckyErrorCode::DecNotRunning, __buf__)),
                70usize => {
                    let (__field0, __buf__): (u16, &[u8]) = BuckyErrorCode_call(__buf__)?;
                    Ok((BuckyErrorCode::MetaError(__field0), __buf__))
                }
                71usize => {
                    let (__field0, __buf__): (u16, &[u8]) = BuckyErrorCode_call(__buf__)?;
                    Ok((BuckyErrorCode::DecError(__field0), __buf__))
                }
                _ => Err(CodecError::new(CodecErrorCode::NotSupport, format!("file:{} line:{} NotSupport", file!(), line!()))),
            }
        }
    }
}
impl RawEncode for BuckyError {
    fn raw_measure(&self, __purpose__: &Option<RawEncodePurpose>) -> CodecResult<usize> {
        Ok(0 + self.code().raw_measure(__purpose__)?
            + self.msg().raw_measure(__purpose__)?
            + self.origin().raw_measure(__purpose__)?)
    }
    fn raw_encode<'__de__>(
        &self,
        __buf__: &'__de__ mut [u8],
        __purpose__: &Option<RawEncodePurpose>,
    ) -> CodecResult<&'__de__ mut [u8]> {
        let __buf__ = self.code().raw_encode(__buf__, __purpose__)?;
        let __buf__ = self.msg().raw_encode(__buf__, __purpose__)?;
        let __buf__ = self.origin().raw_encode(__buf__, __purpose__)?;
        Ok(__buf__)
    }
}
#[automatically_derived]
#[allow(non_snake_case)]
fn BuckyError_call<'a, T: RawDecode<'a>>(__buf__: &'a [u8]) -> CodecResult<(T, &'a [u8])> {
    T::raw_decode(__buf__)
}
#[automatically_derived]
#[allow(non_snake_case)]
impl<'__de__> RawDecode<'__de__> for BuckyError {
    fn raw_decode(__buf__: &'__de__ [u8]) -> CodecResult<(Self, &'__de__ [u8])> {
        {
            let (code, __buf__): (BuckyErrorCode, &[u8]) = BuckyError_call(__buf__)?;
            let (msg, __buf__): (String, &[u8]) = BuckyError_call(__buf__)?;
            let (origin, __buf__): (Option<BuckyOriginError>, &[u8]) = BuckyError_call(__buf__)?;
            Ok((BuckyError::from((code, msg, origin)), __buf__))
        }
    }
}

impl RawEncode for BuckyOriginError {
    fn raw_measure(&self, purpose: &Option<RawEncodePurpose>) -> BuckyResult<usize> {
        if cfg!(not(target_arch = "wasm32")) {
            #[cfg(feature = "zip")]
            if let BuckyOriginError::ZipError(e) = self {
                let msg = format!("{:?}", e);
                return Ok(USize(2).raw_measure(purpose)? + msg.raw_measure(purpose)?);
            }

            #[cfg(feature = "rusqlite")]
            if let BuckyOriginError::SqliteError(e) = self {
                let msg = format!("{:?}", e);
                return Ok(USize(2).raw_measure(purpose)? + msg.raw_measure(purpose)?);
            }
        }

        match self {
            BuckyOriginError::IoError(e) => {
                let msg = format!("{:?}", e);
                Ok(USize(2).raw_measure(purpose)? + msg.raw_measure(purpose)?)
            }
            BuckyOriginError::SerdeJsonError(e) => {
                let msg = format!("{:?}", e);
                Ok(USize(2).raw_measure(purpose)? + msg.raw_measure(purpose)?)
            }
            #[cfg(feature = "http-types")]
            BuckyOriginError::HttpError(e) => {
                let msg = format!("{:?}", e);
                Ok(USize(2).raw_measure(purpose)? + msg.raw_measure(purpose)?)
            }
            #[cfg(feature = "url")]
            BuckyOriginError::UrlError(e) => {
                let msg = format!("{:?}", e);
                Ok(USize(2).raw_measure(purpose)? + msg.raw_measure(purpose)?)
            }
            #[cfg(feature = "http-types")]
            BuckyOriginError::HttpStatusCodeError(e) => {
                let msg = format!("{:?}", e);
                Ok(USize(2).raw_measure(purpose)? + msg.raw_measure(purpose)?)
            }
            #[cfg(feature = "hex")]
            BuckyOriginError::HexError(e) => {
                let msg = format!("{:?}", e);
                Ok(USize(2).raw_measure(purpose)? + msg.raw_measure(purpose)?)
            }
            #[cfg(feature = "rsa")]
            BuckyOriginError::RsaError(e) => {
                let msg = format!("{:?}", e);
                Ok(USize(2).raw_measure(purpose)? + msg.raw_measure(purpose)?)
            }
            BuckyOriginError::CodeError(e) => {
                Ok(USize(1).raw_measure(purpose)? + e.raw_measure(purpose)?)
            }
            BuckyOriginError::ParseIntError(e) => {
                let msg = format!("{:?}", e);
                Ok(USize(2).raw_measure(purpose)? + msg.raw_measure(purpose)?)
            }
            BuckyOriginError::ParseFloatError(e) => {
                let msg = format!("{:?}", e);
                Ok(USize(2).raw_measure(purpose)? + msg.raw_measure(purpose)?)
            }
            BuckyOriginError::AddrParseError(e) => {
                let msg = format!("{:?}", e);
                Ok(USize(2).raw_measure(purpose)? + msg.raw_measure(purpose)?)
            }
            BuckyOriginError::StripPrefixError(e) => {
                let msg = format!("{:?}", e);
                Ok(USize(2).raw_measure(purpose)? + msg.raw_measure(purpose)?)
            }
            BuckyOriginError::ParseUtf8Error(e) => {
                let msg = format!("{:?}", e);
                Ok(USize(2).raw_measure(purpose)? + msg.raw_measure(purpose)?)
            }
            BuckyOriginError::ErrorMsg(msg) => {
                Ok(USize(2).raw_measure(purpose)? + msg.raw_measure(purpose)?)
            }
            #[cfg(feature = "sqlx")]
            BuckyOriginError::SqlxError(e) => {
                let msg = format!("{:?}", e);
                Ok(USize(2).raw_measure(purpose)? + msg.raw_measure(purpose)?)
            }
            _ => Ok(USize(3).raw_measure(purpose)?),
        }
    }

    fn raw_encode<'a>(
        &self,
        buf: &'a mut [u8],
        purpose: &Option<RawEncodePurpose>,
    ) -> BuckyResult<&'a mut [u8]> {
        if cfg!(not(target_arch = "wasm32")) {
            #[cfg(feature = "zip")]
            if let BuckyOriginError::ZipError(e) = self {
                let msg = format!("{:?}", e);
                let buf = USize(2).raw_encode(buf, purpose)?;
                let buf = msg.raw_encode(buf, purpose)?;
                return Ok(buf);
            }

            #[cfg(feature = "rusqlite")]
            if let BuckyOriginError::SqliteError(e) = self {
                let msg = format!("{:?}", e);
                let buf = USize(2).raw_encode(buf, purpose)?;
                let buf = msg.raw_encode(buf, purpose)?;
                return Ok(buf);
            }
        }

        match self {
            BuckyOriginError::IoError(e) => {
                let msg = format!("{:?}", e);
                let buf = USize(2).raw_encode(buf, purpose)?;
                let buf = msg.raw_encode(buf, purpose)?;
                Ok(buf)
            }
            BuckyOriginError::SerdeJsonError(e) => {
                let msg = format!("{:?}", e);
                let buf = USize(2).raw_encode(buf, purpose)?;
                let buf = msg.raw_encode(buf, purpose)?;
                Ok(buf)
            }
            #[cfg(feature = "rsa")]
            BuckyOriginError::HttpError(e) => {
                let msg = format!("{:?}", e);
                let buf = USize(2).raw_encode(buf, purpose)?;
                let buf = msg.raw_encode(buf, purpose)?;
                Ok(buf)
            }
            #[cfg(feature = "url")]
            BuckyOriginError::UrlError(e) => {
                let msg = format!("{:?}", e);
                let buf = USize(2).raw_encode(buf, purpose)?;
                let buf = msg.raw_encode(buf, purpose)?;
                Ok(buf)
            }
            #[cfg(feature = "http-types")]
            BuckyOriginError::HttpStatusCodeError(e) => {
                let msg = format!("{:?}", e);
                let buf = USize(2).raw_encode(buf, purpose)?;
                let buf = msg.raw_encode(buf, purpose)?;
                Ok(buf)
            }
            #[cfg(feature = "hex")]
            BuckyOriginError::HexError(e) => {
                let msg = format!("{:?}", e);
                let buf = USize(2).raw_encode(buf, purpose)?;
                let buf = msg.raw_encode(buf, purpose)?;
                Ok(buf)
            }
            #[cfg(feature = "rsa")]
            BuckyOriginError::RsaError(e) => {
                let msg = format!("{:?}", e);
                let buf = USize(2).raw_encode(buf, purpose)?;
                let buf = msg.raw_encode(buf, purpose)?;
                Ok(buf)
            }
            BuckyOriginError::CodeError(e) => {
                let buf = USize(1).raw_encode(buf, purpose)?;
                let buf = e.raw_encode(buf, purpose)?;
                Ok(buf)
            }
            BuckyOriginError::ParseIntError(e) => {
                let msg = format!("{:?}", e);
                let buf = USize(2).raw_encode(buf, purpose)?;
                let buf = msg.raw_encode(buf, purpose)?;
                Ok(buf)
            }
            BuckyOriginError::ParseFloatError(e) => {
                let msg = format!("{:?}", e);
                let buf = USize(2).raw_encode(buf, purpose)?;
                let buf = msg.raw_encode(buf, purpose)?;
                Ok(buf)
            }
            BuckyOriginError::AddrParseError(e) => {
                let msg = format!("{:?}", e);
                let buf = USize(2).raw_encode(buf, purpose)?;
                let buf = msg.raw_encode(buf, purpose)?;
                Ok(buf)
            }
            BuckyOriginError::StripPrefixError(e) => {
                let msg = format!("{:?}", e);
                let buf = USize(2).raw_encode(buf, purpose)?;
                let buf = msg.raw_encode(buf, purpose)?;
                Ok(buf)
            }
            BuckyOriginError::ParseUtf8Error(e) => {
                let msg = format!("{:?}", e);
                let buf = USize(2).raw_encode(buf, purpose)?;
                let buf = msg.raw_encode(buf, purpose)?;
                Ok(buf)
            }
            BuckyOriginError::ErrorMsg(msg) => {
                let buf = USize(2).raw_encode(buf, purpose)?;
                let buf = msg.raw_encode(buf, purpose)?;
                Ok(buf)
            }
            #[cfg(feature = "sqlx")]
            BuckyOriginError::SqlxError(e) => {
                let msg = format!("{:?}", e);
                let buf = USize(2).raw_encode(buf, purpose)?;
                let buf = msg.raw_encode(buf, purpose)?;
                Ok(buf)
            }
            _ => {
                let buf = USize(3).raw_encode(buf, purpose)?;
                Ok(buf)
            }
        }
    }
}

impl<'de> RawDecode<'de> for BuckyOriginError {
    fn raw_decode(buf: &'de [u8]) -> BuckyResult<(Self, &'de [u8])> {
        let (t, buf) = USize::raw_decode(buf)?;
        return if t.0 == 1 {
            let (code, buf) = u32::raw_decode(buf)?;
            Ok((BuckyOriginError::CodeError(code), buf))
        } else if t.0 == 2 {
            let (msg, buf) = String::raw_decode(buf)?;
            Ok((BuckyOriginError::ErrorMsg(msg), buf))
        } else {
            Ok((BuckyOriginError::ErrorMsg("".to_string()), buf))
        };
    }
}
