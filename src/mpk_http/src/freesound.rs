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
use crate::Client;
use crate::Result;
pub struct FreeSoundClient {
  pub client: Client,
}

impl FreeSoundClient {
  pub fn new() -> Result<FreeSoundClient> {
    Ok(
      FreeSoundClient {
	client: Client::new()
      }
    )
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
