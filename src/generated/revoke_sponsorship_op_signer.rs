#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// RevokeSponsorshipOpSigner is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///     {
///         AccountID accountID;
///         SignerKey signerKey;
///     }
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
pub struct RevokeSponsorshipOpSigner {
    pub account_id: AccountId,
    pub signer_key: SignerKey,
}

impl ReadXdr for RevokeSponsorshipOpSigner {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                account_id: AccountId::read_xdr(r)?,
                signer_key: SignerKey::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for RevokeSponsorshipOpSigner {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.account_id.write_xdr(w)?;
            self.signer_key.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`RevokeSponsorshipOpSigner`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyRevokeSponsorshipOpSigner(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyRevokeSponsorshipOpSigner {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyRevokeSponsorshipOpSigner {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.account_id().cmp(&other.account_id()))
            .then_with(|| self.signer_key().cmp(&other.signer_key()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyRevokeSponsorshipOpSigner {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len = <LazyAccountId as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len = <LazySignerKey as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyAccountId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazySignerKey as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyRevokeSponsorshipOpSigner {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyRevokeSponsorshipOpSigner {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyRevokeSponsorshipOpSigner {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyRevokeSponsorshipOpSigner {
    /// Access field `account_id`.
    #[must_use]
    pub fn account_id(&self) -> LazyAccountId {
        <LazyAccountId as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `signer_key`.
    #[must_use]
    pub fn signer_key(&self) -> LazySignerKey {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyAccountId as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazySignerKey as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&RevokeSponsorshipOpSigner> for LazyRevokeSponsorshipOpSigner {
    type Error = Error;
    fn try_from(val: &RevokeSponsorshipOpSigner) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyRevokeSponsorshipOpSigner> for RevokeSponsorshipOpSigner {
    type Error = Error;
    fn try_from(lazy: &LazyRevokeSponsorshipOpSigner) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
