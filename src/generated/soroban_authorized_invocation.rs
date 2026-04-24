#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// SorobanAuthorizedInvocation is an XDR Struct defined as:
///
/// ```text
/// struct SorobanAuthorizedInvocation
/// {
///     SorobanAuthorizedFunction function;
///     SorobanAuthorizedInvocation subInvocations<>;
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
pub struct SorobanAuthorizedInvocation {
    pub function: SorobanAuthorizedFunction,
    pub sub_invocations: VecM<SorobanAuthorizedInvocation>,
}

impl ReadXdr for SorobanAuthorizedInvocation {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                function: SorobanAuthorizedFunction::read_xdr(r)?,
                sub_invocations: VecM::<SorobanAuthorizedInvocation>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SorobanAuthorizedInvocation {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.function.write_xdr(w)?;
            self.sub_invocations.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`SorobanAuthorizedInvocation`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazySorobanAuthorizedInvocation(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazySorobanAuthorizedInvocation {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazySorobanAuthorizedInvocation {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.function().cmp(&other.function()))
            .then_with(|| self.sub_invocations().cmp(&other.sub_invocations()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazySorobanAuthorizedInvocation {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len = <LazySorobanAuthorizedFunction as LazyXdr>::xdr_validate(
                &buf[pos as usize..],
                depth,
            )?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len = <LazyVecM<LazySorobanAuthorizedInvocation> as LazyXdr>::xdr_validate(
                &buf[pos as usize..],
                depth,
            )?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazySorobanAuthorizedFunction as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos +=
            <LazyVecM<LazySorobanAuthorizedInvocation> as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazySorobanAuthorizedInvocation {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazySorobanAuthorizedInvocation {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazySorobanAuthorizedInvocation {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazySorobanAuthorizedInvocation {
    /// Access field `function`.
    #[must_use]
    pub fn function(&self) -> LazySorobanAuthorizedFunction {
        <LazySorobanAuthorizedFunction as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `sub_invocations`.
    #[must_use]
    pub fn sub_invocations(&self) -> LazyVecM<LazySorobanAuthorizedInvocation> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazySorobanAuthorizedFunction as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyVecM<LazySorobanAuthorizedInvocation> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&SorobanAuthorizedInvocation> for LazySorobanAuthorizedInvocation {
    type Error = Error;
    fn try_from(val: &SorobanAuthorizedInvocation) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazySorobanAuthorizedInvocation> for SorobanAuthorizedInvocation {
    type Error = Error;
    fn try_from(lazy: &LazySorobanAuthorizedInvocation) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
