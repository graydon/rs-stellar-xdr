#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// InflationResult is an XDR Union defined as:
///
/// ```text
/// union InflationResult switch (InflationResultCode code)
/// {
/// case INFLATION_SUCCESS:
///     InflationPayout payouts<>;
/// case INFLATION_NOT_TIME:
///     void;
/// };
/// ```
///
// union with discriminant InflationResultCode
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
pub enum InflationResult {
    Success(VecM<InflationPayout>),
    NotTime,
}

#[cfg(feature = "alloc")]
impl Default for InflationResult {
    fn default() -> Self {
        Self::Success(VecM::<InflationPayout>::default())
    }
}

impl InflationResult {
    const _VARIANTS: &[InflationResultCode] =
        &[InflationResultCode::Success, InflationResultCode::NotTime];
    pub const VARIANTS: [InflationResultCode; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["Success", "NotTime"];
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
            Self::NotTime => "NotTime",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> InflationResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success(_) => InflationResultCode::Success,
            Self::NotTime => InflationResultCode::NotTime,
        }
    }

    #[must_use]
    pub const fn variants() -> [InflationResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for InflationResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<InflationResultCode> for InflationResult {
    #[must_use]
    fn discriminant(&self) -> InflationResultCode {
        Self::discriminant(self)
    }
}

impl Variants<InflationResultCode> for InflationResult {
    fn variants() -> slice::Iter<'static, InflationResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<InflationResultCode> for InflationResult {}

impl ReadXdr for InflationResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: InflationResultCode = <InflationResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                InflationResultCode::Success => {
                    Self::Success(VecM::<InflationPayout>::read_xdr(r)?)
                }
                InflationResultCode::NotTime => Self::NotTime,
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for InflationResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Success(v) => v.write_xdr(w)?,
                Self::NotTime => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`InflationResult`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyInflationResult(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyInflationResult {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyInflationResult {
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
impl LazyXdr for LazyInflationResult {
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
                let field_len = <LazyVecM<LazyInflationPayout> as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            -1 => {
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
                pos += <LazyVecM<LazyInflationPayout> as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            -1 => {
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
impl From<LazyHandle> for LazyInflationResult {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyInflationResult {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyInflationResult {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyInflationResult {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> InflationResultCode {
        // Validated — unwrap is safe.
        InflationResultCode::try_from(self.discriminant_i32()).unwrap()
    }
    /// Access arm `Success`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_success(&self) -> Option<LazyVecM<LazyInflationPayout>> {
        if self.discriminant_i32() == 0 {
            Some(<LazyVecM<LazyInflationPayout> as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&InflationResult> for LazyInflationResult {
    type Error = Error;
    fn try_from(val: &InflationResult) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyInflationResult> for InflationResult {
    type Error = Error;
    fn try_from(lazy: &LazyInflationResult) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
