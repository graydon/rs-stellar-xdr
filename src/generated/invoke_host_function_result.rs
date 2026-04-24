#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// InvokeHostFunctionResult is an XDR Union defined as:
///
/// ```text
/// union InvokeHostFunctionResult switch (InvokeHostFunctionResultCode code)
/// {
/// case INVOKE_HOST_FUNCTION_SUCCESS:
///     Hash success; // sha256(InvokeHostFunctionSuccessPreImage)
/// case INVOKE_HOST_FUNCTION_MALFORMED:
/// case INVOKE_HOST_FUNCTION_TRAPPED:
/// case INVOKE_HOST_FUNCTION_RESOURCE_LIMIT_EXCEEDED:
/// case INVOKE_HOST_FUNCTION_ENTRY_ARCHIVED:
/// case INVOKE_HOST_FUNCTION_INSUFFICIENT_REFUNDABLE_FEE:
///     void;
/// };
/// ```
///
// union with discriminant InvokeHostFunctionResultCode
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
pub enum InvokeHostFunctionResult {
    Success(Hash),
    Malformed,
    Trapped,
    ResourceLimitExceeded,
    EntryArchived,
    InsufficientRefundableFee,
}

#[cfg(feature = "alloc")]
impl Default for InvokeHostFunctionResult {
    fn default() -> Self {
        Self::Success(Hash::default())
    }
}

impl InvokeHostFunctionResult {
    const _VARIANTS: &[InvokeHostFunctionResultCode] = &[
        InvokeHostFunctionResultCode::Success,
        InvokeHostFunctionResultCode::Malformed,
        InvokeHostFunctionResultCode::Trapped,
        InvokeHostFunctionResultCode::ResourceLimitExceeded,
        InvokeHostFunctionResultCode::EntryArchived,
        InvokeHostFunctionResultCode::InsufficientRefundableFee,
    ];
    pub const VARIANTS: [InvokeHostFunctionResultCode; Self::_VARIANTS.len()] = {
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
        "Trapped",
        "ResourceLimitExceeded",
        "EntryArchived",
        "InsufficientRefundableFee",
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
            Self::Success(_) => "Success",
            Self::Malformed => "Malformed",
            Self::Trapped => "Trapped",
            Self::ResourceLimitExceeded => "ResourceLimitExceeded",
            Self::EntryArchived => "EntryArchived",
            Self::InsufficientRefundableFee => "InsufficientRefundableFee",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> InvokeHostFunctionResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success(_) => InvokeHostFunctionResultCode::Success,
            Self::Malformed => InvokeHostFunctionResultCode::Malformed,
            Self::Trapped => InvokeHostFunctionResultCode::Trapped,
            Self::ResourceLimitExceeded => InvokeHostFunctionResultCode::ResourceLimitExceeded,
            Self::EntryArchived => InvokeHostFunctionResultCode::EntryArchived,
            Self::InsufficientRefundableFee => {
                InvokeHostFunctionResultCode::InsufficientRefundableFee
            }
        }
    }

    #[must_use]
    pub const fn variants() -> [InvokeHostFunctionResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for InvokeHostFunctionResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<InvokeHostFunctionResultCode> for InvokeHostFunctionResult {
    #[must_use]
    fn discriminant(&self) -> InvokeHostFunctionResultCode {
        Self::discriminant(self)
    }
}

impl Variants<InvokeHostFunctionResultCode> for InvokeHostFunctionResult {
    fn variants() -> slice::Iter<'static, InvokeHostFunctionResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<InvokeHostFunctionResultCode> for InvokeHostFunctionResult {}

impl ReadXdr for InvokeHostFunctionResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: InvokeHostFunctionResultCode =
                <InvokeHostFunctionResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                InvokeHostFunctionResultCode::Success => Self::Success(Hash::read_xdr(r)?),
                InvokeHostFunctionResultCode::Malformed => Self::Malformed,
                InvokeHostFunctionResultCode::Trapped => Self::Trapped,
                InvokeHostFunctionResultCode::ResourceLimitExceeded => Self::ResourceLimitExceeded,
                InvokeHostFunctionResultCode::EntryArchived => Self::EntryArchived,
                InvokeHostFunctionResultCode::InsufficientRefundableFee => {
                    Self::InsufficientRefundableFee
                }
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for InvokeHostFunctionResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Success(v) => v.write_xdr(w)?,
                Self::Malformed => ().write_xdr(w)?,
                Self::Trapped => ().write_xdr(w)?,
                Self::ResourceLimitExceeded => ().write_xdr(w)?,
                Self::EntryArchived => ().write_xdr(w)?,
                Self::InsufficientRefundableFee => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`InvokeHostFunctionResult`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyInvokeHostFunctionResult(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyInvokeHostFunctionResult {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyInvokeHostFunctionResult {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let ord = self.discriminant().cmp(&other.discriminant());
        if ord != core::cmp::Ordering::Equal {
            return ord;
        }
        #[allow(clippy::match_same_arms)]
        match self.discriminant_i32() {
            0 => self.as_success().cmp(&other.as_success()),
            _ => core::cmp::Ordering::Equal,
        }
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyInvokeHostFunctionResult {
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
                let field_len = <LazyHash as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
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
                pos += <LazyHash as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyInvokeHostFunctionResult {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyInvokeHostFunctionResult {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyInvokeHostFunctionResult {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyInvokeHostFunctionResult {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> InvokeHostFunctionResultCode {
        // Validated — unwrap is safe.
        InvokeHostFunctionResultCode::try_from(self.discriminant_i32()).unwrap()
    }
    /// Access arm `Success`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_success(&self) -> Option<LazyHash> {
        if self.discriminant_i32() == 0 {
            Some(<LazyHash as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&InvokeHostFunctionResult> for LazyInvokeHostFunctionResult {
    type Error = Error;
    fn try_from(val: &InvokeHostFunctionResult) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyInvokeHostFunctionResult> for InvokeHostFunctionResult {
    type Error = Error;
    fn try_from(lazy: &LazyInvokeHostFunctionResult) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
