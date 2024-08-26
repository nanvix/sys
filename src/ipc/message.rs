// Copyright(c) The Maintainers of Nanvix.
// Licensed under the MIT License.

//==================================================================================================
//  Imports
//==================================================================================================

use crate::pm::ProcessIdentifier;
use ::core::fmt;

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

pub struct Message {
    pub source: ProcessIdentifier,
    pub destination: ProcessIdentifier,
    pub message_type: MessageType,
    pub payload: [u8; Self::SIZE],
}

//==================================================================================================
//  Implementations
//==================================================================================================

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
    pub const SIZE: usize = 64;

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
        payload: [u8; Self::SIZE],
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
            payload: [0; Self::SIZE],
            message_type: MessageType::Ipc,
        }
    }
}
