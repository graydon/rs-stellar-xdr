#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ChangeTrustAsset is an XDR Union defined as:
///
/// ```text
/// union ChangeTrustAsset switch (AssetType type)
/// {
/// case ASSET_TYPE_NATIVE: // Not credit
///     void;
///
/// case ASSET_TYPE_CREDIT_ALPHANUM4:
///     AlphaNum4 alphaNum4;
///
/// case ASSET_TYPE_CREDIT_ALPHANUM12:
///     AlphaNum12 alphaNum12;
///
/// case ASSET_TYPE_POOL_SHARE:
///     LiquidityPoolParameters liquidityPool;
///
///     // add other asset types here in the future
/// };
/// ```
///
// union with discriminant AssetType
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
pub enum ChangeTrustAsset {
    Native,
    CreditAlphanum4(AlphaNum4),
    CreditAlphanum12(AlphaNum12),
    PoolShare(LiquidityPoolParameters),
}

#[cfg(feature = "alloc")]
impl Default for ChangeTrustAsset {
    fn default() -> Self {
        Self::Native
    }
}

impl ChangeTrustAsset {
    const _VARIANTS: &[AssetType] = &[
        AssetType::Native,
        AssetType::CreditAlphanum4,
        AssetType::CreditAlphanum12,
        AssetType::PoolShare,
    ];
    pub const VARIANTS: [AssetType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["Native", "CreditAlphanum4", "CreditAlphanum12", "PoolShare"];
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
            Self::Native => "Native",
            Self::CreditAlphanum4(_) => "CreditAlphanum4",
            Self::CreditAlphanum12(_) => "CreditAlphanum12",
            Self::PoolShare(_) => "PoolShare",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> AssetType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Native => AssetType::Native,
            Self::CreditAlphanum4(_) => AssetType::CreditAlphanum4,
            Self::CreditAlphanum12(_) => AssetType::CreditAlphanum12,
            Self::PoolShare(_) => AssetType::PoolShare,
        }
    }

    #[must_use]
    pub const fn variants() -> [AssetType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ChangeTrustAsset {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<AssetType> for ChangeTrustAsset {
    #[must_use]
    fn discriminant(&self) -> AssetType {
        Self::discriminant(self)
    }
}

impl Variants<AssetType> for ChangeTrustAsset {
    fn variants() -> slice::Iter<'static, AssetType> {
        Self::VARIANTS.iter()
    }
}

impl Union<AssetType> for ChangeTrustAsset {}

impl ReadXdr for ChangeTrustAsset {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: AssetType = <AssetType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                AssetType::Native => Self::Native,
                AssetType::CreditAlphanum4 => Self::CreditAlphanum4(AlphaNum4::read_xdr(r)?),
                AssetType::CreditAlphanum12 => Self::CreditAlphanum12(AlphaNum12::read_xdr(r)?),
                AssetType::PoolShare => Self::PoolShare(LiquidityPoolParameters::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for ChangeTrustAsset {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Native => ().write_xdr(w)?,
                Self::CreditAlphanum4(v) => v.write_xdr(w)?,
                Self::CreditAlphanum12(v) => v.write_xdr(w)?,
                Self::PoolShare(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ChangeTrustAsset`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyChangeTrustAsset(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyChangeTrustAsset {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyChangeTrustAsset {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let ord = self.discriminant().cmp(&other.discriminant());
        if ord != core::cmp::Ordering::Equal {
            return ord;
        }
        #[allow(clippy::match_same_arms)]
        match self.discriminant_i32() {
            1 => self.as_credit_alphanum4().cmp(&other.as_credit_alphanum4()),
            2 => self
                .as_credit_alphanum12()
                .cmp(&other.as_credit_alphanum12()),
            3 => self.as_pool_share().cmp(&other.as_pool_share()),
            _ => core::cmp::Ordering::Equal,
        }
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyChangeTrustAsset {
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
                // void — no additional data
            }
            1 => {
                let field_len =
                    <LazyAlphaNum4 as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            2 => {
                let field_len =
                    <LazyAlphaNum12 as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            3 => {
                let field_len = <LazyLiquidityPoolParameters as LazyXdr>::xdr_validate(
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
                // void
            }
            1 => {
                pos += <LazyAlphaNum4 as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            2 => {
                pos += <LazyAlphaNum12 as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            3 => {
                pos += <LazyLiquidityPoolParameters as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyChangeTrustAsset {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyChangeTrustAsset {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyChangeTrustAsset {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyChangeTrustAsset {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> AssetType {
        // Validated — unwrap is safe.
        AssetType::try_from(self.discriminant_i32()).unwrap()
    }
    /// Access arm `CreditAlphanum4`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_credit_alphanum4(&self) -> Option<LazyAlphaNum4> {
        if self.discriminant_i32() == 1 {
            Some(<LazyAlphaNum4 as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `CreditAlphanum12`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_credit_alphanum12(&self) -> Option<LazyAlphaNum12> {
        if self.discriminant_i32() == 2 {
            Some(<LazyAlphaNum12 as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `PoolShare`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_pool_share(&self) -> Option<LazyLiquidityPoolParameters> {
        if self.discriminant_i32() == 3 {
            Some(<LazyLiquidityPoolParameters as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ChangeTrustAsset> for LazyChangeTrustAsset {
    type Error = Error;
    fn try_from(val: &ChangeTrustAsset) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyChangeTrustAsset> for ChangeTrustAsset {
    type Error = Error;
    fn try_from(lazy: &LazyChangeTrustAsset) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
