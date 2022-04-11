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
use crate::{Client, Result};
use mpk_config::ClientConfig;
use oauth2::{
  basic::BasicClient, AuthUrl, AuthorizationCode, ClientId, ClientSecret, RedirectUrl,
  Scope, TokenResponse, TokenUrl,
};
use reqwest::Url;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

pub const FREESOUND_ENDPOINT: &str = "https://freesound.org/apiv2";

pub struct FreeSoundClient {
  pub client: Client,
  pub cfg: ClientConfig,
}

impl FreeSoundClient {
  pub fn new() -> FreeSoundClient {
    FreeSoundClient {
      client: Client::new(),
      cfg: ClientConfig {
        client_id: "".to_string(),
        client_secret: "".to_string(),
        redirect_url: "http://localhost/freesound/auth".to_string(),
      },
    }
  }

  pub fn new_with_config(cfg: ClientConfig) -> Self {
    FreeSoundClient {
      client: Client::new(),
      cfg,
    }
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
  pub async fn authorize(&self) -> Result<()> {
    let client_id = ClientId::new(self.cfg.client_id.clone());
    let client_secret = ClientSecret::new(self.cfg.client_secret.clone());
    let auth_url = AuthUrl::new(format!(
      "{}/oauth2/authorize/?client_id={}&response_type=code",
      FREESOUND_ENDPOINT, self.cfg.client_id
    ))
    .unwrap();
    let token_url =
      TokenUrl::new("https://freesound.org/apiv2/oauth2/access_token/".to_string())
        .unwrap();
    println!(
      "go to: {}/authorize/?client_id={}&response_type=code",
      FREESOUND_ENDPOINT, self.cfg.client_id
    );
    let client =
      BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
        .set_redirect_uri(RedirectUrl::new(self.cfg.redirect_url.clone()).unwrap());

    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    while let Ok((mut stream, _)) = listener.accept().await {
      let mut reader = BufReader::new(&mut stream);
      let mut line = String::new();
      reader.read_line(&mut line).await.unwrap();
      let redirect_url = line.split_whitespace().nth(1).unwrap();
      let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

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
      println!("Freesound returned the following token:\n{:?}\n", token_res);
      break;
    }
    Ok(())
  }
}

pub enum FreeSoundRequest {
  SearchText,
  SearchContent,
  SearchCombined,
  Sound,
  SoundAnalysis,
  SoundSimilar,
  SoundComments,
  SoundDownload,
  SoundUpload,
  SoundDescribe,
  SoundPendingUpload,
  SoundEdit,
  SoundBookmark,
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
