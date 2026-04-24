#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// SorobanTransactionMeta is an XDR Struct defined as:
///
/// ```text
/// struct SorobanTransactionMeta
/// {
///     SorobanTransactionMetaExt ext;
///
///     ContractEvent events<>;             // custom events populated by the
///                                         // contracts themselves.
///     SCVal returnValue;                  // return value of the host fn invocation
///
///     // Diagnostics events that are not hashed.
///     // This will contain all contract and diagnostic events. Even ones
///     // that were emitted in a failed contract call.
///     DiagnosticEvent diagnosticEvents<>;
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
pub struct SorobanTransactionMeta {
    pub ext: SorobanTransactionMetaExt,
    pub events: VecM<ContractEvent>,
    pub return_value: ScVal,
    pub diagnostic_events: VecM<DiagnosticEvent>,
}

impl ReadXdr for SorobanTransactionMeta {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ext: SorobanTransactionMetaExt::read_xdr(r)?,
                events: VecM::<ContractEvent>::read_xdr(r)?,
                return_value: ScVal::read_xdr(r)?,
                diagnostic_events: VecM::<DiagnosticEvent>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SorobanTransactionMeta {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.events.write_xdr(w)?;
            self.return_value.write_xdr(w)?;
            self.diagnostic_events.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`SorobanTransactionMeta`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazySorobanTransactionMeta(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazySorobanTransactionMeta {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazySorobanTransactionMeta {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.ext().cmp(&other.ext()))
            .then_with(|| self.events().cmp(&other.events()))
            .then_with(|| self.return_value().cmp(&other.return_value()))
            .then_with(|| self.diagnostic_events().cmp(&other.diagnostic_events()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazySorobanTransactionMeta {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len = <LazySorobanTransactionMetaExt as LazyXdr>::xdr_validate(
                &buf[pos as usize..],
                depth,
            )?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len = <LazyVecM<LazyContractEvent> as LazyXdr>::xdr_validate(
                &buf[pos as usize..],
                depth,
            )?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len = <LazyScVal as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len = <LazyVecM<LazyDiagnosticEvent> as LazyXdr>::xdr_validate(
                &buf[pos as usize..],
                depth,
            )?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazySorobanTransactionMetaExt as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyContractEvent> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyScVal as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyDiagnosticEvent> as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazySorobanTransactionMeta {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazySorobanTransactionMeta {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazySorobanTransactionMeta {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazySorobanTransactionMeta {
    /// Access field `ext`.
    #[must_use]
    pub fn ext(&self) -> LazySorobanTransactionMetaExt {
        <LazySorobanTransactionMetaExt as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `events`.
    #[must_use]
    pub fn events(&self) -> LazyVecM<LazyContractEvent> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazySorobanTransactionMetaExt as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyVecM<LazyContractEvent> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `return_value`.
    #[must_use]
    pub fn return_value(&self) -> LazyScVal {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazySorobanTransactionMetaExt as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyContractEvent> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyScVal as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `diagnostic_events`.
    #[must_use]
    pub fn diagnostic_events(&self) -> LazyVecM<LazyDiagnosticEvent> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazySorobanTransactionMetaExt as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyContractEvent> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyScVal as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyVecM<LazyDiagnosticEvent> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&SorobanTransactionMeta> for LazySorobanTransactionMeta {
    type Error = Error;
    fn try_from(val: &SorobanTransactionMeta) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazySorobanTransactionMeta> for SorobanTransactionMeta {
    type Error = Error;
    fn try_from(lazy: &LazySorobanTransactionMeta) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
