//! # CRAbE_framework
//!
//! This crate provides shared utilities and settings for the CRAbE project.
//!
//! The `config` module contains settings that are common to multiple CRAbE crates.
//!
//! This crate is intended to be used as a dependency by other CRAbE crates, and should not be used on its own. Please refer to the individual modules for more information and usage instructions.

/// This module contains the structure of the configuration settings that are shared across multiple other crates in the CRAbE project.
/// These settings are meant to be accessed and used by other CRAbE crates as a way of maintaining consistency across the project.
/// Please refer to the documentation of individual settings for more information and usage instructions.
pub mod config;

/// This module defines the schema of all data that is passed between the various modules in the system.
/// Each crate typically implements a subset of these data types to handle a specific aspect of the system's functionality.
pub mod data;