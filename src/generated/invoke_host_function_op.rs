#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// InvokeHostFunctionOp is an XDR Struct defined as:
///
/// ```text
/// struct InvokeHostFunctionOp
/// {
///     // Host function to invoke.
///     HostFunction hostFunction;
///     // Per-address authorizations for this host function.
///     SorobanAuthorizationEntry auth<>;
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
pub struct InvokeHostFunctionOp {
    pub host_function: HostFunction,
    pub auth: VecM<SorobanAuthorizationEntry>,
}

impl ReadXdr for InvokeHostFunctionOp {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                host_function: HostFunction::read_xdr(r)?,
                auth: VecM::<SorobanAuthorizationEntry>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for InvokeHostFunctionOp {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.host_function.write_xdr(w)?;
            self.auth.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`InvokeHostFunctionOp`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyInvokeHostFunctionOp(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyInvokeHostFunctionOp {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyInvokeHostFunctionOp {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.host_function().cmp(&other.host_function()))
            .then_with(|| self.auth().cmp(&other.auth()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyInvokeHostFunctionOp {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len =
                <LazyHostFunction as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len = <LazyVecM<LazySorobanAuthorizationEntry> as LazyXdr>::xdr_validate(
                &buf[pos as usize..],
                depth,
            )?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyHostFunction as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazySorobanAuthorizationEntry> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyInvokeHostFunctionOp {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyInvokeHostFunctionOp {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyInvokeHostFunctionOp {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyInvokeHostFunctionOp {
    /// Access field `host_function`.
    #[must_use]
    pub fn host_function(&self) -> LazyHostFunction {
        <LazyHostFunction as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `auth`.
    #[must_use]
    pub fn auth(&self) -> LazyVecM<LazySorobanAuthorizationEntry> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyHostFunction as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyVecM<LazySorobanAuthorizationEntry> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&InvokeHostFunctionOp> for LazyInvokeHostFunctionOp {
    type Error = Error;
    fn try_from(val: &InvokeHostFunctionOp) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyInvokeHostFunctionOp> for InvokeHostFunctionOp {
    type Error = Error;
    fn try_from(lazy: &LazyInvokeHostFunctionOp) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
