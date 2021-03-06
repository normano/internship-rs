use std::ops::{Deref, Index, Range, RangeFrom, RangeTo, RangeFull};
use std::cmp::{PartialEq};
use std::hash::{Hash, Hasher};
use std::borrow::{Borrow, Cow};
use std::fmt;
use std::str::{from_utf8, Utf8Error};

use handle::Handle;
use istr::IStr;
use icstr::ICStr;

/// Interned byte string type
///
/// `IBytes` is like `IStr`, but for arbitrary byte string.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IBytes(pub(crate) Handle);

impl IBytes {
    pub fn new(src: &[u8]) -> Self {
        IBytes(Handle::new(src))
    }

    pub fn from_str(src: &str) -> Self {
        IBytes(Handle::new(src.as_bytes()))
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.0.get()
    }

    #[inline]
    pub fn to_istr(&self) -> Result<IStr, Utf8Error> {
        from_utf8(self).map(|_| IStr(self.0.clone()))
    }
}

impl Deref for IBytes {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<'a> From<Cow<'a, [u8]>> for IBytes {
    fn from(v: Cow<[u8]>) -> Self {
        IBytes::new(&v)
    }
}

impl<'a> From<&'a [u8]> for IBytes {
    #[inline]
    fn from(v: &[u8]) -> Self {
        IBytes::new(v)
    }
}

impl<'a> From<&'a mut [u8]> for IBytes {
    fn from(v: &mut [u8]) -> Self {
        IBytes::new(&v)
    }
}

impl From<Vec<u8>> for IBytes {
    fn from(v: Vec<u8>) -> Self {
        IBytes::new(&v)
    }
}

impl From<Box<[u8]>> for IBytes {
    fn from(v: Box<[u8]>) -> Self {
        IBytes::new(&v)
    }
}

impl From<String> for IBytes {
    fn from(v: String) -> Self {
        IBytes::new(v.as_bytes())
    }
}

impl<'a> From<&'a str> for IBytes {
    fn from(v: &str) -> Self {
        IBytes::new(v.as_bytes())
    }
}

impl From<IStr> for IBytes {
    fn from(v: IStr) -> Self {
        v.to_ibytes()
    }
}

impl From<ICStr> for IBytes {
    fn from(v: ICStr) -> Self {
        v.to_ibytes_with_nul()
    }
}

impl PartialEq<Vec<u8>> for IBytes {
    fn eq(&self, other: &Vec<u8>) -> bool {
        PartialEq::eq(self.as_bytes(), &**other)
    }
}

impl<'a> PartialEq<&'a [u8]> for IBytes {
    fn eq(&self, other: &&[u8]) -> bool {
        PartialEq::eq(self.as_bytes(), *other)
    }
}

impl PartialEq<[u8]> for IBytes {
    fn eq(&self, other: &[u8]) -> bool {
        PartialEq::eq(self.as_bytes(), other)
    }
}

impl Default for IBytes {
    #[inline]
    fn default() -> Self {
        IBytes::new(&b""[..])
    }
}

impl Hash for IBytes {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        Hash::hash(self.as_bytes(), hasher)
    }
}

impl Borrow<[u8]> for IBytes {
    #[inline]
    fn borrow(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl Index<Range<usize>> for IBytes {
    type Output = [u8];

    #[inline]
    fn index(&self, index: Range<usize>) -> &[u8] {
        &self.as_bytes()[index]
    }
}

impl Index<RangeFrom<usize>> for IBytes {
    type Output = [u8];

    #[inline]
    fn index(&self, index: RangeFrom<usize>) -> &[u8] {
        &self.as_bytes()[index]
    }
}

impl Index<RangeTo<usize>> for IBytes {
    type Output = [u8];

    #[inline]
    fn index(&self, index: RangeTo<usize>) -> &[u8] {
        &self.as_bytes()[index]
    }
}

impl Index<RangeFull> for IBytes {
    type Output = [u8];

    #[inline]
    fn index(&self, index: RangeFull) -> &[u8] {
        &self.as_bytes()[index]
    }
}

impl AsRef<[u8]> for IBytes {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl fmt::Debug for IBytes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.as_bytes(), f)
    }
}

#[cfg(feature = "serde-compat")]
mod serde_compat {
    use super::*;
    use serde::{Serialize, Serializer, Deserialize, Deserializer, de};

    impl Serialize for IBytes {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            Serialize::serialize(self.as_bytes(), s)
        }
    }

    impl<'d> Deserialize<'d> for IBytes {
        fn deserialize<D: Deserializer<'d>>(d: D) -> Result<IBytes, D::Error> {
            d.deserialize_bytes(Visitor)
        }
    }

    pub struct Visitor;

    impl<'d> de::Visitor<'d> for Visitor {
        type Value = IBytes;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("byte slice")
        }

        fn visit_bytes<E: de::Error>(self, value: &[u8]) -> Result<IBytes, E> {
            Ok(IBytes::new(value))
        }
    }
}
