#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// DataValue is an XDR Typedef defined as:
///
/// ```text
/// typedef opaque DataValue<64>;
/// ```
///
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[derive(Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug)]
pub struct DataValue(pub BytesM<64>);

impl From<DataValue> for BytesM<64> {
    #[must_use]
    fn from(x: DataValue) -> Self {
        x.0
    }
}

impl From<BytesM<64>> for DataValue {
    #[must_use]
    fn from(x: BytesM<64>) -> Self {
        DataValue(x)
    }
}

impl AsRef<BytesM<64>> for DataValue {
    #[must_use]
    fn as_ref(&self) -> &BytesM<64> {
        &self.0
    }
}

impl ReadXdr for DataValue {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = BytesM::<64>::read_xdr(r)?;
            let v = DataValue(i);
            Ok(v)
        })
    }
}

impl WriteXdr for DataValue {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

impl Deref for DataValue {
    type Target = BytesM<64>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<DataValue> for Vec<u8> {
    #[must_use]
    fn from(x: DataValue) -> Self {
        x.0 .0
    }
}

impl TryFrom<Vec<u8>> for DataValue {
    type Error = Error;
    fn try_from(x: Vec<u8>) -> Result<Self, Error> {
        Ok(DataValue(x.try_into()?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<u8>> for DataValue {
    type Error = Error;
    fn try_from(x: &Vec<u8>) -> Result<Self, Error> {
        Ok(DataValue(x.try_into()?))
    }
}

impl AsRef<Vec<u8>> for DataValue {
    #[must_use]
    fn as_ref(&self) -> &Vec<u8> {
        &self.0 .0
    }
}

impl AsRef<[u8]> for DataValue {
    #[cfg(feature = "alloc")]
    #[must_use]
    fn as_ref(&self) -> &[u8] {
        &self.0 .0
    }
    #[cfg(not(feature = "alloc"))]
    #[must_use]
    fn as_ref(&self) -> &[u8] {
        self.0 .0
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`DataValue`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyDataValue(LazyBytesM<64>);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyDataValue {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyDataValue {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyDataValue {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        <LazyBytesM<64> as LazyXdr>::xdr_validate(buf, depth)
    }

    #[inline]
    fn xdr_len(buf: &[u8]) -> u32 {
        <LazyBytesM<64> as LazyXdr>::xdr_len(buf)
    }

    fn from_xdr_consume(parent: &LazyHandle, buf: &mut &[u8]) -> Self {
        Self(<LazyBytesM<64> as LazyXdr>::from_xdr_consume(parent, buf))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyDataValue {
    fn from(h: LazyHandle) -> Self {
        Self(<LazyBytesM<64>>::from(h))
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyDataValue {
    fn as_ref(&self) -> &LazyHandle {
        self.0.as_ref()
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyDataValue {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(<LazyBytesM<64>>::from(LazyHandle::from_arc(
            buf, 0, len,
        ))))
    }
}
#[cfg(feature = "alloc")]
impl core::ops::Deref for LazyDataValue {
    type Target = LazyBytesM<64>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&DataValue> for LazyDataValue {
    type Error = Error;
    fn try_from(val: &DataValue) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyDataValue> for DataValue {
    type Error = Error;
    fn try_from(lazy: &LazyDataValue) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
