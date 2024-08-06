// Copyright(c) The Maintainers of Nanvix.
// Licensed under the MIT License.

//==================================================================================================
// Configuration
//==================================================================================================

#![deny(clippy::all)]
#![forbid(clippy::large_stack_frames)]
#![forbid(clippy::large_stack_arrays)]
#![no_std]

//==================================================================================================
// Modules
//==================================================================================================

/// System configuration constants.
pub mod config;

/// System constants.
pub mod constants;

/// Events.
pub mod event;

/// Inter process communication.
pub mod ipc;

/// Helper macros.
pub mod macros;

/// Memory management.
pub mod mm;

/// Numbers for kernel calls.
pub mod number;

/// Process management.
pub mod pm;
