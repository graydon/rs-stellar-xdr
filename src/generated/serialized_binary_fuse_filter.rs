#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// SerializedBinaryFuseFilter is an XDR Struct defined as:
///
/// ```text
/// struct SerializedBinaryFuseFilter
/// {
///     BinaryFuseFilterType type;
///
///     // Seed used to hash input to filter
///     ShortHashSeed inputHashSeed;
///
///     // Seed used for internal filter hash operations
///     ShortHashSeed filterSeed;
///     uint32 segmentLength;
///     uint32 segementLengthMask;
///     uint32 segmentCount;
///     uint32 segmentCountLength;
///     uint32 fingerprintLength; // Length in terms of element count, not bytes
///
///     // Array of uint8_t, uint16_t, or uint32_t depending on filter type
///     opaque fingerprints<>;
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
pub struct SerializedBinaryFuseFilter {
    pub type_: BinaryFuseFilterType,
    pub input_hash_seed: ShortHashSeed,
    pub filter_seed: ShortHashSeed,
    pub segment_length: u32,
    pub segement_length_mask: u32,
    pub segment_count: u32,
    pub segment_count_length: u32,
    pub fingerprint_length: u32,
    pub fingerprints: BytesM,
}

impl ReadXdr for SerializedBinaryFuseFilter {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                type_: BinaryFuseFilterType::read_xdr(r)?,
                input_hash_seed: ShortHashSeed::read_xdr(r)?,
                filter_seed: ShortHashSeed::read_xdr(r)?,
                segment_length: u32::read_xdr(r)?,
                segement_length_mask: u32::read_xdr(r)?,
                segment_count: u32::read_xdr(r)?,
                segment_count_length: u32::read_xdr(r)?,
                fingerprint_length: u32::read_xdr(r)?,
                fingerprints: BytesM::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SerializedBinaryFuseFilter {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.type_.write_xdr(w)?;
            self.input_hash_seed.write_xdr(w)?;
            self.filter_seed.write_xdr(w)?;
            self.segment_length.write_xdr(w)?;
            self.segement_length_mask.write_xdr(w)?;
            self.segment_count.write_xdr(w)?;
            self.segment_count_length.write_xdr(w)?;
            self.fingerprint_length.write_xdr(w)?;
            self.fingerprints.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`SerializedBinaryFuseFilter`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazySerializedBinaryFuseFilter(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazySerializedBinaryFuseFilter {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazySerializedBinaryFuseFilter {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.type_().cmp(&other.type_()))
            .then_with(|| self.input_hash_seed().cmp(&other.input_hash_seed()))
            .then_with(|| self.filter_seed().cmp(&other.filter_seed()))
            .then_with(|| self.segment_length().cmp(&other.segment_length()))
            .then_with(|| {
                self.segement_length_mask()
                    .cmp(&other.segement_length_mask())
            })
            .then_with(|| self.segment_count().cmp(&other.segment_count()))
            .then_with(|| {
                self.segment_count_length()
                    .cmp(&other.segment_count_length())
            })
            .then_with(|| self.fingerprint_length().cmp(&other.fingerprint_length()))
            .then_with(|| self.fingerprints().cmp(&other.fingerprints()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazySerializedBinaryFuseFilter {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        let next_pos = pos.checked_add(56).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        <BinaryFuseFilterType as LazyXdr>::xdr_validate(&buf[(pos + 0) as usize..], depth)?;
        <LazyShortHashSeed as LazyXdr>::xdr_validate(&buf[(pos + 4) as usize..], depth)?;
        <LazyShortHashSeed as LazyXdr>::xdr_validate(&buf[(pos + 20) as usize..], depth)?;
        pos = next_pos;
        {
            let field_len = <LazyBytesM as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += 56;
        pos += <LazyBytesM as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazySerializedBinaryFuseFilter {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazySerializedBinaryFuseFilter {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazySerializedBinaryFuseFilter {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazySerializedBinaryFuseFilter {
    /// Access field `type_`.
    #[must_use]
    pub fn type_(&self) -> BinaryFuseFilterType {
        <BinaryFuseFilterType as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `input_hash_seed`.
    #[must_use]
    pub fn input_hash_seed(&self) -> LazyShortHashSeed {
        <LazyShortHashSeed as LazyXdr>::from_xdr_at(&self.0, 4)
    }
    /// Access field `filter_seed`.
    #[must_use]
    pub fn filter_seed(&self) -> LazyShortHashSeed {
        <LazyShortHashSeed as LazyXdr>::from_xdr_at(&self.0, 20)
    }
    /// Access field `segment_length`.
    #[must_use]
    pub fn segment_length(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 36)
    }
    /// Access field `segement_length_mask`.
    #[must_use]
    pub fn segement_length_mask(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 40)
    }
    /// Access field `segment_count`.
    #[must_use]
    pub fn segment_count(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 44)
    }
    /// Access field `segment_count_length`.
    #[must_use]
    pub fn segment_count_length(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 48)
    }
    /// Access field `fingerprint_length`.
    #[must_use]
    pub fn fingerprint_length(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 52)
    }
    /// Access field `fingerprints`.
    #[must_use]
    pub fn fingerprints(&self) -> LazyBytesM {
        <LazyBytesM as LazyXdr>::from_xdr_at(&self.0, 56)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&SerializedBinaryFuseFilter> for LazySerializedBinaryFuseFilter {
    type Error = Error;
    fn try_from(val: &SerializedBinaryFuseFilter) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazySerializedBinaryFuseFilter> for SerializedBinaryFuseFilter {
    type Error = Error;
    fn try_from(lazy: &LazySerializedBinaryFuseFilter) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
