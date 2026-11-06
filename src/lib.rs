//! SigmaOS Auth
//! Authentication and authorization framework

#![no_std]

pub mod auth;

pub use auth::SigmaPAM as AuthManager;

// Component modules
pub mod config;
pub mod error;
pub mod service;
