#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// LedgerEntry is an XDR Struct defined as:
///
/// ```text
/// struct LedgerEntry
/// {
///     uint32 lastModifiedLedgerSeq; // ledger the LedgerEntry was last changed
///
///     union switch (LedgerEntryType type)
///     {
///     case ACCOUNT:
///         AccountEntry account;
///     case TRUSTLINE:
///         TrustLineEntry trustLine;
///     case OFFER:
///         OfferEntry offer;
///     case DATA:
///         DataEntry data;
///     case CLAIMABLE_BALANCE:
///         ClaimableBalanceEntry claimableBalance;
///     case LIQUIDITY_POOL:
///         LiquidityPoolEntry liquidityPool;
///     case CONTRACT_DATA:
///         ContractDataEntry contractData;
///     case CONTRACT_CODE:
///         ContractCodeEntry contractCode;
///     case CONFIG_SETTING:
///         ConfigSettingEntry configSetting;
///     case TTL:
///         TTLEntry ttl;
///     }
///     data;
///
///     // reserved for future use
///     union switch (int v)
///     {
///     case 0:
///         void;
///     case 1:
///         LedgerEntryExtensionV1 v1;
///     }
///     ext;
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
pub struct LedgerEntry {
    pub last_modified_ledger_seq: u32,
    pub data: LedgerEntryData,
    pub ext: LedgerEntryExt,
}

impl ReadXdr for LedgerEntry {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                last_modified_ledger_seq: u32::read_xdr(r)?,
                data: LedgerEntryData::read_xdr(r)?,
                ext: LedgerEntryExt::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for LedgerEntry {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.last_modified_ledger_seq.write_xdr(w)?;
            self.data.write_xdr(w)?;
            self.ext.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`LedgerEntry`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyLedgerEntry(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyLedgerEntry {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyLedgerEntry {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| {
                self.last_modified_ledger_seq()
                    .cmp(&other.last_modified_ledger_seq())
            })
            .then_with(|| self.data().cmp(&other.data()))
            .then_with(|| self.ext().cmp(&other.ext()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyLedgerEntry {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        let next_pos = pos.checked_add(4).ok_or(Error::LengthExceedsMax)?;
        if buf.len() < next_pos as usize {
            return Err(Error::Invalid);
        }
        pos = next_pos;
        {
            let field_len =
                <LazyLedgerEntryData as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len =
                <LazyLedgerEntryExt as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += 4;
        pos += <LazyLedgerEntryData as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyLedgerEntryExt as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyLedgerEntry {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyLedgerEntry {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyLedgerEntry {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyLedgerEntry {
    /// Access field `last_modified_ledger_seq`.
    #[must_use]
    pub fn last_modified_ledger_seq(&self) -> u32 {
        <u32 as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `data`.
    #[must_use]
    pub fn data(&self) -> LazyLedgerEntryData {
        <LazyLedgerEntryData as LazyXdr>::from_xdr_at(&self.0, 4)
    }
    /// Access field `ext`.
    #[must_use]
    pub fn ext(&self) -> LazyLedgerEntryExt {
        let buf = self.0.as_slice();
        let mut pos: u32 = 4;
        pos += <LazyLedgerEntryData as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyLedgerEntryExt as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LedgerEntry> for LazyLedgerEntry {
    type Error = Error;
    fn try_from(val: &LedgerEntry) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyLedgerEntry> for LedgerEntry {
    type Error = Error;
    fn try_from(lazy: &LazyLedgerEntry) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
