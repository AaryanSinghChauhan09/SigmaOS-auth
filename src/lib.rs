//! SigmaOS Auth
//! Authentication and authorization framework

#![no_std]

pub mod auth;

pub use auth::SigmaPAM as AuthManager;
