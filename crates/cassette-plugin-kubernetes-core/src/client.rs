use anyhow::{anyhow, Error, Result};
use cassette_core::net::gateway::get_gateway;
use gloo_net::http::{Headers, RequestBuilder};
use http::Request;
use js_sys::Uint8Array;
use once_cell::sync::OnceCell;
use serde::de::DeserializeOwned;

#[derive(Clone)]
pub struct Client {}

impl Client {
    pub fn current() -> Result<&'static Self> {
        static SELF: OnceCell<Result<Client, String>> = OnceCell::new();
        SELF.get_or_init(Self::try_default);
        match SELF.get().unwrap() {
            Ok(client) => Ok(client),
            Err(error) => Err(Error::msg(error)),
        }
    }

    fn try_default() -> Result<Self, String> {
        Ok(Self {})
    }
}

impl Client {
    pub(crate) async fn request<T>(
        &self,
        name: &'static str,
        request: Request<Vec<u8>>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let (parts, body) = request.into_parts();

        let Self {} = self;
        let url = {
            let gateway = get_gateway();
            let url_path = parts.uri;
            format!("{gateway}/kube{url_path}")
        };

        let headers = Headers::new();
        for (key, value) in &parts.headers {
            headers.append(key.as_str(), value.to_str()?)
        }

        let builder = RequestBuilder::new(&url)
            .method(parts.method.as_str().parse()?)
            .headers(headers);

        let request = if body.is_empty() {
            builder.build()
        } else {
            let array = unsafe { Uint8Array::view(&body) };
            builder.body(array)
        }
        .map_err(|error| anyhow!("Failed to create a builder {name:?}: {error}"))?;

        let response = request
            .send()
            .await
            .map_err(|error| anyhow!("Failed to fetch {name:?}: {error}"))?;

        let status = response.status();
        if status >= 200 && status < 400 {
            response
                .json()
                .await
                .map_err(|error| anyhow!("Failed to parse response {name:?}: {error}"))
        } else {
            let error = response
                .text()
                .await
                .map_err(|error| anyhow!("Failed to parse response error {name:?}: {error}"))?;
            Err(Error::msg(error))
        }
    }
}
