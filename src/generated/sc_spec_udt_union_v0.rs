#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ScSpecUdtUnionV0 is an XDR Struct defined as:
///
/// ```text
/// struct SCSpecUDTUnionV0
/// {
///     string doc<SC_SPEC_DOC_LIMIT>;
///     string lib<80>;
///     string name<60>;
///     SCSpecUDTUnionCaseV0 cases<>;
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
pub struct ScSpecUdtUnionV0 {
    pub doc: StringM<1024>,
    pub lib: StringM<80>,
    pub name: StringM<60>,
    pub cases: VecM<ScSpecUdtUnionCaseV0>,
}

impl ReadXdr for ScSpecUdtUnionV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                doc: StringM::<1024>::read_xdr(r)?,
                lib: StringM::<80>::read_xdr(r)?,
                name: StringM::<60>::read_xdr(r)?,
                cases: VecM::<ScSpecUdtUnionCaseV0>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScSpecUdtUnionV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.doc.write_xdr(w)?;
            self.lib.write_xdr(w)?;
            self.name.write_xdr(w)?;
            self.cases.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ScSpecUdtUnionV0`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyScSpecUdtUnionV0(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyScSpecUdtUnionV0 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyScSpecUdtUnionV0 {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.doc().cmp(&other.doc()))
            .then_with(|| self.lib().cmp(&other.lib()))
            .then_with(|| self.name().cmp(&other.name()))
            .then_with(|| self.cases().cmp(&other.cases()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyScSpecUdtUnionV0 {
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
            let field_len =
                <LazyStringM<60> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len = <LazyVecM<LazyScSpecUdtUnionCaseV0> as LazyXdr>::xdr_validate(
                &buf[pos as usize..],
                depth,
            )?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyStringM<1024> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyStringM<80> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyStringM<60> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyScSpecUdtUnionCaseV0> as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyScSpecUdtUnionV0 {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyScSpecUdtUnionV0 {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyScSpecUdtUnionV0 {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyScSpecUdtUnionV0 {
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
    pub fn name(&self) -> LazyStringM<60> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyStringM<1024> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyStringM<80> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyStringM<60> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `cases`.
    #[must_use]
    pub fn cases(&self) -> LazyVecM<LazyScSpecUdtUnionCaseV0> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyStringM<1024> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyStringM<80> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyStringM<60> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyVecM<LazyScSpecUdtUnionCaseV0> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ScSpecUdtUnionV0> for LazyScSpecUdtUnionV0 {
    type Error = Error;
    fn try_from(val: &ScSpecUdtUnionV0) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyScSpecUdtUnionV0> for ScSpecUdtUnionV0 {
    type Error = Error;
    fn try_from(lazy: &LazyScSpecUdtUnionV0) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
