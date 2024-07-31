use std::{collections::BTreeMap, process::Stdio};

use anyhow::{anyhow, bail, Result};
use cassette_plugin_helm_core::{
    HelmDelete, HelmDeleteOutput, HelmPost, HelmPostOutput, HelmPut, HelmPutOutput,
};
use cassette_plugin_kubernetes_api::UserClient;
use itertools::Itertools;
use k8s_openapi::api::core::v1::Secret;
use kube::{api::ListParams, Api, Client, ResourceExt};
use serde::Deserialize;
use tokio::{
    io::AsyncWriteExt,
    process::{Child, Command},
};
use tracing::{info, instrument, Level};
use uuid::Uuid;

pub async fn delete(client: UserClient, id: Uuid, data: HelmDelete) -> Result<HelmDeleteOutput> {
    // Parse namespace
    let name = &data.name;
    let namespace = data
        .namespace
        .as_deref()
        .unwrap_or(client.spec.namespace.as_str());

    // Validate data
    validate_id(&client.kube, id, namespace, name).await?;

    todo!()
}

pub async fn install(client: UserClient, data: HelmPut) -> Result<HelmPutOutput> {
    execute(
        &client.spec.token,
        client.spec.namespace,
        &data,
        UpdateMode::Install,
    )
    .await
}

pub async fn upgrade(client: UserClient, id: Uuid, data: HelmPost) -> Result<HelmPostOutput> {
    // Parse namespace
    let name = &data.name;
    let namespace = data
        .namespace
        .as_deref()
        .unwrap_or(client.spec.namespace.as_str());

    // Validate data
    validate_id(&client.kube, id, namespace, name).await?;

    execute(
        &client.spec.token,
        client.spec.namespace,
        &data,
        UpdateMode::Upgrade,
    )
    .await
}

async fn validate_id(kube: &Client, id: Uuid, namespace: &str, name: &str) -> Result<()> {
    // Load old helm releases
    let api = Api::<Secret>::namespaced(kube.clone(), namespace);
    let lp = ListParams {
        label_selector: Some(format!("name={name},owner=helm")),
        ..Default::default()
    };
    let list = api
        .list_metadata(&lp)
        .await
        .map_err(|_| anyhow!("No such helm chart: {name}"))?;

    // Validate data
    let uid = Some(id.to_string());
    if list.iter().any(|item| item.uid() == uid) {
        Ok(())
    } else {
        bail!("No such helm chart: {name}")
    }
}

#[instrument(level = Level::INFO, skip(token))]
async fn execute(
    token: &str,
    user_namespace: String,
    data: &HelmPost,
    mode: UpdateMode,
) -> Result<HelmPostOutput> {
    let HelmPost {
        chart_name,
        name,
        namespace,
        repo,
        values,
    } = data;

    let chart = fetch_chart_url(chart_name, repo).await?;
    let namespace = namespace.clone().unwrap_or(user_namespace);
    let values = ::serde_json::to_string(values)?;

    info!("Executing helm command");
    let mut command = Command::new("helm")
        .args(mode.as_args())
        .arg(name)
        .arg(chart)
        .arg("--namespace")
        .arg(namespace)
        .arg("--values")
        .arg("-")
        .env("HELM_KUBETOKEN", token)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    if let Some(mut pipe) = command.stdin.take() {
        pipe.write_all(values.as_bytes()).await?;
    }
    drop(values);

    parse_command_output(command).await
}

#[instrument(level = Level::INFO, skip(token))]
async fn execute_delete(
    token: &str,
    user_namespace: String,
    data: &HelmDelete,
) -> Result<HelmPostOutput> {
    let HelmDelete { name, namespace } = data;

    let namespace = namespace.clone().unwrap_or(user_namespace);

    info!("Executing helm command");
    let command = Command::new("helm")
        .arg("delete")
        .arg(name)
        .arg("--namespace")
        .arg(namespace)
        .env("HELM_KUBETOKEN", token)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    parse_command_output(command).await
}

async fn parse_command_output(command: Child) -> Result<String> {
    let output = command.wait_with_output().await?;
    let status = output.status;
    if status.success() {
        let message = String::from_utf8(output.stdout)?;
        info!("Completed helm command: {message}");
        Ok(message)
    } else {
        let message = String::from_utf8(output.stderr)?;
        bail!("{message}")
    }
}

#[derive(Copy, Clone, Debug)]
enum UpdateMode {
    Install,
    Upgrade,
}

impl UpdateMode {
    const fn as_args(&self) -> &[&str] {
        match self {
            Self::Install => &["install"],
            Self::Upgrade => &["upgrade", "--install"],
        }
    }
}

async fn fetch_chart_url(chart_name: &str, repo: impl ToString) -> Result<String> {
    let response = ::reqwest::Client::new()
        .get(format!("{}/index.yaml", repo.to_string()))
        .send()
        .await
        .map_err(|_| anyhow!("Failed to request helm chart index: {chart_name}"))?;

    let HelmIndex { entries } = response
        .text()
        .await
        .map_err(|_| anyhow!("Failed to get helm chart index: {chart_name}"))
        .and_then(|body| {
            ::serde_yml::from_str(&body)
                .map_err(|_| anyhow!("Failed to parse helm chart index: {chart_name}"))
        })?;
    let entry = entries
        .iter()
        .filter(|(name, _)| *name == chart_name)
        .flat_map(|(_, entries)| entries)
        .filter(|entry| entry.r#type == "application")
        .sorted_by_key(|entry| entry.version.clone())
        .last()
        .ok_or_else(|| anyhow!("No such chart: {chart_name}"))?;
    entry
        .urls
        .first()
        .cloned()
        .ok_or_else(|| anyhow!("No such chart URL: {chart_name}"))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct HelmIndex {
    entries: BTreeMap<String, Vec<HelmIndexEntry>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct HelmIndexEntry {
    // name: String,
    r#type: String,
    urls: Vec<String>,
    version: String,
}
