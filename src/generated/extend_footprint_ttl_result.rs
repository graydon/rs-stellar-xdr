#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ExtendFootprintTtlResult is an XDR Union defined as:
///
/// ```text
/// union ExtendFootprintTTLResult switch (ExtendFootprintTTLResultCode code)
/// {
/// case EXTEND_FOOTPRINT_TTL_SUCCESS:
///     void;
/// case EXTEND_FOOTPRINT_TTL_MALFORMED:
/// case EXTEND_FOOTPRINT_TTL_RESOURCE_LIMIT_EXCEEDED:
/// case EXTEND_FOOTPRINT_TTL_INSUFFICIENT_REFUNDABLE_FEE:
///     void;
/// };
/// ```
///
// union with discriminant ExtendFootprintTtlResultCode
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
pub enum ExtendFootprintTtlResult {
    Success,
    Malformed,
    ResourceLimitExceeded,
    InsufficientRefundableFee,
}

#[cfg(feature = "alloc")]
impl Default for ExtendFootprintTtlResult {
    fn default() -> Self {
        Self::Success
    }
}

impl ExtendFootprintTtlResult {
    const _VARIANTS: &[ExtendFootprintTtlResultCode] = &[
        ExtendFootprintTtlResultCode::Success,
        ExtendFootprintTtlResultCode::Malformed,
        ExtendFootprintTtlResultCode::ResourceLimitExceeded,
        ExtendFootprintTtlResultCode::InsufficientRefundableFee,
    ];
    pub const VARIANTS: [ExtendFootprintTtlResultCode; Self::_VARIANTS.len()] = {
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
        "ResourceLimitExceeded",
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
            Self::Success => "Success",
            Self::Malformed => "Malformed",
            Self::ResourceLimitExceeded => "ResourceLimitExceeded",
            Self::InsufficientRefundableFee => "InsufficientRefundableFee",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> ExtendFootprintTtlResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => ExtendFootprintTtlResultCode::Success,
            Self::Malformed => ExtendFootprintTtlResultCode::Malformed,
            Self::ResourceLimitExceeded => ExtendFootprintTtlResultCode::ResourceLimitExceeded,
            Self::InsufficientRefundableFee => {
                ExtendFootprintTtlResultCode::InsufficientRefundableFee
            }
        }
    }

    #[must_use]
    pub const fn variants() -> [ExtendFootprintTtlResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ExtendFootprintTtlResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ExtendFootprintTtlResultCode> for ExtendFootprintTtlResult {
    #[must_use]
    fn discriminant(&self) -> ExtendFootprintTtlResultCode {
        Self::discriminant(self)
    }
}

impl Variants<ExtendFootprintTtlResultCode> for ExtendFootprintTtlResult {
    fn variants() -> slice::Iter<'static, ExtendFootprintTtlResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<ExtendFootprintTtlResultCode> for ExtendFootprintTtlResult {}

impl ReadXdr for ExtendFootprintTtlResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ExtendFootprintTtlResultCode =
                <ExtendFootprintTtlResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ExtendFootprintTtlResultCode::Success => Self::Success,
                ExtendFootprintTtlResultCode::Malformed => Self::Malformed,
                ExtendFootprintTtlResultCode::ResourceLimitExceeded => Self::ResourceLimitExceeded,
                ExtendFootprintTtlResultCode::InsufficientRefundableFee => {
                    Self::InsufficientRefundableFee
                }
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for ExtendFootprintTtlResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Success => ().write_xdr(w)?,
                Self::Malformed => ().write_xdr(w)?,
                Self::ResourceLimitExceeded => ().write_xdr(w)?,
                Self::InsufficientRefundableFee => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ExtendFootprintTtlResult`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyExtendFootprintTtlResult(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyExtendFootprintTtlResult {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyExtendFootprintTtlResult {
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
impl LazyXdr for LazyExtendFootprintTtlResult {
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
impl From<LazyHandle> for LazyExtendFootprintTtlResult {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyExtendFootprintTtlResult {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyExtendFootprintTtlResult {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyExtendFootprintTtlResult {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> ExtendFootprintTtlResultCode {
        // Validated — unwrap is safe.
        ExtendFootprintTtlResultCode::try_from(self.discriminant_i32()).unwrap()
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ExtendFootprintTtlResult> for LazyExtendFootprintTtlResult {
    type Error = Error;
    fn try_from(val: &ExtendFootprintTtlResult) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyExtendFootprintTtlResult> for ExtendFootprintTtlResult {
    type Error = Error;
    fn try_from(lazy: &LazyExtendFootprintTtlResult) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
