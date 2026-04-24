#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ScSpecEventParamV0 is an XDR Struct defined as:
///
/// ```text
/// struct SCSpecEventParamV0
/// {
///     string doc<SC_SPEC_DOC_LIMIT>;
///     string name<30>;
///     SCSpecTypeDef type;
///     SCSpecEventParamLocationV0 location;
/// };
/// ```
///
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct ScSpecEventParamV0 {
    pub doc: StringM<1024>,
    pub name: StringM<30>,
    pub type_: ScSpecTypeDef,
    pub location: ScSpecEventParamLocationV0,
}

impl ReadXdr for ScSpecEventParamV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                doc: StringM::<1024>::read_xdr(r)?,
                name: StringM::<30>::read_xdr(r)?,
                type_: ScSpecTypeDef::read_xdr(r)?,
                location: ScSpecEventParamLocationV0::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScSpecEventParamV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.doc.write_xdr(w)?;
            self.name.write_xdr(w)?;
            self.type_.write_xdr(w)?;
            self.location.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ScSpecEventParamV0`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyScSpecEventParamV0(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyScSpecEventParamV0 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyScSpecEventParamV0 {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.doc().cmp(&other.doc()))
            .then_with(|| self.name().cmp(&other.name()))
            .then_with(|| self.type_().cmp(&other.type_()))
            .then_with(|| self.location().cmp(&other.location()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyScSpecEventParamV0 {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len =
                <LazyStringM<1024> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len =
                <LazyStringM<30> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len =
                <LazyScSpecTypeDef as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        let next_pos = pos.checked_add(4).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        <ScSpecEventParamLocationV0 as LazyXdr>::xdr_validate(&buf[(pos + 0) as usize..], depth)?;
        pos = next_pos;
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyStringM<1024> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyStringM<30> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyScSpecTypeDef as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 4;
        pos
    }

    fn from_xdr_consume(parent: &LazyHandle, buf: &mut &[u8]) -> Self {
        let len = Self::xdr_len(buf);
        let offset = (parent.len() as usize - buf.len()) as u32;
        let handle = parent.sub_handle(offset, len);
        *buf = &buf[len as usize..];
        Self(handle)
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyScSpecEventParamV0 {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyScSpecEventParamV0 {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyScSpecEventParamV0 {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyScSpecEventParamV0 {
    /// Access field `doc`.
    #[must_use]
    pub fn doc(&self) -> LazyStringM<1024> {
        <LazyStringM<1024> as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `name`.
    #[must_use]
    pub fn name(&self) -> LazyStringM<30> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyStringM<1024> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyStringM<30> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `type_`.
    #[must_use]
    pub fn type_(&self) -> LazyScSpecTypeDef {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyStringM<1024> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyStringM<30> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyScSpecTypeDef as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `location`.
    #[must_use]
    pub fn location(&self) -> ScSpecEventParamLocationV0 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyStringM<1024> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyStringM<30> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyScSpecTypeDef as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <ScSpecEventParamLocationV0 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ScSpecEventParamV0> for LazyScSpecEventParamV0 {
    type Error = Error;
    fn try_from(val: &ScSpecEventParamV0) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyScSpecEventParamV0> for ScSpecEventParamV0 {
    type Error = Error;
    fn try_from(lazy: &LazyScSpecEventParamV0) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
