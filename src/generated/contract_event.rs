#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ContractEvent is an XDR Struct defined as:
///
/// ```text
/// struct ContractEvent
/// {
///     // We can use this to add more fields, or because it
///     // is first, to change ContractEvent into a union.
///     ExtensionPoint ext;
///
///     ContractID* contractID;
///     ContractEventType type;
///
///     union switch (int v)
///     {
///     case 0:
///         struct
///         {
///             SCVal topics<>;
///             SCVal data;
///         } v0;
///     }
///     body;
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
pub struct ContractEvent {
    pub ext: ExtensionPoint,
    pub contract_id: Option<ContractId>,
    pub type_: ContractEventType,
    pub body: ContractEventBody,
}

impl ReadXdr for ContractEvent {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ext: ExtensionPoint::read_xdr(r)?,
                contract_id: Option::<ContractId>::read_xdr(r)?,
                type_: ContractEventType::read_xdr(r)?,
                body: ContractEventBody::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ContractEvent {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.contract_id.write_xdr(w)?;
            self.type_.write_xdr(w)?;
            self.body.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ContractEvent`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyContractEvent(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyContractEvent {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyContractEvent {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.ext().cmp(&other.ext()))
            .then_with(|| self.contract_id().cmp(&other.contract_id()))
            .then_with(|| self.type_().cmp(&other.type_()))
            .then_with(|| self.body().cmp(&other.body()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyContractEvent {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len =
                <LazyExtensionPoint as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len =
                <LazyOption<LazyContractId> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        let next_pos = pos.checked_add(4).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        <ContractEventType as LazyXdr>::xdr_validate(&buf[(pos + 0) as usize..], depth)?;
        pos = next_pos;
        {
            let field_len =
                <LazyContractEventBody as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyExtensionPoint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<LazyContractId> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 4;
        pos += <LazyContractEventBody as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyContractEvent {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyContractEvent {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyContractEvent {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyContractEvent {
    /// Access field `ext`.
    #[must_use]
    pub fn ext(&self) -> LazyExtensionPoint {
        <LazyExtensionPoint as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `contract_id`.
    #[must_use]
    pub fn contract_id(&self) -> LazyOption<LazyContractId> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyExtensionPoint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyOption<LazyContractId> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `type_`.
    #[must_use]
    pub fn type_(&self) -> ContractEventType {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyExtensionPoint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<LazyContractId> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <ContractEventType as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `body`.
    #[must_use]
    pub fn body(&self) -> LazyContractEventBody {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyExtensionPoint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<LazyContractId> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 4;
        <LazyContractEventBody as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ContractEvent> for LazyContractEvent {
    type Error = Error;
    fn try_from(val: &ContractEvent) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyContractEvent> for ContractEvent {
    type Error = Error;
    fn try_from(lazy: &LazyContractEvent) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
