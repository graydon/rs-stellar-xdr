#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ScVec is an XDR Typedef defined as:
///
/// ```text
/// typedef SCVal SCVec<>;
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
pub struct ScVec(pub VecM<ScVal>);

impl From<ScVec> for VecM<ScVal> {
    #[must_use]
    fn from(x: ScVec) -> Self {
        x.0
    }
}

impl From<VecM<ScVal>> for ScVec {
    #[must_use]
    fn from(x: VecM<ScVal>) -> Self {
        ScVec(x)
    }
}

impl AsRef<VecM<ScVal>> for ScVec {
    #[must_use]
    fn as_ref(&self) -> &VecM<ScVal> {
        &self.0
    }
}

impl ReadXdr for ScVec {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = VecM::<ScVal>::read_xdr(r)?;
            let v = ScVec(i);
            Ok(v)
        })
    }
}

impl WriteXdr for ScVec {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

impl Deref for ScVec {
    type Target = VecM<ScVal>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<ScVec> for Vec<ScVal> {
    #[must_use]
    fn from(x: ScVec) -> Self {
        x.0 .0
    }
}

impl TryFrom<Vec<ScVal>> for ScVec {
    type Error = Error;
    fn try_from(x: Vec<ScVal>) -> Result<Self, Error> {
        Ok(ScVec(x.try_into()?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<ScVal>> for ScVec {
    type Error = Error;
    fn try_from(x: &Vec<ScVal>) -> Result<Self, Error> {
        Ok(ScVec(x.try_into()?))
    }
}

impl AsRef<Vec<ScVal>> for ScVec {
    #[must_use]
    fn as_ref(&self) -> &Vec<ScVal> {
        &self.0 .0
    }
}

impl AsRef<[ScVal]> for ScVec {
    #[cfg(feature = "alloc")]
    #[must_use]
    fn as_ref(&self) -> &[ScVal] {
        &self.0 .0
    }
    #[cfg(not(feature = "alloc"))]
    #[must_use]
    fn as_ref(&self) -> &[ScVal] {
        self.0 .0
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ScVec`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyScVec(LazyVecM<LazyScVal>);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyScVec {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyScVec {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyScVec {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        <LazyVecM<LazyScVal> as LazyXdr>::xdr_validate(buf, depth)
    }

    #[inline]
    fn xdr_len(buf: &[u8]) -> u32 {
        <LazyVecM<LazyScVal> as LazyXdr>::xdr_len(buf)
    }

    fn from_xdr_consume(parent: &LazyHandle, buf: &mut &[u8]) -> Self {
        Self(<LazyVecM<LazyScVal> as LazyXdr>::from_xdr_consume(
            parent, buf,
        ))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyScVec {
    fn from(h: LazyHandle) -> Self {
        Self(<LazyVecM<LazyScVal>>::from(h))
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyScVec {
    fn as_ref(&self) -> &LazyHandle {
        self.0.as_ref()
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyScVec {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(<LazyVecM<LazyScVal>>::from(LazyHandle::from_arc(
            buf, 0, len,
        ))))
    }
}
#[cfg(feature = "alloc")]
impl core::ops::Deref for LazyScVec {
    type Target = LazyVecM<LazyScVal>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ScVec> for LazyScVec {
    type Error = Error;
    fn try_from(val: &ScVec) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyScVec> for ScVec {
    type Error = Error;
    fn try_from(lazy: &LazyScVec) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
