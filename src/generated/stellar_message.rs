#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::sync::Arc;

/// StellarMessage is an XDR Union defined as:
///
/// ```text
/// union StellarMessage switch (MessageType type)
/// {
/// case ERROR_MSG:
///     Error error;
/// case HELLO:
///     Hello hello;
/// case AUTH:
///     Auth auth;
/// case DONT_HAVE:
///     DontHave dontHave;
/// case PEERS:
///     PeerAddress peers<100>;
///
/// case GET_TX_SET:
///     uint256 txSetHash;
/// case TX_SET:
///     TransactionSet txSet;
/// case GENERALIZED_TX_SET:
///     GeneralizedTransactionSet generalizedTxSet;
///
/// case TRANSACTION:
///     TransactionEnvelope transaction;
///
/// case TIME_SLICED_SURVEY_REQUEST:
///     SignedTimeSlicedSurveyRequestMessage signedTimeSlicedSurveyRequestMessage;
///
/// case TIME_SLICED_SURVEY_RESPONSE:
///     SignedTimeSlicedSurveyResponseMessage signedTimeSlicedSurveyResponseMessage;
///
/// case TIME_SLICED_SURVEY_START_COLLECTING:
///     SignedTimeSlicedSurveyStartCollectingMessage
///         signedTimeSlicedSurveyStartCollectingMessage;
///
/// case TIME_SLICED_SURVEY_STOP_COLLECTING:
///     SignedTimeSlicedSurveyStopCollectingMessage
///         signedTimeSlicedSurveyStopCollectingMessage;
///
/// // SCP
/// case GET_SCP_QUORUMSET:
///     uint256 qSetHash;
/// case SCP_QUORUMSET:
///     SCPQuorumSet qSet;
/// case SCP_MESSAGE:
///     SCPEnvelope envelope;
/// case GET_SCP_STATE:
///     uint32 getSCPLedgerSeq; // ledger seq requested ; if 0, requests the latest
/// case SEND_MORE:
///     SendMore sendMoreMessage;
/// case SEND_MORE_EXTENDED:
///     SendMoreExtended sendMoreExtendedMessage;
/// // Pull mode
/// case FLOOD_ADVERT:
///      FloodAdvert floodAdvert;
/// case FLOOD_DEMAND:
///      FloodDemand floodDemand;
/// };
/// ```
///
// union with discriminant MessageType
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
pub enum StellarMessage {
    ErrorMsg(SError),
    Hello(Hello),
    Auth(Auth),
    DontHave(DontHave),
    Peers(VecM<PeerAddress, 100>),
    GetTxSet(Uint256),
    TxSet(TransactionSet),
    GeneralizedTxSet(GeneralizedTransactionSet),
    Transaction(TransactionEnvelope),
    TimeSlicedSurveyRequest(SignedTimeSlicedSurveyRequestMessage),
    TimeSlicedSurveyResponse(SignedTimeSlicedSurveyResponseMessage),
    TimeSlicedSurveyStartCollecting(SignedTimeSlicedSurveyStartCollectingMessage),
    TimeSlicedSurveyStopCollecting(SignedTimeSlicedSurveyStopCollectingMessage),
    GetScpQuorumset(Uint256),
    ScpQuorumset(ScpQuorumSet),
    ScpMessage(ScpEnvelope),
    GetScpState(u32),
    SendMore(SendMore),
    SendMoreExtended(SendMoreExtended),
    FloodAdvert(FloodAdvert),
    FloodDemand(FloodDemand),
}

#[cfg(feature = "alloc")]
impl Default for StellarMessage {
    fn default() -> Self {
        Self::ErrorMsg(SError::default())
    }
}

