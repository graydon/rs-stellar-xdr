#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// SorobanResources is an XDR Struct defined as:
///
/// ```text
/// struct SorobanResources
/// {   
///     // The ledger footprint of the transaction.
///     LedgerFootprint footprint;
///     // The maximum number of instructions this transaction can use
///     uint32 instructions;
///
///     // The maximum number of bytes this transaction can read from disk backed entries
///     uint32 diskReadBytes;
///     // The maximum number of bytes this transaction can write to ledger
///     uint32 writeBytes;
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
pub struct SorobanResources {
    pub footprint: LedgerFootprint,
    pub instructions: u32,
    pub disk_read_bytes: u32,
    pub write_bytes: u32,
}

impl ReadXdr for SorobanResources {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                footprint: LedgerFootprint::read_xdr(r)?,
                instructions: u32::read_xdr(r)?,
                disk_read_bytes: u32::read_xdr(r)?,
                write_bytes: u32::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SorobanResources {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.footprint.write_xdr(w)?;
            self.instructions.write_xdr(w)?;
            self.disk_read_bytes.write_xdr(w)?;
            self.write_bytes.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`SorobanResources`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazySorobanResources(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazySorobanResources {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazySorobanResources {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.footprint().cmp(&other.footprint()))
            .then_with(|| self.instructions().cmp(&other.instructions()))
            .then_with(|| self.disk_read_bytes().cmp(&other.disk_read_bytes()))
            .then_with(|| self.write_bytes().cmp(&other.write_bytes()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazySorobanResources {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len =
                <LazyLedgerFootprint as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        let next_pos = pos.checked_add(12).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        pos = next_pos;
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyLedgerFootprint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 12;
        pos
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazySorobanResources {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazySorobanResources {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazySorobanResources {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazySorobanResources {
    /// Access field `footprint`.
    #[must_use]
    pub fn footprint(&self) -> LazyLedgerFootprint {
        <LazyLedgerFootprint as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `instructions`.
    #[must_use]
    pub fn instructions(&self) -> u32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyLedgerFootprint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <u32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `disk_read_bytes`.
    #[must_use]
    pub fn disk_read_bytes(&self) -> u32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyLedgerFootprint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 4;
        <u32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `write_bytes`.
    #[must_use]
    pub fn write_bytes(&self) -> u32 {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyLedgerFootprint as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += 8;
        <u32 as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&SorobanResources> for LazySorobanResources {
    type Error = Error;
    fn try_from(val: &SorobanResources) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazySorobanResources> for SorobanResources {
    type Error = Error;
    fn try_from(lazy: &LazySorobanResources) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
