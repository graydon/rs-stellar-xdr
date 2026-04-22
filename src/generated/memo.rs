#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// Memo is an XDR Union defined as:
///
/// ```text
/// union Memo switch (MemoType type)
/// {
/// case MEMO_NONE:
///     void;
/// case MEMO_TEXT:
///     string text<28>;
/// case MEMO_ID:
///     uint64 id;
/// case MEMO_HASH:
///     Hash hash; // the hash of what to pull from the content server
/// case MEMO_RETURN:
///     Hash retHash; // the hash of the tx you are rejecting
/// };
/// ```
///
// union with discriminant MemoType
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
pub enum Memo {
    None,
    Text(StringM<28>),
    Id(
        #[cfg_attr(
            all(feature = "serde", feature = "alloc"),
            serde_as(as = "NumberOrString")
        )]
        u64,
    ),
    Hash(Hash),
    Return(Hash),
}

#[cfg(feature = "alloc")]
impl Default for Memo {
    fn default() -> Self {
        Self::None
    }
}

impl Memo {
    const _VARIANTS: &[MemoType] = &[
        MemoType::None,
        MemoType::Text,
        MemoType::Id,
        MemoType::Hash,
        MemoType::Return,
    ];
    pub const VARIANTS: [MemoType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["None", "Text", "Id", "Hash", "Return"];
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
            Self::None => "None",
            Self::Text(_) => "Text",
            Self::Id(_) => "Id",
            Self::Hash(_) => "Hash",
            Self::Return(_) => "Return",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> MemoType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::None => MemoType::None,
            Self::Text(_) => MemoType::Text,
            Self::Id(_) => MemoType::Id,
            Self::Hash(_) => MemoType::Hash,
            Self::Return(_) => MemoType::Return,
        }
    }

    #[must_use]
    pub const fn variants() -> [MemoType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for Memo {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<MemoType> for Memo {
    #[must_use]
    fn discriminant(&self) -> MemoType {
        Self::discriminant(self)
    }
}

impl Variants<MemoType> for Memo {
    fn variants() -> slice::Iter<'static, MemoType> {
        Self::VARIANTS.iter()
    }
}

impl Union<MemoType> for Memo {}

impl ReadXdr for Memo {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: MemoType = <MemoType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                MemoType::None => Self::None,
                MemoType::Text => Self::Text(StringM::<28>::read_xdr(r)?),
                MemoType::Id => Self::Id(u64::read_xdr(r)?),
                MemoType::Hash => Self::Hash(Hash::read_xdr(r)?),
                MemoType::Return => Self::Return(Hash::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for Memo {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::None => ().write_xdr(w)?,
                Self::Text(v) => v.write_xdr(w)?,
                Self::Id(v) => v.write_xdr(w)?,
                Self::Hash(v) => v.write_xdr(w)?,
                Self::Return(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`Memo`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyMemo(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyMemo {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyMemo {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let ord = self.discriminant().cmp(&other.discriminant());
        if ord != core::cmp::Ordering::Equal {
            return ord;
        }
        #[allow(clippy::match_same_arms)]
        match self.discriminant_i32() {
            1 => self.as_text().cmp(&other.as_text()),
            2 => self.as_id().cmp(&other.as_id()),
            3 => self.as_hash().cmp(&other.as_hash()),
            4 => self.as_return().cmp(&other.as_return()),
            _ => core::cmp::Ordering::Equal,
        }
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyMemo {
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
                // void — no additional data
            }
            1 => {
                let field_len =
                    <LazyStringM<28> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            2 => {
                let field_len = <u64 as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            3 => {
                let field_len = <LazyHash as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            4 => {
                let field_len = <LazyHash as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
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
                // void
            }
            1 => {
                pos += <LazyStringM<28> as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            2 => {
                pos += <u64 as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            3 => {
                pos += <LazyHash as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            4 => {
                pos += <LazyHash as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyMemo {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyMemo {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyMemo {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyMemo {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> MemoType {
        // Validated — unwrap is safe.
        MemoType::try_from(self.discriminant_i32()).unwrap()
    }
    /// Access arm `Text`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_text(&self) -> Option<LazyStringM<28>> {
        if self.discriminant_i32() == 1 {
            Some(<LazyStringM<28> as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Id`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_id(&self) -> Option<u64> {
        if self.discriminant_i32() == 2 {
            Some(<u64 as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Hash`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_hash(&self) -> Option<LazyHash> {
        if self.discriminant_i32() == 3 {
            Some(<LazyHash as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Return`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_return(&self) -> Option<LazyHash> {
        if self.discriminant_i32() == 4 {
            Some(<LazyHash as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&Memo> for LazyMemo {
    type Error = Error;
    fn try_from(val: &Memo) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyMemo> for Memo {
    type Error = Error;
    fn try_from(lazy: &LazyMemo) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
