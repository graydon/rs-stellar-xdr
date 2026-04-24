#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ScSpecEntry is an XDR Union defined as:
///
/// ```text
/// union SCSpecEntry switch (SCSpecEntryKind kind)
/// {
/// case SC_SPEC_ENTRY_FUNCTION_V0:
///     SCSpecFunctionV0 functionV0;
/// case SC_SPEC_ENTRY_UDT_STRUCT_V0:
///     SCSpecUDTStructV0 udtStructV0;
/// case SC_SPEC_ENTRY_UDT_UNION_V0:
///     SCSpecUDTUnionV0 udtUnionV0;
/// case SC_SPEC_ENTRY_UDT_ENUM_V0:
///     SCSpecUDTEnumV0 udtEnumV0;
/// case SC_SPEC_ENTRY_UDT_ERROR_ENUM_V0:
///     SCSpecUDTErrorEnumV0 udtErrorEnumV0;
/// case SC_SPEC_ENTRY_EVENT_V0:
///     SCSpecEventV0 eventV0;
/// };
/// ```
///
// union with discriminant ScSpecEntryKind
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
pub enum ScSpecEntry {
    FunctionV0(ScSpecFunctionV0),
    UdtStructV0(ScSpecUdtStructV0),
    UdtUnionV0(ScSpecUdtUnionV0),
    UdtEnumV0(ScSpecUdtEnumV0),
    UdtErrorEnumV0(ScSpecUdtErrorEnumV0),
    EventV0(ScSpecEventV0),
}

#[cfg(feature = "alloc")]
impl Default for ScSpecEntry {
    fn default() -> Self {
        Self::FunctionV0(ScSpecFunctionV0::default())
    }
}

impl ScSpecEntry {
    const _VARIANTS: &[ScSpecEntryKind] = &[
        ScSpecEntryKind::FunctionV0,
        ScSpecEntryKind::UdtStructV0,
        ScSpecEntryKind::UdtUnionV0,
        ScSpecEntryKind::UdtEnumV0,
        ScSpecEntryKind::UdtErrorEnumV0,
        ScSpecEntryKind::EventV0,
    ];
    pub const VARIANTS: [ScSpecEntryKind; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "FunctionV0",
        "UdtStructV0",
        "UdtUnionV0",
        "UdtEnumV0",
        "UdtErrorEnumV0",
        "EventV0",
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
            Self::FunctionV0(_) => "FunctionV0",
            Self::UdtStructV0(_) => "UdtStructV0",
            Self::UdtUnionV0(_) => "UdtUnionV0",
            Self::UdtEnumV0(_) => "UdtEnumV0",
            Self::UdtErrorEnumV0(_) => "UdtErrorEnumV0",
            Self::EventV0(_) => "EventV0",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> ScSpecEntryKind {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::FunctionV0(_) => ScSpecEntryKind::FunctionV0,
            Self::UdtStructV0(_) => ScSpecEntryKind::UdtStructV0,
            Self::UdtUnionV0(_) => ScSpecEntryKind::UdtUnionV0,
            Self::UdtEnumV0(_) => ScSpecEntryKind::UdtEnumV0,
            Self::UdtErrorEnumV0(_) => ScSpecEntryKind::UdtErrorEnumV0,
            Self::EventV0(_) => ScSpecEntryKind::EventV0,
        }
    }

