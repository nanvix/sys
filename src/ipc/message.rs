// Copyright(c) The Maintainers of Nanvix.
// Licensed under the MIT License.

//==================================================================================================
//  Imports
//==================================================================================================

use crate::{
    ipc::typ::MessageType,
    pm::ProcessIdentifier,
};
use ::core::mem;
use ::error::Error;

//==================================================================================================
//  Structures
//==================================================================================================

///
/// # Description
///
/// A structure that represents a message that can be sent between processes.
///
#[derive(Debug)]
#[repr(C)]
pub struct Message {
    /// Process that sent the message.
    pub source: ProcessIdentifier,
    /// Process that should receive the message.
    pub destination: ProcessIdentifier,
    /// Type of the message.
    pub message_type: MessageType,
    /// Payload of the message.
    pub payload: [u8; Self::PAYLOAD_SIZE],
}
crate::static_assert_size!(Message, 76);

//==================================================================================================
//  Implementations
//==================================================================================================

impl Message {
    /// The size of the message header fields (source, destination and type).
    pub const HEADER_SIZE: usize = 2 * mem::size_of::<ProcessIdentifier>() + MessageType::SIZE;
    /// The size of the message's payload.
    pub const PAYLOAD_SIZE: usize = 64;

    ///
    /// # Description
    ///
    /// Creates a new message.
    ///
    /// # Parameters
    ///
    /// - `source`: The source process.
    /// - `destination`: The destination process.
    /// - `message_type`: The type of the message.
    /// - `payload`: The message payload.
    ///
    /// # Returns
    ///
    /// The new message.
    ///
    pub fn new(
        source: ProcessIdentifier,
        destination: ProcessIdentifier,
        message_type: MessageType,
        payload: [u8; Self::PAYLOAD_SIZE],
    ) -> Self {
        Self {
            source,
            destination,
            payload,
            message_type,
        }
    }

    ///
    /// # Description
    ///
    /// Converts the target message to a byte array.
    ///
    /// # Returns
    ///
    /// A byte array that represents the target message.
    ///
    pub fn to_bytes(&self) -> [u8; Self::HEADER_SIZE + Self::PAYLOAD_SIZE] {
        let mut bytes: [u8; Self::HEADER_SIZE + Self::PAYLOAD_SIZE] =
            [0; Self::HEADER_SIZE + Self::PAYLOAD_SIZE];

        let mut offset: usize = 0;

        // Serialize the source process identifier.
        bytes[offset..(offset + mem::size_of::<ProcessIdentifier>())]
            .copy_from_slice(&self.source.to_ne_bytes());
        offset += mem::size_of::<ProcessIdentifier>();

        // Serialize the destination process identifier.
        bytes[offset..(offset + mem::size_of::<ProcessIdentifier>())]
            .copy_from_slice(&self.destination.to_ne_bytes());
        offset += mem::size_of::<ProcessIdentifier>();

        // Serialize the message type.
        bytes[offset..(offset + MessageType::SIZE)].copy_from_slice(&self.message_type.to_bytes());
        offset += MessageType::SIZE;

        // Serialize the payload.
        bytes[offset..(offset + Self::PAYLOAD_SIZE)].copy_from_slice(&self.payload);

        bytes
    }

    ///
    /// # Description
    ///
    /// Attempts to convert a byte array to a message.
    ///
    /// # Parameters
    ///
    /// - `bytes`: The byte array to convert.
    ///
    /// # Returns
    ///
    /// Upon success, the message is returned. Upon failure, an error is returned instead.
    ///
    pub fn try_from_bytes(
        bytes: [u8; Self::HEADER_SIZE + Self::PAYLOAD_SIZE],
    ) -> Result<Self, Error> {
        let mut offset: usize = 0;

        // Deserialize the source process identifier.
        let source: ProcessIdentifier = ProcessIdentifier::from_ne_bytes(
            match bytes[offset..(offset + mem::size_of::<ProcessIdentifier>())].try_into() {
                Ok(bytes) => bytes,
                Err(_) => {
                    return Err(Error::new(error::ErrorCode::InvalidMessage, "invalid message"))
                },
            },
        );
        offset += mem::size_of::<ProcessIdentifier>();

        // Deserialize the destination process identifier.
        let destination: ProcessIdentifier = ProcessIdentifier::from_ne_bytes(
            match bytes[offset..(offset + mem::size_of::<ProcessIdentifier>())].try_into() {
                Ok(bytes) => bytes,
                Err(_) => {
                    return Err(Error::new(error::ErrorCode::InvalidMessage, "invalid message"))
                },
            },
        );
        offset += mem::size_of::<ProcessIdentifier>();

        // Deserialize the message type.
        let message_type: MessageType = MessageType::try_from_bytes(
            match bytes[offset..(offset + MessageType::SIZE)].try_into() {
                Ok(bytes) => bytes,
                Err(_) => {
                    return Err(Error::new(error::ErrorCode::InvalidMessage, "invalid message"))
                },
            },
        )?;
        offset += MessageType::SIZE;

        // Deserialize the payload.
        let mut payload: [u8; Self::PAYLOAD_SIZE] = [0; Self::PAYLOAD_SIZE];
        payload.copy_from_slice(&bytes[offset..(offset + Self::PAYLOAD_SIZE)]);

        Ok(Self {
            source,
            destination,
            message_type,
            payload,
        })
    }
}

impl Default for Message {
    fn default() -> Self {
        Self {
            source: ProcessIdentifier::KERNEL,
            destination: ProcessIdentifier::KERNEL,
            payload: [0; Self::PAYLOAD_SIZE],
            message_type: MessageType::Ipc,
        }
    }
}
