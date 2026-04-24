#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// SetOptionsOp is an XDR Struct defined as:
///
/// ```text
/// struct SetOptionsOp
/// {
///     AccountID* inflationDest; // sets the inflation destination
///
///     uint32* clearFlags; // which flags to clear
///     uint32* setFlags;   // which flags to set
///
///     // account threshold manipulation
///     uint32* masterWeight; // weight of the master account
///     uint32* lowThreshold;
///     uint32* medThreshold;
///     uint32* highThreshold;
///
///     string32* homeDomain; // sets the home domain
///
///     // Add, update or remove a signer for the account
///     // signer is deleted if the weight is 0
///     Signer* signer;
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
pub struct SetOptionsOp {
    pub inflation_dest: Option<AccountId>,
    pub clear_flags: Option<u32>,
    pub set_flags: Option<u32>,
    pub master_weight: Option<u32>,
    pub low_threshold: Option<u32>,
    pub med_threshold: Option<u32>,
    pub high_threshold: Option<u32>,
    pub home_domain: Option<String32>,
    pub signer: Option<Signer>,
}

impl ReadXdr for SetOptionsOp {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                inflation_dest: Option::<AccountId>::read_xdr(r)?,
                clear_flags: Option::<u32>::read_xdr(r)?,
                set_flags: Option::<u32>::read_xdr(r)?,
                master_weight: Option::<u32>::read_xdr(r)?,
                low_threshold: Option::<u32>::read_xdr(r)?,
                med_threshold: Option::<u32>::read_xdr(r)?,
                high_threshold: Option::<u32>::read_xdr(r)?,
                home_domain: Option::<String32>::read_xdr(r)?,
                signer: Option::<Signer>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SetOptionsOp {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.inflation_dest.write_xdr(w)?;
            self.clear_flags.write_xdr(w)?;
            self.set_flags.write_xdr(w)?;
            self.master_weight.write_xdr(w)?;
            self.low_threshold.write_xdr(w)?;
            self.med_threshold.write_xdr(w)?;
            self.high_threshold.write_xdr(w)?;
            self.home_domain.write_xdr(w)?;
            self.signer.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`SetOptionsOp`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazySetOptionsOp(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazySetOptionsOp {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazySetOptionsOp {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.inflation_dest().cmp(&other.inflation_dest()))
            .then_with(|| self.clear_flags().cmp(&other.clear_flags()))
            .then_with(|| self.set_flags().cmp(&other.set_flags()))
            .then_with(|| self.master_weight().cmp(&other.master_weight()))
            .then_with(|| self.low_threshold().cmp(&other.low_threshold()))
            .then_with(|| self.med_threshold().cmp(&other.med_threshold()))
            .then_with(|| self.high_threshold().cmp(&other.high_threshold()))
            .then_with(|| self.home_domain().cmp(&other.home_domain()))
            .then_with(|| self.signer().cmp(&other.signer()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazySetOptionsOp {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len =
                <LazyOption<LazyAccountId> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len =
                <LazyOption<u32> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len =
                <LazyOption<u32> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len =
                <LazyOption<u32> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len =
                <LazyOption<u32> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len =
                <LazyOption<u32> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len =
                <LazyOption<u32> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len =
                <LazyOption<LazyString32> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len =
                <LazyOption<LazySigner> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyOption<LazyAccountId> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<u32> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<u32> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<u32> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<u32> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<u32> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<u32> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<LazyString32> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<LazySigner> as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazySetOptionsOp {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazySetOptionsOp {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazySetOptionsOp {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazySetOptionsOp {
    /// Access field `inflation_dest`.
    #[must_use]
    pub fn inflation_dest(&self) -> LazyOption<LazyAccountId> {
        <LazyOption<LazyAccountId> as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `clear_flags`.
    #[must_use]
    pub fn clear_flags(&self) -> LazyOption<u32> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyOption<LazyAccountId> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyOption<u32> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `set_flags`.
    #[must_use]
    pub fn set_flags(&self) -> LazyOption<u32> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyOption<LazyAccountId> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<u32> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyOption<u32> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `master_weight`.
    #[must_use]
    pub fn master_weight(&self) -> LazyOption<u32> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyOption<LazyAccountId> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<u32> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<u32> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyOption<u32> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `low_threshold`.
    #[must_use]
    pub fn low_threshold(&self) -> LazyOption<u32> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyOption<LazyAccountId> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<u32> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<u32> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<u32> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyOption<u32> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `med_threshold`.
    #[must_use]
    pub fn med_threshold(&self) -> LazyOption<u32> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyOption<LazyAccountId> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<u32> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<u32> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<u32> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<u32> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyOption<u32> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `high_threshold`.
    #[must_use]
    pub fn high_threshold(&self) -> LazyOption<u32> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyOption<LazyAccountId> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<u32> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<u32> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<u32> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<u32> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<u32> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyOption<u32> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `home_domain`.
    #[must_use]
    pub fn home_domain(&self) -> LazyOption<LazyString32> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyOption<LazyAccountId> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<u32> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<u32> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<u32> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<u32> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<u32> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<u32> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyOption<LazyString32> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
    /// Access field `signer`.
    #[must_use]
    pub fn signer(&self) -> LazyOption<LazySigner> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyOption<LazyAccountId> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<u32> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<u32> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<u32> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<u32> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<u32> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<u32> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyOption<LazyString32> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyOption<LazySigner> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&SetOptionsOp> for LazySetOptionsOp {
    type Error = Error;
    fn try_from(val: &SetOptionsOp) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazySetOptionsOp> for SetOptionsOp {
    type Error = Error;
    fn try_from(lazy: &LazySetOptionsOp) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
