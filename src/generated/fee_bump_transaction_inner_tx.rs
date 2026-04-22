#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// FeeBumpTransactionInnerTx is an XDR NestedUnion defined as:
///
/// ```text
/// union switch (EnvelopeType type)
///     {
///     case ENVELOPE_TYPE_TX:
///         TransactionV1Envelope v1;
///     }
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
pub enum FeeBumpTransactionInnerTx {
    Tx(TransactionV1Envelope),
}

#[cfg(feature = "alloc")]
impl Default for FeeBumpTransactionInnerTx {
    fn default() -> Self {
        Self::Tx(TransactionV1Envelope::default())
    }
}

impl FeeBumpTransactionInnerTx {
    const _VARIANTS: &[EnvelopeType] = &[EnvelopeType::Tx];
    pub const VARIANTS: [EnvelopeType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["Tx"];
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
            Self::Tx(_) => "Tx",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> EnvelopeType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Tx(_) => EnvelopeType::Tx,
        }
    }

    #[must_use]
    pub const fn variants() -> [EnvelopeType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for FeeBumpTransactionInnerTx {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<EnvelopeType> for FeeBumpTransactionInnerTx {
    #[must_use]
    fn discriminant(&self) -> EnvelopeType {
        Self::discriminant(self)
    }
}

impl Variants<EnvelopeType> for FeeBumpTransactionInnerTx {
    fn variants() -> slice::Iter<'static, EnvelopeType> {
        Self::VARIANTS.iter()
    }
}

impl Union<EnvelopeType> for FeeBumpTransactionInnerTx {}

impl ReadXdr for FeeBumpTransactionInnerTx {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: EnvelopeType = <EnvelopeType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                EnvelopeType::Tx => Self::Tx(TransactionV1Envelope::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for FeeBumpTransactionInnerTx {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Tx(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`FeeBumpTransactionInnerTx`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyFeeBumpTransactionInnerTx(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyFeeBumpTransactionInnerTx {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyFeeBumpTransactionInnerTx {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let ord = self.discriminant().cmp(&other.discriminant());
        if ord != core::cmp::Ordering::Equal {
            return ord;
        }
        #[allow(clippy::match_same_arms)]
        match self.discriminant_i32() {
            2 => self.as_tx().cmp(&other.as_tx()),
            _ => core::cmp::Ordering::Equal,
        }
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyFeeBumpTransactionInnerTx {
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
            2 => {
                let field_len = <LazyTransactionV1Envelope as LazyXdr>::xdr_validate(
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
            2 => {
                pos += <LazyTransactionV1Envelope as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyFeeBumpTransactionInnerTx {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyFeeBumpTransactionInnerTx {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyFeeBumpTransactionInnerTx {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyFeeBumpTransactionInnerTx {
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
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&FeeBumpTransactionInnerTx> for LazyFeeBumpTransactionInnerTx {
    type Error = Error;
    fn try_from(val: &FeeBumpTransactionInnerTx) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyFeeBumpTransactionInnerTx> for FeeBumpTransactionInnerTx {
    type Error = Error;
    fn try_from(lazy: &LazyFeeBumpTransactionInnerTx) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
