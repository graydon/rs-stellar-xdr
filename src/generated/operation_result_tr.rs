#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// OperationResultTr is an XDR NestedUnion defined as:
///
/// ```text
/// union switch (OperationType type)
///     {
///     case CREATE_ACCOUNT:
///         CreateAccountResult createAccountResult;
///     case PAYMENT:
///         PaymentResult paymentResult;
///     case PATH_PAYMENT_STRICT_RECEIVE:
///         PathPaymentStrictReceiveResult pathPaymentStrictReceiveResult;
///     case MANAGE_SELL_OFFER:
///         ManageSellOfferResult manageSellOfferResult;
///     case CREATE_PASSIVE_SELL_OFFER:
///         ManageSellOfferResult createPassiveSellOfferResult;
///     case SET_OPTIONS:
///         SetOptionsResult setOptionsResult;
///     case CHANGE_TRUST:
///         ChangeTrustResult changeTrustResult;
///     case ALLOW_TRUST:
///         AllowTrustResult allowTrustResult;
///     case ACCOUNT_MERGE:
///         AccountMergeResult accountMergeResult;
///     case INFLATION:
///         InflationResult inflationResult;
///     case MANAGE_DATA:
///         ManageDataResult manageDataResult;
///     case BUMP_SEQUENCE:
///         BumpSequenceResult bumpSeqResult;
///     case MANAGE_BUY_OFFER:
///         ManageBuyOfferResult manageBuyOfferResult;
///     case PATH_PAYMENT_STRICT_SEND:
///         PathPaymentStrictSendResult pathPaymentStrictSendResult;
///     case CREATE_CLAIMABLE_BALANCE:
///         CreateClaimableBalanceResult createClaimableBalanceResult;
///     case CLAIM_CLAIMABLE_BALANCE:
///         ClaimClaimableBalanceResult claimClaimableBalanceResult;
///     case BEGIN_SPONSORING_FUTURE_RESERVES:
///         BeginSponsoringFutureReservesResult beginSponsoringFutureReservesResult;
///     case END_SPONSORING_FUTURE_RESERVES:
///         EndSponsoringFutureReservesResult endSponsoringFutureReservesResult;
///     case REVOKE_SPONSORSHIP:
///         RevokeSponsorshipResult revokeSponsorshipResult;
///     case CLAWBACK:
///         ClawbackResult clawbackResult;
///     case CLAWBACK_CLAIMABLE_BALANCE:
///         ClawbackClaimableBalanceResult clawbackClaimableBalanceResult;
///     case SET_TRUST_LINE_FLAGS:
///         SetTrustLineFlagsResult setTrustLineFlagsResult;
///     case LIQUIDITY_POOL_DEPOSIT:
///         LiquidityPoolDepositResult liquidityPoolDepositResult;
///     case LIQUIDITY_POOL_WITHDRAW:
///         LiquidityPoolWithdrawResult liquidityPoolWithdrawResult;
///     case INVOKE_HOST_FUNCTION:
///         InvokeHostFunctionResult invokeHostFunctionResult;
///     case EXTEND_FOOTPRINT_TTL:
///         ExtendFootprintTTLResult extendFootprintTTLResult;
///     case RESTORE_FOOTPRINT:
///         RestoreFootprintResult restoreFootprintResult;
///     }
/// ```
///
// union with discriminant OperationType
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
pub enum OperationResultTr {
    CreateAccount(CreateAccountResult),
    Payment(PaymentResult),
    PathPaymentStrictReceive(PathPaymentStrictReceiveResult),
    ManageSellOffer(ManageSellOfferResult),
    CreatePassiveSellOffer(ManageSellOfferResult),
    SetOptions(SetOptionsResult),
    ChangeTrust(ChangeTrustResult),
    AllowTrust(AllowTrustResult),
    AccountMerge(AccountMergeResult),
    Inflation(InflationResult),
    ManageData(ManageDataResult),
    BumpSequence(BumpSequenceResult),
    ManageBuyOffer(ManageBuyOfferResult),
    PathPaymentStrictSend(PathPaymentStrictSendResult),
    CreateClaimableBalance(CreateClaimableBalanceResult),
    ClaimClaimableBalance(ClaimClaimableBalanceResult),
    BeginSponsoringFutureReserves(BeginSponsoringFutureReservesResult),
    EndSponsoringFutureReserves(EndSponsoringFutureReservesResult),
    RevokeSponsorship(RevokeSponsorshipResult),
    Clawback(ClawbackResult),
    ClawbackClaimableBalance(ClawbackClaimableBalanceResult),
    SetTrustLineFlags(SetTrustLineFlagsResult),
    LiquidityPoolDeposit(LiquidityPoolDepositResult),
    LiquidityPoolWithdraw(LiquidityPoolWithdrawResult),
    InvokeHostFunction(InvokeHostFunctionResult),
    ExtendFootprintTtl(ExtendFootprintTtlResult),
    RestoreFootprint(RestoreFootprintResult),
}

