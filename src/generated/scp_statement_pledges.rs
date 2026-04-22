#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ScpStatementPledges is an XDR NestedUnion defined as:
///
/// ```text
/// union switch (SCPStatementType type)
///     {
///     case SCP_ST_PREPARE:
///         struct
///         {
///             Hash quorumSetHash;       // D
///             SCPBallot ballot;         // b
///             SCPBallot* prepared;      // p
///             SCPBallot* preparedPrime; // p'
///             uint32 nC;                // c.n
///             uint32 nH;                // h.n
///         } prepare;
///     case SCP_ST_CONFIRM:
///         struct
///         {
///             SCPBallot ballot;   // b
///             uint32 nPrepared;   // p.n
///             uint32 nCommit;     // c.n
///             uint32 nH;          // h.n
///             Hash quorumSetHash; // D
///         } confirm;
///     case SCP_ST_EXTERNALIZE:
///         struct
///         {
///             SCPBallot commit;         // c
///             uint32 nH;                // h.n
///             Hash commitQuorumSetHash; // D used before EXTERNALIZE
///         } externalize;
///     case SCP_ST_NOMINATE:
///         SCPNomination nominate;
///     }
/// ```
///
// union with discriminant ScpStatementType
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
pub enum ScpStatementPledges {
    Prepare(ScpStatementPrepare),
    Confirm(ScpStatementConfirm),
    Externalize(ScpStatementExternalize),
    Nominate(ScpNomination),
}

#[cfg(feature = "alloc")]
impl Default for ScpStatementPledges {
    fn default() -> Self {
        Self::Prepare(ScpStatementPrepare::default())
    }
}

impl ScpStatementPledges {
    const _VARIANTS: &[ScpStatementType] = &[
        ScpStatementType::Prepare,
        ScpStatementType::Confirm,
        ScpStatementType::Externalize,
        ScpStatementType::Nominate,
    ];
    pub const VARIANTS: [ScpStatementType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["Prepare", "Confirm", "Externalize", "Nominate"];
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
            Self::Prepare(_) => "Prepare",
            Self::Confirm(_) => "Confirm",
            Self::Externalize(_) => "Externalize",
            Self::Nominate(_) => "Nominate",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> ScpStatementType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Prepare(_) => ScpStatementType::Prepare,
            Self::Confirm(_) => ScpStatementType::Confirm,
            Self::Externalize(_) => ScpStatementType::Externalize,
            Self::Nominate(_) => ScpStatementType::Nominate,
        }
    }

    #[must_use]
    pub const fn variants() -> [ScpStatementType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ScpStatementPledges {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ScpStatementType> for ScpStatementPledges {
    #[must_use]
    fn discriminant(&self) -> ScpStatementType {
        Self::discriminant(self)
    }
}

impl Variants<ScpStatementType> for ScpStatementPledges {
    fn variants() -> slice::Iter<'static, ScpStatementType> {
        Self::VARIANTS.iter()
    }
}

impl Union<ScpStatementType> for ScpStatementPledges {}

impl ReadXdr for ScpStatementPledges {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ScpStatementType = <ScpStatementType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ScpStatementType::Prepare => Self::Prepare(ScpStatementPrepare::read_xdr(r)?),
                ScpStatementType::Confirm => Self::Confirm(ScpStatementConfirm::read_xdr(r)?),
                ScpStatementType::Externalize => {
                    Self::Externalize(ScpStatementExternalize::read_xdr(r)?)
                }
                ScpStatementType::Nominate => Self::Nominate(ScpNomination::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for ScpStatementPledges {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Prepare(v) => v.write_xdr(w)?,
                Self::Confirm(v) => v.write_xdr(w)?,
                Self::Externalize(v) => v.write_xdr(w)?,
                Self::Nominate(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ScpStatementPledges`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyScpStatementPledges(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyScpStatementPledges {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyScpStatementPledges {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let ord = self.discriminant().cmp(&other.discriminant());
        if ord != core::cmp::Ordering::Equal {
            return ord;
        }
        #[allow(clippy::match_same_arms)]
        match self.discriminant_i32() {
            0 => self.as_prepare().cmp(&other.as_prepare()),
            1 => self.as_confirm().cmp(&other.as_confirm()),
            2 => self.as_externalize().cmp(&other.as_externalize()),
            3 => self.as_nominate().cmp(&other.as_nominate()),
            _ => core::cmp::Ordering::Equal,
        }
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyScpStatementPledges {
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
                let field_len = <LazyScpStatementPrepare as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            1 => {
                let field_len = <LazyScpStatementConfirm as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            2 => {
                let field_len = <LazyScpStatementExternalize as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            3 => {
                let field_len =
                    <LazyScpNomination as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
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
                pos += <LazyScpStatementPrepare as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            1 => {
                pos += <LazyScpStatementConfirm as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            2 => {
                pos += <LazyScpStatementExternalize as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            3 => {
                pos += <LazyScpNomination as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyScpStatementPledges {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyScpStatementPledges {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyScpStatementPledges {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyScpStatementPledges {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> ScpStatementType {
        // Validated — unwrap is safe.
        ScpStatementType::try_from(self.discriminant_i32()).unwrap()
    }
    /// Access arm `Prepare`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_prepare(&self) -> Option<LazyScpStatementPrepare> {
        if self.discriminant_i32() == 0 {
            Some(<LazyScpStatementPrepare as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `Confirm`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_confirm(&self) -> Option<LazyScpStatementConfirm> {
        if self.discriminant_i32() == 1 {
            Some(<LazyScpStatementConfirm as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `Externalize`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_externalize(&self) -> Option<LazyScpStatementExternalize> {
        if self.discriminant_i32() == 2 {
            Some(<LazyScpStatementExternalize as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `Nominate`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_nominate(&self) -> Option<LazyScpNomination> {
        if self.discriminant_i32() == 3 {
            Some(<LazyScpNomination as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ScpStatementPledges> for LazyScpStatementPledges {
    type Error = Error;
    fn try_from(val: &ScpStatementPledges) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyScpStatementPledges> for ScpStatementPledges {
    type Error = Error;
    fn try_from(lazy: &LazyScpStatementPledges) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
