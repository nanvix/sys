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
    /// Type of the message.
    pub message_type: MessageType,
    /// Process that sent the message.
    pub source: ProcessIdentifier,
    /// Process that should receive the message.
    pub destination: ProcessIdentifier,
    /// Payload of the message.
    pub payload: [u8; Self::PAYLOAD_SIZE],
}
crate::static_assert_size!(Message, Message::TOTAL_SIZE);

//==================================================================================================
//  Implementations
//==================================================================================================

impl Message {
    /// Total Size of a message.
    pub const TOTAL_SIZE: usize = 64;
    /// The size of the message header fields (source, destination and type).
    pub const HEADER_SIZE: usize = 2 * mem::size_of::<ProcessIdentifier>() + MessageType::SIZE;
    /// The size of the message's payload.
    pub const PAYLOAD_SIZE: usize = Self::TOTAL_SIZE - Self::HEADER_SIZE;

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
            message_type,
            source,
            destination,
            payload,
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

        // Serialize the message type.
        bytes[offset..(offset + MessageType::SIZE)].copy_from_slice(&self.message_type.to_bytes());
        offset += MessageType::SIZE;

        // Serialize the source process identifier.
        bytes[offset..(offset + mem::size_of::<ProcessIdentifier>())]
            .copy_from_slice(&self.source.to_ne_bytes());
        offset += mem::size_of::<ProcessIdentifier>();

        // Serialize the destination process identifier.
        bytes[offset..(offset + mem::size_of::<ProcessIdentifier>())]
            .copy_from_slice(&self.destination.to_ne_bytes());
        offset += mem::size_of::<ProcessIdentifier>();

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

        // Check for empty message.
        if message_type == MessageType::Empty {
            return Err(Error::new(error::ErrorCode::NoMessageAvailable, "no message available"));
        }

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

        // Deserialize the payload.
        let mut payload: [u8; Self::PAYLOAD_SIZE] = [0; Self::PAYLOAD_SIZE];
        payload.copy_from_slice(&bytes[offset..(offset + Self::PAYLOAD_SIZE)]);

        Ok(Self {
            message_type,
            source,
            destination,
            payload,
        })
    }
}

impl Default for Message {
    fn default() -> Self {
        Self {
            message_type: MessageType::Empty,
            source: ProcessIdentifier::KERNEL,
            destination: ProcessIdentifier::KERNEL,
            payload: [0; Self::PAYLOAD_SIZE],
        }
    }
}
