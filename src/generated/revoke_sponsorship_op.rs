#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// RevokeSponsorshipOp is an XDR Union defined as:
///
/// ```text
/// union RevokeSponsorshipOp switch (RevokeSponsorshipType type)
/// {
/// case REVOKE_SPONSORSHIP_LEDGER_ENTRY:
///     LedgerKey ledgerKey;
/// case REVOKE_SPONSORSHIP_SIGNER:
///     struct
///     {
///         AccountID accountID;
///         SignerKey signerKey;
///     } signer;
/// };
/// ```
///
// union with discriminant RevokeSponsorshipType
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
pub enum RevokeSponsorshipOp {
    LedgerEntry(LedgerKey),
    Signer(RevokeSponsorshipOpSigner),
}

#[cfg(feature = "alloc")]
impl Default for RevokeSponsorshipOp {
    fn default() -> Self {
        Self::LedgerEntry(LedgerKey::default())
    }
}

impl RevokeSponsorshipOp {
    const _VARIANTS: &[RevokeSponsorshipType] = &[
        RevokeSponsorshipType::LedgerEntry,
        RevokeSponsorshipType::Signer,
    ];
    pub const VARIANTS: [RevokeSponsorshipType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["LedgerEntry", "Signer"];
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
            Self::LedgerEntry(_) => "LedgerEntry",
            Self::Signer(_) => "Signer",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> RevokeSponsorshipType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::LedgerEntry(_) => RevokeSponsorshipType::LedgerEntry,
            Self::Signer(_) => RevokeSponsorshipType::Signer,
        }
    }

    #[must_use]
    pub const fn variants() -> [RevokeSponsorshipType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for RevokeSponsorshipOp {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<RevokeSponsorshipType> for RevokeSponsorshipOp {
    #[must_use]
    fn discriminant(&self) -> RevokeSponsorshipType {
        Self::discriminant(self)
    }
}

impl Variants<RevokeSponsorshipType> for RevokeSponsorshipOp {
    fn variants() -> slice::Iter<'static, RevokeSponsorshipType> {
        Self::VARIANTS.iter()
    }
}

impl Union<RevokeSponsorshipType> for RevokeSponsorshipOp {}

impl ReadXdr for RevokeSponsorshipOp {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: RevokeSponsorshipType = <RevokeSponsorshipType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                RevokeSponsorshipType::LedgerEntry => Self::LedgerEntry(LedgerKey::read_xdr(r)?),
                RevokeSponsorshipType::Signer => {
                    Self::Signer(RevokeSponsorshipOpSigner::read_xdr(r)?)
                }
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for RevokeSponsorshipOp {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::LedgerEntry(v) => v.write_xdr(w)?,
                Self::Signer(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`RevokeSponsorshipOp`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyRevokeSponsorshipOp(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyRevokeSponsorshipOp {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyRevokeSponsorshipOp {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let ord = self.discriminant().cmp(&other.discriminant());
        if ord != core::cmp::Ordering::Equal {
            return ord;
        }
        #[allow(clippy::match_same_arms)]
        match self.discriminant_i32() {
            0 => self.as_ledger_entry().cmp(&other.as_ledger_entry()),
            1 => self.as_signer().cmp(&other.as_signer()),
            _ => core::cmp::Ordering::Equal,
        }
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyRevokeSponsorshipOp {
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
                    <LazyLedgerKey as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            1 => {
                let field_len = <LazyRevokeSponsorshipOpSigner as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
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
                pos += <LazyLedgerKey as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            1 => {
                pos += <LazyRevokeSponsorshipOpSigner as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyRevokeSponsorshipOp {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyRevokeSponsorshipOp {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyRevokeSponsorshipOp {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyRevokeSponsorshipOp {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> RevokeSponsorshipType {
        // Validated — unwrap is safe.
        RevokeSponsorshipType::try_from(self.discriminant_i32()).unwrap()
    }
    /// Access arm `LedgerEntry`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_ledger_entry(&self) -> Option<LazyLedgerKey> {
        if self.discriminant_i32() == 0 {
            Some(<LazyLedgerKey as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Signer`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_signer(&self) -> Option<LazyRevokeSponsorshipOpSigner> {
        if self.discriminant_i32() == 1 {
            Some(<LazyRevokeSponsorshipOpSigner as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&RevokeSponsorshipOp> for LazyRevokeSponsorshipOp {
    type Error = Error;
    fn try_from(val: &RevokeSponsorshipOp) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyRevokeSponsorshipOp> for RevokeSponsorshipOp {
    type Error = Error;
    fn try_from(lazy: &LazyRevokeSponsorshipOp) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
