#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// ParallelTxsComponent is an XDR Struct defined as:
///
/// ```text
/// struct ParallelTxsComponent
/// {
///   int64* baseFee;
///   // A sequence of stages that *may* have arbitrary data dependencies between
///   // each other, i.e. in a general case the stage execution order may not be
///   // arbitrarily shuffled without affecting the end result.
///   ParallelTxExecutionStage executionStages<>;
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
pub struct ParallelTxsComponent {
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "Option<NumberOrString>")
    )]
    pub base_fee: Option<i64>,
    pub execution_stages: VecM<ParallelTxExecutionStage>,
}

impl ReadXdr for ParallelTxsComponent {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                base_fee: Option::<i64>::read_xdr(r)?,
                execution_stages: VecM::<ParallelTxExecutionStage>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ParallelTxsComponent {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.base_fee.write_xdr(w)?;
            self.execution_stages.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`ParallelTxsComponent`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyParallelTxsComponent(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyParallelTxsComponent {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyParallelTxsComponent {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
            .then_with(|| self.base_fee().cmp(&other.base_fee()))
            .then_with(|| self.execution_stages().cmp(&other.execution_stages()))
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyParallelTxsComponent {
    fn xdr_validate(buf: &[u8], depth: u32) -> Result<u32, Error> {
        #[allow(unused_variables)]
        let depth = depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)?;
        let mut pos: u32 = 0;
        {
            let field_len =
                <LazyOption<i64> as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        {
            let field_len = <LazyVecM<LazyParallelTxExecutionStage> as LazyXdr>::xdr_validate(
                &buf[pos as usize..],
                depth,
            )?;
            pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
        }
        Ok(pos)
    }

    fn xdr_len(buf: &[u8]) -> u32 {
        let mut pos: u32 = 0;
        pos += <LazyOption<i64> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        pos += <LazyVecM<LazyParallelTxExecutionStage> as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyParallelTxsComponent {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyParallelTxsComponent {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyParallelTxsComponent {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyParallelTxsComponent {
    /// Access field `base_fee`.
    #[must_use]
    pub fn base_fee(&self) -> LazyOption<i64> {
        <LazyOption<i64> as LazyXdr>::from_xdr_at(&self.0, 0)
    }
    /// Access field `execution_stages`.
    #[must_use]
    pub fn execution_stages(&self) -> LazyVecM<LazyParallelTxExecutionStage> {
        let buf = self.0.as_slice();
        let mut pos: u32 = 0;
        pos += <LazyOption<i64> as LazyXdr>::xdr_len(&buf[pos as usize..]);
        <LazyVecM<LazyParallelTxExecutionStage> as LazyXdr>::from_xdr_at(&self.0, pos)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&ParallelTxsComponent> for LazyParallelTxsComponent {
    type Error = Error;
    fn try_from(val: &ParallelTxsComponent) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyParallelTxsComponent> for ParallelTxsComponent {
    type Error = Error;
    fn try_from(lazy: &LazyParallelTxsComponent) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
