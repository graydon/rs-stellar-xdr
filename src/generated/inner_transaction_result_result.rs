#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// InnerTransactionResultResult is an XDR NestedUnion defined as:
///
/// ```text
/// union switch (TransactionResultCode code)
///     {
///     // txFEE_BUMP_INNER_SUCCESS is not included
///     case txSUCCESS:
///     case txFAILED:
///         OperationResult results<>;
///     case txTOO_EARLY:
///     case txTOO_LATE:
///     case txMISSING_OPERATION:
///     case txBAD_SEQ:
///     case txBAD_AUTH:
///     case txINSUFFICIENT_BALANCE:
///     case txNO_ACCOUNT:
///     case txINSUFFICIENT_FEE:
///     case txBAD_AUTH_EXTRA:
///     case txINTERNAL_ERROR:
///     case txNOT_SUPPORTED:
///     // txFEE_BUMP_INNER_FAILED is not included
///     case txBAD_SPONSORSHIP:
///     case txBAD_MIN_SEQ_AGE_OR_GAP:
///     case txMALFORMED:
///     case txSOROBAN_INVALID:
///     case txFROZEN_KEY_ACCESSED:
///         void;
///     }
/// ```
///
// union with discriminant TransactionResultCode
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
pub enum InnerTransactionResultResult {
    TxSuccess(VecM<OperationResult>),
    TxFailed(VecM<OperationResult>),
    TxTooEarly,
    TxTooLate,
    TxMissingOperation,
    TxBadSeq,
    TxBadAuth,
    TxInsufficientBalance,
    TxNoAccount,
    TxInsufficientFee,
    TxBadAuthExtra,
    TxInternalError,
    TxNotSupported,
    TxBadSponsorship,
    TxBadMinSeqAgeOrGap,
    TxMalformed,
    TxSorobanInvalid,
    TxFrozenKeyAccessed,
}

#[cfg(feature = "alloc")]
impl Default for InnerTransactionResultResult {
    fn default() -> Self {
        Self::TxSuccess(VecM::<OperationResult>::default())
    }
}

