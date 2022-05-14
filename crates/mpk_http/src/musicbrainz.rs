//! MPK_HTTP MUSICBRAINZ API
//!
//! Musicbrainz.org is an open music encyclopedia that collects music
//! metadata and makes it available to the public. It's a great tool
//! for musical discovery and provides an API for searching the
//! encyclopedia.
//!
//! This module implements the client-side musicbrainz.org API. It is
//! used in MPK_SESH with help from the track_tags_musicbrainz table
//! in MPK_DB.
//!
//! REF: https://musicbrainz.org/doc/MusicBrainz_API
//! ENDPOINT: https://musicbrainz.org/ws/2/
use std::fmt;
use std::str::FromStr;

use mpk_config::Config;
use reqwest::{IntoUrl, RequestBuilder, Response};

use crate::{Client, ClientConfig, ClientExt, Error, Result, USER_AGENT};

pub const MUSICBRAINZ_ENDPOINT: &str = "https://musicbrainz.org/ws/2";

pub struct MusicBrainzClient {
  pub client: Client,
  pub cfg: ClientConfig,
}

impl ClientExt for MusicBrainzClient {}

impl MusicBrainzClient {
  pub fn new() -> MusicBrainzClient {
    MusicBrainzClient {
      client: Client::new(),
      cfg: ClientConfig::default(),
    }
  }
  pub fn new_with_config(cfg: &ClientConfig) -> Self {
    MusicBrainzClient {
      client: Client::new(),
      cfg: cfg.to_owned(),
    }
  }
  pub fn save_to_config(&self, cfg: &mut Config) {
    cfg.net.musicbrainz = Some(self.cfg.to_owned());
  }
  pub async fn request<'a>(&self, req: MusicBrainzRequest<'a>) -> Result<Response> {
    let res = self
      .client
      .execute(
        req
          .request(&self.client)
          .header("User-Agent", USER_AGENT)
          .header("Accept", "application/json")
          .build()?,
      )
      .await?;
    Ok(res)
  }
  pub async fn get_raw<U: IntoUrl>(&self, url: U) -> Result<Response> {
    let res = self
      .client
      .get(url)
      .header("User-Agent", USER_AGENT)
      .header("Accept", "application/json")
      .send()
      .await?;
    Ok(res)
  }
}

#[derive(Debug)]
pub enum MusicBrainzRequest<'a> {
  Lookup {
    resource: MusicBrainzResource,
    mbid: &'a str,
    inc: Option<&'a [&'a str]>,
  },
  Browse {
    result: MusicBrainzResource,
    browse: MusicBrainzResource,
    mbid: &'a str,
    limit: u32,
    offset: u32,
  },
  Search {
    resource: MusicBrainzResource,
    query: &'a str,
    limit: u32,
    offset: u32,
  },
}

impl<'a> MusicBrainzRequest<'a> {
  pub fn request(&self, client: &Client) -> RequestBuilder {
    client.get(self.addr()).query(self.params().as_slice())
  }
  pub fn addr(&self) -> String {
    let slug = match self {
      MusicBrainzRequest::Lookup {
        ref resource,
        ref mbid,
        ..
      } => format!("/{}/{}", resource, mbid),
      MusicBrainzRequest::Browse { result, .. } => format!("/{}", result),
      MusicBrainzRequest::Search { resource, .. } => format!("/{}", resource),
    };
    format!("{}{}", MUSICBRAINZ_ENDPOINT, slug)
  }
  pub fn params(&self) -> Vec<(String, String)> {
    match self {
      MusicBrainzRequest::Lookup { inc, .. } => {
        if let Some(inc) = inc {
          vec![("inc".to_string(), inc.join("+"))]
        } else {
          vec![]
        }
      }
      MusicBrainzRequest::Browse {
        browse,
        mbid,
        limit,
        offset,
        ..
      } => {
        vec![
          (browse.to_string(), mbid.to_string()),
          ("limit".to_string(), limit.to_string()),
          ("offset".to_string(), offset.to_string()),
        ]
      }
      MusicBrainzRequest::Search {
        query,
        limit,
        offset,
        ..
      } => {
        vec![
          ("query".to_string(), query.to_string()),
          ("limit".to_string(), limit.to_string()),
          ("offset".to_string(), offset.to_string()),
        ]
      }
    }
  }
}

#[derive(Debug)]
pub enum MusicBrainzResource {
  Area,
  Artist,
  Event,
  Genre,
  Instrument,
  Label,
  Place,
  Recording,
  Release,
  ReleaseGroup,
  Series,
  Work,
  Url,
  Rating,
  Tag,
  Collection,
  DiscId,
  Isrc,
  Iswc,
}

impl fmt::Display for MusicBrainzResource {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      MusicBrainzResource::Area => f.write_str("area"),
      MusicBrainzResource::Artist => f.write_str("artist"),
      MusicBrainzResource::Event => f.write_str("event"),
      MusicBrainzResource::Genre => f.write_str("genre"),
      MusicBrainzResource::Instrument => f.write_str("instrument"),
      MusicBrainzResource::Label => f.write_str("label"),
      MusicBrainzResource::Place => f.write_str("place"),
      MusicBrainzResource::Recording => f.write_str("recording"),
      MusicBrainzResource::Release => f.write_str("release"),
      MusicBrainzResource::ReleaseGroup => f.write_str("release-group"),
      MusicBrainzResource::Series => f.write_str("series"),
      MusicBrainzResource::Work => f.write_str("work"),
      MusicBrainzResource::Url => f.write_str("url"),
      MusicBrainzResource::Rating => f.write_str("rating"),
      MusicBrainzResource::Tag => f.write_str("tag"),
      MusicBrainzResource::Collection => f.write_str("collection"),
      MusicBrainzResource::DiscId => f.write_str("disc-id"),
      MusicBrainzResource::Isrc => f.write_str("isrc"),
      MusicBrainzResource::Iswc => f.write_str("iswc"),
    }
  }
}

impl FromStr for MusicBrainzResource {
  type Err = Error;
  fn from_str(s: &str) -> Result<MusicBrainzResource> {
    match s {
      "area" => Ok(MusicBrainzResource::Area),
      "artist" => Ok(MusicBrainzResource::Artist),
      "event" => Ok(MusicBrainzResource::Event),
      "genre" => Ok(MusicBrainzResource::Genre),
      "instrument" => Ok(MusicBrainzResource::Instrument),
      "label" => Ok(MusicBrainzResource::Label),
      "place" => Ok(MusicBrainzResource::Place),
      "recording" => Ok(MusicBrainzResource::Recording),
      "release" => Ok(MusicBrainzResource::Release),
      "release-group" | "release group" => Ok(MusicBrainzResource::ReleaseGroup),
      "series" => Ok(MusicBrainzResource::Series),
      "work" => Ok(MusicBrainzResource::Work),
      "url" => Ok(MusicBrainzResource::Url),
      "rating" => Ok(MusicBrainzResource::Rating),
      "tag" => Ok(MusicBrainzResource::Tag),
      "collection" => Ok(MusicBrainzResource::Collection),
      "disk-id" | "disk id" => Ok(MusicBrainzResource::DiscId),
      "isrc" => Ok(MusicBrainzResource::Isrc),
      "iswc" => Ok(MusicBrainzResource::Iswc),
      e => Err(Error::Value(e.to_string())),
    }
  }
}
