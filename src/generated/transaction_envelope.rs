#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// TransactionEnvelope is an XDR Union defined as:
///
/// ```text
/// union TransactionEnvelope switch (EnvelopeType type)
/// {
/// case ENVELOPE_TYPE_TX_V0:
///     TransactionV0Envelope v0;
/// case ENVELOPE_TYPE_TX:
///     TransactionV1Envelope v1;
/// case ENVELOPE_TYPE_TX_FEE_BUMP:
///     FeeBumpTransactionEnvelope feeBump;
/// };
/// ```
///
// union with discriminant EnvelopeType
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
pub enum TransactionEnvelope {
    TxV0(TransactionV0Envelope),
    Tx(TransactionV1Envelope),
    TxFeeBump(FeeBumpTransactionEnvelope),
}

impl TransactionEnvelope {
    const _VARIANTS: &[EnvelopeType] = &[
        EnvelopeType::TxV0,
        EnvelopeType::Tx,
        EnvelopeType::TxFeeBump,
    ];
    pub const VARIANTS: [EnvelopeType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["TxV0", "Tx", "TxFeeBump"];
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
            Self::TxV0(_) => "TxV0",
            Self::Tx(_) => "Tx",
            Self::TxFeeBump(_) => "TxFeeBump",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> EnvelopeType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::TxV0(_) => EnvelopeType::TxV0,
            Self::Tx(_) => EnvelopeType::Tx,
            Self::TxFeeBump(_) => EnvelopeType::TxFeeBump,
        }
    }

    #[must_use]
    pub const fn variants() -> [EnvelopeType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for TransactionEnvelope {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<EnvelopeType> for TransactionEnvelope {
    #[must_use]
    fn discriminant(&self) -> EnvelopeType {
        Self::discriminant(self)
    }
}

impl Variants<EnvelopeType> for TransactionEnvelope {
    fn variants() -> slice::Iter<'static, EnvelopeType> {
        Self::VARIANTS.iter()
    }
}

impl Union<EnvelopeType> for TransactionEnvelope {}

impl ReadXdr for TransactionEnvelope {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: EnvelopeType = <EnvelopeType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                EnvelopeType::TxV0 => Self::TxV0(TransactionV0Envelope::read_xdr(r)?),
                EnvelopeType::Tx => Self::Tx(TransactionV1Envelope::read_xdr(r)?),
                EnvelopeType::TxFeeBump => {
                    Self::TxFeeBump(FeeBumpTransactionEnvelope::read_xdr(r)?)
                }
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for TransactionEnvelope {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::TxV0(v) => v.write_xdr(w)?,
                Self::Tx(v) => v.write_xdr(w)?,
                Self::TxFeeBump(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`TransactionEnvelope`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyTransactionEnvelope(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyTransactionEnvelope {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyTransactionEnvelope {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let ord = self.discriminant().cmp(&other.discriminant());
        if ord != core::cmp::Ordering::Equal {
            return ord;
        }
        #[allow(clippy::match_same_arms)]
        match self.discriminant_i32() {
            0 => self.as_tx_v0().cmp(&other.as_tx_v0()),
            2 => self.as_tx().cmp(&other.as_tx()),
            5 => self.as_tx_fee_bump().cmp(&other.as_tx_fee_bump()),
            _ => core::cmp::Ordering::Equal,
        }
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyTransactionEnvelope {
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
                let field_len = <LazyTransactionV0Envelope as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            2 => {
                let field_len = <LazyTransactionV1Envelope as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            5 => {
                let field_len = <LazyFeeBumpTransactionEnvelope as LazyXdr>::xdr_validate(
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
                pos += <LazyTransactionV0Envelope as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            2 => {
                pos += <LazyTransactionV1Envelope as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            5 => {
                pos += <LazyFeeBumpTransactionEnvelope as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyTransactionEnvelope {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyTransactionEnvelope {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyTransactionEnvelope {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyTransactionEnvelope {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> EnvelopeType {
        // Validated — unwrap is safe.
        EnvelopeType::try_from(self.discriminant_i32()).unwrap()
    }
    /// Access arm `TxV0`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_tx_v0(&self) -> Option<LazyTransactionV0Envelope> {
        if self.discriminant_i32() == 0 {
            Some(<LazyTransactionV0Envelope as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `Tx`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_tx(&self) -> Option<LazyTransactionV1Envelope> {
        if self.discriminant_i32() == 2 {
            Some(<LazyTransactionV1Envelope as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `TxFeeBump`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_tx_fee_bump(&self) -> Option<LazyFeeBumpTransactionEnvelope> {
        if self.discriminant_i32() == 5 {
            Some(<LazyFeeBumpTransactionEnvelope as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&TransactionEnvelope> for LazyTransactionEnvelope {
    type Error = Error;
    fn try_from(val: &TransactionEnvelope) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyTransactionEnvelope> for TransactionEnvelope {
    type Error = Error;
    fn try_from(lazy: &LazyTransactionEnvelope) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
