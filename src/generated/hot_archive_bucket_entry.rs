#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// HotArchiveBucketEntry is an XDR Union defined as:
///
/// ```text
/// union HotArchiveBucketEntry switch (HotArchiveBucketEntryType type)
/// {
/// case HOT_ARCHIVE_ARCHIVED:
///     LedgerEntry archivedEntry;
///
/// case HOT_ARCHIVE_LIVE:
///     LedgerKey key;
/// case HOT_ARCHIVE_METAENTRY:
///     BucketMetadata metaEntry;
/// };
/// ```
///
// union with discriminant HotArchiveBucketEntryType
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[allow(clippy::large_enum_variant)]
pub enum HotArchiveBucketEntry {
    Archived(LedgerEntry),
    Live(LedgerKey),
    Metaentry(BucketMetadata),
}

#[cfg(feature = "alloc")]
impl Default for HotArchiveBucketEntry {
    fn default() -> Self {
        Self::Archived(LedgerEntry::default())
    }
}

impl HotArchiveBucketEntry {
    const _VARIANTS: &[HotArchiveBucketEntryType] = &[
        HotArchiveBucketEntryType::Archived,
        HotArchiveBucketEntryType::Live,
        HotArchiveBucketEntryType::Metaentry,
    ];
    pub const VARIANTS: [HotArchiveBucketEntryType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["Archived", "Live", "Metaentry"];
    pub const VARIANTS_STR: [&'static str; Self::_VARIANTS_STR.len()] = {
        let mut arr = [Self::_VARIANTS_STR[0]; Self::_VARIANTS_STR.len()];
        let mut i = 1;
        while i < Self::_VARIANTS_STR.len() {
            arr[i] = Self::_VARIANTS_STR[i];
            i += 1;
        }
        arr
    };

    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Archived(_) => "Archived",
            Self::Live(_) => "Live",
            Self::Metaentry(_) => "Metaentry",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> HotArchiveBucketEntryType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Archived(_) => HotArchiveBucketEntryType::Archived,
            Self::Live(_) => HotArchiveBucketEntryType::Live,
            Self::Metaentry(_) => HotArchiveBucketEntryType::Metaentry,
        }
    }

    #[must_use]
    pub const fn variants() -> [HotArchiveBucketEntryType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for HotArchiveBucketEntry {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<HotArchiveBucketEntryType> for HotArchiveBucketEntry {
    #[must_use]
    fn discriminant(&self) -> HotArchiveBucketEntryType {
        Self::discriminant(self)
    }
}

impl Variants<HotArchiveBucketEntryType> for HotArchiveBucketEntry {
    fn variants() -> slice::Iter<'static, HotArchiveBucketEntryType> {
        Self::VARIANTS.iter()
    }
}

impl Union<HotArchiveBucketEntryType> for HotArchiveBucketEntry {}

impl ReadXdr for HotArchiveBucketEntry {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: HotArchiveBucketEntryType =
                <HotArchiveBucketEntryType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                HotArchiveBucketEntryType::Archived => Self::Archived(LedgerEntry::read_xdr(r)?),
                HotArchiveBucketEntryType::Live => Self::Live(LedgerKey::read_xdr(r)?),
                HotArchiveBucketEntryType::Metaentry => {
                    Self::Metaentry(BucketMetadata::read_xdr(r)?)
                }
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for HotArchiveBucketEntry {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Archived(v) => v.write_xdr(w)?,
                Self::Live(v) => v.write_xdr(w)?,
                Self::Metaentry(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`HotArchiveBucketEntry`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyHotArchiveBucketEntry(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyHotArchiveBucketEntry {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyHotArchiveBucketEntry {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let ord = self.discriminant().cmp(&other.discriminant());
        if ord != core::cmp::Ordering::Equal {
            return ord;
        }
        #[allow(clippy::match_same_arms)]
        match self.discriminant_i32() {
            0 => self.as_archived().cmp(&other.as_archived()),
            1 => self.as_live().cmp(&other.as_live()),
            -1 => self.as_metaentry().cmp(&other.as_metaentry()),
            _ => core::cmp::Ordering::Equal,
        }
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyHotArchiveBucketEntry {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        if buf.len() < 4 {
            return Err(Error::Invalid);
        }
        let bytes: [u8; 4] = buf[..4].try_into().unwrap();
        let disc = i32::from_be_bytes(bytes);
        #[allow(unused_mut)]
        let mut pos: u32 = 4;
        #[allow(clippy::match_same_arms)]
        match disc {
            0 => {
                let field_len =
                    <LazyLedgerEntry as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            1 => {
                let field_len =
                    <LazyLedgerKey as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            -1 => {
                let field_len =
                    <LazyBucketMetadata as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            _ => return Err(Error::Invalid),
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let bytes: [u8; 4] = buf[..4].try_into().unwrap();
        let disc = i32::from_be_bytes(bytes);
        #[allow(unused_mut)]
        let mut pos: u32 = 4;
        #[allow(clippy::match_same_arms)]
        match disc {
            0 => {
                pos += <LazyLedgerEntry as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            1 => {
                pos += <LazyLedgerKey as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            -1 => {
                pos += <LazyBucketMetadata as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            _ => {}
        }
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
impl From<LazyHandle> for LazyHotArchiveBucketEntry {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyHotArchiveBucketEntry {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyHotArchiveBucketEntry {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyHotArchiveBucketEntry {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> HotArchiveBucketEntryType {
        // Validated — unwrap is safe.
        HotArchiveBucketEntryType::try_from(self.discriminant_i32()).unwrap()
    }
    /// Access arm `Archived`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_archived(&self) -> Option<LazyLedgerEntry> {
        if self.discriminant_i32() == 0 {
            Some(<LazyLedgerEntry as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Live`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_live(&self) -> Option<LazyLedgerKey> {
        if self.discriminant_i32() == 1 {
            Some(<LazyLedgerKey as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Metaentry`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_metaentry(&self) -> Option<LazyBucketMetadata> {
        if self.discriminant_i32() == -1 {
            Some(<LazyBucketMetadata as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&HotArchiveBucketEntry> for LazyHotArchiveBucketEntry {
    type Error = Error;
    fn try_from(val: &HotArchiveBucketEntry) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyHotArchiveBucketEntry> for HotArchiveBucketEntry {
    type Error = Error;
    fn try_from(lazy: &LazyHotArchiveBucketEntry) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
