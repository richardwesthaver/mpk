//! MPK_HTTP FREESOUND API
//!
//! Freesound.org is a collaborative database of Creative Commons
//! Licensed sounds. It has a growing library of audio uploaded by
//! people around the world. It also provides an intricate API with
//! features such as basic search, upload, download, and even
//! fingerprint search based on analysis files from Essentia.
//!
//! This module implements the client-side freesound.org API. It is
//! used in MPK_SESH and is especially useful with the analysis data
//! from MPK_DB.
//!
//! REF: https://freesound.org/docs/api/
//! ENDPOINT: https://freesound.org/apiv2/
use crate::{Client, Error, Result};
use futures_util::StreamExt;
use mpk_config::{ClientConfig, Config};
use mpk_util::{open_browser, ProgressBar, ProgressStyle};
use oauth2::{
  basic::{BasicClient, BasicTokenType},
  AuthUrl, AuthorizationCode, ClientId, ClientSecret, EmptyExtraTokenFields,
  RedirectUrl, RefreshToken, StandardTokenResponse, TokenResponse, TokenUrl,
};
use reqwest::{IntoUrl, RequestBuilder, Response, Url};
use serde::Deserialize;
use std::cmp::min;
use std::fmt;
use std::path::Path;
use std::time::SystemTime;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
pub const FREESOUND_ENDPOINT: &str = "https://freesound.org/apiv2";

pub async fn write_sound<P: AsRef<Path>>(
  res: Response,
  dst: P,
  progress: bool,
) -> Result<()> {
  let size = res.content_length().unwrap();
  let mut dst = File::create(dst.as_ref()).await.unwrap();
  let mut downloaded: u64 = 0;
  let pb = if progress {
    Some(
      ProgressBar::new(size)
	.with_style(ProgressStyle::default_spinner().template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
		    .progress_chars("#>-"))
    )
  } else {
    None
  };

  let mut stream = res.bytes_stream();
  while let Some(b) = stream.next().await {
    let chunk = b.unwrap();
    dst.write_all(&chunk).await.unwrap();
    let new = min(downloaded + (chunk.len() as u64), size);
    downloaded = new;
    if let Some(pb) = &pb {
      pb.set_position(new);
    }
  }
  dst.flush().await.unwrap();
  if let Some(pb) = &pb {
    pb.finish();
  }
  Ok(())
}

#[derive(Debug, Default)]
pub struct FreeSoundClient {
  pub client: Client,
  pub cfg: ClientConfig,
}

impl FreeSoundClient {
  /// Create a new FreeSoundClient.
  ///
  /// Note: freesound.org is an authenticated API and thus requires
  /// the CFG field to be populated. Calls to the API will fail if
  /// required CFG fields aren't updated. Prefer `new_with_config`
  /// method for initialization.
  pub fn new() -> FreeSoundClient {
    FreeSoundClient {
      client: Client::new(),
      cfg: ClientConfig {
        redirect_url: "http://localhost/freesound/auth".to_string(),
        ..Default::default()
      },
    }
  }

  /// Create a new FreeSoundClient with the given CFG.
  pub fn new_with_config(cfg: &ClientConfig) -> Self {
    FreeSoundClient {
      client: Client::new(),
      cfg: cfg.to_owned(),
    }
  }

  /// Update the net.freesound fields of a GlobalConfig.
  pub fn save_to_config(&self, cfg: &mut Config) {
    cfg.net.freesound = Some(self.cfg.to_owned());
  }

  pub fn auth_client(&self) -> BasicClient {
    let client_id = ClientId::new(self.cfg.client_id.clone());
    let client_secret = ClientSecret::new(self.cfg.client_secret.clone());
    let auth_url = AuthUrl::new(format!(
      "{}/oauth2/authorize/?client_id={}&response_type=code",
      FREESOUND_ENDPOINT, self.cfg.client_id
    ))
    .unwrap();
    let token_url =
      TokenUrl::new(format!("{}/oauth2/access_token/", FREESOUND_ENDPOINT)).unwrap();
    BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
      .set_redirect_uri(RedirectUrl::new(self.cfg.redirect_url.clone()).unwrap())
  }

  pub fn update_cfg(
    &mut self,
    token: StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
  ) {
    self.cfg.update(
      token.access_token().secret().as_str(),
      token.refresh_token().unwrap().secret().as_str(),
      token.expires_in().unwrap(),
      &token
        .scopes()
        .unwrap()
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>(),
    );
  }

