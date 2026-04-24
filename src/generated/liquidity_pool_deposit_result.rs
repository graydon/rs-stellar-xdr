#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// LiquidityPoolDepositResult is an XDR Union defined as:
///
/// ```text
/// union LiquidityPoolDepositResult switch (LiquidityPoolDepositResultCode code)
/// {
/// case LIQUIDITY_POOL_DEPOSIT_SUCCESS:
///     void;
/// case LIQUIDITY_POOL_DEPOSIT_MALFORMED:
/// case LIQUIDITY_POOL_DEPOSIT_NO_TRUST:
/// case LIQUIDITY_POOL_DEPOSIT_NOT_AUTHORIZED:
/// case LIQUIDITY_POOL_DEPOSIT_UNDERFUNDED:
/// case LIQUIDITY_POOL_DEPOSIT_LINE_FULL:
/// case LIQUIDITY_POOL_DEPOSIT_BAD_PRICE:
/// case LIQUIDITY_POOL_DEPOSIT_POOL_FULL:
/// case LIQUIDITY_POOL_DEPOSIT_TRUSTLINE_FROZEN:
///     void;
/// };
/// ```
///
// union with discriminant LiquidityPoolDepositResultCode
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
pub enum LiquidityPoolDepositResult {
    Success,
    Malformed,
    NoTrust,
    NotAuthorized,
    Underfunded,
    LineFull,
    BadPrice,
    PoolFull,
    TrustlineFrozen,
}

#[cfg(feature = "alloc")]
impl Default for LiquidityPoolDepositResult {
    fn default() -> Self {
        Self::Success
    }
}

impl LiquidityPoolDepositResult {
    const _VARIANTS: &[LiquidityPoolDepositResultCode] = &[
        LiquidityPoolDepositResultCode::Success,
        LiquidityPoolDepositResultCode::Malformed,
        LiquidityPoolDepositResultCode::NoTrust,
        LiquidityPoolDepositResultCode::NotAuthorized,
        LiquidityPoolDepositResultCode::Underfunded,
        LiquidityPoolDepositResultCode::LineFull,
        LiquidityPoolDepositResultCode::BadPrice,
        LiquidityPoolDepositResultCode::PoolFull,
        LiquidityPoolDepositResultCode::TrustlineFrozen,
    ];
    pub const VARIANTS: [LiquidityPoolDepositResultCode; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "Success",
        "Malformed",
        "NoTrust",
        "NotAuthorized",
        "Underfunded",
        "LineFull",
        "BadPrice",
        "PoolFull",
        "TrustlineFrozen",
    ];
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
            Self::Success => "Success",
            Self::Malformed => "Malformed",
            Self::NoTrust => "NoTrust",
            Self::NotAuthorized => "NotAuthorized",
            Self::Underfunded => "Underfunded",
            Self::LineFull => "LineFull",
            Self::BadPrice => "BadPrice",
            Self::PoolFull => "PoolFull",
            Self::TrustlineFrozen => "TrustlineFrozen",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> LiquidityPoolDepositResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => LiquidityPoolDepositResultCode::Success,
            Self::Malformed => LiquidityPoolDepositResultCode::Malformed,
            Self::NoTrust => LiquidityPoolDepositResultCode::NoTrust,
            Self::NotAuthorized => LiquidityPoolDepositResultCode::NotAuthorized,
            Self::Underfunded => LiquidityPoolDepositResultCode::Underfunded,
            Self::LineFull => LiquidityPoolDepositResultCode::LineFull,
            Self::BadPrice => LiquidityPoolDepositResultCode::BadPrice,
            Self::PoolFull => LiquidityPoolDepositResultCode::PoolFull,
            Self::TrustlineFrozen => LiquidityPoolDepositResultCode::TrustlineFrozen,
        }
    }

    #[must_use]
    pub const fn variants() -> [LiquidityPoolDepositResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for LiquidityPoolDepositResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<LiquidityPoolDepositResultCode> for LiquidityPoolDepositResult {
    #[must_use]
    fn discriminant(&self) -> LiquidityPoolDepositResultCode {
        Self::discriminant(self)
    }
}

impl Variants<LiquidityPoolDepositResultCode> for LiquidityPoolDepositResult {
    fn variants() -> slice::Iter<'static, LiquidityPoolDepositResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<LiquidityPoolDepositResultCode> for LiquidityPoolDepositResult {}

impl ReadXdr for LiquidityPoolDepositResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: LiquidityPoolDepositResultCode =
                <LiquidityPoolDepositResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                LiquidityPoolDepositResultCode::Success => Self::Success,
                LiquidityPoolDepositResultCode::Malformed => Self::Malformed,
                LiquidityPoolDepositResultCode::NoTrust => Self::NoTrust,
                LiquidityPoolDepositResultCode::NotAuthorized => Self::NotAuthorized,
                LiquidityPoolDepositResultCode::Underfunded => Self::Underfunded,
                LiquidityPoolDepositResultCode::LineFull => Self::LineFull,
                LiquidityPoolDepositResultCode::BadPrice => Self::BadPrice,
                LiquidityPoolDepositResultCode::PoolFull => Self::PoolFull,
                LiquidityPoolDepositResultCode::TrustlineFrozen => Self::TrustlineFrozen,
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for LiquidityPoolDepositResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Success => ().write_xdr(w)?,
                Self::Malformed => ().write_xdr(w)?,
                Self::NoTrust => ().write_xdr(w)?,
                Self::NotAuthorized => ().write_xdr(w)?,
                Self::Underfunded => ().write_xdr(w)?,
                Self::LineFull => ().write_xdr(w)?,
                Self::BadPrice => ().write_xdr(w)?,
                Self::PoolFull => ().write_xdr(w)?,
                Self::TrustlineFrozen => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`LiquidityPoolDepositResult`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyLiquidityPoolDepositResult(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyLiquidityPoolDepositResult {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyLiquidityPoolDepositResult {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let ord = self.discriminant().cmp(&other.discriminant());
        if ord != core::cmp::Ordering::Equal {
            return ord;
        }
        #[allow(clippy::match_same_arms)]
        match self.discriminant_i32() {
            _ => core::cmp::Ordering::Equal,
        }
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyLiquidityPoolDepositResult {
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
                // void — no additional data
            }
            -1 => {
                // void — no additional data
            }
            -2 => {
                // void — no additional data
            }
            -3 => {
                // void — no additional data
            }
            -4 => {
                // void — no additional data
            }
            -5 => {
                // void — no additional data
            }
            -6 => {
                // void — no additional data
            }
            -7 => {
                // void — no additional data
            }
            -8 => {
                // void — no additional data
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
                // void
            }
            -1 => {
                // void
            }
            -2 => {
                // void
            }
            -3 => {
                // void
            }
            -4 => {
                // void
            }
            -5 => {
                // void
            }
            -6 => {
                // void
            }
            -7 => {
                // void
            }
            -8 => {
                // void
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
impl From<LazyHandle> for LazyLiquidityPoolDepositResult {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyLiquidityPoolDepositResult {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyLiquidityPoolDepositResult {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyLiquidityPoolDepositResult {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> LiquidityPoolDepositResultCode {
        // Validated — unwrap is safe.
        LiquidityPoolDepositResultCode::try_from(self.discriminant_i32()).unwrap()
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LiquidityPoolDepositResult> for LazyLiquidityPoolDepositResult {
    type Error = Error;
    fn try_from(val: &LiquidityPoolDepositResult) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyLiquidityPoolDepositResult> for LiquidityPoolDepositResult {
    type Error = Error;
    fn try_from(lazy: &LazyLiquidityPoolDepositResult) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
