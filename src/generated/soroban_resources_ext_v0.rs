#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// SorobanResourcesExtV0 is an XDR Struct defined as:
///
/// ```text
/// struct SorobanResourcesExtV0
/// {
///     // Vector of indices representing what Soroban
///     // entries in the footprint are archived, based on the
///     // order of keys provided in the readWrite footprint.
///     uint32 archivedSorobanEntries<>;
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
pub struct SorobanResourcesExtV0 {
    pub archived_soroban_entries: VecM<u32>,
}

impl ReadXdr for SorobanResourcesExtV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                archived_soroban_entries: VecM::<u32>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SorobanResourcesExtV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.archived_soroban_entries.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`SorobanResourcesExtV0`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazySorobanResourcesExtV0(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazySorobanResourcesExtV0 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazySorobanResourcesExtV0 {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal.then_with(|| {
            self.archived_soroban_entries()
                .cmp(&other.archived_soroban_entries())
        })
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazySorobanResourcesExtV0 {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len = <LazyVecM<u32> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyVecM<u32> as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazySorobanResourcesExtV0 {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazySorobanResourcesExtV0 {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazySorobanResourcesExtV0 {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazySorobanResourcesExtV0 {
    /// Access field `archived_soroban_entries`.
    #[must_use]
    pub fn archived_soroban_entries(&self) -> LazyVecM<u32> {
        <LazyVecM<u32> as LazyXdr>::from_xdr_at(&self.0, 0)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&SorobanResourcesExtV0> for LazySorobanResourcesExtV0 {
    type Error = Error;
    fn try_from(val: &SorobanResourcesExtV0) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazySorobanResourcesExtV0> for SorobanResourcesExtV0 {
    type Error = Error;
    fn try_from(lazy: &LazySorobanResourcesExtV0) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