impl InnerTransactionResultResult {
    const _VARIANTS: &[TransactionResultCode] = &[
        TransactionResultCode::TxSuccess,
        TransactionResultCode::TxFailed,
        TransactionResultCode::TxTooEarly,
        TransactionResultCode::TxTooLate,
        TransactionResultCode::TxMissingOperation,
        TransactionResultCode::TxBadSeq,
        TransactionResultCode::TxBadAuth,
        TransactionResultCode::TxInsufficientBalance,
        TransactionResultCode::TxNoAccount,
        TransactionResultCode::TxInsufficientFee,
        TransactionResultCode::TxBadAuthExtra,
        TransactionResultCode::TxInternalError,
        TransactionResultCode::TxNotSupported,
        TransactionResultCode::TxBadSponsorship,
        TransactionResultCode::TxBadMinSeqAgeOrGap,
        TransactionResultCode::TxMalformed,
        TransactionResultCode::TxSorobanInvalid,
        TransactionResultCode::TxFrozenKeyAccessed,
    ];
    pub const VARIANTS: [TransactionResultCode; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "TxSuccess",
        "TxFailed",
        "TxTooEarly",
        "TxTooLate",
        "TxMissingOperation",
        "TxBadSeq",
        "TxBadAuth",
        "TxInsufficientBalance",
        "TxNoAccount",
        "TxInsufficientFee",
        "TxBadAuthExtra",
        "TxInternalError",
        "TxNotSupported",
        "TxBadSponsorship",
        "TxBadMinSeqAgeOrGap",
        "TxMalformed",
        "TxSorobanInvalid",
        "TxFrozenKeyAccessed",
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
            Self::TxSuccess(_) => "TxSuccess",
            Self::TxFailed(_) => "TxFailed",
            Self::TxTooEarly => "TxTooEarly",
            Self::TxTooLate => "TxTooLate",
            Self::TxMissingOperation => "TxMissingOperation",
            Self::TxBadSeq => "TxBadSeq",
            Self::TxBadAuth => "TxBadAuth",
            Self::TxInsufficientBalance => "TxInsufficientBalance",
            Self::TxNoAccount => "TxNoAccount",
            Self::TxInsufficientFee => "TxInsufficientFee",
            Self::TxBadAuthExtra => "TxBadAuthExtra",
            Self::TxInternalError => "TxInternalError",
            Self::TxNotSupported => "TxNotSupported",
            Self::TxBadSponsorship => "TxBadSponsorship",
            Self::TxBadMinSeqAgeOrGap => "TxBadMinSeqAgeOrGap",
            Self::TxMalformed => "TxMalformed",
            Self::TxSorobanInvalid => "TxSorobanInvalid",
            Self::TxFrozenKeyAccessed => "TxFrozenKeyAccessed",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> TransactionResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::TxSuccess(_) => TransactionResultCode::TxSuccess,
            Self::TxFailed(_) => TransactionResultCode::TxFailed,
            Self::TxTooEarly => TransactionResultCode::TxTooEarly,
            Self::TxTooLate => TransactionResultCode::TxTooLate,
            Self::TxMissingOperation => TransactionResultCode::TxMissingOperation,
            Self::TxBadSeq => TransactionResultCode::TxBadSeq,
            Self::TxBadAuth => TransactionResultCode::TxBadAuth,
            Self::TxInsufficientBalance => TransactionResultCode::TxInsufficientBalance,
            Self::TxNoAccount => TransactionResultCode::TxNoAccount,
            Self::TxInsufficientFee => TransactionResultCode::TxInsufficientFee,
            Self::TxBadAuthExtra => TransactionResultCode::TxBadAuthExtra,
            Self::TxInternalError => TransactionResultCode::TxInternalError,
            Self::TxNotSupported => TransactionResultCode::TxNotSupported,
            Self::TxBadSponsorship => TransactionResultCode::TxBadSponsorship,
            Self::TxBadMinSeqAgeOrGap => TransactionResultCode::TxBadMinSeqAgeOrGap,
            Self::TxMalformed => TransactionResultCode::TxMalformed,
            Self::TxSorobanInvalid => TransactionResultCode::TxSorobanInvalid,
            Self::TxFrozenKeyAccessed => TransactionResultCode::TxFrozenKeyAccessed,
        }
    }

    #[must_use]
    pub const fn variants() -> [TransactionResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for InnerTransactionResultResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<TransactionResultCode> for InnerTransactionResultResult {
    #[must_use]
    fn discriminant(&self) -> TransactionResultCode {
        Self::discriminant(self)
    }
}

impl Variants<TransactionResultCode> for InnerTransactionResultResult {
    fn variants() -> slice::Iter<'static, TransactionResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<TransactionResultCode> for InnerTransactionResultResult {}

impl ReadXdr for InnerTransactionResultResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: TransactionResultCode = <TransactionResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                TransactionResultCode::TxSuccess => {
                    Self::TxSuccess(VecM::<OperationResult>::read_xdr(r)?)
                }
                TransactionResultCode::TxFailed => {
                    Self::TxFailed(VecM::<OperationResult>::read_xdr(r)?)
                }
                TransactionResultCode::TxTooEarly => Self::TxTooEarly,
                TransactionResultCode::TxTooLate => Self::TxTooLate,
                TransactionResultCode::TxMissingOperation => Self::TxMissingOperation,
                TransactionResultCode::TxBadSeq => Self::TxBadSeq,
                TransactionResultCode::TxBadAuth => Self::TxBadAuth,
                TransactionResultCode::TxInsufficientBalance => Self::TxInsufficientBalance,
                TransactionResultCode::TxNoAccount => Self::TxNoAccount,
                TransactionResultCode::TxInsufficientFee => Self::TxInsufficientFee,
                TransactionResultCode::TxBadAuthExtra => Self::TxBadAuthExtra,
                TransactionResultCode::TxInternalError => Self::TxInternalError,
                TransactionResultCode::TxNotSupported => Self::TxNotSupported,
                TransactionResultCode::TxBadSponsorship => Self::TxBadSponsorship,
                TransactionResultCode::TxBadMinSeqAgeOrGap => Self::TxBadMinSeqAgeOrGap,
                TransactionResultCode::TxMalformed => Self::TxMalformed,
                TransactionResultCode::TxSorobanInvalid => Self::TxSorobanInvalid,
                TransactionResultCode::TxFrozenKeyAccessed => Self::TxFrozenKeyAccessed,
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for InnerTransactionResultResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::TxSuccess(v) => v.write_xdr(w)?,
                Self::TxFailed(v) => v.write_xdr(w)?,
                Self::TxTooEarly => ().write_xdr(w)?,
                Self::TxTooLate => ().write_xdr(w)?,
                Self::TxMissingOperation => ().write_xdr(w)?,
                Self::TxBadSeq => ().write_xdr(w)?,
                Self::TxBadAuth => ().write_xdr(w)?,
                Self::TxInsufficientBalance => ().write_xdr(w)?,
                Self::TxNoAccount => ().write_xdr(w)?,
                Self::TxInsufficientFee => ().write_xdr(w)?,
                Self::TxBadAuthExtra => ().write_xdr(w)?,
                Self::TxInternalError => ().write_xdr(w)?,
                Self::TxNotSupported => ().write_xdr(w)?,
                Self::TxBadSponsorship => ().write_xdr(w)?,
                Self::TxBadMinSeqAgeOrGap => ().write_xdr(w)?,
                Self::TxMalformed => ().write_xdr(w)?,
                Self::TxSorobanInvalid => ().write_xdr(w)?,
                Self::TxFrozenKeyAccessed => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`InnerTransactionResultResult`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyInnerTransactionResultResult(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyInnerTransactionResultResult {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyInnerTransactionResultResult {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let ord = self.discriminant().cmp(&other.discriminant());
        if ord != core::cmp::Ordering::Equal {
            return ord;
        }
        #[allow(clippy::match_same_arms)]
        match self.discriminant_i32() {
            0 => self.as_tx_success().cmp(&other.as_tx_success()),
            -1 => self.as_tx_failed().cmp(&other.as_tx_failed()),
            _ => core::cmp::Ordering::Equal,
        }
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyInnerTransactionResultResult {
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
                let field_len = <LazyVecM<LazyOperationResult> as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            -1 => {
                let field_len = <LazyVecM<LazyOperationResult> as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
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
            -9 => {
                // void — no additional data
            }
            -10 => {
                // void — no additional data
            }
            -11 => {
                // void — no additional data
            }
            -12 => {
                // void — no additional data
            }
            -14 => {
                // void — no additional data
            }
            -15 => {
                // void — no additional data
            }
            -16 => {
                // void — no additional data
            }
            -17 => {
                // void — no additional data
            }
            -18 => {
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
                pos += <LazyVecM<LazyOperationResult> as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            -1 => {
                pos += <LazyVecM<LazyOperationResult> as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
            -9 => {
                // void
            }
            -10 => {
                // void
            }
            -11 => {
                // void
            }
            -12 => {
                // void
            }
            -14 => {
                // void
            }
            -15 => {
                // void
            }
            -16 => {
                // void
            }
            -17 => {
                // void
            }
            -18 => {
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
impl From<LazyHandle> for LazyInnerTransactionResultResult {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyInnerTransactionResultResult {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyInnerTransactionResultResult {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyInnerTransactionResultResult {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> TransactionResultCode {
        // Validated — unwrap is safe.
        TransactionResultCode::try_from(self.discriminant_i32()).unwrap()
    }
    /// Access arm `TxSuccess`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_tx_success(&self) -> Option<LazyVecM<LazyOperationResult>> {
        if self.discriminant_i32() == 0 {
            Some(<LazyVecM<LazyOperationResult> as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `TxFailed`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_tx_failed(&self) -> Option<LazyVecM<LazyOperationResult>> {
        if self.discriminant_i32() == -1 {
            Some(<LazyVecM<LazyOperationResult> as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&InnerTransactionResultResult> for LazyInnerTransactionResultResult {
    type Error = Error;
    fn try_from(val: &InnerTransactionResultResult) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyInnerTransactionResultResult> for InnerTransactionResultResult {
    type Error = Error;
    fn try_from(lazy: &LazyInnerTransactionResultResult) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
