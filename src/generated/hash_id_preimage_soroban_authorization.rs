#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// HashIdPreimageSorobanAuthorization is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///     {
///         Hash networkID;
///         int64 nonce;
///         uint32 signatureExpirationLedger;
///         SorobanAuthorizedInvocation invocation;
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
pub struct HashIdPreimageSorobanAuthorization {
    pub network_id: Hash,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub nonce: i64,
    pub signature_expiration_ledger: u32,
    pub invocation: SorobanAuthorizedInvocation,
}

impl ReadXdr for HashIdPreimageSorobanAuthorization {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                network_id: Hash::read_xdr(r)?,
                nonce: i64::read_xdr(r)?,
                signature_expiration_ledger: u32::read_xdr(r)?,
                invocation: SorobanAuthorizedInvocation::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for HashIdPreimageSorobanAuthorization {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.network_id.write_xdr(w)?;
            self.nonce.write_xdr(w)?;
            self.signature_expiration_ledger.write_xdr(w)?;
            self.invocation.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`HashIdPreimageSorobanAuthorization`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyHashIdPreimageSorobanAuthorization(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyHashIdPreimageSorobanAuthorization {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyHashIdPreimageSorobanAuthorization {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.network_id().cmp(&other.network_id()))
            .then_with(|| self.nonce().cmp(&other.nonce()))
            .then_with(|| {
                self.signature_expiration_ledger()
                    .cmp(&other.signature_expiration_ledger())
            })
            .then_with(|| self.invocation().cmp(&other.invocation()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyHashIdPreimageSorobanAuthorization {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        let next_pos = pos.checked_add(44).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        <LazyHash as LazyXdr>::xdr_validate(&buf[(pos + 0) as usize..], depth)?;
        pos = next_pos;
        {
            let field_len = <LazySorobanAuthorizedInvocation as LazyXdr>::xdr_validate(
                &buf[pos as usize..],
                depth,
            )?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += 44;
        pos += <LazySorobanAuthorizedInvocation as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyHashIdPreimageSorobanAuthorization {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyHashIdPreimageSorobanAuthorization {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyHashIdPreimageSorobanAuthorization {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyHashIdPreimageSorobanAuthorization {
    /// Access field `network_id`.
    #[must_use]
    pub fn network_id(&self) -> LazyHash {
        <LazyHash as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `nonce`.
    #[must_use]
    pub fn nonce(&self) -> i64 {
        <i64 as LazyXdr>::from_xdr_at(&self.0, 32)
    }
    /// Access field `signature_expiration_ledger`.
    #[must_use]
    pub fn signature_expiration_ledger(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 40)
    }
    /// Access field `invocation`.
    #[must_use]
    pub fn invocation(&self) -> LazySorobanAuthorizedInvocation {
        <LazySorobanAuthorizedInvocation as LazyXdr>::from_xdr_at(&self.0, 44)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&HashIdPreimageSorobanAuthorization> for LazyHashIdPreimageSorobanAuthorization {
    type Error = Error;
    fn try_from(val: &HashIdPreimageSorobanAuthorization) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyHashIdPreimageSorobanAuthorization> for HashIdPreimageSorobanAuthorization {
    type Error = Error;
    fn try_from(lazy: &LazyHashIdPreimageSorobanAuthorization) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
