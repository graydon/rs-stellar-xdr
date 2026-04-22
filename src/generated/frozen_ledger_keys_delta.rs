#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// FrozenLedgerKeysDelta is an XDR Struct defined as:
///
/// ```text
/// struct FrozenLedgerKeysDelta {
///     EncodedLedgerKey keysToFreeze<>;
///     EncodedLedgerKey keysToUnfreeze<>;
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
pub struct FrozenLedgerKeysDelta {
    pub keys_to_freeze: VecM<EncodedLedgerKey>,
    pub keys_to_unfreeze: VecM<EncodedLedgerKey>,
}

impl ReadXdr for FrozenLedgerKeysDelta {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                keys_to_freeze: VecM::<EncodedLedgerKey>::read_xdr(r)?,
                keys_to_unfreeze: VecM::<EncodedLedgerKey>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for FrozenLedgerKeysDelta {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.keys_to_freeze.write_xdr(w)?;
            self.keys_to_unfreeze.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`FrozenLedgerKeysDelta`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyFrozenLedgerKeysDelta(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyFrozenLedgerKeysDelta {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyFrozenLedgerKeysDelta {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.keys_to_freeze().cmp(&other.keys_to_freeze()))
            .then_with(|| self.keys_to_unfreeze().cmp(&other.keys_to_unfreeze()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyFrozenLedgerKeysDelta {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len = <LazyVecM<LazyEncodedLedgerKey> as LazyXdr>::xdr_validate(
                &buf[pos as usize..],
                depth,
            )?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len = <LazyVecM<LazyEncodedLedgerKey> as LazyXdr>::xdr_validate(
                &buf[pos as usize..],
                depth,
            )?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyVecM<LazyEncodedLedgerKey> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyEncodedLedgerKey> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyFrozenLedgerKeysDelta {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyFrozenLedgerKeysDelta {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyFrozenLedgerKeysDelta {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyFrozenLedgerKeysDelta {
    /// Access field `keys_to_freeze`.
    #[must_use]
    pub fn keys_to_freeze(&self) -> LazyVecM<LazyEncodedLedgerKey> {
        <LazyVecM<LazyEncodedLedgerKey> as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `keys_to_unfreeze`.
    #[must_use]
    pub fn keys_to_unfreeze(&self) -> LazyVecM<LazyEncodedLedgerKey> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyVecM<LazyEncodedLedgerKey> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyVecM<LazyEncodedLedgerKey> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&FrozenLedgerKeysDelta> for LazyFrozenLedgerKeysDelta {
    type Error = Error;
    fn try_from(val: &FrozenLedgerKeysDelta) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyFrozenLedgerKeysDelta> for FrozenLedgerKeysDelta {
    type Error = Error;
    fn try_from(lazy: &LazyFrozenLedgerKeysDelta) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