    #[must_use]
    pub const fn variants() -> [ScSpecEntryKind; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ScSpecEntry {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ScSpecEntryKind> for ScSpecEntry {
    #[must_use]
    fn discriminant(&self) -> ScSpecEntryKind {
        Self::discriminant(self)
    }
}

impl Variants<ScSpecEntryKind> for ScSpecEntry {
    fn variants() -> slice::Iter<'static, ScSpecEntryKind> {
        Self::VARIANTS.iter()
    }
}

impl Union<ScSpecEntryKind> for ScSpecEntry {}

impl ReadXdr for ScSpecEntry {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ScSpecEntryKind = <ScSpecEntryKind as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ScSpecEntryKind::FunctionV0 => Self::FunctionV0(ScSpecFunctionV0::read_xdr(r)?),
                ScSpecEntryKind::UdtStructV0 => Self::UdtStructV0(ScSpecUdtStructV0::read_xdr(r)?),
                ScSpecEntryKind::UdtUnionV0 => Self::UdtUnionV0(ScSpecUdtUnionV0::read_xdr(r)?),
                ScSpecEntryKind::UdtEnumV0 => Self::UdtEnumV0(ScSpecUdtEnumV0::read_xdr(r)?),
                ScSpecEntryKind::UdtErrorEnumV0 => {
                    Self::UdtErrorEnumV0(ScSpecUdtErrorEnumV0::read_xdr(r)?)
                }
                ScSpecEntryKind::EventV0 => Self::EventV0(ScSpecEventV0::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for ScSpecEntry {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::FunctionV0(v) => v.write_xdr(w)?,
                Self::UdtStructV0(v) => v.write_xdr(w)?,
                Self::UdtUnionV0(v) => v.write_xdr(w)?,
                Self::UdtEnumV0(v) => v.write_xdr(w)?,
                Self::UdtErrorEnumV0(v) => v.write_xdr(w)?,
                Self::EventV0(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ScSpecEntry`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyScSpecEntry(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyScSpecEntry {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyScSpecEntry {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let ord = self.discriminant().cmp(&other.discriminant());
        if ord != core::cmp::Ordering::Equal {
            return ord;
        }
        #[allow(clippy::match_same_arms)]
        match self.discriminant_i32() {
            0 => self.as_function_v0().cmp(&other.as_function_v0()),
            1 => self.as_udt_struct_v0().cmp(&other.as_udt_struct_v0()),
            2 => self.as_udt_union_v0().cmp(&other.as_udt_union_v0()),
            3 => self.as_udt_enum_v0().cmp(&other.as_udt_enum_v0()),
            4 => self
                .as_udt_error_enum_v0()
                .cmp(&other.as_udt_error_enum_v0()),
            5 => self.as_event_v0().cmp(&other.as_event_v0()),
            _ => core::cmp::Ordering::Equal,
        }
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyScSpecEntry {
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
                    <LazyScSpecFunctionV0 as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            1 => {
                let field_len =
                    <LazyScSpecUdtStructV0 as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            2 => {
                let field_len =
                    <LazyScSpecUdtUnionV0 as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            3 => {
                let field_len =
                    <LazyScSpecUdtEnumV0 as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            4 => {
                let field_len = <LazyScSpecUdtErrorEnumV0 as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            5 => {
                let field_len =
                    <LazyScSpecEventV0 as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
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
                pos += <LazyScSpecFunctionV0 as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            1 => {
                pos += <LazyScSpecUdtStructV0 as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            2 => {
                pos += <LazyScSpecUdtUnionV0 as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            3 => {
                pos += <LazyScSpecUdtEnumV0 as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            4 => {
                pos += <LazyScSpecUdtErrorEnumV0 as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            5 => {
                pos += <LazyScSpecEventV0 as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyScSpecEntry {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyScSpecEntry {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyScSpecEntry {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyScSpecEntry {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> ScSpecEntryKind {
        // Validated — unwrap is safe.
        ScSpecEntryKind::try_from(self.discriminant_i32()).unwrap()
    }
    /// Access arm `FunctionV0`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_function_v0(&self) -> Option<LazyScSpecFunctionV0> {
        if self.discriminant_i32() == 0 {
            Some(<LazyScSpecFunctionV0 as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `UdtStructV0`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_udt_struct_v0(&self) -> Option<LazyScSpecUdtStructV0> {
        if self.discriminant_i32() == 1 {
            Some(<LazyScSpecUdtStructV0 as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `UdtUnionV0`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_udt_union_v0(&self) -> Option<LazyScSpecUdtUnionV0> {
        if self.discriminant_i32() == 2 {
            Some(<LazyScSpecUdtUnionV0 as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `UdtEnumV0`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_udt_enum_v0(&self) -> Option<LazyScSpecUdtEnumV0> {
        if self.discriminant_i32() == 3 {
            Some(<LazyScSpecUdtEnumV0 as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `UdtErrorEnumV0`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_udt_error_enum_v0(&self) -> Option<LazyScSpecUdtErrorEnumV0> {
        if self.discriminant_i32() == 4 {
            Some(<LazyScSpecUdtErrorEnumV0 as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `EventV0`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_event_v0(&self) -> Option<LazyScSpecEventV0> {
        if self.discriminant_i32() == 5 {
            Some(<LazyScSpecEventV0 as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ScSpecEntry> for LazyScSpecEntry {
    type Error = Error;
    fn try_from(val: &ScSpecEntry) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyScSpecEntry> for ScSpecEntry {
    type Error = Error;
    fn try_from(lazy: &LazyScSpecEntry) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
