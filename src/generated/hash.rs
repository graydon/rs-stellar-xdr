#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// Hash is an XDR Typedef defined as:
///
/// ```text
/// typedef opaque Hash[32];
/// ```
///
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr)
)]
pub struct Hash(pub [u8; 32]);

impl core::fmt::Debug for Hash {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let v = &self.0;
        write!(f, "Hash(")?;
        for b in v {
            write!(f, "{b:02x}")?;
        }
        write!(f, ")")?;
        Ok(())
    }
}
impl core::fmt::Display for Hash {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let v = &self.0;
        for b in v {
            write!(f, "{b:02x}")?;
        }
        Ok(())
    }
}

#[cfg(feature = "alloc")]
impl core::str::FromStr for Hash {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        hex::decode(s).map_err(|_| Error::InvalidHex)?.try_into()
    }
}
#[cfg(feature = "schemars")]
impl schemars::JsonSchema for Hash {
    fn schema_name() -> String {
        "Hash".to_string()
    }

    fn is_referenceable() -> bool {
        false
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        let schema = String::json_schema(gen);
        if let schemars::schema::Schema::Object(mut schema) = schema {
            schema.extensions.insert(
                "contentEncoding".to_owned(),
                serde_json::Value::String("hex".to_string()),
            );
            schema.extensions.insert(
                "contentMediaType".to_owned(),
                serde_json::Value::String("application/binary".to_string()),
            );
            let string = *schema.string.unwrap_or_default().clone();
            schema.string = Some(Box::new(schemars::schema::StringValidation {
                max_length: 32_u32.checked_mul(2).map(Some).unwrap_or_default(),
                min_length: 32_u32.checked_mul(2).map(Some).unwrap_or_default(),
                ..string
            }));
            schema.into()
        } else {
            schema
        }
    }
}
impl From<Hash> for [u8; 32] {
    #[must_use]
    fn from(x: Hash) -> Self {
        x.0
    }
}

impl From<[u8; 32]> for Hash {
    #[must_use]
    fn from(x: [u8; 32]) -> Self {
        Hash(x)
    }
}

impl AsRef<[u8; 32]> for Hash {
    #[must_use]
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

impl ReadXdr for Hash {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = <[u8; 32]>::read_xdr(r)?;
            let v = Hash(i);
            Ok(v)
        })
    }
}

impl WriteXdr for Hash {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

impl Hash {
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<Vec<u8>> for Hash {
    type Error = Error;
    fn try_from(x: Vec<u8>) -> Result<Self, Error> {
        x.as_slice().try_into()
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<u8>> for Hash {
    type Error = Error;
    fn try_from(x: &Vec<u8>) -> Result<Self, Error> {
        x.as_slice().try_into()
    }
}

impl TryFrom<&[u8]> for Hash {
    type Error = Error;
    fn try_from(x: &[u8]) -> Result<Self, Error> {
        Ok(Hash(x.try_into()?))
    }
}

impl AsRef<[u8]> for Hash {
    #[must_use]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`Hash`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyHash(LazyOpaqueFixed<32>);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyHash {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyHash {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyHash {
    const FIXED_XDR_SIZE: Option<u32> = Some(32);

    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        <LazyOpaqueFixed<32> as LazyXdr>::xdr_validate(buf, depth)
    }

    #[inline]
    fn xdr_len(buf: &[u8]) -> u32 {
        <LazyOpaqueFixed<32> as LazyXdr>::xdr_len(buf)
    }

    fn from_xdr_consume(parent: &LazyHandle, buf: &mut &[u8]) -> Self {
        Self(<LazyOpaqueFixed<32> as LazyXdr>::from_xdr_consume(
            parent, buf,
        ))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyHash {
    fn from(h: LazyHandle) -> Self {
        Self(<LazyOpaqueFixed<32>>::from(h))
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyHash {
    fn as_ref(&self) -> &LazyHandle {
        self.0.as_ref()
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyHash {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(<LazyOpaqueFixed<32>>::from(LazyHandle::from_arc(
            buf, 0, len,
        ))))
    }
}
#[cfg(feature = "alloc")]
impl core::ops::Deref for LazyHash {
    type Target = LazyOpaqueFixed<32>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&Hash> for LazyHash {
    type Error = Error;
    fn try_from(val: &Hash) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyHash> for Hash {
    type Error = Error;
    fn try_from(lazy: &LazyHash) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
