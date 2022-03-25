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
//! ENDPOINT: https://musicbrainz.org/ws/2/
use crate::Client;

pub struct MusicBrainzClient {
  pub client: Client,
}

pub enum MusicBrainzRequest {
  Lookup,
  Browse,
  Search,
}

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
