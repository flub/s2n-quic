// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

struct Meta {
    #[builder(crate::endpoint::Type)]
    endpoint_type: EndpointType,

    subject: Subject,

    #[builder(crate::time::Timestamp)]
    timestamp: crate::event::Timestamp,
}

//= https://tools.ietf.org/id/draft-marx-qlog-event-definitions-quic-h3-02.txt#A.4
struct PacketHeader {
    packet_type: PacketType,
    version: Option<u32>,
}

struct Path<'a> {
    // TODO uncomment once we record the local Address/CID
    // pub local_addr: SocketAddress<'a>,
    // pub local_cid: ConnectionId<'a>,
    remote_addr: SocketAddress<'a>,
    remote_cid: ConnectionId<'a>,
    id: u64,
}

struct ConnectionId<'a> {
    bytes: &'a [u8],
}

macro_rules! impl_conn_id {
    ($name:ident) => {
        impl<'a> IntoEvent<builder::ConnectionId<'a>> for &'a crate::connection::id::$name {
            fn into_event(self) -> builder::ConnectionId<'a> {
                builder::ConnectionId {
                    bytes: self.as_bytes(),
                }
            }
        }
    };
}

impl_conn_id!(LocalId);
impl_conn_id!(PeerId);
impl_conn_id!(UnboundedId);
impl_conn_id!(InitialId);

enum SocketAddress<'a> {
    IpV4 { ip: &'a [u8; 4], port: u16 },
    IpV6 { ip: &'a [u8; 16], port: u16 },
}

impl<'a> IntoEvent<builder::SocketAddress<'a>> for &'a crate::inet::SocketAddress {
    fn into_event(self) -> builder::SocketAddress<'a> {
        match self {
            crate::inet::SocketAddress::IpV4(addr) => builder::SocketAddress::IpV4 {
                ip: &addr.ip.octets,
                port: addr.port.into(),
            },
            crate::inet::SocketAddress::IpV6(addr) => builder::SocketAddress::IpV6 {
                ip: &addr.ip.octets,
                port: addr.port.into(),
            },
        }
    }
}

#[cfg(feature = "std")]
impl From<SocketAddress<'_>> for std::net::SocketAddr {
    fn from(address: SocketAddress) -> Self {
        use std::net;
        match address {
            SocketAddress::IpV4 { ip, port } => {
                let ip = net::IpAddr::V4(net::Ipv4Addr::from(*ip));
                Self::new(ip, port)
            }
            SocketAddress::IpV6 { ip, port } => {
                let ip = net::IpAddr::V6(net::Ipv6Addr::from(*ip));
                Self::new(ip, port)
            }
        }
    }
}

enum DuplicatePacketError {
    /// The packet number was already received and is a duplicate.
    Duplicate,

    /// The received packet number was outside the range of tracked packet numbers.
    ///
    /// This can happen when packets are heavily delayed or reordered. Currently, the maximum
    /// amount of reordering is limited to 128 packets. For example, if packet number `142`
    /// is received, the allowed range would be limited to `14-142`. If an endpoint received
    /// packet `< 14`, it would trigger this event.
    TooOld,
}

impl IntoEvent<builder::DuplicatePacketError> for crate::packet::number::SlidingWindowError {
    fn into_event(self) -> builder::DuplicatePacketError {
        use crate::packet::number::SlidingWindowError;
        match self {
            SlidingWindowError::TooOld => builder::DuplicatePacketError::TooOld {},
            SlidingWindowError::Duplicate => builder::DuplicatePacketError::Duplicate {},
        }
    }
}

//= https://tools.ietf.org/id/draft-marx-qlog-event-definitions-quic-h3-02.txt#A.7
enum Frame {
    Padding,
    Ping,
    Ack,
    ResetStream,
    StopSending,
    Crypto {
        offset: u64,
        len: u16,
    },
    NewToken,
    Stream {
        id: u64,
        offset: u64,
        len: u16,
        is_fin: bool,
    },
    MaxData,
    MaxStreamData,
    MaxStreams,
    DataBlocked,
    StreamDataBlocked,
    StreamsBlocked,
    NewConnectionId,
    RetireConnectionId,
    PathChallenge,
    PathResponse,
    ConnectionClose,
    HandshakeDone,
}

impl IntoEvent<builder::Frame> for &crate::frame::Padding {
    fn into_event(self) -> builder::Frame {
        builder::Frame::Padding {}
    }
}

impl IntoEvent<builder::Frame> for &crate::frame::Ping {
    fn into_event(self) -> builder::Frame {
        builder::Frame::Ping {}
    }
}

impl<AckRanges> IntoEvent<builder::Frame> for &crate::frame::Ack<AckRanges> {
    fn into_event(self) -> builder::Frame {
        builder::Frame::Ack {}
    }
}

impl IntoEvent<builder::Frame> for &crate::frame::ResetStream {
    fn into_event(self) -> builder::Frame {
        builder::Frame::ResetStream {}
    }
}

impl IntoEvent<builder::Frame> for &crate::frame::StopSending {
    fn into_event(self) -> builder::Frame {
        builder::Frame::ResetStream {}
    }
}

impl<'a> IntoEvent<builder::Frame> for &crate::frame::NewToken<'a> {
    fn into_event(self) -> builder::Frame {
        builder::Frame::NewToken {}
    }
}

impl IntoEvent<builder::Frame> for &crate::frame::MaxData {
    fn into_event(self) -> builder::Frame {
        builder::Frame::MaxData {}
    }
}

impl IntoEvent<builder::Frame> for &crate::frame::MaxStreamData {
    fn into_event(self) -> builder::Frame {
        builder::Frame::MaxStreamData {}
    }
}

