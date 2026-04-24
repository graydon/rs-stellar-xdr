#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ScSpecUdtUnionCaseV0 is an XDR Union defined as:
///
/// ```text
/// union SCSpecUDTUnionCaseV0 switch (SCSpecUDTUnionCaseV0Kind kind)
/// {
/// case SC_SPEC_UDT_UNION_CASE_VOID_V0:
///     SCSpecUDTUnionCaseVoidV0 voidCase;
/// case SC_SPEC_UDT_UNION_CASE_TUPLE_V0:
///     SCSpecUDTUnionCaseTupleV0 tupleCase;
/// };
/// ```
///
// union with discriminant ScSpecUdtUnionCaseV0Kind
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
pub enum ScSpecUdtUnionCaseV0 {
    VoidV0(ScSpecUdtUnionCaseVoidV0),
    TupleV0(ScSpecUdtUnionCaseTupleV0),
}

#[cfg(feature = "alloc")]
impl Default for ScSpecUdtUnionCaseV0 {
    fn default() -> Self {
        Self::VoidV0(ScSpecUdtUnionCaseVoidV0::default())
    }
}

impl ScSpecUdtUnionCaseV0 {
    const _VARIANTS: &[ScSpecUdtUnionCaseV0Kind] = &[
        ScSpecUdtUnionCaseV0Kind::VoidV0,
        ScSpecUdtUnionCaseV0Kind::TupleV0,
    ];
    pub const VARIANTS: [ScSpecUdtUnionCaseV0Kind; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["VoidV0", "TupleV0"];
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
            Self::VoidV0(_) => "VoidV0",
            Self::TupleV0(_) => "TupleV0",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> ScSpecUdtUnionCaseV0Kind {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::VoidV0(_) => ScSpecUdtUnionCaseV0Kind::VoidV0,
            Self::TupleV0(_) => ScSpecUdtUnionCaseV0Kind::TupleV0,
        }
    }

    #[must_use]
    pub const fn variants() -> [ScSpecUdtUnionCaseV0Kind; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ScSpecUdtUnionCaseV0 {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ScSpecUdtUnionCaseV0Kind> for ScSpecUdtUnionCaseV0 {
    #[must_use]
    fn discriminant(&self) -> ScSpecUdtUnionCaseV0Kind {
        Self::discriminant(self)
    }
}

impl Variants<ScSpecUdtUnionCaseV0Kind> for ScSpecUdtUnionCaseV0 {
    fn variants() -> slice::Iter<'static, ScSpecUdtUnionCaseV0Kind> {
        Self::VARIANTS.iter()
    }
}

impl Union<ScSpecUdtUnionCaseV0Kind> for ScSpecUdtUnionCaseV0 {}

impl ReadXdr for ScSpecUdtUnionCaseV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ScSpecUdtUnionCaseV0Kind = <ScSpecUdtUnionCaseV0Kind as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ScSpecUdtUnionCaseV0Kind::VoidV0 => {
                    Self::VoidV0(ScSpecUdtUnionCaseVoidV0::read_xdr(r)?)
                }
                ScSpecUdtUnionCaseV0Kind::TupleV0 => {
                    Self::TupleV0(ScSpecUdtUnionCaseTupleV0::read_xdr(r)?)
                }
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for ScSpecUdtUnionCaseV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::VoidV0(v) => v.write_xdr(w)?,
                Self::TupleV0(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ScSpecUdtUnionCaseV0`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyScSpecUdtUnionCaseV0(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyScSpecUdtUnionCaseV0 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyScSpecUdtUnionCaseV0 {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let ord = self.discriminant().cmp(&other.discriminant());
        if ord != core::cmp::Ordering::Equal {
            return ord;
        }
        #[allow(clippy::match_same_arms)]
        match self.discriminant_i32() {
            0 => self.as_void_v0().cmp(&other.as_void_v0()),
            1 => self.as_tuple_v0().cmp(&other.as_tuple_v0()),
            _ => core::cmp::Ordering::Equal,
        }
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyScSpecUdtUnionCaseV0 {
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
                let field_len = <LazyScSpecUdtUnionCaseVoidV0 as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            1 => {
                let field_len = <LazyScSpecUdtUnionCaseTupleV0 as LazyXdr>::xdr_validate(
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
                pos += <LazyScSpecUdtUnionCaseVoidV0 as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            1 => {
                pos += <LazyScSpecUdtUnionCaseTupleV0 as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyScSpecUdtUnionCaseV0 {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyScSpecUdtUnionCaseV0 {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyScSpecUdtUnionCaseV0 {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyScSpecUdtUnionCaseV0 {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> ScSpecUdtUnionCaseV0Kind {
        // Validated — unwrap is safe.
        ScSpecUdtUnionCaseV0Kind::try_from(self.discriminant_i32()).unwrap()
    }
    /// Access arm `VoidV0`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_void_v0(&self) -> Option<LazyScSpecUdtUnionCaseVoidV0> {
        if self.discriminant_i32() == 0 {
            Some(<LazyScSpecUdtUnionCaseVoidV0 as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `TupleV0`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_tuple_v0(&self) -> Option<LazyScSpecUdtUnionCaseTupleV0> {
        if self.discriminant_i32() == 1 {
            Some(<LazyScSpecUdtUnionCaseTupleV0 as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ScSpecUdtUnionCaseV0> for LazyScSpecUdtUnionCaseV0 {
    type Error = Error;
    fn try_from(val: &ScSpecUdtUnionCaseV0) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyScSpecUdtUnionCaseV0> for ScSpecUdtUnionCaseV0 {
    type Error = Error;
    fn try_from(lazy: &LazyScSpecUdtUnionCaseV0) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