  /// Do the Oauth2 Dance as described in
  /// https://freesound.org/docs/api/authentication.html#oauth2-authentication
  ///
  /// Step 1: user is redirected to a Freesound page where they log in
  /// and are asked to give permission to MPK.
  ///
  /// Step 2: If user grants access, Freesound redirects to the
  /// REDIRECT_URL with an authorization grant as a GET parameter.
  ///
  /// Step 3: MPK uses that authorization grant to request an access
  /// token that 'links' user with MPK and that needs to be added to
  /// all API requests.
  ///
  /// Note: all requests using OAuth2 API need to be made over HTTPS.
  pub async fn auth(&mut self, auto: bool) -> Result<()> {
    if self.refresh_auth().await.is_ok() {
      println!("token refresh successful");
      Ok(())
    } else {
      let client = self.auth_client();
      println!(
        "go to: {}/oauth2/authorize/?client_id={}&response_type=code",
        FREESOUND_ENDPOINT, self.cfg.client_id
      );
      if auto {
        open_browser(
          format!(
            "{}/oauth2/authorize/?client_id={}&response_type=code",
            FREESOUND_ENDPOINT, self.cfg.client_id
          )
          .as_str(),
        );
      }
      let listener = TcpListener::bind("localhost:8080").await.unwrap();
      loop {
        if let Ok((mut stream, _)) = listener.accept().await {
          let mut reader = BufReader::new(&mut stream);
          let mut line = String::new();
          reader.read_line(&mut line).await.unwrap();
          let redirect_url = line.split_whitespace().nth(1).unwrap();
          let url =
            Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

          let code_pair = url
            .query_pairs()
            .find(|pair| {
              let &(ref key, _) = pair;
              key == "code"
            })
            .unwrap();

          let (_, value) = code_pair;
          let code = AuthorizationCode::new(value.into_owned());
          println!("got code: {:?}", code.secret());
          // let state_pair = url
          //   .query_pairs()
          //   .find(|pair| {
          //     let &(ref key, _) = pair;
          //     key == "state"
          //   })
          //   .unwrap();

          // let (_, value) = state_pair;
          // state = CsrfToken::new(value.into_owned());
          let message = "Go back to your terminal :)";
          let response = format!(
            "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
            message.len(),
            message
          );
          stream.write_all(response.as_bytes()).await.unwrap();
          let token_res = client
            .exchange_code(code)
            .request_async(oauth2::reqwest::async_http_client)
            .await
            .unwrap();

          self.update_cfg(token_res);
          break Ok(());
        }
      }
    }
  }

  pub async fn refresh_auth(&mut self) -> Result<()> {
    if let Some(d) = self.cfg.expires {
      let exp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("SystemTime is befoer UNIX_EPOCH!?")
        .as_secs();
      if exp < d {
        let client = self.auth_client();
        if let Some(t) = &self.cfg.refresh_token {
          let token_res = client
            .exchange_refresh_token(&RefreshToken::new(t.to_string()))
            .request_async(oauth2::reqwest::async_http_client)
            .await
            .unwrap();
          self.update_cfg(token_res);
          return Ok(());
        } else {
          return Err(Error::TokenRefreshFailed);
        }
      } else {
        return Err(Error::TokenExpired);
      }
    }
    Ok(())
  }

  pub async fn request<'a>(&mut self, req: FreeSoundRequest<'a>) -> Result<Response> {
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

  pub async fn get_raw<U: IntoUrl>(&mut self, url: U) -> Result<Response> {
    let res = self
      .client
      .get(url)
      .bearer_auth(self.cfg.access_token.as_ref().unwrap())
      .send()
      .await?;
    Ok(res)
  }
}

pub enum FreeSoundRequest<'a> {
  SearchText {
    query: &'a str,
    filter: &'a str,
    sort: &'a str,
    group_by_pack: bool,
    weights: &'a str,
    page: usize,
    /// max = 150
    page_size: u8,
    fields: &'a [&'a str],
    descriptors: &'a [&'a str],
    normalized: bool,
  },
  SearchContent,
  SearchCombined,
  Sound {
    id: u64,
  },
  SoundAnalysis {
    id: u64,
  },
  SoundSimilar {
    id: u64,
  },
  SoundComments {
    id: u64,
  },
  SoundDownload {
    id: u64,
  },
  SoundUpload,
  SoundDescribe,
  SoundPendingUpload,
  SoundEdit,
  SoundBookmark {
    id: u64,
    name: &'a str,
    category: &'a str,
  },
  SoundRate,
  SoundComment,
  User,
  UserSounds,
  UserPacks,
  UserBookmarkCategories,
  UserBookmarkCategorySounds,
  Pack,
  PackSounds,
  PackDownload,
  Me,
  Descriptors,
}

