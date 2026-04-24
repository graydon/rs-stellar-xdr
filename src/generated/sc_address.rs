#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ScAddress is an XDR Union defined as:
///
/// ```text
/// union SCAddress switch (SCAddressType type)
/// {
/// case SC_ADDRESS_TYPE_ACCOUNT:
///     AccountID accountId;
/// case SC_ADDRESS_TYPE_CONTRACT:
///     ContractID contractId;
/// case SC_ADDRESS_TYPE_MUXED_ACCOUNT:
///     MuxedEd25519Account muxedAccount;
/// case SC_ADDRESS_TYPE_CLAIMABLE_BALANCE:
///     ClaimableBalanceID claimableBalanceId;
/// case SC_ADDRESS_TYPE_LIQUIDITY_POOL:
///     PoolID liquidityPoolId;
/// };
/// ```
///
// union with discriminant ScAddressType
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr)
)]
#[allow(clippy::large_enum_variant)]
pub enum ScAddress {
    Account(AccountId),
    Contract(ContractId),
    MuxedAccount(MuxedEd25519Account),
    ClaimableBalance(ClaimableBalanceId),
    LiquidityPool(PoolId),
}

#[cfg(feature = "alloc")]
impl Default for ScAddress {
    fn default() -> Self {
        Self::Account(AccountId::default())
    }
}

impl ScAddress {
    const _VARIANTS: &[ScAddressType] = &[
        ScAddressType::Account,
        ScAddressType::Contract,
        ScAddressType::MuxedAccount,
        ScAddressType::ClaimableBalance,
        ScAddressType::LiquidityPool,
    ];
    pub const VARIANTS: [ScAddressType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "Account",
        "Contract",
        "MuxedAccount",
        "ClaimableBalance",
        "LiquidityPool",
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
            Self::Account(_) => "Account",
            Self::Contract(_) => "Contract",
            Self::MuxedAccount(_) => "MuxedAccount",
            Self::ClaimableBalance(_) => "ClaimableBalance",
            Self::LiquidityPool(_) => "LiquidityPool",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> ScAddressType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Account(_) => ScAddressType::Account,
            Self::Contract(_) => ScAddressType::Contract,
            Self::MuxedAccount(_) => ScAddressType::MuxedAccount,
            Self::ClaimableBalance(_) => ScAddressType::ClaimableBalance,
            Self::LiquidityPool(_) => ScAddressType::LiquidityPool,
        }
    }

    #[must_use]
    pub const fn variants() -> [ScAddressType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ScAddress {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ScAddressType> for ScAddress {
    #[must_use]
    fn discriminant(&self) -> ScAddressType {
        Self::discriminant(self)
    }
}

impl Variants<ScAddressType> for ScAddress {
    fn variants() -> slice::Iter<'static, ScAddressType> {
        Self::VARIANTS.iter()
    }
}

impl Union<ScAddressType> for ScAddress {}

impl ReadXdr for ScAddress {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ScAddressType = <ScAddressType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ScAddressType::Account => Self::Account(AccountId::read_xdr(r)?),
                ScAddressType::Contract => Self::Contract(ContractId::read_xdr(r)?),
                ScAddressType::MuxedAccount => {
                    Self::MuxedAccount(MuxedEd25519Account::read_xdr(r)?)
                }
                ScAddressType::ClaimableBalance => {
                    Self::ClaimableBalance(ClaimableBalanceId::read_xdr(r)?)
                }
                ScAddressType::LiquidityPool => Self::LiquidityPool(PoolId::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for ScAddress {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Account(v) => v.write_xdr(w)?,
                Self::Contract(v) => v.write_xdr(w)?,
                Self::MuxedAccount(v) => v.write_xdr(w)?,
                Self::ClaimableBalance(v) => v.write_xdr(w)?,
                Self::LiquidityPool(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ScAddress`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyScAddress(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyScAddress {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyScAddress {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let ord = self.discriminant().cmp(&other.discriminant());
        if ord != core::cmp::Ordering::Equal {
            return ord;
        }
        #[allow(clippy::match_same_arms)]
        match self.discriminant_i32() {
            0 => self.as_account().cmp(&other.as_account()),
            1 => self.as_contract().cmp(&other.as_contract()),
            2 => self.as_muxed_account().cmp(&other.as_muxed_account()),
            3 => self
                .as_claimable_balance()
                .cmp(&other.as_claimable_balance()),
            4 => self.as_liquidity_pool().cmp(&other.as_liquidity_pool()),
            _ => core::cmp::Ordering::Equal,
        }
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyScAddress {
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
                    <LazyAccountId as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            1 => {
                let field_len =
                    <LazyContractId as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            2 => {
                let field_len = <LazyMuxedEd25519Account as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            3 => {
                let field_len =
                    <LazyClaimableBalanceId as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            4 => {
                let field_len = <LazyPoolId as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
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
                pos += <LazyAccountId as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            1 => {
                pos += <LazyContractId as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            2 => {
                pos += <LazyMuxedEd25519Account as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            3 => {
                pos += <LazyClaimableBalanceId as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            4 => {
                pos += <LazyPoolId as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyScAddress {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyScAddress {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyScAddress {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyScAddress {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> ScAddressType {
        // Validated — unwrap is safe.
        ScAddressType::try_from(self.discriminant_i32()).unwrap()
    }
    /// Access arm `Account`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_account(&self) -> Option<LazyAccountId> {
        if self.discriminant_i32() == 0 {
            Some(<LazyAccountId as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Contract`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_contract(&self) -> Option<LazyContractId> {
        if self.discriminant_i32() == 1 {
            Some(<LazyContractId as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `MuxedAccount`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_muxed_account(&self) -> Option<LazyMuxedEd25519Account> {
        if self.discriminant_i32() == 2 {
            Some(<LazyMuxedEd25519Account as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `ClaimableBalance`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_claimable_balance(&self) -> Option<LazyClaimableBalanceId> {
        if self.discriminant_i32() == 3 {
            Some(<LazyClaimableBalanceId as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `LiquidityPool`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_liquidity_pool(&self) -> Option<LazyPoolId> {
        if self.discriminant_i32() == 4 {
            Some(<LazyPoolId as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ScAddress> for LazyScAddress {
    type Error = Error;
    fn try_from(val: &ScAddress) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyScAddress> for ScAddress {
    type Error = Error;
    fn try_from(lazy: &LazyScAddress) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
