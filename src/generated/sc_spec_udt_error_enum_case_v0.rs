#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ScSpecUdtErrorEnumCaseV0 is an XDR Struct defined as:
///
/// ```text
/// struct SCSpecUDTErrorEnumCaseV0
/// {
///     string doc<SC_SPEC_DOC_LIMIT>;
///     string name<60>;
///     uint32 value;
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
pub struct ScSpecUdtErrorEnumCaseV0 {
    pub doc: StringM<1024>,
    pub name: StringM<60>,
    pub value: u32,
}

impl ReadXdr for ScSpecUdtErrorEnumCaseV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                doc: StringM::<1024>::read_xdr(r)?,
                name: StringM::<60>::read_xdr(r)?,
                value: u32::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScSpecUdtErrorEnumCaseV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.doc.write_xdr(w)?;
            self.name.write_xdr(w)?;
            self.value.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ScSpecUdtErrorEnumCaseV0`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyScSpecUdtErrorEnumCaseV0(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyScSpecUdtErrorEnumCaseV0 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyScSpecUdtErrorEnumCaseV0 {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.doc().cmp(&other.doc()))
            .then_with(|| self.name().cmp(&other.name()))
            .then_with(|| self.value().cmp(&other.value()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyScSpecUdtErrorEnumCaseV0 {
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
                <LazyStringM<60> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        let next_pos = pos.checked_add(4).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        pos = next_pos;
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyStringM<1024> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyStringM<60> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 4;
        pos
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyScSpecUdtErrorEnumCaseV0 {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyScSpecUdtErrorEnumCaseV0 {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyScSpecUdtErrorEnumCaseV0 {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyScSpecUdtErrorEnumCaseV0 {
    /// Access field `doc`.
    #[must_use]
    pub fn doc(&self) -> LazyStringM<1024> {
        <LazyStringM<1024> as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `name`.
    #[must_use]
    pub fn name(&self) -> LazyStringM<60> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyStringM<1024> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyStringM<60> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `value`.
    #[must_use]
    pub fn value(&self) -> u32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyStringM<1024> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyStringM<60> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <u32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ScSpecUdtErrorEnumCaseV0> for LazyScSpecUdtErrorEnumCaseV0 {
    type Error = Error;
    fn try_from(val: &ScSpecUdtErrorEnumCaseV0) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyScSpecUdtErrorEnumCaseV0> for ScSpecUdtErrorEnumCaseV0 {
    type Error = Error;
    fn try_from(lazy: &LazyScSpecUdtErrorEnumCaseV0) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
