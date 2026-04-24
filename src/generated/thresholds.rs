#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// Thresholds is an XDR Typedef defined as:
///
/// ```text
/// typedef opaque Thresholds[4];
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
pub struct Thresholds(pub [u8; 4]);

impl core::fmt::Debug for Thresholds {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let v = &self.0;
        write!(f, "Thresholds(")?;
        for b in v {
            write!(f, "{b:02x}")?;
        }
        write!(f, ")")?;
        Ok(())
    }
}
impl core::fmt::Display for Thresholds {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let v = &self.0;
        for b in v {
            write!(f, "{b:02x}")?;
        }
        Ok(())
    }
}

#[cfg(feature = "alloc")]
impl core::str::FromStr for Thresholds {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        hex::decode(s).map_err(|_| Error::InvalidHex)?.try_into()
    }
}
#[cfg(feature = "schemars")]
impl schemars::JsonSchema for Thresholds {
    fn schema_name() -> String {
        "Thresholds".to_string()
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
                max_length: 4_u32.checked_mul(2).map(Some).unwrap_or_default(),
                min_length: 4_u32.checked_mul(2).map(Some).unwrap_or_default(),
                ..string
            }));
            schema.into()
        } else {
            schema
        }
    }
}
impl From<Thresholds> for [u8; 4] {
    #[must_use]
    fn from(x: Thresholds) -> Self {
        x.0
    }
}

impl From<[u8; 4]> for Thresholds {
    #[must_use]
    fn from(x: [u8; 4]) -> Self {
        Thresholds(x)
    }
}

impl AsRef<[u8; 4]> for Thresholds {
    #[must_use]
    fn as_ref(&self) -> &[u8; 4] {
        &self.0
    }
}

impl ReadXdr for Thresholds {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = <[u8; 4]>::read_xdr(r)?;
            let v = Thresholds(i);
            Ok(v)
        })
    }
}

impl WriteXdr for Thresholds {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

impl Thresholds {
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<Vec<u8>> for Thresholds {
    type Error = Error;
    fn try_from(x: Vec<u8>) -> Result<Self, Error> {
        x.as_slice().try_into()
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<u8>> for Thresholds {
    type Error = Error;
    fn try_from(x: &Vec<u8>) -> Result<Self, Error> {
        x.as_slice().try_into()
    }
}

impl TryFrom<&[u8]> for Thresholds {
    type Error = Error;
    fn try_from(x: &[u8]) -> Result<Self, Error> {
        Ok(Thresholds(x.try_into()?))
    }
}

impl AsRef<[u8]> for Thresholds {
    #[must_use]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`Thresholds`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyThresholds(LazyOpaqueFixed<4>);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyThresholds {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyThresholds {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyThresholds {
    const FIXED_XDR_SIZE: Option<u32> = Some(4);

    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        <LazyOpaqueFixed<4> as LazyXdr>::xdr_validate(buf, depth)
    }

    #[inline]
    fn xdr_len(buf: &[u8]) -> u32 {
        <LazyOpaqueFixed<4> as LazyXdr>::xdr_len(buf)
    }

    fn from_xdr_consume(parent: &LazyHandle, buf: &mut &[u8]) -> Self {
        Self(<LazyOpaqueFixed<4> as LazyXdr>::from_xdr_consume(
            parent, buf,
        ))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyThresholds {
    fn from(h: LazyHandle) -> Self {
        Self(<LazyOpaqueFixed<4>>::from(h))
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyThresholds {
    fn as_ref(&self) -> &LazyHandle {
        self.0.as_ref()
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyThresholds {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(<LazyOpaqueFixed<4>>::from(LazyHandle::from_arc(
            buf, 0, len,
        ))))
    }
}
#[cfg(feature = "alloc")]
impl core::ops::Deref for LazyThresholds {
    type Target = LazyOpaqueFixed<4>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&Thresholds> for LazyThresholds {
    type Error = Error;
    fn try_from(val: &Thresholds) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyThresholds> for Thresholds {
    type Error = Error;
    fn try_from(lazy: &LazyThresholds) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
