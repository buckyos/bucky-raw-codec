use crate::*;

use std::any::Any;
use crate::error::CodecResult;

//能静态确定编码后大小
pub trait RawFixedBytes {
    fn raw_bytes() -> Option<usize> {
        None
    }
    fn raw_max_bytes() -> Option<usize> {
        Self::raw_bytes()
    }
    fn raw_min_bytes() -> Option<usize> {
        Self::raw_bytes()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RawEncodePurpose {
    // 默认值，为序列化而编码，需要是完整编码
    Serialize,

    // 为计算hash而编码
    Hash,
}

#[derive(Debug, Clone)]
pub struct RawDecodeOption {
    pub version: u8,
    pub format: u8,
}

impl Default for RawDecodeOption {
    fn default() -> Self {
        Self {
            version: 0,
            format: OBJECT_CONTENT_CODEC_FORMAT_RAW,
        }
    }
}

//编码
pub trait RawEncode {
    fn raw_measure(&self, purpose: &Option<RawEncodePurpose>) -> CodecResult<usize>;
    fn raw_encode<'a>(
        &self,
        buf: &'a mut [u8],
        purpose: &Option<RawEncodePurpose>,
    ) -> CodecResult<&'a mut [u8]>;
    fn raw_tail_encode<'a>(
        &self,
        buf: &'a mut [u8],
        purpose: &Option<RawEncodePurpose>,
    ) -> CodecResult<&'a [u8]> {
        let remain_buf = self.raw_encode(buf, purpose)?;
        let remain_len = remain_buf.len();
        Ok(&buf[..(buf.len() - remain_len)])
    }

    // 直接编码到buffer
    fn raw_encode_to_buffer(&self) -> CodecResult<Vec<u8>> {
        let size = self.raw_measure(&None)?;
        let mut encode_buf = vec![0u8; size];

        let buf = self.raw_encode(&mut encode_buf, &None)?;
        assert_eq!(buf.len(), 0);

        Ok(encode_buf)
    }

    // 默认hash编码实现，子类可以覆盖
    fn raw_hash_encode(&self) -> CodecResult<Vec<u8>> {
        let size = self.raw_measure(&Some(RawEncodePurpose::Hash))?;
        let mut buf = vec![0u8; size];
        let remain_buf = self.raw_encode(&mut buf, &Some(RawEncodePurpose::Hash))?;
        assert!(remain_buf.len() == 0);

        Ok(buf)
    }
}

pub trait RawEncodeWithContext<Context> {
    fn raw_measure_with_context(
        &self,
        _: &mut Context,
        purpose: &Option<RawEncodePurpose>,
    ) -> CodecResult<usize>;
    fn raw_encode_with_context<'a>(
        &self,
        buf: &'a mut [u8],
        _: &mut Context,
        purpose: &Option<RawEncodePurpose>,
    ) -> CodecResult<&'a mut [u8]>;
    fn raw_tail_encode_with_context<'a>(
        &self,
        buf: &'a mut [u8],
        context: &mut Context,
        purpose: &Option<RawEncodePurpose>,
    ) -> CodecResult<&'a [u8]> {
        let remain_buf = self.raw_encode_with_context(buf, context, purpose)?;
        let remain_len = remain_buf.len();
        Ok(&buf[..(buf.len() - remain_len)])
    }
}

//解码
pub trait RawDecode<'de>: Sized {
    // 不带opt的解码，默认一般实现此方法
    fn raw_decode(buf: &'de [u8]) -> CodecResult<(Self, &'de [u8])>;

    // 带opt的解码，如果想使用版本等高级解码特性，需要实现此方法
    fn raw_decode_with_option(
        buf: &'de [u8],
        _opt: &RawDecodeOption,
    ) -> CodecResult<(Self, &'de [u8])> {
        Self::raw_decode(buf)
    }
}

pub trait RawDecodeWithContext<'de, Context>: Sized {
    fn raw_decode_with_context(buf: &'de [u8], _: Context) -> CodecResult<(Self, &'de [u8])>;
}

pub trait RawMergable: Clone + Any {
    fn raw_merge_ok(&self, other: &Self) -> bool;
}

impl<T: RawEncode + Eq + Clone + Any> RawMergable for T {
    fn raw_merge_ok(&self, other: &Self) -> bool {
        self.eq(other)
    }
}
