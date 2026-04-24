#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ScSpecEventV0 is an XDR Struct defined as:
///
/// ```text
/// struct SCSpecEventV0
/// {
///     string doc<SC_SPEC_DOC_LIMIT>;
///     string lib<80>;
///     SCSymbol name;
///     SCSymbol prefixTopics<2>;
///     SCSpecEventParamV0 params<>;
///     SCSpecEventDataFormat dataFormat;
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
pub struct ScSpecEventV0 {
    pub doc: StringM<1024>,
    pub lib: StringM<80>,
    pub name: ScSymbol,
    pub prefix_topics: VecM<ScSymbol, 2>,
    pub params: VecM<ScSpecEventParamV0>,
    pub data_format: ScSpecEventDataFormat,
}

impl ReadXdr for ScSpecEventV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                doc: StringM::<1024>::read_xdr(r)?,
                lib: StringM::<80>::read_xdr(r)?,
                name: ScSymbol::read_xdr(r)?,
                prefix_topics: VecM::<ScSymbol, 2>::read_xdr(r)?,
                params: VecM::<ScSpecEventParamV0>::read_xdr(r)?,
                data_format: ScSpecEventDataFormat::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScSpecEventV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.doc.write_xdr(w)?;
            self.lib.write_xdr(w)?;
            self.name.write_xdr(w)?;
            self.prefix_topics.write_xdr(w)?;
            self.params.write_xdr(w)?;
            self.data_format.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ScSpecEventV0`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyScSpecEventV0(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyScSpecEventV0 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyScSpecEventV0 {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.doc().cmp(&other.doc()))
            .then_with(|| self.lib().cmp(&other.lib()))
            .then_with(|| self.name().cmp(&other.name()))
            .then_with(|| self.prefix_topics().cmp(&other.prefix_topics()))
            .then_with(|| self.params().cmp(&other.params()))
            .then_with(|| self.data_format().cmp(&other.data_format()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyScSpecEventV0 {
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
                <LazyStringM<80> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len = <LazyScSymbol as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len =
                <LazyVecM<LazyScSymbol, 2> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len = <LazyVecM<LazyScSpecEventParamV0> as LazyXdr>::xdr_validate(
                &buf[pos as usize..],
                depth,
            )?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        let next_pos = pos.checked_add(4).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        <ScSpecEventDataFormat as LazyXdr>::xdr_validate(&buf[(pos + 0) as usize..], depth)?;
        pos = next_pos;
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyStringM<1024> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyStringM<80> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyScSymbol as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyScSymbol, 2> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyScSpecEventParamV0> as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyScSpecEventV0 {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyScSpecEventV0 {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyScSpecEventV0 {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyScSpecEventV0 {
    /// Access field `doc`.
    #[must_use]
    pub fn doc(&self) -> LazyStringM<1024> {
        <LazyStringM<1024> as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `lib`.
    #[must_use]
    pub fn lib(&self) -> LazyStringM<80> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyStringM<1024> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyStringM<80> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `name`.
    #[must_use]
    pub fn name(&self) -> LazyScSymbol {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyStringM<1024> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyStringM<80> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyScSymbol as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `prefix_topics`.
    #[must_use]
    pub fn prefix_topics(&self) -> LazyVecM<LazyScSymbol, 2> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyStringM<1024> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyStringM<80> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyScSymbol as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyVecM<LazyScSymbol, 2> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `params`.
    #[must_use]
    pub fn params(&self) -> LazyVecM<LazyScSpecEventParamV0> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyStringM<1024> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyStringM<80> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyScSymbol as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyScSymbol, 2> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyVecM<LazyScSpecEventParamV0> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `data_format`.
    #[must_use]
    pub fn data_format(&self) -> ScSpecEventDataFormat {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyStringM<1024> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyStringM<80> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyScSymbol as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyScSymbol, 2> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyScSpecEventParamV0> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <ScSpecEventDataFormat as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ScSpecEventV0> for LazyScSpecEventV0 {
    type Error = Error;
    fn try_from(val: &ScSpecEventV0) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyScSpecEventV0> for ScSpecEventV0 {
    type Error = Error;
    fn try_from(lazy: &LazyScSpecEventV0) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
