//! Support for the `docker-compose.yml` version 2 file format.

use regex::Regex;
use serde::de::{self, Deserialize, Deserializer, SeqVisitor, Visitor};
use serde::ser::{self, Serialize, Serializer};
use serde_yaml;
use std::collections::BTreeMap;
use std::fmt;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use void::Void;

use self::helpers::*;
use self::string_or_struct::*;

mod helpers;
#[macro_use]
mod string_serialize_deserialize;
mod string_or_struct;

macro_rules! assert_roundtrip {
    ( $ty:ty, $yaml:expr ) => {
        {
            let yaml: &str = $yaml;
            let data: $ty = serde_yaml::from_str(&yaml).unwrap();
            let yaml2 = serde_yaml::to_string(&data).unwrap();
            assert_eq!(normalize_yaml(yaml), normalize_yaml(&yaml2));
        }
    }
}

/// A macro for including another source file directly into this one,
/// without defining a normal submodule, and with support for preprocessing
/// the source code using serde_codegen if necessary.
///
/// We generate as much of our (de)serialization code as possible using
/// serde, either in `serde_macros` mode (with nightly Rust) or in
/// `serde_codegen` mode called by `build.rs` (with stable Rust).
macro_rules! serde_include {
    ( $basename:expr ) => {
        // This code is run if we have a nightly build of Rust, and hence
        // compiler plugins.
        #[cfg(feature = "serde_macros")]
        include!(concat!($basename, ".in.rs"));

        // This code is run if we have a stable build of Rust.  The
        // `$preprocessed` file is generated from `$original` by `build.rs`
        // at build time.
        #[cfg(feature = "serde_codegen")]
        include!(concat!(env!("OUT_DIR"), "/v2/", $basename, ".rs"));
    };
}

// Support types.
serde_include!("aliased_name");
serde_include!("command_line");
serde_include!("memory_size");
serde_include!("host_mapping");

// Basic file structure.
serde_include!("file");
serde_include!("service");

// Service-related types.
serde_include!("build");
serde_include!("context");
serde_include!("extends");
serde_include!("logging");

