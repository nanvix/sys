// Copyright(c) The Maintainers of Nanvix.
// Licensed under the MIT License.

//==================================================================================================
//  Imports
//==================================================================================================

use crate::pm::ProcessIdentifier;
use ::core::{
    fmt,
    mem,
};
use ::error::Error;

//==================================================================================================
//  Structures
//==================================================================================================

///
/// # Description
///
/// Type that describes what the message is about.
///
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum MessageType {
    /// The message encodes information about an interrupt that occurred.
    Interrupt,
    /// The message encodes information about an exception that occurred.
    Exception,
    /// The message carries information sent by a process to another.
    Ipc,
    /// The message encodes information about a scheduling event.
    SchedulingEvent,
}
crate::static_assert_size!(MessageType, 4);

#[derive(Debug)]
pub struct Message {
    pub source: ProcessIdentifier,
    pub destination: ProcessIdentifier,
    pub message_type: MessageType,
    pub payload: [u8; Self::PAYLOAD_SIZE],
}

//==================================================================================================
//  Implementations
//==================================================================================================

impl MessageType {
    /// The size of a message type.
    pub const SIZE: usize = mem::size_of::<u32>();

    ///
    /// # Description
    ///
    /// Converts the targets message type to a byte array.
    ///
    /// # Returns
    ///
    /// A byte array representing the target message type.
    ///
    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        match self {
            MessageType::Interrupt => 0u32.to_ne_bytes(),
            MessageType::Exception => 1u32.to_ne_bytes(),
            MessageType::Ipc => 2u32.to_ne_bytes(),
            MessageType::SchedulingEvent => 3u32.to_ne_bytes(),
        }
    }

    ///
    /// # Description
    ///
    /// Attempts to convert a byte array to a message type.
    ///
    /// # Parameters
    ///
    /// - `bytes`: The byte array to convert.
    ///
    /// # Returns
    ///
    /// On success, the message type encoded in the byte array is returned. On error, an error is
    /// returned instead.
    ///
    pub fn try_from_bytes(bytes: [u8; Self::SIZE]) -> Result<Self, Error> {
        match u32::from_ne_bytes(bytes) {
            0 => Ok(MessageType::Interrupt),
            1 => Ok(MessageType::Exception),
            2 => Ok(MessageType::Ipc),
            3 => Ok(MessageType::SchedulingEvent),
            _ => Err(Error::new(error::ErrorCode::InvalidMessage, "invalid message type")),
        }
    }
}

impl fmt::Debug for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageType::Interrupt => write!(f, "interrupt"),
            MessageType::Exception => write!(f, "exception"),
            MessageType::Ipc => write!(f, "ipc"),
            MessageType::SchedulingEvent => write!(f, "scheduling event"),
        }
    }
}

impl Message {
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
    /// - `payload`: The message payload.
    ///
    /// # Returns
    ///
    /// The new message.
    ///
    pub fn new(
        source: ProcessIdentifier,
        destination: ProcessIdentifier,
        payload: [u8; Self::PAYLOAD_SIZE],
        message_type: MessageType,
    ) -> Self {
        Self {
            source,
            destination,
            payload,
            message_type,
        }
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
