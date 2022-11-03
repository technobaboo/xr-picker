// Copyright 2022, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

pub(crate) trait GenericManifest {
    /// Get the library path as stored in the manifest
    fn library_path(&self) -> &str;

    /// Does the library path use the system shared library search path?
    fn uses_search_path(&self) -> bool {
        !self.library_path().contains('/') && !self.library_path().contains('\\')
    }

    /// Should the library be searched for relative to the manifest?
    fn library_relative_to_manifest(&self) -> bool {
        let path = self.library_path();
        !self.uses_search_path()
            && !path.starts_with('/')
            && !path.starts_with('\\')
            && path.chars().nth(1) != Some(':')
    }

    /// Check the file format version
    fn is_file_format_version_ok(&self) -> bool;

    fn describe_manifest(&self, manifest_path: &Path) -> String {
        let manifest = manifest_path.display();
        if self.uses_search_path() {
            format!(
                "{} -> {} in the dynamic library search path",
                manifest,
                self.library_path()
            )
        } else {
            if self.library_relative_to_manifest() {
                format!(
                    "{} -> {} relative to the manifest",
                    manifest,
                    self.library_path()
                )
            } else {
                format!("{} -> {}", manifest, self.library_path())
            }
        }
    }
}

use std::path::Path;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct RuntimeFunctions {
    #[serde(rename = "xrNegotiateLoaderRuntimeInterface")]
    pub(crate) xr_negotiate_loader_runtime_interface: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Runtime {
    pub(crate) library_path: String,
    pub(crate) name: Option<String>,
    pub(crate) functions: Option<RuntimeFunctions>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct RuntimeManifest {
    file_format_version: String,
    pub(crate) runtime: Runtime,
}

impl GenericManifest for RuntimeManifest {
    fn library_path(&self) -> &str {
        &self.runtime.library_path
    }
    fn is_file_format_version_ok(&self) -> bool {
        self.file_format_version == "1.0.0"
    }
}
