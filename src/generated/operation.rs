#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// Operation is an XDR Struct defined as:
///
/// ```text
/// struct Operation
/// {
///     // sourceAccount is the account used to run the operation
///     // if not set, the runtime defaults to "sourceAccount" specified at
///     // the transaction level
///     MuxedAccount* sourceAccount;
///
///     union switch (OperationType type)
///     {
///     case CREATE_ACCOUNT:
///         CreateAccountOp createAccountOp;
///     case PAYMENT:
///         PaymentOp paymentOp;
///     case PATH_PAYMENT_STRICT_RECEIVE:
///         PathPaymentStrictReceiveOp pathPaymentStrictReceiveOp;
///     case MANAGE_SELL_OFFER:
///         ManageSellOfferOp manageSellOfferOp;
///     case CREATE_PASSIVE_SELL_OFFER:
///         CreatePassiveSellOfferOp createPassiveSellOfferOp;
///     case SET_OPTIONS:
///         SetOptionsOp setOptionsOp;
///     case CHANGE_TRUST:
///         ChangeTrustOp changeTrustOp;
///     case ALLOW_TRUST:
///         AllowTrustOp allowTrustOp;
///     case ACCOUNT_MERGE:
///         MuxedAccount destination;
///     case INFLATION:
///         void;
///     case MANAGE_DATA:
///         ManageDataOp manageDataOp;
///     case BUMP_SEQUENCE:
///         BumpSequenceOp bumpSequenceOp;
///     case MANAGE_BUY_OFFER:
///         ManageBuyOfferOp manageBuyOfferOp;
///     case PATH_PAYMENT_STRICT_SEND:
///         PathPaymentStrictSendOp pathPaymentStrictSendOp;
///     case CREATE_CLAIMABLE_BALANCE:
///         CreateClaimableBalanceOp createClaimableBalanceOp;
///     case CLAIM_CLAIMABLE_BALANCE:
///         ClaimClaimableBalanceOp claimClaimableBalanceOp;
///     case BEGIN_SPONSORING_FUTURE_RESERVES:
///         BeginSponsoringFutureReservesOp beginSponsoringFutureReservesOp;
///     case END_SPONSORING_FUTURE_RESERVES:
///         void;
///     case REVOKE_SPONSORSHIP:
///         RevokeSponsorshipOp revokeSponsorshipOp;
///     case CLAWBACK:
///         ClawbackOp clawbackOp;
///     case CLAWBACK_CLAIMABLE_BALANCE:
///         ClawbackClaimableBalanceOp clawbackClaimableBalanceOp;
///     case SET_TRUST_LINE_FLAGS:
///         SetTrustLineFlagsOp setTrustLineFlagsOp;
///     case LIQUIDITY_POOL_DEPOSIT:
///         LiquidityPoolDepositOp liquidityPoolDepositOp;
///     case LIQUIDITY_POOL_WITHDRAW:
///         LiquidityPoolWithdrawOp liquidityPoolWithdrawOp;
///     case INVOKE_HOST_FUNCTION:
///         InvokeHostFunctionOp invokeHostFunctionOp;
///     case EXTEND_FOOTPRINT_TTL:
///         ExtendFootprintTTLOp extendFootprintTTLOp;
///     case RESTORE_FOOTPRINT:
///         RestoreFootprintOp restoreFootprintOp;
///     }
///     body;
/// };
/// ```
///
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Operation {
    pub source_account: Option<MuxedAccount>,
    pub body: OperationBody,
}

impl ReadXdr for Operation {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                source_account: Option::<MuxedAccount>::read_xdr(r)?,
                body: OperationBody::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for Operation {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.source_account.write_xdr(w)?;
            self.body.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`Operation`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyOperation(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyOperation {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyOperation {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.source_account().cmp(&other.source_account()))
            .then_with(|| self.body().cmp(&other.body()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyOperation {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len = <LazyOption<LazyMuxedAccount> as LazyXdr>::xdr_validate(
                &buf[pos as usize..],
                depth,
            )?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len =
                <LazyOperationBody as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyOption<LazyMuxedAccount> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOperationBody as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos
    }

    fn from_xdr_at(parent: &LazyHandle, offset: u32) -> Self {
        let buf = &parent.as_slice()[offset as usize..];
        let len = Self::xdr_len(buf);
        Self(parent.sub_handle(offset, len))
    }
}
#[cfg(feature = "alloc")]
impl From<LazyHandle> for LazyOperation {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyOperation {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyOperation {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyOperation {
    /// Access field `source_account`.
    #[must_use]
    pub fn source_account(&self) -> LazyOption<LazyMuxedAccount> {
        <LazyOption<LazyMuxedAccount> as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `body`.
    #[must_use]
    pub fn body(&self) -> LazyOperationBody {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyOption<LazyMuxedAccount> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyOperationBody as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&Operation> for LazyOperation {
    type Error = Error;
    fn try_from(val: &Operation) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyOperation> for Operation {
    type Error = Error;
    fn try_from(lazy: &LazyOperation) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
