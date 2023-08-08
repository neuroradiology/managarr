use std::fmt::Debug;
use std::sync::Arc;

use anyhow::anyhow;
use log::{debug, error};
use regex::Regex;
use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;
use serde::Serialize;
use strum_macros::Display;
use tokio::sync::{Mutex, MutexGuard};

use crate::app::App;
use crate::network::radarr_network::RadarrEvent;

pub(crate) mod radarr_network;
mod utils;

#[derive(PartialEq, Eq, Debug)]
pub enum NetworkEvent {
  Radarr(RadarrEvent),
}

pub struct Network<'a, 'b> {
  pub client: Client,
  pub app: &'a Arc<Mutex<App<'b>>>,
}

impl<'a, 'b> Network<'a, 'b> {
  pub fn new(client: Client, app: &'a Arc<Mutex<App<'b>>>) -> Self {
    Network { client, app }
  }

  pub async fn handle_network_event(&self, network_event: NetworkEvent) {
    match network_event {
      NetworkEvent::Radarr(radarr_event) => self.handle_radarr_event(radarr_event).await,
    }

    let mut app = self.app.lock().await;
    app.is_loading = false;
  }

  pub async fn handle_request<B, R>(
    &self,
    request_props: RequestProps<B>,
    mut app_update_fn: impl FnMut(R, MutexGuard<'_, App<'_>>),
  ) where
    B: Serialize + Default + Debug,
    R: DeserializeOwned,
  {
    let method = request_props.method;
    match self.call_api(request_props).await.send().await {
      Ok(response) => {
        if response.status().is_success() {
          match method {
            RequestMethod::Get | RequestMethod::Post => {
              match utils::parse_response::<R>(response).await {
                Ok(value) => {
                  let app = self.app.lock().await;
                  app_update_fn(value, app);
                }
                Err(e) => {
                  error!("Failed to parse response! {:?}", e);
                  self
                    .app
                    .lock()
                    .await
                    .handle_error(anyhow!("Failed to parse response! {:?}", e));
                }
              }
            }
            RequestMethod::Delete | RequestMethod::Put => (),
          }
        } else {
          let status = response.status();
          let whitespace_regex = Regex::new(r"\s+").unwrap();
          let response_body = response.text().await.unwrap_or_default();
          let error_body = whitespace_regex
            .replace_all(&response_body.replace('\n', " "), " ")
            .to_string();

          error!(
            "Request failed. Received {} response code with body: {}",
            status, response_body
          );
          self.app.lock().await.handle_error(anyhow!(
            "Request failed. Received {} response code with body: {}",
            status,
            error_body
          ));
        }
      }
      Err(e) => {
        error!("Failed to send request. {:?}", e);
        self
          .app
          .lock()
          .await
          .handle_error(anyhow!("Failed to send request. {} ", e));
      }
    }
  }

  async fn call_api<T: Serialize + Default + Debug>(
    &self,
    request_props: RequestProps<T>,
  ) -> RequestBuilder {
    let RequestProps {
      uri,
      method,
      body,
      api_token,
    } = request_props;
    debug!("Creating RequestBuilder for resource: {:?}", uri);
    let app = self.app.lock().await;
    debug!(
      "Sending {:?} request to {} with body {:?}",
      method, uri, body
    );

    match method {
      RequestMethod::Get => app.client.get(uri).header("X-Api-Key", api_token),
      RequestMethod::Post => app
        .client
        .post(uri)
        .json(&body.unwrap_or_default())
        .header("X-Api-Key", api_token),
      RequestMethod::Put => app
        .client
        .put(uri)
        .json(&body.unwrap_or_default())
        .header("X-Api-Key", api_token),
      RequestMethod::Delete => app.client.delete(uri).header("X-Api-Key", api_token),
    }
  }
}

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq)]
pub enum RequestMethod {
  Get,
  Post,
  Put,
  Delete,
}

#[derive(Debug)]
pub struct RequestProps<T: Serialize + Debug> {
  pub uri: String,
  pub method: RequestMethod,
  pub body: Option<T>,
  pub api_token: String,
}

#[cfg(test)]
mod tests {
  use std::fmt::Debug;
  use std::string::ToString;
  use std::sync::Arc;

