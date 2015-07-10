//! Library that allows do en- and decode numbers from byte array in little endian,
//! big endian, network or native byte order.
//!
//! ## Usage
//! ```
//! use byteio::{BigEndian, LittleEndian, ReadBytesExt};
//! let data = vec![1, 2];
//! let val: u16 = (&data[..]).read_as::<LittleEndian>().unwrap();
//! assert_eq!(val, 513);
//! let val: u16 = (&data[..]).read_as::<BigEndian>().unwrap();
//! assert_eq!(val, 258);
//! ```

use std::io;
use std::mem;

/// Type that can easily be converted to a (mutable) reference to a slice.
///
/// ## Note
/// This is part of a workaround until associated constants are stable
pub trait AsSlice<T>: AsRef<[T]> + AsMut<[T]> {}
impl<T, V> AsSlice<T> for V where V: AsRef<[T]> + AsMut<[T]> {}

/// Conversion of a (small) type to a byte array and vice versa.
///
/// ## Note 
/// This should not be used to convert big types to byte arrays as it will
/// overflow the stack.
pub trait ByteOrder<T> {
    /// Conversion buffer type.
    ///
    /// Should be big enough to hold a T.
    ///
    /// ## Note
    /// This is a workaround until associated constants are stable
    type Buffer: AsSlice<u8>;
    /// Converts the byte array `buf` into a `T`.
    fn from_bytes(buf: Self::Buffer) -> T;
    /// Converts `n` into a byte array.
    fn into_bytes(n: T) -> Self::Buffer;
    /// Returns a sufficiently big conversion buffer buffer.
    /// 
    /// ## Note
    /// This is a workaround until associated constants are stable
    fn buffer() -> Self::Buffer;
}

/// Little endian byte order.
pub enum LittleEndian {}
/// Little endian byte order.
pub type LE = LittleEndian;
/// Big endian byte order.
pub enum BigEndian {}
/// Big endian byte order.
pub type BE = BigEndian;

/// System-native byte order.
#[cfg(target_endian = "little")]
pub type NativeByteOrder = LittleEndian;
/// Network byte order.
pub type NetworkByteOrder = LittleEndian;
/// System-native byte order.
#[cfg(target_endian = "big")]
pub type NativeByteOrder = BigEndian;

macro_rules! impl_byte_order {
    ($val:ident, $bytes:expr, $byte_order:ident, $convert:ident) => {
        impl ByteOrder<$val> for $byte_order {
            type Buffer = [u8; $bytes];
    
            #[inline]
            fn from_bytes(buf: Self::Buffer) -> $val {
                unsafe { mem::transmute::<_, $val>(buf) }.$convert()
                
            }
            
            #[inline]
            fn into_bytes(n: $val) -> Self::Buffer {
                unsafe { mem::transmute(n.$convert()) }
            }
            
            #[inline]
            fn buffer() -> Self::Buffer {
                [0; $bytes]
            }
        }
    };
    ($byte_order:ident, $convert:ident) => {
        impl_byte_order!(u8 , 1, $byte_order, $convert);
        impl_byte_order!(u16, 2, $byte_order, $convert);
        impl_byte_order!(u32, 4, $byte_order, $convert);
        impl_byte_order!(u64, 8, $byte_order, $convert);
        impl_byte_order!(i8 , 1, $byte_order, $convert);
        impl_byte_order!(i16, 2, $byte_order, $convert);
        impl_byte_order!(i32, 4, $byte_order, $convert);
        impl_byte_order!(i64, 8, $byte_order, $convert);
        
        impl ByteOrder<f32> for $byte_order {
            type Buffer = [u8; 4];
    
            #[inline]
            fn from_bytes(buf: Self::Buffer) -> f32 {
                unsafe {
                    mem::transmute(mem::transmute::<_, u32>(buf).$convert())
                }
            }
            
            #[inline]
            fn into_bytes(n: f32) -> Self::Buffer {
                unsafe { 
                    mem::transmute(mem::transmute::<_, u32>(n).$convert())
                }
            }
            
            #[inline]
            fn buffer() -> Self::Buffer {
                [0; 4]
            }
        }
        
        impl ByteOrder<f64> for $byte_order {
            type Buffer = [u8; 8];
    
            #[inline]
            fn from_bytes(buf: Self::Buffer) -> f64 {
                unsafe {
                    mem::transmute(mem::transmute::<_, u64>(buf).$convert())
                }
            }
            
            #[inline]
            fn into_bytes(n: f64) -> Self::Buffer {
                unsafe { 
                    mem::transmute(mem::transmute::<_, u64>(n).$convert())
                }
            }
            
            #[inline]
            fn buffer() -> Self::Buffer {
                [0; 8]
            }
        }
    }
}

impl_byte_order!(LittleEndian, to_le);
impl_byte_order!(BigEndian, to_be);

/// Extension trait for `io::Read` that allows to read `T`s from it.
pub trait ReadBytesExt<T> {
    fn read_as<B: ByteOrder<T>>(&mut self) -> io::Result<T>;
}
/// Extension trait for `io::Write` that allows to write `T`s from it.
pub trait WriteBytesExt<T> {
    fn write_as<B: ByteOrder<T>>(&mut self, n: T) -> io::Result<()>;
}

impl<T, R: io::Read> ReadBytesExt<T> for R {
    #[inline]
    fn read_as<B: ByteOrder<T>>(&mut self) -> io::Result<T> {
        let mut buf = B::buffer();
        if try!(self.read(buf.as_mut())) != buf.as_ref().len() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "could not read all bytes"
            ))
        }
        Ok(B::from_bytes(buf))
    }
}

impl<T, W: io::Write> WriteBytesExt<T> for W {
    #[inline]
    fn write_as<B: ByteOrder<T>>(&mut self, n: T) -> io::Result<()> {
        let buf = B::into_bytes(n);
        self.write_all(buf.as_ref())
    }
}