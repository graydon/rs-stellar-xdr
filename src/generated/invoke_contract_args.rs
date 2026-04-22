#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// InvokeContractArgs is an XDR Struct defined as:
///
/// ```text
/// struct InvokeContractArgs {
///     SCAddress contractAddress;
///     SCSymbol functionName;
///     SCVal args<>;
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
pub struct InvokeContractArgs {
    pub contract_address: ScAddress,
    pub function_name: ScSymbol,
    pub args: VecM<ScVal>,
}

impl ReadXdr for InvokeContractArgs {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                contract_address: ScAddress::read_xdr(r)?,
                function_name: ScSymbol::read_xdr(r)?,
                args: VecM::<ScVal>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for InvokeContractArgs {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.contract_address.write_xdr(w)?;
            self.function_name.write_xdr(w)?;
            self.args.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`InvokeContractArgs`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyInvokeContractArgs(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyInvokeContractArgs {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyInvokeContractArgs {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.contract_address().cmp(&other.contract_address()))
            .then_with(|| self.function_name().cmp(&other.function_name()))
            .then_with(|| self.args().cmp(&other.args()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyInvokeContractArgs {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len = <LazyScAddress as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len = <LazyScSymbol as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len =
                <LazyVecM<LazyScVal> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyScAddress as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyScSymbol as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyScVal> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyInvokeContractArgs {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyInvokeContractArgs {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyInvokeContractArgs {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyInvokeContractArgs {
    /// Access field `contract_address`.
    #[must_use]
    pub fn contract_address(&self) -> LazyScAddress {
        <LazyScAddress as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `function_name`.
    #[must_use]
    pub fn function_name(&self) -> LazyScSymbol {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyScAddress as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyScSymbol as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `args`.
    #[must_use]
    pub fn args(&self) -> LazyVecM<LazyScVal> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyScAddress as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyScSymbol as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyVecM<LazyScVal> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&InvokeContractArgs> for LazyInvokeContractArgs {
    type Error = Error;
    fn try_from(val: &InvokeContractArgs) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyInvokeContractArgs> for InvokeContractArgs {
    type Error = Error;
    fn try_from(lazy: &LazyInvokeContractArgs) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