  use mockito::{Mock, Server, ServerGuard};
  use pretty_assertions::assert_str_eq;
  use rstest::rstest;
  use serde::{Deserialize, Serialize};
  use tokio::sync::Mutex;

  use crate::app::{App, RadarrConfig};
  use crate::models::HorizontallyScrollableText;
  use crate::network::radarr_network::RadarrEvent;
  use crate::network::{Network, RequestMethod, RequestProps};

  #[tokio::test]
  async fn test_handle_network_event_radarr_event() {
    let mut server = Server::new_async().await;
    let radarr_server = server
      .mock("GET", "/api/v3/health")
      .with_status(200)
      .create_async()
      .await;
    let host = server.host_with_port().split(':').collect::<Vec<&str>>()[0].to_owned();
    let port = Some(
      server.host_with_port().split(':').collect::<Vec<&str>>()[1]
        .parse()
        .unwrap(),
    );
    let mut app = App::default();
    app.is_loading = true;
    let radarr_config = RadarrConfig {
      host,
      api_token: String::default(),
      port,
    };
    app.config.radarr = radarr_config;
    let app_arc = Arc::new(Mutex::new(app));
    let network = Network::new(reqwest::Client::new(), &app_arc);

    network
      .handle_network_event(RadarrEvent::HealthCheck.into())
      .await;

    radarr_server.assert_async().await;
    assert!(!app_arc.lock().await.is_loading);
  }

  #[rstest]
  #[tokio::test]
  async fn test_handle_request_no_response_body(
    #[values(RequestMethod::Post, RequestMethod::Put, RequestMethod::Delete)]
    request_method: RequestMethod,
  ) {
    let mut server = Server::new_async().await;
    let async_server = server
      .mock(&request_method.to_string().to_uppercase(), "/test")
      .match_header("X-Api-Key", "test1234")
      .with_status(200)
      .create_async()
      .await;
    let app_arc = Arc::new(Mutex::new(App::default()));
    let network = Network::new(reqwest::Client::new(), &app_arc);

    network
      .handle_request::<Test, ()>(
        RequestProps {
          uri: format!("{}/test", server.url()),
          method: request_method,
          body: Some(Test {
            value: "Test".to_owned(),
          }),
          api_token: "test1234".to_owned(),
        },
        |_, _| (),
      )
      .await;

    async_server.assert_async().await;
  }

  #[rstest]
  #[tokio::test]
  async fn test_handle_request_with_response_body(
    #[values(RequestMethod::Get, RequestMethod::Post)] request_method: RequestMethod,
  ) {
    let (async_server, app_arc, server) = mock_api(request_method, 200, true).await;
    let network = Network::new(reqwest::Client::new(), &app_arc);

    network
      .handle_request::<(), Test>(
        RequestProps {
          uri: format!("{}/test", server.url()),
          method: request_method,
          body: None,
          api_token: "test1234".to_owned(),
        },
        |response, mut app| app.error = HorizontallyScrollableText::from(response.value),
      )
      .await;

    async_server.assert_async().await;
    assert_str_eq!(app_arc.lock().await.error.text, "Test");
  }