#[cfg(feature = "alloc")]
impl Default for OperationResultTr {
    fn default() -> Self {
        Self::CreateAccount(CreateAccountResult::default())
    }
}

impl OperationResultTr {
    const _VARIANTS: &[OperationType] = &[
        OperationType::CreateAccount,
        OperationType::Payment,
        OperationType::PathPaymentStrictReceive,
        OperationType::ManageSellOffer,
        OperationType::CreatePassiveSellOffer,
        OperationType::SetOptions,
        OperationType::ChangeTrust,
        OperationType::AllowTrust,
        OperationType::AccountMerge,
        OperationType::Inflation,
        OperationType::ManageData,
        OperationType::BumpSequence,
        OperationType::ManageBuyOffer,
        OperationType::PathPaymentStrictSend,
        OperationType::CreateClaimableBalance,
        OperationType::ClaimClaimableBalance,
        OperationType::BeginSponsoringFutureReserves,
        OperationType::EndSponsoringFutureReserves,
        OperationType::RevokeSponsorship,
        OperationType::Clawback,
        OperationType::ClawbackClaimableBalance,
        OperationType::SetTrustLineFlags,
        OperationType::LiquidityPoolDeposit,
        OperationType::LiquidityPoolWithdraw,
        OperationType::InvokeHostFunction,
        OperationType::ExtendFootprintTtl,
        OperationType::RestoreFootprint,
    ];
    pub const VARIANTS: [OperationType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "CreateAccount",
        "Payment",
        "PathPaymentStrictReceive",
        "ManageSellOffer",
        "CreatePassiveSellOffer",
        "SetOptions",
        "ChangeTrust",
        "AllowTrust",
        "AccountMerge",
        "Inflation",
        "ManageData",
        "BumpSequence",
        "ManageBuyOffer",
        "PathPaymentStrictSend",
        "CreateClaimableBalance",
        "ClaimClaimableBalance",
        "BeginSponsoringFutureReserves",
        "EndSponsoringFutureReserves",
        "RevokeSponsorship",
        "Clawback",
        "ClawbackClaimableBalance",
        "SetTrustLineFlags",
        "LiquidityPoolDeposit",
        "LiquidityPoolWithdraw",
        "InvokeHostFunction",
        "ExtendFootprintTtl",
        "RestoreFootprint",
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
            Self::CreateAccount(_) => "CreateAccount",
            Self::Payment(_) => "Payment",
            Self::PathPaymentStrictReceive(_) => "PathPaymentStrictReceive",
            Self::ManageSellOffer(_) => "ManageSellOffer",
            Self::CreatePassiveSellOffer(_) => "CreatePassiveSellOffer",
            Self::SetOptions(_) => "SetOptions",
            Self::ChangeTrust(_) => "ChangeTrust",
            Self::AllowTrust(_) => "AllowTrust",
            Self::AccountMerge(_) => "AccountMerge",
            Self::Inflation(_) => "Inflation",
            Self::ManageData(_) => "ManageData",
            Self::BumpSequence(_) => "BumpSequence",
            Self::ManageBuyOffer(_) => "ManageBuyOffer",
            Self::PathPaymentStrictSend(_) => "PathPaymentStrictSend",
            Self::CreateClaimableBalance(_) => "CreateClaimableBalance",
            Self::ClaimClaimableBalance(_) => "ClaimClaimableBalance",
            Self::BeginSponsoringFutureReserves(_) => "BeginSponsoringFutureReserves",
            Self::EndSponsoringFutureReserves(_) => "EndSponsoringFutureReserves",
            Self::RevokeSponsorship(_) => "RevokeSponsorship",
            Self::Clawback(_) => "Clawback",
            Self::ClawbackClaimableBalance(_) => "ClawbackClaimableBalance",
            Self::SetTrustLineFlags(_) => "SetTrustLineFlags",
            Self::LiquidityPoolDeposit(_) => "LiquidityPoolDeposit",
            Self::LiquidityPoolWithdraw(_) => "LiquidityPoolWithdraw",
            Self::InvokeHostFunction(_) => "InvokeHostFunction",
            Self::ExtendFootprintTtl(_) => "ExtendFootprintTtl",
            Self::RestoreFootprint(_) => "RestoreFootprint",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> OperationType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::CreateAccount(_) => OperationType::CreateAccount,
            Self::Payment(_) => OperationType::Payment,
            Self::PathPaymentStrictReceive(_) => OperationType::PathPaymentStrictReceive,
            Self::ManageSellOffer(_) => OperationType::ManageSellOffer,
            Self::CreatePassiveSellOffer(_) => OperationType::CreatePassiveSellOffer,
            Self::SetOptions(_) => OperationType::SetOptions,
            Self::ChangeTrust(_) => OperationType::ChangeTrust,
            Self::AllowTrust(_) => OperationType::AllowTrust,
            Self::AccountMerge(_) => OperationType::AccountMerge,
            Self::Inflation(_) => OperationType::Inflation,
            Self::ManageData(_) => OperationType::ManageData,
            Self::BumpSequence(_) => OperationType::BumpSequence,
            Self::ManageBuyOffer(_) => OperationType::ManageBuyOffer,
            Self::PathPaymentStrictSend(_) => OperationType::PathPaymentStrictSend,
            Self::CreateClaimableBalance(_) => OperationType::CreateClaimableBalance,
            Self::ClaimClaimableBalance(_) => OperationType::ClaimClaimableBalance,
            Self::BeginSponsoringFutureReserves(_) => OperationType::BeginSponsoringFutureReserves,
            Self::EndSponsoringFutureReserves(_) => OperationType::EndSponsoringFutureReserves,
            Self::RevokeSponsorship(_) => OperationType::RevokeSponsorship,
            Self::Clawback(_) => OperationType::Clawback,
            Self::ClawbackClaimableBalance(_) => OperationType::ClawbackClaimableBalance,
            Self::SetTrustLineFlags(_) => OperationType::SetTrustLineFlags,
            Self::LiquidityPoolDeposit(_) => OperationType::LiquidityPoolDeposit,
            Self::LiquidityPoolWithdraw(_) => OperationType::LiquidityPoolWithdraw,
            Self::InvokeHostFunction(_) => OperationType::InvokeHostFunction,
            Self::ExtendFootprintTtl(_) => OperationType::ExtendFootprintTtl,
            Self::RestoreFootprint(_) => OperationType::RestoreFootprint,
        }
    }

    #[must_use]
    pub const fn variants() -> [OperationType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for OperationResultTr {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<OperationType> for OperationResultTr {
    #[must_use]
    fn discriminant(&self) -> OperationType {
        Self::discriminant(self)
    }
}

impl Variants<OperationType> for OperationResultTr {
    fn variants() -> slice::Iter<'static, OperationType> {
        Self::VARIANTS.iter()
    }
}

impl Union<OperationType> for OperationResultTr {}

impl ReadXdr for OperationResultTr {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: OperationType = <OperationType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                OperationType::CreateAccount => {
                    Self::CreateAccount(CreateAccountResult::read_xdr(r)?)
                }
                OperationType::Payment => Self::Payment(PaymentResult::read_xdr(r)?),
                OperationType::PathPaymentStrictReceive => {
                    Self::PathPaymentStrictReceive(PathPaymentStrictReceiveResult::read_xdr(r)?)
                }
                OperationType::ManageSellOffer => {
                    Self::ManageSellOffer(ManageSellOfferResult::read_xdr(r)?)
                }
                OperationType::CreatePassiveSellOffer => {
                    Self::CreatePassiveSellOffer(ManageSellOfferResult::read_xdr(r)?)
                }
                OperationType::SetOptions => Self::SetOptions(SetOptionsResult::read_xdr(r)?),
                OperationType::ChangeTrust => Self::ChangeTrust(ChangeTrustResult::read_xdr(r)?),
                OperationType::AllowTrust => Self::AllowTrust(AllowTrustResult::read_xdr(r)?),
                OperationType::AccountMerge => Self::AccountMerge(AccountMergeResult::read_xdr(r)?),
                OperationType::Inflation => Self::Inflation(InflationResult::read_xdr(r)?),
                OperationType::ManageData => Self::ManageData(ManageDataResult::read_xdr(r)?),
                OperationType::BumpSequence => Self::BumpSequence(BumpSequenceResult::read_xdr(r)?),
                OperationType::ManageBuyOffer => {
                    Self::ManageBuyOffer(ManageBuyOfferResult::read_xdr(r)?)
                }
                OperationType::PathPaymentStrictSend => {
                    Self::PathPaymentStrictSend(PathPaymentStrictSendResult::read_xdr(r)?)
                }
                OperationType::CreateClaimableBalance => {
                    Self::CreateClaimableBalance(CreateClaimableBalanceResult::read_xdr(r)?)
                }
                OperationType::ClaimClaimableBalance => {
                    Self::ClaimClaimableBalance(ClaimClaimableBalanceResult::read_xdr(r)?)
                }
                OperationType::BeginSponsoringFutureReserves => {
                    Self::BeginSponsoringFutureReserves(
                        BeginSponsoringFutureReservesResult::read_xdr(r)?,
                    )
                }
                OperationType::EndSponsoringFutureReserves => Self::EndSponsoringFutureReserves(
                    EndSponsoringFutureReservesResult::read_xdr(r)?,
                ),
                OperationType::RevokeSponsorship => {
                    Self::RevokeSponsorship(RevokeSponsorshipResult::read_xdr(r)?)
                }
                OperationType::Clawback => Self::Clawback(ClawbackResult::read_xdr(r)?),
                OperationType::ClawbackClaimableBalance => {
                    Self::ClawbackClaimableBalance(ClawbackClaimableBalanceResult::read_xdr(r)?)
                }
                OperationType::SetTrustLineFlags => {
                    Self::SetTrustLineFlags(SetTrustLineFlagsResult::read_xdr(r)?)
                }
                OperationType::LiquidityPoolDeposit => {
                    Self::LiquidityPoolDeposit(LiquidityPoolDepositResult::read_xdr(r)?)
                }
                OperationType::LiquidityPoolWithdraw => {
                    Self::LiquidityPoolWithdraw(LiquidityPoolWithdrawResult::read_xdr(r)?)
                }
                OperationType::InvokeHostFunction => {
                    Self::InvokeHostFunction(InvokeHostFunctionResult::read_xdr(r)?)
                }
                OperationType::ExtendFootprintTtl => {
                    Self::ExtendFootprintTtl(ExtendFootprintTtlResult::read_xdr(r)?)
                }
                OperationType::RestoreFootprint => {
                    Self::RestoreFootprint(RestoreFootprintResult::read_xdr(r)?)
                }
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for OperationResultTr {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::CreateAccount(v) => v.write_xdr(w)?,
                Self::Payment(v) => v.write_xdr(w)?,
                Self::PathPaymentStrictReceive(v) => v.write_xdr(w)?,
                Self::ManageSellOffer(v) => v.write_xdr(w)?,
                Self::CreatePassiveSellOffer(v) => v.write_xdr(w)?,
                Self::SetOptions(v) => v.write_xdr(w)?,
                Self::ChangeTrust(v) => v.write_xdr(w)?,
                Self::AllowTrust(v) => v.write_xdr(w)?,
                Self::AccountMerge(v) => v.write_xdr(w)?,
                Self::Inflation(v) => v.write_xdr(w)?,
                Self::ManageData(v) => v.write_xdr(w)?,
                Self::BumpSequence(v) => v.write_xdr(w)?,
                Self::ManageBuyOffer(v) => v.write_xdr(w)?,
                Self::PathPaymentStrictSend(v) => v.write_xdr(w)?,
                Self::CreateClaimableBalance(v) => v.write_xdr(w)?,
                Self::ClaimClaimableBalance(v) => v.write_xdr(w)?,
                Self::BeginSponsoringFutureReserves(v) => v.write_xdr(w)?,
                Self::EndSponsoringFutureReserves(v) => v.write_xdr(w)?,
                Self::RevokeSponsorship(v) => v.write_xdr(w)?,
                Self::Clawback(v) => v.write_xdr(w)?,
                Self::ClawbackClaimableBalance(v) => v.write_xdr(w)?,
                Self::SetTrustLineFlags(v) => v.write_xdr(w)?,
                Self::LiquidityPoolDeposit(v) => v.write_xdr(w)?,
                Self::LiquidityPoolWithdraw(v) => v.write_xdr(w)?,
                Self::InvokeHostFunction(v) => v.write_xdr(w)?,
                Self::ExtendFootprintTtl(v) => v.write_xdr(w)?,
                Self::RestoreFootprint(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`OperationResultTr`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyOperationResultTr(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyOperationResultTr {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyOperationResultTr {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let ord = self.discriminant().cmp(&other.discriminant());
        if ord != core::cmp::Ordering::Equal {
            return ord;
        }
        #[allow(clippy::match_same_arms)]
        match self.discriminant_i32() {
            0 => self.as_create_account().cmp(&other.as_create_account()),
            1 => self.as_payment().cmp(&other.as_payment()),
            2 => self
                .as_path_payment_strict_receive()
                .cmp(&other.as_path_payment_strict_receive()),
            3 => self
                .as_manage_sell_offer()
                .cmp(&other.as_manage_sell_offer()),
            4 => self
                .as_create_passive_sell_offer()
                .cmp(&other.as_create_passive_sell_offer()),
            5 => self.as_set_options().cmp(&other.as_set_options()),
            6 => self.as_change_trust().cmp(&other.as_change_trust()),
            7 => self.as_allow_trust().cmp(&other.as_allow_trust()),
            8 => self.as_account_merge().cmp(&other.as_account_merge()),
            9 => self.as_inflation().cmp(&other.as_inflation()),
            10 => self.as_manage_data().cmp(&other.as_manage_data()),
            11 => self.as_bump_sequence().cmp(&other.as_bump_sequence()),
            12 => self.as_manage_buy_offer().cmp(&other.as_manage_buy_offer()),
            13 => self
                .as_path_payment_strict_send()
                .cmp(&other.as_path_payment_strict_send()),
            14 => self
                .as_create_claimable_balance()
                .cmp(&other.as_create_claimable_balance()),
            15 => self
                .as_claim_claimable_balance()
                .cmp(&other.as_claim_claimable_balance()),
            16 => self
                .as_begin_sponsoring_future_reserves()
                .cmp(&other.as_begin_sponsoring_future_reserves()),
            17 => self
                .as_end_sponsoring_future_reserves()
                .cmp(&other.as_end_sponsoring_future_reserves()),
            18 => self
                .as_revoke_sponsorship()
                .cmp(&other.as_revoke_sponsorship()),
            19 => self.as_clawback().cmp(&other.as_clawback()),
            20 => self
                .as_clawback_claimable_balance()
                .cmp(&other.as_clawback_claimable_balance()),
            21 => self
                .as_set_trust_line_flags()
                .cmp(&other.as_set_trust_line_flags()),
            22 => self
                .as_liquidity_pool_deposit()
                .cmp(&other.as_liquidity_pool_deposit()),
            23 => self
                .as_liquidity_pool_withdraw()
                .cmp(&other.as_liquidity_pool_withdraw()),
            24 => self
                .as_invoke_host_function()
                .cmp(&other.as_invoke_host_function()),
            25 => self
                .as_extend_footprint_ttl()
                .cmp(&other.as_extend_footprint_ttl()),
            26 => self
                .as_restore_footprint()
                .cmp(&other.as_restore_footprint()),
            _ => core::cmp::Ordering::Equal,
        }
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyOperationResultTr {
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
                let field_len = <LazyCreateAccountResult as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            1 => {
                let field_len =
                    <LazyPaymentResult as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            2 => {
                let field_len = <LazyPathPaymentStrictReceiveResult as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            3 => {
                let field_len = <LazyManageSellOfferResult as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            4 => {
                let field_len = <LazyManageSellOfferResult as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            5 => {
                let field_len =
                    <LazySetOptionsResult as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            6 => {
                let field_len =
                    <LazyChangeTrustResult as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            7 => {
                let field_len =
                    <LazyAllowTrustResult as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            8 => {
                let field_len =
                    <LazyAccountMergeResult as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            9 => {
                let field_len =
                    <LazyInflationResult as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            10 => {
                let field_len =
                    <LazyManageDataResult as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            11 => {
                let field_len =
                    <LazyBumpSequenceResult as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            12 => {
                let field_len = <LazyManageBuyOfferResult as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            13 => {
                let field_len = <LazyPathPaymentStrictSendResult as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            14 => {
                let field_len = <LazyCreateClaimableBalanceResult as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            15 => {
                let field_len = <LazyClaimClaimableBalanceResult as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            16 => {
                let field_len = <LazyBeginSponsoringFutureReservesResult as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            17 => {
                let field_len = <LazyEndSponsoringFutureReservesResult as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            18 => {
                let field_len = <LazyRevokeSponsorshipResult as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            19 => {
                let field_len =
                    <LazyClawbackResult as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            20 => {
                let field_len = <LazyClawbackClaimableBalanceResult as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            21 => {
                let field_len = <LazySetTrustLineFlagsResult as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            22 => {
                let field_len = <LazyLiquidityPoolDepositResult as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            23 => {
                let field_len = <LazyLiquidityPoolWithdrawResult as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            24 => {
                let field_len = <LazyInvokeHostFunctionResult as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            25 => {
                let field_len = <LazyExtendFootprintTtlResult as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            26 => {
                let field_len = <LazyRestoreFootprintResult as LazyXdr>::xdr_validate(
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
        let bytes: [u8; 4] = buf[..4].try_into().unwrap();
        let disc = i32::from_be_bytes(bytes);
        #[allow(unused_mut)]
        let mut pos: u32 = 4;
        #[allow(clippy::match_same_arms)]
        match disc {
            0 => {
                pos += <LazyCreateAccountResult as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            1 => {
                pos += <LazyPaymentResult as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            2 => {
                pos +=
                    <LazyPathPaymentStrictReceiveResult as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            3 => {
                pos += <LazyManageSellOfferResult as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            4 => {
                pos += <LazyManageSellOfferResult as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            5 => {
                pos += <LazySetOptionsResult as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            6 => {
                pos += <LazyChangeTrustResult as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            7 => {
                pos += <LazyAllowTrustResult as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            8 => {
                pos += <LazyAccountMergeResult as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            9 => {
                pos += <LazyInflationResult as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            10 => {
                pos += <LazyManageDataResult as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            11 => {
                pos += <LazyBumpSequenceResult as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            12 => {
                pos += <LazyManageBuyOfferResult as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            13 => {
                pos += <LazyPathPaymentStrictSendResult as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            14 => {
                pos += <LazyCreateClaimableBalanceResult as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            15 => {
                pos += <LazyClaimClaimableBalanceResult as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            16 => {
                pos += <LazyBeginSponsoringFutureReservesResult as LazyXdr>::xdr_len(
                    &buf[pos as usize..],
                );
            }
            17 => {
                pos += <LazyEndSponsoringFutureReservesResult as LazyXdr>::xdr_len(
                    &buf[pos as usize..],
                );
            }
            18 => {
                pos += <LazyRevokeSponsorshipResult as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            19 => {
                pos += <LazyClawbackResult as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            20 => {
                pos +=
                    <LazyClawbackClaimableBalanceResult as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            21 => {
                pos += <LazySetTrustLineFlagsResult as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            22 => {
                pos += <LazyLiquidityPoolDepositResult as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            23 => {
                pos += <LazyLiquidityPoolWithdrawResult as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            24 => {
                pos += <LazyInvokeHostFunctionResult as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            25 => {
                pos += <LazyExtendFootprintTtlResult as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            26 => {
                pos += <LazyRestoreFootprintResult as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyOperationResultTr {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyOperationResultTr {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyOperationResultTr {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyOperationResultTr {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> OperationType {
        // Validated — unwrap is safe.
        OperationType::try_from(self.discriminant_i32()).unwrap()
    }
    /// Access arm `CreateAccount`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_create_account(&self) -> Option<LazyCreateAccountResult> {
        if self.discriminant_i32() == 0 {
            Some(<LazyCreateAccountResult as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `Payment`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_payment(&self) -> Option<LazyPaymentResult> {
        if self.discriminant_i32() == 1 {
            Some(<LazyPaymentResult as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `PathPaymentStrictReceive`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_path_payment_strict_receive(&self) -> Option<LazyPathPaymentStrictReceiveResult> {
        if self.discriminant_i32() == 2 {
            Some(<LazyPathPaymentStrictReceiveResult as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `ManageSellOffer`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_manage_sell_offer(&self) -> Option<LazyManageSellOfferResult> {
        if self.discriminant_i32() == 3 {
            Some(<LazyManageSellOfferResult as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `CreatePassiveSellOffer`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_create_passive_sell_offer(&self) -> Option<LazyManageSellOfferResult> {
        if self.discriminant_i32() == 4 {
            Some(<LazyManageSellOfferResult as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `SetOptions`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_set_options(&self) -> Option<LazySetOptionsResult> {
        if self.discriminant_i32() == 5 {
            Some(<LazySetOptionsResult as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `ChangeTrust`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_change_trust(&self) -> Option<LazyChangeTrustResult> {
        if self.discriminant_i32() == 6 {
            Some(<LazyChangeTrustResult as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `AllowTrust`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_allow_trust(&self) -> Option<LazyAllowTrustResult> {
        if self.discriminant_i32() == 7 {
            Some(<LazyAllowTrustResult as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `AccountMerge`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_account_merge(&self) -> Option<LazyAccountMergeResult> {
        if self.discriminant_i32() == 8 {
            Some(<LazyAccountMergeResult as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Inflation`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_inflation(&self) -> Option<LazyInflationResult> {
        if self.discriminant_i32() == 9 {
            Some(<LazyInflationResult as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `ManageData`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_manage_data(&self) -> Option<LazyManageDataResult> {
        if self.discriminant_i32() == 10 {
            Some(<LazyManageDataResult as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `BumpSequence`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_bump_sequence(&self) -> Option<LazyBumpSequenceResult> {
        if self.discriminant_i32() == 11 {
            Some(<LazyBumpSequenceResult as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `ManageBuyOffer`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_manage_buy_offer(&self) -> Option<LazyManageBuyOfferResult> {
        if self.discriminant_i32() == 12 {
            Some(<LazyManageBuyOfferResult as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `PathPaymentStrictSend`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_path_payment_strict_send(&self) -> Option<LazyPathPaymentStrictSendResult> {
        if self.discriminant_i32() == 13 {
            Some(<LazyPathPaymentStrictSendResult as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `CreateClaimableBalance`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_create_claimable_balance(&self) -> Option<LazyCreateClaimableBalanceResult> {
        if self.discriminant_i32() == 14 {
            Some(<LazyCreateClaimableBalanceResult as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `ClaimClaimableBalance`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_claim_claimable_balance(&self) -> Option<LazyClaimClaimableBalanceResult> {
        if self.discriminant_i32() == 15 {
            Some(<LazyClaimClaimableBalanceResult as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `BeginSponsoringFutureReserves`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_begin_sponsoring_future_reserves(
        &self,
    ) -> Option<LazyBeginSponsoringFutureReservesResult> {
        if self.discriminant_i32() == 16 {
            Some(<LazyBeginSponsoringFutureReservesResult as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `EndSponsoringFutureReserves`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_end_sponsoring_future_reserves(
        &self,
    ) -> Option<LazyEndSponsoringFutureReservesResult> {
        if self.discriminant_i32() == 17 {
            Some(<LazyEndSponsoringFutureReservesResult as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `RevokeSponsorship`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_revoke_sponsorship(&self) -> Option<LazyRevokeSponsorshipResult> {
        if self.discriminant_i32() == 18 {
            Some(<LazyRevokeSponsorshipResult as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `Clawback`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_clawback(&self) -> Option<LazyClawbackResult> {
        if self.discriminant_i32() == 19 {
            Some(<LazyClawbackResult as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `ClawbackClaimableBalance`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_clawback_claimable_balance(&self) -> Option<LazyClawbackClaimableBalanceResult> {
        if self.discriminant_i32() == 20 {
            Some(<LazyClawbackClaimableBalanceResult as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `SetTrustLineFlags`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_set_trust_line_flags(&self) -> Option<LazySetTrustLineFlagsResult> {
        if self.discriminant_i32() == 21 {
            Some(<LazySetTrustLineFlagsResult as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `LiquidityPoolDeposit`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_liquidity_pool_deposit(&self) -> Option<LazyLiquidityPoolDepositResult> {
        if self.discriminant_i32() == 22 {
            Some(<LazyLiquidityPoolDepositResult as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `LiquidityPoolWithdraw`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_liquidity_pool_withdraw(&self) -> Option<LazyLiquidityPoolWithdrawResult> {
        if self.discriminant_i32() == 23 {
            Some(<LazyLiquidityPoolWithdrawResult as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `InvokeHostFunction`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_invoke_host_function(&self) -> Option<LazyInvokeHostFunctionResult> {
        if self.discriminant_i32() == 24 {
            Some(<LazyInvokeHostFunctionResult as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `ExtendFootprintTtl`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_extend_footprint_ttl(&self) -> Option<LazyExtendFootprintTtlResult> {
        if self.discriminant_i32() == 25 {
            Some(<LazyExtendFootprintTtlResult as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `RestoreFootprint`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_restore_footprint(&self) -> Option<LazyRestoreFootprintResult> {
        if self.discriminant_i32() == 26 {
            Some(<LazyRestoreFootprintResult as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&OperationResultTr> for LazyOperationResultTr {
    type Error = Error;
    fn try_from(val: &OperationResultTr) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyOperationResultTr> for OperationResultTr {
    type Error = Error;
    fn try_from(lazy: &LazyOperationResultTr) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
