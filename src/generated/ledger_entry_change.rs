#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// LedgerEntryChange is an XDR Union defined as:
///
/// ```text
/// union LedgerEntryChange switch (LedgerEntryChangeType type)
/// {
/// case LEDGER_ENTRY_CREATED:
///     LedgerEntry created;
/// case LEDGER_ENTRY_UPDATED:
///     LedgerEntry updated;
/// case LEDGER_ENTRY_REMOVED:
///     LedgerKey removed;
/// case LEDGER_ENTRY_STATE:
///     LedgerEntry state;
/// case LEDGER_ENTRY_RESTORED:
///     LedgerEntry restored;
/// };
/// ```
///
// union with discriminant LedgerEntryChangeType
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
pub enum LedgerEntryChange {
    Created(LedgerEntry),
    Updated(LedgerEntry),
    Removed(LedgerKey),
    State(LedgerEntry),
    Restored(LedgerEntry),
}

#[cfg(feature = "alloc")]
impl Default for LedgerEntryChange {
    fn default() -> Self {
        Self::Created(LedgerEntry::default())
    }
}

impl LedgerEntryChange {
    const _VARIANTS: &[LedgerEntryChangeType] = &[
        LedgerEntryChangeType::Created,
        LedgerEntryChangeType::Updated,
        LedgerEntryChangeType::Removed,
        LedgerEntryChangeType::State,
        LedgerEntryChangeType::Restored,
    ];
    pub const VARIANTS: [LedgerEntryChangeType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["Created", "Updated", "Removed", "State", "Restored"];
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
            Self::Created(_) => "Created",
            Self::Updated(_) => "Updated",
            Self::Removed(_) => "Removed",
            Self::State(_) => "State",
            Self::Restored(_) => "Restored",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> LedgerEntryChangeType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Created(_) => LedgerEntryChangeType::Created,
            Self::Updated(_) => LedgerEntryChangeType::Updated,
            Self::Removed(_) => LedgerEntryChangeType::Removed,
            Self::State(_) => LedgerEntryChangeType::State,
            Self::Restored(_) => LedgerEntryChangeType::Restored,
        }
    }

    #[must_use]
    pub const fn variants() -> [LedgerEntryChangeType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for LedgerEntryChange {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<LedgerEntryChangeType> for LedgerEntryChange {
    #[must_use]
    fn discriminant(&self) -> LedgerEntryChangeType {
        Self::discriminant(self)
    }
}

impl Variants<LedgerEntryChangeType> for LedgerEntryChange {
    fn variants() -> slice::Iter<'static, LedgerEntryChangeType> {
        Self::VARIANTS.iter()
    }
}

impl Union<LedgerEntryChangeType> for LedgerEntryChange {}

impl ReadXdr for LedgerEntryChange {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: LedgerEntryChangeType = <LedgerEntryChangeType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                LedgerEntryChangeType::Created => Self::Created(LedgerEntry::read_xdr(r)?),
                LedgerEntryChangeType::Updated => Self::Updated(LedgerEntry::read_xdr(r)?),
                LedgerEntryChangeType::Removed => Self::Removed(LedgerKey::read_xdr(r)?),
                LedgerEntryChangeType::State => Self::State(LedgerEntry::read_xdr(r)?),
                LedgerEntryChangeType::Restored => Self::Restored(LedgerEntry::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for LedgerEntryChange {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Created(v) => v.write_xdr(w)?,
                Self::Updated(v) => v.write_xdr(w)?,
                Self::Removed(v) => v.write_xdr(w)?,
                Self::State(v) => v.write_xdr(w)?,
                Self::Restored(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`LedgerEntryChange`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyLedgerEntryChange(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyLedgerEntryChange {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyLedgerEntryChange {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let ord = self.discriminant().cmp(&other.discriminant());
        if ord != core::cmp::Ordering::Equal {
            return ord;
        }
        #[allow(clippy::match_same_arms)]
        match self.discriminant_i32() {
            0 => self.as_created().cmp(&other.as_created()),
            1 => self.as_updated().cmp(&other.as_updated()),
            2 => self.as_removed().cmp(&other.as_removed()),
            3 => self.as_state().cmp(&other.as_state()),
            4 => self.as_restored().cmp(&other.as_restored()),
            _ => core::cmp::Ordering::Equal,
        }
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyLedgerEntryChange {
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
            1 => {
                let field_len =
                    <LazyLedgerEntry as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            2 => {
                let field_len =
                    <LazyLedgerKey as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            3 => {
                let field_len =
                    <LazyLedgerEntry as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            4 => {
                let field_len =
                    <LazyLedgerEntry as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
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
            1 => {
                pos += <LazyLedgerEntry as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            2 => {
                pos += <LazyLedgerKey as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            3 => {
                pos += <LazyLedgerEntry as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            4 => {
                pos += <LazyLedgerEntry as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyLedgerEntryChange {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyLedgerEntryChange {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyLedgerEntryChange {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyLedgerEntryChange {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> LedgerEntryChangeType {
        // Validated — unwrap is safe.
        LedgerEntryChangeType::try_from(self.discriminant_i32()).unwrap()
    }
    /// Access arm `Created`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_created(&self) -> Option<LazyLedgerEntry> {
        if self.discriminant_i32() == 0 {
            Some(<LazyLedgerEntry as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Updated`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_updated(&self) -> Option<LazyLedgerEntry> {
        if self.discriminant_i32() == 1 {
            Some(<LazyLedgerEntry as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Removed`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_removed(&self) -> Option<LazyLedgerKey> {
        if self.discriminant_i32() == 2 {
            Some(<LazyLedgerKey as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `State`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_state(&self) -> Option<LazyLedgerEntry> {
        if self.discriminant_i32() == 3 {
            Some(<LazyLedgerEntry as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Restored`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_restored(&self) -> Option<LazyLedgerEntry> {
        if self.discriminant_i32() == 4 {
            Some(<LazyLedgerEntry as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LedgerEntryChange> for LazyLedgerEntryChange {
    type Error = Error;
    fn try_from(val: &LedgerEntryChange) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyLedgerEntryChange> for LedgerEntryChange {
    type Error = Error;
    fn try_from(lazy: &LazyLedgerEntryChange) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
