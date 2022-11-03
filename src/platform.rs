// Copyright 2022, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::path::PathBuf;

use crate::{ActiveState, Error};

pub trait PlatformRuntime {
    /// Attempt to make this runtime active.
    fn make_active(&self) -> Result<(), Error>;

    /// Get a name for the runtime, preferably the self-declared one.
    ///
    /// Not promised to be unique, though!
    fn get_runtime_name(&self) -> String;
}

pub trait Platform {
    type PlatformRuntimeType: PlatformRuntime;
    /// Platform-specific data describing the currently active runtime(s)
    type PlatformActiveData;

    /// Enumerate all available runtimes we might be aware of.
    fn find_available_runtimes(&self) -> Result<Vec<Self::PlatformRuntimeType>, Error>;

    fn get_active_runtime_manifests(&self) -> Vec<PathBuf>;

    /// Get a snapshot of what the active runtime(s) is/are,
    /// to use when checking if a runtime we know about is active.
    fn get_active_data(&self) -> Self::PlatformActiveData;

    /// Is the given runtime marked as active?
    ///
    /// Some platforms might have separate 32-bit and 64-bit active runtime settings,
    /// which makes this more complex than a bool.
    fn get_runtime_active_state(
        &self,
        runtime: &Self::PlatformRuntimeType,
        active_data: &Self::PlatformActiveData,
    ) -> ActiveState;
}
