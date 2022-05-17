//! MPK_HTTP -- ACOUSTID
//!
//! Acoustid.org provides a complete audio identifaction web
//! service. The web service supports basic operations: uploading and
//! searching.
//!
//! REF: <https://acoustid.org/webservice>
//! ENDPOINT: <https://api.acoustid.org/v2/>
use std::fmt;

use reqwest::{IntoUrl, RequestBuilder, Response, Url};

use crate::{Client, ClientConfig, ClientExt, Error, Result};
pub const ACOUSTID_ENDPOINT: &str = "https://api.acoustid.org/v2";

#[derive(Debug, Default)]
pub struct AcoustIdClient {
  pub client: Client,
  pub cfg: ClientConfig,
}

impl ClientExt for AcoustIdClient {}

impl AcoustIdClient {
  pub fn new() -> AcoustIdClient {
    AcoustIdClient {
      client: Client::new(),
      cfg: ClientConfig::default(),
    }
  }
  /// Create a new AcoustIdClient with the given CFG.
  pub fn new_with_config(cfg: &ClientConfig) -> AcoustIdClient {
    AcoustIdClient {
      client: Client::new(),
      cfg: cfg.to_owned(),
    }
  }
  pub async fn request<'a>(&self, req: AcoustIdRequest<'a>) -> Result<Response> {
    let res = self
      .client
      .execute(
        req
          .request(&self.client)
          .bearer_auth(self.cfg.access_token.as_ref().unwrap())
          .build()?,
      )
      .await?;
    Ok(res)
  }
}

pub enum AcoustIdRequest<'a> {
  Lookup {
    client: &'a str,
    duration: u16,
    fingerprint: &'a str,
    meta: Option<&'a [&'a str]>,
    format: Option<ResponseFormat>,
  },
}

impl<'a> AcoustIdRequest<'a> {
  pub fn request(&self, client: &Client) -> RequestBuilder {
    client.get(self.addr()).query(self.params().as_slice())
  }

  pub fn addr(&self) -> String {
    let slug = match self {
      AcoustIdRequest::Lookup { .. } => "/lookup".to_string(),
    };
    format!("{}{}", ACOUSTID_ENDPOINT, slug)
  }

  pub fn params(&self) -> Vec<(String, String)> {
    match self {
      AcoustIdRequest::Lookup {
        format,
        client,
        duration,
        fingerprint,
        meta,
      } => {
        let mut params = vec![
          ("client".to_string(), client.to_string()),
          ("duration".to_string(), duration.to_string()),
          ("fingerprint".to_string(), fingerprint.to_string()),
        ];
        if let Some(m) = meta {
          params.push(("meta".to_string(), m.join("+")));
        }
        if let Some(f) = format {
          params.push(("format".to_string(), f.to_string()));
        }
        params
      }
    }
  }
}

pub enum ResponseFormat {
  Json,
  Jsonp,
  Xml,
}

impl fmt::Display for ResponseFormat {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      ResponseFormat::Json => f.write_str("json"),
      ResponseFormat::Jsonp => f.write_str("jsonp"),
      ResponseFormat::Xml => f.write_str("xml"),
    }
  }
}
