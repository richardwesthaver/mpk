//! MPK_HTTP COVERARTARCHIVE API
//!
//! Coverartarchive.org is a joint project between the Internet
//! Archive and Musicbrainz whose goal is to make cover art images
//! available to everyone on the internet. An API is provided for
//! retrieving cover art via a release MBID.
//!
//! This module implements the coverartarchive.org API.
//!
//! REF: https://wiki.musicbrainz.org/Cover_Art_Archive/API
//! ENDPOINT: https://coverartarchive.org/
use crate::Client;

pub struct CoverArtArchiveClient {
  pub client: Client,
}