  #[tokio::test]
  async fn test_handle_request_get_invalid_body() {
    let mut server = Server::new_async().await;
    let async_server = server
      .mock("GET", "/test")
      .match_header("X-Api-Key", "test1234")
      .with_status(200)
      .with_body(r#"{ "invalid": "INVALID" }"#)
      .create_async()
      .await;
    let app_arc = Arc::new(Mutex::new(App::default()));
    let network = Network::new(reqwest::Client::new(), &app_arc);

    network
      .handle_request::<(), Test>(
        RequestProps {
          uri: format!("{}/test", server.url()),
          method: RequestMethod::Get,
          body: None,
          api_token: "test1234".to_owned(),
        },
        |response, mut app| app.error = HorizontallyScrollableText::from(response.value),
      )
      .await;

    async_server.assert_async().await;
    assert!(app_arc
      .lock()
      .await
      .error
      .text
      .starts_with("Failed to parse response!"));
  }

  #[tokio::test]
  async fn test_handle_request_failure_to_send_request() {
    let app_arc = Arc::new(Mutex::new(App::default()));
    let network = Network::new(reqwest::Client::new(), &app_arc);

    network
      .handle_request::<(), Test>(
        RequestProps {
          uri: String::default(),
          method: RequestMethod::Get,
          body: None,
          api_token: "test1234".to_owned(),
        },
        |response, mut app| app.error = HorizontallyScrollableText::from(response.value),
      )
      .await;

    assert!(app_arc
      .lock()
      .await
      .error
      .text
      .starts_with("Failed to send request."));
  }

  #[rstest]
  #[tokio::test]
  async fn test_handle_request_non_success_code(
    #[values(
      RequestMethod::Get,
      RequestMethod::Post,
      RequestMethod::Put,
      RequestMethod::Delete
    )]
    request_method: RequestMethod,
  ) {
    let (async_server, app_arc, server) = mock_api(request_method, 404, true).await;
    let network = Network::new(reqwest::Client::new(), &app_arc);

    network
      .handle_request::<(), Test>(
        RequestProps {
          uri: format!("{}/test", server.url()),
          method: request_method,
          body: None,
          api_token: "test1234".to_owned(),
        },
        |response, mut app| app.error = HorizontallyScrollableText::from(response.value),
      )
      .await;

    async_server.assert_async().await;
    assert_str_eq!(
      app_arc.lock().await.error.text,
      r#"Request failed. Received 404 Not Found response code with body: { "value": "Test" }"#
    );
  }

  #[tokio::test]
  async fn test_handle_request_non_success_code_empty_response_body() {
    let (async_server, app_arc, server) = mock_api(RequestMethod::Post, 404, false).await;
    let network = Network::new(reqwest::Client::new(), &app_arc);

    network
      .handle_request::<(), Test>(
        RequestProps {
          uri: format!("{}/test", server.url()),
          method: RequestMethod::Post,
          body: None,
          api_token: "test1234".to_owned(),
        },
        |response, mut app| app.error = HorizontallyScrollableText::from(response.value),
      )
      .await;

    async_server.assert_async().await;
    assert_str_eq!(
      app_arc.lock().await.error.text,
      r#"Request failed. Received 404 Not Found response code with body: "#
    );
  }

  #[rstest]
  #[tokio::test]
  async fn test_call_api(
    #[values(
      RequestMethod::Get,
      RequestMethod::Post,
      RequestMethod::Put,
      RequestMethod::Delete
    )]
    request_method: RequestMethod,
  ) {
    let mut server = Server::new_async().await;
    let mut async_server = server
      .mock(&request_method.to_string().to_uppercase(), "/test")
      .match_header("X-Api-Key", "test1234")
      .with_status(200);
    let mut body = None::<Test>;

    if request_method == RequestMethod::Post {
      async_server = async_server.with_body(
        r#"{ 
        "value": "Test" 
      }"#,
      );
      body = Some(Test {
        value: "Test".to_owned(),
      });
    }

    async_server = async_server.create_async().await;
    let app_arc = Arc::new(Mutex::new(App::default()));
    let network = Network::new(reqwest::Client::new(), &app_arc);

    network
      .call_api(RequestProps {
        uri: format!("{}/test", server.url()),
        method: request_method,
        body,
        api_token: "test1234".to_owned(),
      })
      .await
      .send()
      .await
      .unwrap();

    async_server.assert_async().await;
  }

  #[derive(Serialize, Deserialize, Debug, Default)]
  struct Test {
    pub value: String,
  }

  async fn mock_api<'a>(
    method: RequestMethod,
    response_status: usize,
    has_response_body: bool,
  ) -> (Mock, Arc<Mutex<App<'a>>>, ServerGuard) {
    let mut server = Server::new_async().await;
    let mut async_server = server
      .mock(&method.to_string().to_uppercase(), "/test")
      .match_header("X-Api-Key", "test1234")
      .with_status(response_status);

    if has_response_body {
      async_server = async_server.with_body(
        r#"{ 
        "value": "Test" 
      }"#,
      );
    }

    async_server = async_server.create_async().await;
    let app_arc = Arc::new(Mutex::new(App::default()));

    (async_server, app_arc, server)
  }
}
