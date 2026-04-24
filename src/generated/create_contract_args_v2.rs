#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// CreateContractArgsV2 is an XDR Struct defined as:
///
/// ```text
/// struct CreateContractArgsV2
/// {
///     ContractIDPreimage contractIDPreimage;
///     ContractExecutable executable;
///     // Arguments of the contract's constructor.
///     SCVal constructorArgs<>;
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
pub struct CreateContractArgsV2 {
    pub contract_id_preimage: ContractIdPreimage,
    pub executable: ContractExecutable,
    pub constructor_args: VecM<ScVal>,
}

impl ReadXdr for CreateContractArgsV2 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                contract_id_preimage: ContractIdPreimage::read_xdr(r)?,
                executable: ContractExecutable::read_xdr(r)?,
                constructor_args: VecM::<ScVal>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for CreateContractArgsV2 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.contract_id_preimage.write_xdr(w)?;
            self.executable.write_xdr(w)?;
            self.constructor_args.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`CreateContractArgsV2`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyCreateContractArgsV2(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyCreateContractArgsV2 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyCreateContractArgsV2 {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| {
                self.contract_id_preimage()
                    .cmp(&other.contract_id_preimage())
            })
            .then_with(|| self.executable().cmp(&other.executable()))
            .then_with(|| self.constructor_args().cmp(&other.constructor_args()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyCreateContractArgsV2 {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len =
                <LazyContractIdPreimage as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len =
                <LazyContractExecutable as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
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
        pos += <LazyContractIdPreimage as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyContractExecutable as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyScVal> as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyCreateContractArgsV2 {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyCreateContractArgsV2 {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyCreateContractArgsV2 {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyCreateContractArgsV2 {
    /// Access field `contract_id_preimage`.
    #[must_use]
    pub fn contract_id_preimage(&self) -> LazyContractIdPreimage {
        <LazyContractIdPreimage as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `executable`.
    #[must_use]
    pub fn executable(&self) -> LazyContractExecutable {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyContractIdPreimage as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyContractExecutable as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `constructor_args`.
    #[must_use]
    pub fn constructor_args(&self) -> LazyVecM<LazyScVal> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyContractIdPreimage as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyContractExecutable as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyVecM<LazyScVal> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&CreateContractArgsV2> for LazyCreateContractArgsV2 {
    type Error = Error;
    fn try_from(val: &CreateContractArgsV2) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyCreateContractArgsV2> for CreateContractArgsV2 {
    type Error = Error;
    fn try_from(lazy: &LazyCreateContractArgsV2) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