impl StellarMessage {
    const _VARIANTS: &[MessageType] = &[
        MessageType::ErrorMsg,
        MessageType::Hello,
        MessageType::Auth,
        MessageType::DontHave,
        MessageType::Peers,
        MessageType::GetTxSet,
        MessageType::TxSet,
        MessageType::GeneralizedTxSet,
        MessageType::Transaction,
        MessageType::TimeSlicedSurveyRequest,
        MessageType::TimeSlicedSurveyResponse,
        MessageType::TimeSlicedSurveyStartCollecting,
        MessageType::TimeSlicedSurveyStopCollecting,
        MessageType::GetScpQuorumset,
        MessageType::ScpQuorumset,
        MessageType::ScpMessage,
        MessageType::GetScpState,
        MessageType::SendMore,
        MessageType::SendMoreExtended,
        MessageType::FloodAdvert,
        MessageType::FloodDemand,
    ];
    pub const VARIANTS: [MessageType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "ErrorMsg",
        "Hello",
        "Auth",
        "DontHave",
        "Peers",
        "GetTxSet",
        "TxSet",
        "GeneralizedTxSet",
        "Transaction",
        "TimeSlicedSurveyRequest",
        "TimeSlicedSurveyResponse",
        "TimeSlicedSurveyStartCollecting",
        "TimeSlicedSurveyStopCollecting",
        "GetScpQuorumset",
        "ScpQuorumset",
        "ScpMessage",
        "GetScpState",
        "SendMore",
        "SendMoreExtended",
        "FloodAdvert",
        "FloodDemand",
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
            Self::ErrorMsg(_) => "ErrorMsg",
            Self::Hello(_) => "Hello",
            Self::Auth(_) => "Auth",
            Self::DontHave(_) => "DontHave",
            Self::Peers(_) => "Peers",
            Self::GetTxSet(_) => "GetTxSet",
            Self::TxSet(_) => "TxSet",
            Self::GeneralizedTxSet(_) => "GeneralizedTxSet",
            Self::Transaction(_) => "Transaction",
            Self::TimeSlicedSurveyRequest(_) => "TimeSlicedSurveyRequest",
            Self::TimeSlicedSurveyResponse(_) => "TimeSlicedSurveyResponse",
            Self::TimeSlicedSurveyStartCollecting(_) => "TimeSlicedSurveyStartCollecting",
            Self::TimeSlicedSurveyStopCollecting(_) => "TimeSlicedSurveyStopCollecting",
            Self::GetScpQuorumset(_) => "GetScpQuorumset",
            Self::ScpQuorumset(_) => "ScpQuorumset",
            Self::ScpMessage(_) => "ScpMessage",
            Self::GetScpState(_) => "GetScpState",
            Self::SendMore(_) => "SendMore",
            Self::SendMoreExtended(_) => "SendMoreExtended",
            Self::FloodAdvert(_) => "FloodAdvert",
            Self::FloodDemand(_) => "FloodDemand",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> MessageType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::ErrorMsg(_) => MessageType::ErrorMsg,
            Self::Hello(_) => MessageType::Hello,
            Self::Auth(_) => MessageType::Auth,
            Self::DontHave(_) => MessageType::DontHave,
            Self::Peers(_) => MessageType::Peers,
            Self::GetTxSet(_) => MessageType::GetTxSet,
            Self::TxSet(_) => MessageType::TxSet,
            Self::GeneralizedTxSet(_) => MessageType::GeneralizedTxSet,
            Self::Transaction(_) => MessageType::Transaction,
            Self::TimeSlicedSurveyRequest(_) => MessageType::TimeSlicedSurveyRequest,
            Self::TimeSlicedSurveyResponse(_) => MessageType::TimeSlicedSurveyResponse,
            Self::TimeSlicedSurveyStartCollecting(_) => {
                MessageType::TimeSlicedSurveyStartCollecting
            }
            Self::TimeSlicedSurveyStopCollecting(_) => MessageType::TimeSlicedSurveyStopCollecting,
            Self::GetScpQuorumset(_) => MessageType::GetScpQuorumset,
            Self::ScpQuorumset(_) => MessageType::ScpQuorumset,
            Self::ScpMessage(_) => MessageType::ScpMessage,
            Self::GetScpState(_) => MessageType::GetScpState,
            Self::SendMore(_) => MessageType::SendMore,
            Self::SendMoreExtended(_) => MessageType::SendMoreExtended,
            Self::FloodAdvert(_) => MessageType::FloodAdvert,
            Self::FloodDemand(_) => MessageType::FloodDemand,
        }
    }

    #[must_use]
    pub const fn variants() -> [MessageType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for StellarMessage {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<MessageType> for StellarMessage {
    #[must_use]
    fn discriminant(&self) -> MessageType {
        Self::discriminant(self)
    }
}

impl Variants<MessageType> for StellarMessage {
    fn variants() -> slice::Iter<'static, MessageType> {
        Self::VARIANTS.iter()
    }
}

impl Union<MessageType> for StellarMessage {}

impl ReadXdr for StellarMessage {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: MessageType = <MessageType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                MessageType::ErrorMsg => Self::ErrorMsg(SError::read_xdr(r)?),
                MessageType::Hello => Self::Hello(Hello::read_xdr(r)?),
                MessageType::Auth => Self::Auth(Auth::read_xdr(r)?),
                MessageType::DontHave => Self::DontHave(DontHave::read_xdr(r)?),
                MessageType::Peers => Self::Peers(VecM::<PeerAddress, 100>::read_xdr(r)?),
                MessageType::GetTxSet => Self::GetTxSet(Uint256::read_xdr(r)?),
                MessageType::TxSet => Self::TxSet(TransactionSet::read_xdr(r)?),
                MessageType::GeneralizedTxSet => {
                    Self::GeneralizedTxSet(GeneralizedTransactionSet::read_xdr(r)?)
                }
                MessageType::Transaction => Self::Transaction(TransactionEnvelope::read_xdr(r)?),
                MessageType::TimeSlicedSurveyRequest => Self::TimeSlicedSurveyRequest(
                    SignedTimeSlicedSurveyRequestMessage::read_xdr(r)?,
                ),
                MessageType::TimeSlicedSurveyResponse => Self::TimeSlicedSurveyResponse(
                    SignedTimeSlicedSurveyResponseMessage::read_xdr(r)?,
                ),
                MessageType::TimeSlicedSurveyStartCollecting => {
                    Self::TimeSlicedSurveyStartCollecting(
                        SignedTimeSlicedSurveyStartCollectingMessage::read_xdr(r)?,
                    )
                }
                MessageType::TimeSlicedSurveyStopCollecting => {
                    Self::TimeSlicedSurveyStopCollecting(
                        SignedTimeSlicedSurveyStopCollectingMessage::read_xdr(r)?,
                    )
                }
                MessageType::GetScpQuorumset => Self::GetScpQuorumset(Uint256::read_xdr(r)?),
                MessageType::ScpQuorumset => Self::ScpQuorumset(ScpQuorumSet::read_xdr(r)?),
                MessageType::ScpMessage => Self::ScpMessage(ScpEnvelope::read_xdr(r)?),
                MessageType::GetScpState => Self::GetScpState(u32::read_xdr(r)?),
                MessageType::SendMore => Self::SendMore(SendMore::read_xdr(r)?),
                MessageType::SendMoreExtended => {
                    Self::SendMoreExtended(SendMoreExtended::read_xdr(r)?)
                }
                MessageType::FloodAdvert => Self::FloodAdvert(FloodAdvert::read_xdr(r)?),
                MessageType::FloodDemand => Self::FloodDemand(FloodDemand::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for StellarMessage {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::ErrorMsg(v) => v.write_xdr(w)?,
                Self::Hello(v) => v.write_xdr(w)?,
                Self::Auth(v) => v.write_xdr(w)?,
                Self::DontHave(v) => v.write_xdr(w)?,
                Self::Peers(v) => v.write_xdr(w)?,
                Self::GetTxSet(v) => v.write_xdr(w)?,
                Self::TxSet(v) => v.write_xdr(w)?,
                Self::GeneralizedTxSet(v) => v.write_xdr(w)?,
                Self::Transaction(v) => v.write_xdr(w)?,
                Self::TimeSlicedSurveyRequest(v) => v.write_xdr(w)?,
                Self::TimeSlicedSurveyResponse(v) => v.write_xdr(w)?,
                Self::TimeSlicedSurveyStartCollecting(v) => v.write_xdr(w)?,
                Self::TimeSlicedSurveyStopCollecting(v) => v.write_xdr(w)?,
                Self::GetScpQuorumset(v) => v.write_xdr(w)?,
                Self::ScpQuorumset(v) => v.write_xdr(w)?,
                Self::ScpMessage(v) => v.write_xdr(w)?,
                Self::GetScpState(v) => v.write_xdr(w)?,
                Self::SendMore(v) => v.write_xdr(w)?,
                Self::SendMoreExtended(v) => v.write_xdr(w)?,
                Self::FloodAdvert(v) => v.write_xdr(w)?,
                Self::FloodDemand(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
/// Lazy wrapper for [`StellarMessage`].
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LazyStellarMessage(LazyHandle);
#[cfg(feature = "alloc")]
impl PartialOrd for LazyStellarMessage {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(feature = "alloc")]
impl Ord for LazyStellarMessage {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let ord = self.discriminant().cmp(&other.discriminant());
        if ord != core::cmp::Ordering::Equal {
            return ord;
        }
        #[allow(clippy::match_same_arms)]
        match self.discriminant_i32() {
            0 => self.as_error_msg().cmp(&other.as_error_msg()),
            13 => self.as_hello().cmp(&other.as_hello()),
            2 => self.as_auth().cmp(&other.as_auth()),
            3 => self.as_dont_have().cmp(&other.as_dont_have()),
            5 => self.as_peers().cmp(&other.as_peers()),
            6 => self.as_get_tx_set().cmp(&other.as_get_tx_set()),
            7 => self.as_tx_set().cmp(&other.as_tx_set()),
            17 => self
                .as_generalized_tx_set()
                .cmp(&other.as_generalized_tx_set()),
            8 => self.as_transaction().cmp(&other.as_transaction()),
            21 => self
                .as_time_sliced_survey_request()
                .cmp(&other.as_time_sliced_survey_request()),
            22 => self
                .as_time_sliced_survey_response()
                .cmp(&other.as_time_sliced_survey_response()),
            23 => self
                .as_time_sliced_survey_start_collecting()
                .cmp(&other.as_time_sliced_survey_start_collecting()),
            24 => self
                .as_time_sliced_survey_stop_collecting()
                .cmp(&other.as_time_sliced_survey_stop_collecting()),
            9 => self
                .as_get_scp_quorumset()
                .cmp(&other.as_get_scp_quorumset()),
            10 => self.as_scp_quorumset().cmp(&other.as_scp_quorumset()),
            11 => self.as_scp_message().cmp(&other.as_scp_message()),
            12 => self.as_get_scp_state().cmp(&other.as_get_scp_state()),
            16 => self.as_send_more().cmp(&other.as_send_more()),
            20 => self
                .as_send_more_extended()
                .cmp(&other.as_send_more_extended()),
            18 => self.as_flood_advert().cmp(&other.as_flood_advert()),
            19 => self.as_flood_demand().cmp(&other.as_flood_demand()),
            _ => core::cmp::Ordering::Equal,
        }
    }
}
#[cfg(feature = "alloc")]
impl LazyXdr for LazyStellarMessage {
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
                let field_len = <LazySError as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            13 => {
                let field_len = <LazyHello as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            2 => {
                let field_len = <LazyAuth as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            3 => {
                let field_len =
                    <LazyDontHave as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            5 => {
                let field_len = <LazyVecM<LazyPeerAddress, 100> as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            6 => {
                let field_len =
                    <LazyUint256 as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            7 => {
                let field_len =
                    <LazyTransactionSet as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            17 => {
                let field_len = <LazyGeneralizedTransactionSet as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            8 => {
                let field_len = <LazyTransactionEnvelope as LazyXdr>::xdr_validate(
                    &buf[pos as usize..],
                    depth,
                )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            21 => {
                let field_len =
                    <LazySignedTimeSlicedSurveyRequestMessage as LazyXdr>::xdr_validate(
                        &buf[pos as usize..],
                        depth,
                    )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            22 => {
                let field_len =
                    <LazySignedTimeSlicedSurveyResponseMessage as LazyXdr>::xdr_validate(
                        &buf[pos as usize..],
                        depth,
                    )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            23 => {
                let field_len =
                    <LazySignedTimeSlicedSurveyStartCollectingMessage as LazyXdr>::xdr_validate(
                        &buf[pos as usize..],
                        depth,
                    )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            24 => {
                let field_len =
                    <LazySignedTimeSlicedSurveyStopCollectingMessage as LazyXdr>::xdr_validate(
                        &buf[pos as usize..],
                        depth,
                    )?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            9 => {
                let field_len =
                    <LazyUint256 as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            10 => {
                let field_len =
                    <LazyScpQuorumSet as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            11 => {
                let field_len =
                    <LazyScpEnvelope as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            12 => {
                let field_len = <u32 as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            16 => {
                let field_len =
                    <LazySendMore as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            20 => {
                let field_len =
                    <LazySendMoreExtended as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            18 => {
                let field_len =
                    <LazyFloodAdvert as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
                pos = pos.checked_add(field_len).ok_or(Error::LengthExceedsMax)?;
            }
            19 => {
                let field_len =
                    <LazyFloodDemand as LazyXdr>::xdr_validate(&buf[pos as usize..], depth)?;
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
                pos += <LazySError as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            13 => {
                pos += <LazyHello as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            2 => {
                pos += <LazyAuth as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            3 => {
                pos += <LazyDontHave as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            5 => {
                pos += <LazyVecM<LazyPeerAddress, 100> as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            6 => {
                pos += <LazyUint256 as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            7 => {
                pos += <LazyTransactionSet as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            17 => {
                pos += <LazyGeneralizedTransactionSet as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            8 => {
                pos += <LazyTransactionEnvelope as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            21 => {
                pos += <LazySignedTimeSlicedSurveyRequestMessage as LazyXdr>::xdr_len(
                    &buf[pos as usize..],
                );
            }
            22 => {
                pos += <LazySignedTimeSlicedSurveyResponseMessage as LazyXdr>::xdr_len(
                    &buf[pos as usize..],
                );
            }
            23 => {
                pos += <LazySignedTimeSlicedSurveyStartCollectingMessage as LazyXdr>::xdr_len(
                    &buf[pos as usize..],
                );
            }
            24 => {
                pos += <LazySignedTimeSlicedSurveyStopCollectingMessage as LazyXdr>::xdr_len(
                    &buf[pos as usize..],
                );
            }
            9 => {
                pos += <LazyUint256 as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            10 => {
                pos += <LazyScpQuorumSet as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            11 => {
                pos += <LazyScpEnvelope as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            12 => {
                pos += <u32 as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            16 => {
                pos += <LazySendMore as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            20 => {
                pos += <LazySendMoreExtended as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            18 => {
                pos += <LazyFloodAdvert as LazyXdr>::xdr_len(&buf[pos as usize..]);
            }
            19 => {
                pos += <LazyFloodDemand as LazyXdr>::xdr_len(&buf[pos as usize..]);
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
impl From<LazyHandle> for LazyStellarMessage {
    fn from(h: LazyHandle) -> Self {
        Self(h)
    }
}
#[cfg(feature = "alloc")]
impl AsRef<LazyHandle> for LazyStellarMessage {
    fn as_ref(&self) -> &LazyHandle {
        &self.0
    }
}
#[cfg(feature = "alloc")]
impl TryFrom<Arc<[u8]>> for LazyStellarMessage {
    type Error = Error;
    fn try_from(buf: Arc<[u8]>) -> Result<Self, Error> {
        let len = Self::xdr_validate(&buf, DEFAULT_XDR_DEPTH_LIMIT)?;
        Ok(Self(LazyHandle::from_arc(buf, 0, len)))
    }
}
#[cfg(feature = "alloc")]
impl LazyStellarMessage {
    /// Get the discriminant value as i32.
    #[must_use]
    pub fn discriminant_i32(&self) -> i32 {
        i32::from_xdr_at(&self.0, 0)
    }

    /// Get the discriminant.
    #[must_use]
    pub fn discriminant(&self) -> MessageType {
        // Validated — unwrap is safe.
        MessageType::try_from(self.discriminant_i32()).unwrap()
    }
    /// Access arm `ErrorMsg`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_error_msg(&self) -> Option<LazySError> {
        if self.discriminant_i32() == 0 {
            Some(<LazySError as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Hello`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_hello(&self) -> Option<LazyHello> {
        if self.discriminant_i32() == 13 {
            Some(<LazyHello as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Auth`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_auth(&self) -> Option<LazyAuth> {
        if self.discriminant_i32() == 2 {
            Some(<LazyAuth as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `DontHave`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_dont_have(&self) -> Option<LazyDontHave> {
        if self.discriminant_i32() == 3 {
            Some(<LazyDontHave as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `Peers`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_peers(&self) -> Option<LazyVecM<LazyPeerAddress, 100>> {
        if self.discriminant_i32() == 5 {
            Some(<LazyVecM<LazyPeerAddress, 100> as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `GetTxSet`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_get_tx_set(&self) -> Option<LazyUint256> {
        if self.discriminant_i32() == 6 {
            Some(<LazyUint256 as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `TxSet`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_tx_set(&self) -> Option<LazyTransactionSet> {
        if self.discriminant_i32() == 7 {
            Some(<LazyTransactionSet as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `GeneralizedTxSet`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_generalized_tx_set(&self) -> Option<LazyGeneralizedTransactionSet> {
        if self.discriminant_i32() == 17 {
            Some(<LazyGeneralizedTransactionSet as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `Transaction`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_transaction(&self) -> Option<LazyTransactionEnvelope> {
        if self.discriminant_i32() == 8 {
            Some(<LazyTransactionEnvelope as LazyXdr>::from_xdr_at(
                &self.0, 4,
            ))
        } else {
            None
        }
    }
    /// Access arm `TimeSlicedSurveyRequest`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_time_sliced_survey_request(
        &self,
    ) -> Option<LazySignedTimeSlicedSurveyRequestMessage> {
        if self.discriminant_i32() == 21 {
            Some(<LazySignedTimeSlicedSurveyRequestMessage as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `TimeSlicedSurveyResponse`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_time_sliced_survey_response(
        &self,
    ) -> Option<LazySignedTimeSlicedSurveyResponseMessage> {
        if self.discriminant_i32() == 22 {
            Some(<LazySignedTimeSlicedSurveyResponseMessage as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `TimeSlicedSurveyStartCollecting`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_time_sliced_survey_start_collecting(
        &self,
    ) -> Option<LazySignedTimeSlicedSurveyStartCollectingMessage> {
        if self.discriminant_i32() == 23 {
            Some(
                <LazySignedTimeSlicedSurveyStartCollectingMessage as LazyXdr>::from_xdr_at(
                    &self.0, 4,
                ),
            )
        } else {
            None
        }
    }
    /// Access arm `TimeSlicedSurveyStopCollecting`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_time_sliced_survey_stop_collecting(
        &self,
    ) -> Option<LazySignedTimeSlicedSurveyStopCollectingMessage> {
        if self.discriminant_i32() == 24 {
            Some(
                <LazySignedTimeSlicedSurveyStopCollectingMessage as LazyXdr>::from_xdr_at(
                    &self.0, 4,
                ),
            )
        } else {
            None
        }
    }
    /// Access arm `GetScpQuorumset`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_get_scp_quorumset(&self) -> Option<LazyUint256> {
        if self.discriminant_i32() == 9 {
            Some(<LazyUint256 as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `ScpQuorumset`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_scp_quorumset(&self) -> Option<LazyScpQuorumSet> {
        if self.discriminant_i32() == 10 {
            Some(<LazyScpQuorumSet as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `ScpMessage`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_scp_message(&self) -> Option<LazyScpEnvelope> {
        if self.discriminant_i32() == 11 {
            Some(<LazyScpEnvelope as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `GetScpState`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_get_scp_state(&self) -> Option<u32> {
        if self.discriminant_i32() == 12 {
            Some(<u32 as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `SendMore`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_send_more(&self) -> Option<LazySendMore> {
        if self.discriminant_i32() == 16 {
            Some(<LazySendMore as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `SendMoreExtended`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_send_more_extended(&self) -> Option<LazySendMoreExtended> {
        if self.discriminant_i32() == 20 {
            Some(<LazySendMoreExtended as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `FloodAdvert`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_flood_advert(&self) -> Option<LazyFloodAdvert> {
        if self.discriminant_i32() == 18 {
            Some(<LazyFloodAdvert as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
    /// Access arm `FloodDemand`. Returns `Some` if the discriminant matches.
    #[must_use]
    pub fn as_flood_demand(&self) -> Option<LazyFloodDemand> {
        if self.discriminant_i32() == 19 {
            Some(<LazyFloodDemand as LazyXdr>::from_xdr_at(&self.0, 4))
        } else {
            None
        }
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&StellarMessage> for LazyStellarMessage {
    type Error = Error;
    fn try_from(val: &StellarMessage) -> Result<Self, Error> {
        let mut buf = Vec::new();
        val.write_xdr(&mut Limited::new(&mut buf, Limits::none()))?;
        let arc: Arc<[u8]> = buf.into();
        Self::try_from(arc)
    }
}
#[cfg(all(feature = "alloc", feature = "std"))]
impl TryFrom<&LazyStellarMessage> for StellarMessage {
    type Error = Error;
    fn try_from(lazy: &LazyStellarMessage) -> Result<Self, Error> {
        let buf = lazy.as_ref().as_slice();
        Self::read_xdr(&mut Limited::new(&mut &buf[..], Limits::none()))
    }
}