impl<'a> FreeSoundRequest<'a> {
  pub fn request(&self, client: &Client) -> RequestBuilder {
    client.get(self.addr()).query(self.params().as_slice())
  }

  pub fn addr(&self) -> String {
    let slug = match self {
      FreeSoundRequest::SearchText {
        query: _,
        filter: _,
        sort: _,
        group_by_pack: _,
        weights: _,
        page: _,
        page_size: _,
        fields: _,
        descriptors: _,
        normalized: _,
      } => "/search/text".to_string(),
      FreeSoundRequest::SearchContent => "/search/content".to_string(),
      FreeSoundRequest::SearchCombined => "/search/combined".to_string(),
      FreeSoundRequest::Sound { ref id } => format!("/sounds/{}", id),
      FreeSoundRequest::SoundAnalysis { ref id } => format!("/sounds/{}/analysis", id),
      FreeSoundRequest::SoundSimilar { ref id } => format!("/sounds/{}/similar", id),
      FreeSoundRequest::SoundComments { ref id } => format!("/sounds/{}/comments", id),
      FreeSoundRequest::SoundDownload { ref id } => format!("/sounds/{}/download", id),
      _ => "".to_string(),
    };
    format!("{}{}", FREESOUND_ENDPOINT, slug)
  }

  pub fn params(&self) -> Vec<(String, String)> {
    match self {
      FreeSoundRequest::SearchText {
        query,
        filter,
        sort,
        group_by_pack,
        weights,
        page,
        page_size,
        fields,
        descriptors,
        normalized,
      } => {
        let gbp = if *group_by_pack { "1" } else { "0" }.to_string();
        let normalized = if *normalized { "1" } else { "0" }.to_string();
        vec![
          ("query".to_string(), query.to_string()),
          ("filter".to_string(), filter.to_string()),
          ("sort".to_string(), sort.to_string()),
          ("group_by_pack".to_string(), gbp),
          ("normalized".to_string(), normalized),
          ("weights".to_string(), weights.to_string()),
          ("fields".to_string(), fields.join(",")),
          ("descriptors".to_string(), descriptors.join(",")),
          ("page".to_string(), page.to_string()),
          ("page_size".to_string(), page_size.to_string()),
        ]
      }
      _ => {
        vec![]
      }
    }
  }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum FreeSoundResponse {
  SearchText {
    count: usize,
    next: Option<Url>,
    results: Vec<FreeSoundSearchResult>,
    previous: Option<Url>,
  },
}

impl FreeSoundResponse {
  pub async fn parse(res: Response) -> FreeSoundResponse {
    res.json::<FreeSoundResponse>().await.unwrap()
  }
}

impl fmt::Display for FreeSoundResponse {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      FreeSoundResponse::SearchText {
        count,
        next,
        results,
        previous,
      } => {
        let next = next
          .as_ref()
          .map(|u| u.to_string())
          .unwrap_or_else(|| "null".to_string());
        let previous = previous
          .as_ref()
          .map(|u| u.to_string())
          .unwrap_or_else (|| "null".to_string());
        let res: String = results
          .iter()
          .map(|r| r.to_string())
          .collect::<Vec<String>>()
          .join("\n");
        f.write_str(
          format!(
            "count: {}, results: [\n{}\n], next: {}, previous: {}",
            count, res, next, previous
          )
          .as_str(),
        )
      }
    }
  }
}

#[derive(Deserialize, Debug)]
pub struct FreeSoundSearchResult {
  pub id: Option<u64>,
  pub name: Option<String>,
  pub tags: Option<Vec<String>>,
  pub license: Option<Url>,
}

impl fmt::Display for FreeSoundSearchResult {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if let Some(n) = &self.id {
      write!(f, "{}, ", n)?
    };
    if let Some(s) = &self.name {
      write!(f, "name: {} ", s)?
    };
    if let Some(v) = &self.tags {
      write!(f, "tags: {}, ", v.join(":"))?
    };
    if let Some(s) = &self.license {
      write!(f, "license: {}, ", s)?
    };
    Ok(())
  }
}
