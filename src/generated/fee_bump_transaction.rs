#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// FeeBumpTransaction is an XDR Struct defined as:
///
/// ```text
/// struct FeeBumpTransaction
/// {
///     MuxedAccount feeSource;
///     int64 fee;
///     union switch (EnvelopeType type)
///     {
///     case ENVELOPE_TYPE_TX:
///         TransactionV1Envelope v1;
///     }
///     innerTx;
///     union switch (int v)
///     {
///     case 0:
///         void;
///     }
///     ext;
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
pub struct FeeBumpTransaction {
    pub fee_source: MuxedAccount,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub fee: i64,
    pub inner_tx: FeeBumpTransactionInnerTx,
    pub ext: FeeBumpTransactionExt,
}

impl ReadXdr for FeeBumpTransaction {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                fee_source: MuxedAccount::read_xdr(r)?,
                fee: i64::read_xdr(r)?,
                inner_tx: FeeBumpTransactionInnerTx::read_xdr(r)?,
                ext: FeeBumpTransactionExt::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for FeeBumpTransaction {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.fee_source.write_xdr(w)?;
            self.fee.write_xdr(w)?;
            self.inner_tx.write_xdr(w)?;
            self.ext.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`FeeBumpTransaction`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyFeeBumpTransaction(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyFeeBumpTransaction {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyFeeBumpTransaction {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.fee_source().cmp(&other.fee_source()))
            .then_with(|| self.fee().cmp(&other.fee()))
            .then_with(|| self.inner_tx().cmp(&other.inner_tx()))
            .then_with(|| self.ext().cmp(&other.ext()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyFeeBumpTransaction {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len =
                <LazyMuxedAccount as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        let next_pos = pos.checked_add(8).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        pos = next_pos;
        {
            let field_len = <LazyFeeBumpTransactionInnerTx as LazyXdr>::xdr_validate(
                &buf[pos as usize..],
                depth,
            )?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len =
                <LazyFeeBumpTransactionExt as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyMuxedAccount as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
        pos += <LazyFeeBumpTransactionInnerTx as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyFeeBumpTransactionExt as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyFeeBumpTransaction {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyFeeBumpTransaction {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyFeeBumpTransaction {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyFeeBumpTransaction {
    /// Access field `fee_source`.
    #[must_use]
    pub fn fee_source(&self) -> LazyMuxedAccount {
        <LazyMuxedAccount as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `fee`.
    #[must_use]
    pub fn fee(&self) -> i64 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyMuxedAccount as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <i64 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `inner_tx`.
    #[must_use]
    pub fn inner_tx(&self) -> LazyFeeBumpTransactionInnerTx {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyMuxedAccount as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
        <LazyFeeBumpTransactionInnerTx as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `ext`.
    #[must_use]
    pub fn ext(&self) -> LazyFeeBumpTransactionExt {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyMuxedAccount as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
        pos += <LazyFeeBumpTransactionInnerTx as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyFeeBumpTransactionExt as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&FeeBumpTransaction> for LazyFeeBumpTransaction {
    type Error = Error;
    fn try_from(val: &FeeBumpTransaction) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyFeeBumpTransaction> for FeeBumpTransaction {
    type Error = Error;
    fn try_from(lazy: &LazyFeeBumpTransaction) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