impl IntoEvent<builder::Frame> for &crate::frame::MaxStreams {
    fn into_event(self) -> builder::Frame {
        builder::Frame::MaxStreams {}
    }
}

impl IntoEvent<builder::Frame> for &crate::frame::DataBlocked {
    fn into_event(self) -> builder::Frame {
        builder::Frame::DataBlocked {}
    }
}

impl IntoEvent<builder::Frame> for &crate::frame::StreamDataBlocked {
    fn into_event(self) -> builder::Frame {
        builder::Frame::StreamDataBlocked {}
    }
}

impl IntoEvent<builder::Frame> for &crate::frame::StreamsBlocked {
    fn into_event(self) -> builder::Frame {
        builder::Frame::StreamsBlocked {}
    }
}

impl<'a> IntoEvent<builder::Frame> for &crate::frame::NewConnectionId<'a> {
    fn into_event(self) -> builder::Frame {
        builder::Frame::NewConnectionId {}
    }
}

impl IntoEvent<builder::Frame> for &crate::frame::RetireConnectionId {
    fn into_event(self) -> builder::Frame {
        builder::Frame::RetireConnectionId {}
    }
}

impl<'a> IntoEvent<builder::Frame> for &crate::frame::PathChallenge<'a> {
    fn into_event(self) -> builder::Frame {
        builder::Frame::PathChallenge {}
    }
}

impl<'a> IntoEvent<builder::Frame> for &crate::frame::PathResponse<'a> {
    fn into_event(self) -> builder::Frame {
        builder::Frame::PathResponse {}
    }
}

impl<'a> IntoEvent<builder::Frame> for &crate::frame::ConnectionClose<'a> {
    fn into_event(self) -> builder::Frame {
        builder::Frame::ConnectionClose {}
    }
}

impl IntoEvent<builder::Frame> for &crate::frame::HandshakeDone {
    fn into_event(self) -> builder::Frame {
        builder::Frame::HandshakeDone {}
    }
}

impl<Data> IntoEvent<builder::Frame> for &crate::frame::Stream<Data>
where
    Data: s2n_codec::EncoderValue,
{
    fn into_event(self) -> builder::Frame {
        builder::Frame::Stream {
            id: self.stream_id.as_u64(),
            offset: self.offset.as_u64(),
            len: self.data.encoding_size() as _,
            is_fin: self.is_fin,
        }
    }
}

impl<Data> IntoEvent<builder::Frame> for &crate::frame::Crypto<Data>
where
    Data: s2n_codec::EncoderValue,
{
    fn into_event(self) -> builder::Frame {
        builder::Frame::Crypto {
            offset: self.offset.as_u64(),
            len: self.data.encoding_size() as _,
        }
    }
}

//= https://tools.ietf.org/id/draft-marx-qlog-event-definitions-quic-h3-02.txt#A.2
enum PacketType {
    Initial { number: u64 },
    Handshake { number: u64 },
    ZeroRtt { number: u64 },
    OneRtt { number: u64 },
    Retry,
    VersionNegotiation,
    StatelessReset,
}

impl IntoEvent<builder::PacketType> for crate::packet::number::PacketNumber {
    fn into_event(self) -> builder::PacketType {
        use crate::packet::number::PacketNumberSpace;
        use builder::PacketType;

        match self.space() {
            PacketNumberSpace::Initial => PacketType::Initial {
                number: self.as_u64(),
            },
            PacketNumberSpace::Handshake => PacketType::Handshake {
                number: self.as_u64(),
            },
            PacketNumberSpace::ApplicationData => PacketType::OneRtt {
                number: self.as_u64(),
            },
        }
    }
}

enum KeyType {
    Initial,
    Handshake,
    ZeroRtt,
    OneRtt { generation: u16 },
}

/// A context from which the event is being emitted
///
/// An event can occur in the context of an Endpoint or Connection
enum Subject {
    Endpoint,

    //= https://tools.ietf.org/id/draft-marx-qlog-event-definitions-quic-h3-02.txt#4
    //# it is recommended to use
    //# QUIC's Original Destination Connection ID (ODCID, the CID chosen by
    //# the client when first contacting the server)
    /// This maps to an internal connection id, which is a stable identifier across CID changes.
    Connection {
        id: u64,
    },
}

/// Used to disambiguate events that can occur for the local or the remote endpoint.
enum Location {
    /// The Local endpoint
    Local,
    /// The Remote endpoint
    Remote,
}

impl IntoEvent<api::Location> for crate::endpoint::Location {
    fn into_event(self) -> api::Location {
        match self {
            Self::Local => api::Location::Local {},
            Self::Remote => api::Location::Remote {},
        }
    }
}

impl IntoEvent<builder::Location> for crate::endpoint::Location {
    fn into_event(self) -> builder::Location {
        match self {
            Self::Local => builder::Location::Local {},
            Self::Remote => builder::Location::Remote {},
        }
    }
}

enum EndpointType {
    Server,
    Client,
}

impl IntoEvent<api::EndpointType> for crate::endpoint::Type {
    fn into_event(self) -> api::EndpointType {
        match self {
            Self::Client => api::EndpointType::Client {},
            Self::Server => api::EndpointType::Server {},
        }
    }
}

impl IntoEvent<builder::EndpointType> for crate::endpoint::Type {
    fn into_event(self) -> builder::EndpointType {
        match self {
            Self::Client => builder::EndpointType::Client {},
            Self::Server => builder::EndpointType::Server {},
        }
    }
}

enum DropReason {
    DecodingFailed,
    InvalidRetryToken,
    ConnectionNotAllowed,
    UnsupportedVersion,
    InvalidDestinationConnectionId,
    InvalidSourceConnectionId,
}
