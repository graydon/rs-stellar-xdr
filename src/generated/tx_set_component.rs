#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// TxSetComponent is an XDR Union defined as:
///
/// ```text
/// union TxSetComponent switch (TxSetComponentType type)
/// {
/// case TXSET_COMP_TXS_MAYBE_DISCOUNTED_FEE:
///   struct
///   {
///     int64* baseFee;
///     TransactionEnvelope txs<>;
///   } txsMaybeDiscountedFee;
/// };
/// ```
///
// union with discriminant TxSetComponentType
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
pub enum TxSetComponent {
    TxsetCompTxsMaybeDiscountedFee(TxSetComponentTxsMaybeDiscountedFee),
}

#[cfg(feature = "alloc")]
impl Default for TxSetComponent {
    fn default() -> Self {
        Self::TxsetCompTxsMaybeDiscountedFee(TxSetComponentTxsMaybeDiscountedFee::default())
    }
}

impl TxSetComponent {
    const _VARIANTS: &[TxSetComponentType] = &[TxSetComponentType::TxsetCompTxsMaybeDiscountedFee];
    pub const VARIANTS: [TxSetComponentType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["TxsetCompTxsMaybeDiscountedFee"];
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
            Self::TxsetCompTxsMaybeDiscountedFee(_) => "TxsetCompTxsMaybeDiscountedFee",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> TxSetComponentType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::TxsetCompTxsMaybeDiscountedFee(_) => {
                TxSetComponentType::TxsetCompTxsMaybeDiscountedFee
            }
        }
    }

    #[must_use]
    pub const fn variants() -> [TxSetComponentType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for TxSetComponent {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<TxSetComponentType> for TxSetComponent {
    #[must_use]
    fn discriminant(&self) -> TxSetComponentType {
        Self::discriminant(self)
    }
}

impl Variants<TxSetComponentType> for TxSetComponent {
    fn variants() -> slice::Iter<'static, TxSetComponentType> {
        Self::VARIANTS.iter()
    }
}

impl Union<TxSetComponentType> for TxSetComponent {}

impl ReadXdr for TxSetComponent {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: TxSetComponentType = <TxSetComponentType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                TxSetComponentType::TxsetCompTxsMaybeDiscountedFee => {
                    Self::TxsetCompTxsMaybeDiscountedFee(
                        TxSetComponentTxsMaybeDiscountedFee::read_xdr(r)?,
                    )
                }
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for TxSetComponent {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::TxsetCompTxsMaybeDiscountedFee(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`TxSetComponent`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyTxSetComponent(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyTxSetComponent {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyTxSetComponent {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let ord = self.discriminant().cmp(&other.discriminant());
        if ord != core::cmp::Ordering::Equal {
            return ord;
        }
        #[allow(clippy::match_same_arms)]
        match self.discriminant_i32() {
            0 => self
                .as_txset_comp_txs_maybe_discounted_fee()
                .cmp(&other.as_txset_comp_txs_maybe_discounted_fee()),
            _ => core::cmp::Ordering::Equal,
        }
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyTxSetComponent {
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
                let field_len = <LazyTxSetComponentTxsMaybeDiscountedFee as LazyXdr>::xdr_validate(
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
                pos += <LazyTxSetComponentTxsMaybeDiscountedFee as LazyXdr>::xdr_len(
                    &buf[pos as usize..],
                );
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
impl From<LazyHandle> for LazyTxSetComponent {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyTxSetComponent {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyTxSetComponent {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyTxSetComponent {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> TxSetComponentType {
        // Validated — unwrap is safe.
        TxSetComponentType::try_from(self.discriminant_i32()).unwrap()
    }
    /// Access arm `TxsetCompTxsMaybeDiscountedFee`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_txset_comp_txs_maybe_discounted_fee(
        &self,
    ) -> Option<LazyTxSetComponentTxsMaybeDiscountedFee> {
        if self.discriminant_i32() == 0 {
            Some(<LazyTxSetComponentTxsMaybeDiscountedFee as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&TxSetComponent> for LazyTxSetComponent {
    type Error = Error;
    fn try_from(val: &TxSetComponent) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyTxSetComponent> for TxSetComponent {
    type Error = Error;
    fn try_from(lazy: &LazyTxSetComponent) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
