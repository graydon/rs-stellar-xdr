#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// TransactionSignaturePayload is an XDR Struct defined as:
///
/// ```text
/// struct TransactionSignaturePayload
/// {
///     Hash networkId;
///     union switch (EnvelopeType type)
///     {
///     // Backwards Compatibility: Use ENVELOPE_TYPE_TX to sign ENVELOPE_TYPE_TX_V0
///     case ENVELOPE_TYPE_TX:
///         Transaction tx;
///     case ENVELOPE_TYPE_TX_FEE_BUMP:
///         FeeBumpTransaction feeBump;
///     }
///     taggedTransaction;
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
pub struct TransactionSignaturePayload {
    pub network_id: Hash,
    pub tagged_transaction: TransactionSignaturePayloadTaggedTransaction,
}

impl ReadXdr for TransactionSignaturePayload {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                network_id: Hash::read_xdr(r)?,
                tagged_transaction: TransactionSignaturePayloadTaggedTransaction::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for TransactionSignaturePayload {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.network_id.write_xdr(w)?;
            self.tagged_transaction.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`TransactionSignaturePayload`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyTransactionSignaturePayload(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyTransactionSignaturePayload {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyTransactionSignaturePayload {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.network_id().cmp(&other.network_id()))
            .then_with(|| self.tagged_transaction().cmp(&other.tagged_transaction()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyTransactionSignaturePayload {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        let next_pos = pos.checked_add(32).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        <LazyHash as LazyXdr>::xdr_validate(&buf[(pos + 0) as usize..], depth)?;
        pos = next_pos;
        {
            let field_len =
                <LazyTransactionSignaturePayloadTaggedTransaction as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += 32;
        pos += <LazyTransactionSignaturePayloadTaggedTransaction as LazyXdr>::xdr_len(
            &buf[pos as usize..],
        );
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
impl From<LazyHandle> for LazyTransactionSignaturePayload {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyTransactionSignaturePayload {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyTransactionSignaturePayload {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyTransactionSignaturePayload {
    /// Access field `network_id`.
    #[must_use]
    pub fn network_id(&self) -> LazyHash {
        <LazyHash as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `tagged_transaction`.
    #[must_use]
    pub fn tagged_transaction(&self) -> LazyTransactionSignaturePayloadTaggedTransaction {
        <LazyTransactionSignaturePayloadTaggedTransaction as LazyXdr>::from_xdr_at(&self.0, 32)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&TransactionSignaturePayload> for LazyTransactionSignaturePayload {
    type Error = Error;
    fn try_from(val: &TransactionSignaturePayload) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyTransactionSignaturePayload> for TransactionSignaturePayload {
    type Error = Error;
    fn try_from(lazy: &LazyTransactionSignaturePayload) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
