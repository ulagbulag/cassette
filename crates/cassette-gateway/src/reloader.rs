use std::fmt;

use anyhow::Result;
use ark_core::signal::FunctionSignal;
use cassette_core::{cassette::CassetteCrd, components::CassetteComponentCrd};
use futures::{TryFuture, TryStreamExt};
use kube::{
    runtime::watcher::{watcher, Config, Error as WatcherError, Event},
    Api, Client, Resource, ResourceExt,
};
use serde::de::DeserializeOwned;
use tokio::select;
use tracing::{error, instrument, Level};

use crate::db::CassetteDB;

pub(crate) struct CassetteDBReloader {
    db: CassetteDB,
    kube: Client,
}

impl CassetteDBReloader {
    pub(crate) async fn try_new(db: CassetteDB) -> Result<Self> {
        Ok(Self {
            db,
            kube: Client::try_default().await?,
        })
    }

    pub(crate) async fn loop_forever(self, signal: FunctionSignal) {
        select! {
            result = self.try_loop_forever_cassette() => match result {
                Ok(()) => signal.terminate(),
                Err(error) => {
                    error!("failed to operate cassette reloader: {error}");
                    signal.terminate_on_panic();
                }
            },
            result = self.try_loop_forever_cassette_component() => match result {
                Ok(()) => signal.terminate(),
                Err(error) => {
                    error!("failed to operate cassette component reloader: {error}");
                    signal.terminate_on_panic();
                }
            },
        }
    }

    async fn try_loop_forever_cassette(&self) -> Result<()> {
        self.try_loop_forever_with(|e| self.handle_cassette(e))
            .await
    }

    async fn try_loop_forever_cassette_component(&self) -> Result<()> {
        self.try_loop_forever_with(|e| self.handle_cassette_component(e))
            .await
    }

    async fn try_loop_forever_with<K, F, Fut>(&self, handle_event: F) -> Result<()>
    where
        K: 'static + Send + Clone + fmt::Debug + DeserializeOwned + Resource,
        <K as Resource>::DynamicType: Default,
        F: Fn(Event<K>) -> Fut,
        Fut: TryFuture<Ok = (), Error = WatcherError>,
    {
        let api = Api::all(self.kube.clone());
        let config = Config::default();

        watcher(api.clone(), config)
            .try_for_each(handle_event)
            .await
            .map_err(Into::into)
    }

    async fn handle_cassette(&self, event: Event<CassetteCrd>) -> Result<(), WatcherError> {
        match event {
            Event::Apply(cr) | Event::InitApply(cr) => self.handle_cassette_apply(cr).await,
            Event::Delete(cr) => self.handle_cassette_delete(cr).await,
            Event::Init | Event::InitDone => Ok(()),
        }
    }

    #[instrument(
        level = Level::INFO,
        skip(self, cr),
        fields(name = %cr.name_any(), namespace = cr.namespace()),
    )]
    async fn handle_cassette_apply(&self, cr: CassetteCrd) -> Result<(), WatcherError> {
        self.db.insert(cr).await;
        Ok(())
    }

    #[instrument(
        level = Level::INFO,
        skip(self, cr),
        fields(name = %cr.name_any(), namespace = cr.namespace()),
    )]
    async fn handle_cassette_delete(&self, cr: CassetteCrd) -> Result<(), WatcherError> {
        self.db.remove(cr).await;
        Ok(())
    }

    async fn handle_cassette_component(
        &self,
        event: Event<CassetteComponentCrd>,
    ) -> Result<(), WatcherError> {
        match event {
            Event::Apply(cr) | Event::InitApply(cr) => {
                self.handle_cassette_component_apply(cr).await
            }
            Event::Delete(cr) => self.handle_cassette_component_delete(cr).await,
            Event::Init | Event::InitDone => Ok(()),
        }
    }

    #[instrument(
        level = Level::INFO,
        skip(self, cr),
        fields(name = %cr.name_any(), namespace = cr.namespace()),
    )]
    async fn handle_cassette_component_apply(
        &self,
        cr: CassetteComponentCrd,
    ) -> Result<(), WatcherError> {
        self.db.insert_component(cr).await;
        Ok(())
    }

    #[instrument(
        level = Level::INFO,
        skip(self, cr),
        fields(name = %cr.name_any(), namespace = cr.namespace()),
    )]
    async fn handle_cassette_component_delete(
        &self,
        cr: CassetteComponentCrd,
    ) -> Result<(), WatcherError> {
        self.db.remove_component(cr).await;
        Ok(())
    }
}
