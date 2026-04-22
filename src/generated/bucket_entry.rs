#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// BucketEntry is an XDR Union defined as:
///
/// ```text
/// union BucketEntry switch (BucketEntryType type)
/// {
/// case LIVEENTRY:
/// case INITENTRY:
///     LedgerEntry liveEntry;
///
/// case DEADENTRY:
///     LedgerKey deadEntry;
/// case METAENTRY:
///     BucketMetadata metaEntry;
/// };
/// ```
///
// union with discriminant BucketEntryType
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
pub enum BucketEntry {
    Liveentry(LedgerEntry),
    Initentry(LedgerEntry),
    Deadentry(LedgerKey),
    Metaentry(BucketMetadata),
}

#[cfg(feature = "alloc")]
impl Default for BucketEntry {
    fn default() -> Self {
        Self::Liveentry(LedgerEntry::default())
    }
}

impl BucketEntry {
    const _VARIANTS: &[BucketEntryType] = &[
        BucketEntryType::Liveentry,
        BucketEntryType::Initentry,
        BucketEntryType::Deadentry,
        BucketEntryType::Metaentry,
    ];
    pub const VARIANTS: [BucketEntryType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["Liveentry", "Initentry", "Deadentry", "Metaentry"];
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
            Self::Liveentry(_) => "Liveentry",
            Self::Initentry(_) => "Initentry",
            Self::Deadentry(_) => "Deadentry",
            Self::Metaentry(_) => "Metaentry",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> BucketEntryType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Liveentry(_) => BucketEntryType::Liveentry,
            Self::Initentry(_) => BucketEntryType::Initentry,
            Self::Deadentry(_) => BucketEntryType::Deadentry,
            Self::Metaentry(_) => BucketEntryType::Metaentry,
        }
    }

    #[must_use]
    pub const fn variants() -> [BucketEntryType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for BucketEntry {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<BucketEntryType> for BucketEntry {
    #[must_use]
    fn discriminant(&self) -> BucketEntryType {
        Self::discriminant(self)
    }
}

impl Variants<BucketEntryType> for BucketEntry {
    fn variants() -> slice::Iter<'static, BucketEntryType> {
        Self::VARIANTS.iter()
    }
}

impl Union<BucketEntryType> for BucketEntry {}

impl ReadXdr for BucketEntry {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: BucketEntryType = <BucketEntryType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                BucketEntryType::Liveentry => Self::Liveentry(LedgerEntry::read_xdr(r)?),
                BucketEntryType::Initentry => Self::Initentry(LedgerEntry::read_xdr(r)?),
                BucketEntryType::Deadentry => Self::Deadentry(LedgerKey::read_xdr(r)?),
                BucketEntryType::Metaentry => Self::Metaentry(BucketMetadata::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for BucketEntry {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Liveentry(v) => v.write_xdr(w)?,
                Self::Initentry(v) => v.write_xdr(w)?,
                Self::Deadentry(v) => v.write_xdr(w)?,
                Self::Metaentry(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`BucketEntry`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyBucketEntry(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyBucketEntry {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyBucketEntry {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let ord = self.discriminant().cmp(&other.discriminant());
        if ord != core::cmp::Ordering::Equal {
            return ord;
        }
        #[allow(clippy::match_same_arms)]
        match self.discriminant_i32() {
            0 => self.as_liveentry().cmp(&other.as_liveentry()),
            2 => self.as_initentry().cmp(&other.as_initentry()),
            1 => self.as_deadentry().cmp(&other.as_deadentry()),
            -1 => self.as_metaentry().cmp(&other.as_metaentry()),
            _ => core::cmp::Ordering::Equal,
        }
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyBucketEntry {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        if buf.len() < 4 {
            return Err(Error::Invalid);
        }
        let disc = i32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
        #[allow(unused_mut)]
        let mut pos: u32 = 4;
        #[allow(clippy::match_same_arms)]
        match disc {
            0 => {
                let field_len =
                    <LazyLedgerEntry as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            2 => {
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
        let disc = i32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
        #[allow(unused_mut)]
        let mut pos: u32 = 4;
        #[allow(clippy::match_same_arms)]
        match disc {
            0 => {
                pos += <LazyLedgerEntry as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            2 => {
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

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyBucketEntry {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyBucketEntry {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyBucketEntry {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyBucketEntry {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> BucketEntryType {
        // Validated — unwrap is safe.
        BucketEntryType::try_from(self.discriminant_i32()).unwrap()
    }
    /// Access arm `Liveentry`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_liveentry(&self) -> Option<LazyLedgerEntry> {
        if self.discriminant_i32() == 0 {
            Some(<LazyLedgerEntry as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Initentry`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_initentry(&self) -> Option<LazyLedgerEntry> {
        if self.discriminant_i32() == 2 {
            Some(<LazyLedgerEntry as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Deadentry`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_deadentry(&self) -> Option<LazyLedgerKey> {
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
impl TryFrom<&BucketEntry> for LazyBucketEntry {
    type Error = Error;
    fn try_from(val: &BucketEntry) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyBucketEntry> for BucketEntry {
    type Error = Error;
    fn try_from(lazy: &LazyBucketEntry) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
