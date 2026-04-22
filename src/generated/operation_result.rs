#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// OperationResult is an XDR Union defined as:
///
/// ```text
/// union OperationResult switch (OperationResultCode code)
/// {
/// case opINNER:
///     union switch (OperationType type)
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
///     tr;
/// case opBAD_AUTH:
/// case opNO_ACCOUNT:
/// case opNOT_SUPPORTED:
/// case opTOO_MANY_SUBENTRIES:
/// case opEXCEEDED_WORK_LIMIT:
/// case opTOO_MANY_SPONSORING:
///     void;
/// };
/// ```
///
// union with discriminant OperationResultCode
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
pub enum OperationResult {
    OpInner(OperationResultTr),
    OpBadAuth,
    OpNoAccount,
    OpNotSupported,
    OpTooManySubentries,
    OpExceededWorkLimit,
    OpTooManySponsoring,
}

#[cfg(feature = "alloc")]
impl Default for OperationResult {
    fn default() -> Self {
        Self::OpInner(OperationResultTr::default())
    }
}

impl OperationResult {
    const _VARIANTS: &[OperationResultCode] = &[
        OperationResultCode::OpInner,
        OperationResultCode::OpBadAuth,
        OperationResultCode::OpNoAccount,
        OperationResultCode::OpNotSupported,
        OperationResultCode::OpTooManySubentries,
        OperationResultCode::OpExceededWorkLimit,
        OperationResultCode::OpTooManySponsoring,
    ];
    pub const VARIANTS: [OperationResultCode; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "OpInner",
        "OpBadAuth",
        "OpNoAccount",
        "OpNotSupported",
        "OpTooManySubentries",
        "OpExceededWorkLimit",
        "OpTooManySponsoring",
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
            Self::OpInner(_) => "OpInner",
            Self::OpBadAuth => "OpBadAuth",
            Self::OpNoAccount => "OpNoAccount",
            Self::OpNotSupported => "OpNotSupported",
            Self::OpTooManySubentries => "OpTooManySubentries",
            Self::OpExceededWorkLimit => "OpExceededWorkLimit",
            Self::OpTooManySponsoring => "OpTooManySponsoring",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> OperationResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::OpInner(_) => OperationResultCode::OpInner,
            Self::OpBadAuth => OperationResultCode::OpBadAuth,
            Self::OpNoAccount => OperationResultCode::OpNoAccount,
            Self::OpNotSupported => OperationResultCode::OpNotSupported,
            Self::OpTooManySubentries => OperationResultCode::OpTooManySubentries,
            Self::OpExceededWorkLimit => OperationResultCode::OpExceededWorkLimit,
            Self::OpTooManySponsoring => OperationResultCode::OpTooManySponsoring,
        }
    }

    #[must_use]
    pub const fn variants() -> [OperationResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for OperationResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<OperationResultCode> for OperationResult {
    #[must_use]
    fn discriminant(&self) -> OperationResultCode {
        Self::discriminant(self)
    }
}

impl Variants<OperationResultCode> for OperationResult {
    fn variants() -> slice::Iter<'static, OperationResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<OperationResultCode> for OperationResult {}

impl ReadXdr for OperationResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: OperationResultCode = <OperationResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                OperationResultCode::OpInner => Self::OpInner(OperationResultTr::read_xdr(r)?),
                OperationResultCode::OpBadAuth => Self::OpBadAuth,
                OperationResultCode::OpNoAccount => Self::OpNoAccount,
                OperationResultCode::OpNotSupported => Self::OpNotSupported,
                OperationResultCode::OpTooManySubentries => Self::OpTooManySubentries,
                OperationResultCode::OpExceededWorkLimit => Self::OpExceededWorkLimit,
                OperationResultCode::OpTooManySponsoring => Self::OpTooManySponsoring,
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for OperationResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::OpInner(v) => v.write_xdr(w)?,
                Self::OpBadAuth => ().write_xdr(w)?,
                Self::OpNoAccount => ().write_xdr(w)?,
                Self::OpNotSupported => ().write_xdr(w)?,
                Self::OpTooManySubentries => ().write_xdr(w)?,
                Self::OpExceededWorkLimit => ().write_xdr(w)?,
                Self::OpTooManySponsoring => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`OperationResult`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyOperationResult(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyOperationResult {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyOperationResult {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let ord = self.discriminant().cmp(&other.discriminant());
        if ord != core::cmp::Ordering::Equal {
            return ord;
        }
        #[allow(clippy::match_same_arms)]
        match self.discriminant_i32() {
            0 => self.as_op_inner().cmp(&other.as_op_inner()),
            _ => core::cmp::Ordering::Equal,
        }
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyOperationResult {
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
                    <LazyOperationResultTr as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
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
                pos += <LazyOperationResultTr as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyOperationResult {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyOperationResult {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyOperationResult {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyOperationResult {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> OperationResultCode {
        // Validated — unwrap is safe.
        OperationResultCode::try_from(self.discriminant_i32()).unwrap()
    }
    /// Access arm `OpInner`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_op_inner(&self) -> Option<LazyOperationResultTr> {
        if self.discriminant_i32() == 0 {
            Some(<LazyOperationResultTr as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&OperationResult> for LazyOperationResult {
    type Error = Error;
    fn try_from(val: &OperationResult) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyOperationResult> for OperationResult {
    type Error = Error;
    fn try_from(lazy: &LazyOperationResult) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
