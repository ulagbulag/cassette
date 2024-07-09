use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use anyhow::Result;
use cassette_core::{cassette::CassetteRef, component::CassetteComponentSpec};
use clap::Parser;
use tracing::{instrument, Level};
use uuid::Uuid;

use crate::db::CassetteDB;

#[derive(Clone)]
pub struct Agent {
    args: AgentArgs,
    db: CassetteDB,
}

impl Agent {
    #[instrument(level = Level::INFO, skip())]
    pub async fn try_default() -> Result<Self> {
        let args = AgentArgs::try_parse()?;
        Self::try_new(args).await
    }

    #[instrument(level = Level::INFO, skip())]
    pub async fn try_new(args: AgentArgs) -> Result<Self> {
        Ok(Self {
            args,
            db: CassetteDB::default(),
        })
    }

    pub(crate) const fn bind_addr(&self) -> SocketAddr {
        self.args.bind_addr
    }

    pub(crate) const fn db(&self) -> &CassetteDB {
        &self.db
    }
}

impl Agent {
    #[instrument(level = Level::INFO, skip(self))]
    pub async fn get(&self, namespace: &str, id: Uuid) -> Option<CassetteComponentSpec> {
        self.db.get(namespace, id).await
    }

    #[instrument(level = Level::INFO, skip(self))]
    pub async fn list(&self, namespace: &str) -> Vec<CassetteRef> {
        self.db.list(namespace).await
    }
}

#[derive(Clone, Debug, PartialEq, Parser)]
pub struct AgentArgs {
    #[arg(long, env, default_value_t = AgentArgs::default_bind_addr())]
    pub bind_addr: SocketAddr,
}

impl AgentArgs {
    const fn default_bind_addr() -> SocketAddr {
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080))
    }
}
