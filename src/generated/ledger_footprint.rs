#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// LedgerFootprint is an XDR Struct defined as:
///
/// ```text
/// struct LedgerFootprint
/// {
///     LedgerKey readOnly<>;
///     LedgerKey readWrite<>;
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
pub struct LedgerFootprint {
    pub read_only: VecM<LedgerKey>,
    pub read_write: VecM<LedgerKey>,
}

impl ReadXdr for LedgerFootprint {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                read_only: VecM::<LedgerKey>::read_xdr(r)?,
                read_write: VecM::<LedgerKey>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for LedgerFootprint {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.read_only.write_xdr(w)?;
            self.read_write.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`LedgerFootprint`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyLedgerFootprint(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyLedgerFootprint {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyLedgerFootprint {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.read_only().cmp(&other.read_only()))
            .then_with(|| self.read_write().cmp(&other.read_write()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyLedgerFootprint {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len =
                <LazyVecM<LazyLedgerKey> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len =
                <LazyVecM<LazyLedgerKey> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyVecM<LazyLedgerKey> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyLedgerKey> as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyLedgerFootprint {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyLedgerFootprint {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyLedgerFootprint {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyLedgerFootprint {
    /// Access field `read_only`.
    #[must_use]
    pub fn read_only(&self) -> LazyVecM<LazyLedgerKey> {
        <LazyVecM<LazyLedgerKey> as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `read_write`.
    #[must_use]
    pub fn read_write(&self) -> LazyVecM<LazyLedgerKey> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyVecM<LazyLedgerKey> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyVecM<LazyLedgerKey> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LedgerFootprint> for LazyLedgerFootprint {
    type Error = Error;
    fn try_from(val: &LedgerFootprint) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyLedgerFootprint> for LedgerFootprint {
    type Error = Error;
    fn try_from(lazy: &LazyLedgerFootprint) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
