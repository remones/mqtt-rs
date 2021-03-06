//! UNSUBACK

use std::io::Read;

use crate::control::variable_header::PacketIdentifier;
use crate::control::{ControlType, FixedHeader, PacketType};
use crate::packet::{Packet, PacketError};
use crate::Decodable;

/// `UNSUBACK` packet
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct UnsubackPacket {
    fixed_header: FixedHeader,
    packet_identifier: PacketIdentifier,
    payload: (),
}

encodable_packet!(UnsubackPacket(packet_identifier));

impl UnsubackPacket {
    pub fn new(pkid: u16) -> UnsubackPacket {
        UnsubackPacket {
            fixed_header: FixedHeader::new(PacketType::with_default(ControlType::UnsubscribeAcknowledgement), 2),
            packet_identifier: PacketIdentifier(pkid),
            payload: (),
        }
    }

    pub fn packet_identifier(&self) -> u16 {
        self.packet_identifier.0
    }

    pub fn set_packet_identifier(&mut self, pkid: u16) {
        self.packet_identifier.0 = pkid;
    }
}

impl Packet for UnsubackPacket {
    type Payload = ();

    fn payload(self) -> Self::Payload {
        self.payload
    }

    fn payload_ref(&self) -> &Self::Payload {
        &self.payload
    }

    fn decode_packet<R: Read>(reader: &mut R, fixed_header: FixedHeader) -> Result<Self, PacketError<Self>> {
        let packet_identifier: PacketIdentifier = PacketIdentifier::decode(reader)?;
        Ok(UnsubackPacket {
            fixed_header,
            packet_identifier,
            payload: (),
        })
    }
}
