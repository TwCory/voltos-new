// Copyright (C) 2025 Cory Voltz <voltzc@twchl.net>

//!	VoltOS - Network Interface Library
//!
//!	File:           srctree/lib/interface/src/control.rs
//!
//!	Description:    Interface Management and Control Functions

use crate::store::IfStore;
use crate::common::*;

use rtnetlink::{Handle, new_connection};
use std::sync::Arc;
use tokio::sync::RwLock;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IfControlError {
    #[error("Netlink Error: {0}")]
    Netlink(String),
    #[error("I/O Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Not Found Error: {0}")]
    NotFound(String),
    #[error("Invalid Parameter: {0}")]
    Invalid(String),
    #[error("NIX Crate Error: {0}")]
    Nix(#[from] nix::Error),
}

pub type Result<T> = std::result::Result<T, IfControlError>;

#[derive(Clone)]
pub struct IfControlMgr {
    pub nl_handle:          Handle,
    pub store:              Arc<RwLock<IfStore>>,
}

impl IfControlMgr {
    pub async fn new() -> Result<Self> {

    }

    /// Initial scan of interfaces and addresses into the arena.
    pub async fn sync(&self) -> Result<()> {

    }

    pub async fn id_with_name(&self, name: &str) -> Result<IfKey> {

    }

    pub async fn ifp(&self, key: IfKey) -> Result<Interface> {
        
    }
}