use bytes::buf::{Buf, BufMut};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TryError {
    #[error("Not enough bytes to read")]
    NotEnoughBytes,
    #[error("Too many bytes to read")]
    ValueTooLarge,
    #[error("Go home, you're drunk?")]
    Malformed,
}

pub trait TryReadInto<T: Sized>: Buf {
    fn try_read(&mut self) -> Result<T, TryError>;
}

pub trait TryReadFrom: Sized {
    fn try_read(buf: &mut impl Buf) -> Result<Self, TryError>;
}

impl<B, T> TryReadInto<T> for B
where
    B: Buf,
    T: TryReadFrom
{
    fn try_read(&mut self) -> Result<T, TryError> {
        TryReadFrom::try_read(self)
    }
}

pub trait WriteInto: Sized {
    fn write(&self, buf: &mut impl BufMut) -> usize;
}

pub trait WriteFrom<T: Sized>: BufMut {
    fn write(&mut self, value: &T) -> usize;
}

impl<T, B> WriteFrom<T> for B
where
    B: BufMut,
    T: WriteInto,
{
    fn write(&mut self, value: &T) -> usize {
        value.write(self)
    }
}

macro_rules! try_read_write {
    ($this:ident, $size:expr, $get:ident, $put:ident) => {
        impl TryReadFrom for $this {
            fn try_read(buf: &mut impl Buf) -> Result<$this, TryError> {
                if buf.remaining() < $size {
                    Err(TryError::NotEnoughBytes)?
                } else {
                    Ok(buf.$get())
                }
            }
        }

        impl WriteInto for $this {
            fn write(&self, buf: &mut impl BufMut) -> usize {
                buf.$put(*self);
                $size
            }
        }
    };
}

try_read_write!(u8, 1, get_u8, put_u8);
try_read_write!(i8, 1, get_i8, put_i8);

try_read_write!(u16, 2, get_u16, put_u16);
try_read_write!(i16, 2, get_i16, put_i16);

try_read_write!(u32, 4, get_u32, put_u32);
try_read_write!(i32, 4, get_i32, put_i32);

try_read_write!(u64, 8, get_u64, put_u64);
try_read_write!(i64, 8, get_i64, put_i64);

try_read_write!(f32, 4, get_f32, put_f32);
try_read_write!(f64, 8, get_f64, put_f64);

impl TryReadFrom for bool {
    fn try_read(buf: &mut impl Buf) -> Result<Self, TryError> {
        let val: u8 = buf.try_read()?;
        match val {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(TryError::Malformed)?,
        }
    }
}

pub trait Offset {
    const VALUE: usize;
}

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct PrefixedVec<P, T, O>
where
    P: TryReadFrom + Into<u64>,
    T: TryReadFrom,
    O: Offset,
{
    #[serde(skip)]
    offset: std::marker::PhantomData<O>,
    #[serde(skip)]
    prefix: std::marker::PhantomData<P>,
    data: Vec<T>,
}

impl<P, T, O> std::ops::Deref for PrefixedVec<P, T, O>
where
    P: TryReadFrom + Into<u64>,
    T: TryReadFrom,
    O: Offset,
{
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<P, T, O> TryReadFrom for PrefixedVec<P, T, O>
where
    P: TryReadFrom + Into<u64>,
    T: TryReadFrom,
    O: Offset,
{
    fn try_read(buf: &mut impl Buf) -> Result<Self, TryError> {
        let length = P::try_read(buf)?.into() as usize - <O as Offset>::VALUE;
        let mut data = Vec::with_capacity(length);
        for _ in 0..length {
            data.push(T::try_read(buf)?);
        }
        Ok(Self {
            offset: Default::default(),
            prefix: Default::default(),
            data,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct ZeroOffset;
impl Offset for ZeroOffset {
    const VALUE: usize = 0;
}
#[derive(Debug, PartialEq)]
pub struct OneOffset;
impl Offset for OneOffset {
    const VALUE: usize = 1;
}