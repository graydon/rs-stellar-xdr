#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// FeeBumpTransactionEnvelope is an XDR Struct defined as:
///
/// ```text
/// struct FeeBumpTransactionEnvelope
/// {
///     FeeBumpTransaction tx;
///     /* Each decorated signature is a signature over the SHA256 hash of
///      * a TransactionSignaturePayload */
///     DecoratedSignature signatures<20>;
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
pub struct FeeBumpTransactionEnvelope {
    pub tx: FeeBumpTransaction,
    pub signatures: VecM<DecoratedSignature, 20>,
}

impl ReadXdr for FeeBumpTransactionEnvelope {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                tx: FeeBumpTransaction::read_xdr(r)?,
                signatures: VecM::<DecoratedSignature, 20>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for FeeBumpTransactionEnvelope {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.tx.write_xdr(w)?;
            self.signatures.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`FeeBumpTransactionEnvelope`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyFeeBumpTransactionEnvelope(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyFeeBumpTransactionEnvelope {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyFeeBumpTransactionEnvelope {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.tx().cmp(&other.tx()))
            .then_with(|| self.signatures().cmp(&other.signatures()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyFeeBumpTransactionEnvelope {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len =
                <LazyFeeBumpTransaction as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len = <LazyVecM<LazyDecoratedSignature, 20> as LazyXdr>::xdr_validate(
                &buf[pos as usize..],
                depth,
            )?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyFeeBumpTransaction as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyDecoratedSignature, 20> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyFeeBumpTransactionEnvelope {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyFeeBumpTransactionEnvelope {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyFeeBumpTransactionEnvelope {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyFeeBumpTransactionEnvelope {
    /// Access field `tx`.
    #[must_use]
    pub fn tx(&self) -> LazyFeeBumpTransaction {
        <LazyFeeBumpTransaction as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `signatures`.
    #[must_use]
    pub fn signatures(&self) -> LazyVecM<LazyDecoratedSignature, 20> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyFeeBumpTransaction as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyVecM<LazyDecoratedSignature, 20> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&FeeBumpTransactionEnvelope> for LazyFeeBumpTransactionEnvelope {
    type Error = Error;
    fn try_from(val: &FeeBumpTransactionEnvelope) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyFeeBumpTransactionEnvelope> for FeeBumpTransactionEnvelope {
    type Error = Error;
    fn try_from(lazy: &LazyFeeBumpTransactionEnvelope) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
