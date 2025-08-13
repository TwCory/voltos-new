// Copyright (C) 2025 Cory Voltz <voltzc@twchl.net>

//!	VoltOS - Network Interface Library
//!
//!	File:           srctree/lib/interface/src/tasks/create.rs
//!
//!	Description:    Interface Creation Functions

use crate::common::*;
use crate::control::{IfControlMgr, Result, IfControlError};

use rtnetlink::packet::rtnl::link::nlas::Nla;
use rtnetlink::packet::rtnl::link::nlas::InfoKind;
use rtnetlink::packet::rtnl::link::nlas::InfoData;

impl IfControlMgr {
    pub async fn create_dummy(&self, name: &str) -> Result<IfKey> {
        let req = self.nl_handle.link().add();
        req.message_mut().nlas.push(Nla::IfName(name.to_string()));
        req.message_mut().nlas.push(Nla::Info(vec![
            rtnetlink::packet::rtnl::link::nlas::Info::Kind(InfoKind::Dummy),
        ]));
        req.execute().await.map_err(|e| IfControlError::Netlink(e.to_string()))?;
        self.sync().await?;
        self.id_with_name(name).await
    }
}