BEGIN;
pragma foreign_keys = on;

create table if not exists tracks (
id integer primary key,
path text not null unique,
format text,
channels integer,
filesize integer,
bitrate integer,
bitdepth integer,
duration integer,
samplerate integer,
updated datetime default current_timestamp not null);

create table if not exists track_tags (
track_id integer unique,
artist text,
title text,
album text,
genre text,
year integer,
foreign key(track_id) references tracks(id));

create table if not exists track_tags_musicbrainz (
track_id integer unique,
albumartistid text,
albumid text,
albumstatus text,
albumtype text,
artistid text,
releasegroupid text,
releasetrackid text,
trackid text,
foreign key(track_id) references tracks(id));

create table if not exists track_features_lowlevel (
track_id integer unique,
average_loudness real,
barkbands_kurtosis blob,
barkbands_skewness blob,
barkbands_spread blob,
barkbands blob,
dissonance blob,
hfc blob,
pitch blob,
pitch_instantaneous_confidence blob,
pitch_salience blob,
silence_rate_20db blob,
silence_rate_30db blob,
silence_rate_60db blob,
spectral_centroid blob,
spectral_complexity blob,
spectral_crest blob,
spectral_decrease blob,
spectral_energy blob,
spectral_energyband_high blob,
spectral_energyband_low blob,
spectral_energyband_middle_high blob,
spectral_energyband_middle_low blob,
spectral_flatness_db blob,
spectral_flux blob,
spectral_kurtosis blob,
spectral_rms blob,
spectral_rolloff blob,
spectral_skewness blob,
spectral_spread blob,
spectral_strongpeak blob,
zerocrossingrate blob,
mfcc blob,
sccoeffs blob,
scvalleys blob,
foreign key(track_id) references tracks(id));

create table if not exists track_features_rhythm (
track_id integer unique,
bpm real,
confidence real,
onset_rate real,
beats_loudness blob,
first_peak_bpm integer,
first_peak_spread real,
first_peak_weight real,
second_peak_bpm integer,
second_peak_spread real,
second_peak_weight real,
beats_position blob,
bpm_estimates blob,
bpm_intervals blob,
onset_times blob,
beats_loudness_band_ratio blob,
histogram blob,
foreign key(track_id) references tracks(id));

create table if not exists track_features_sfx (
track_id integer unique,
pitch_after_max_to_before_max_energy_ratio real,
pitch_centroid real,
pitch_max_to_total real,
pitch_min_to_total real,
inharmonicity blob,
oddtoevenharmonicenergyratio blob,
tristimulus blob,
foreign key(track_id) references tracks(id));

create table if not exists track_features_tonal (
track_id integer unique,
chords_change_rate real,
chords_number_rate real,
key_strength real,
tuning_diatonic_strength real,
tuning_equal_tempered_deviation real,
tuning_frequency real,
tuning_nontempered_tuning_ratio real,
chords_strength blob,
chords_histogram blob,
thpcp blob,
hpcp blob,
chords_key text,
chords_scale text,
key_key text,
key_scale text,
chord_progression blob,
foreign key(track_id) references tracks(id));

create table if not exists track_images (
track_id integer unique,
mel_spec blob,
log_spec blob,
freq_spec blob,
foreign key(track_id) references tracks(id));

create table if not exists track_user_data (
track_id integer unique,
user_tags text,
notes text,
foreign key(track_id) references tracks(id));

create table if not exists samples (
id integer primary key,
path text not null unique,
format text,
channels integer,
filesize integer,
bitrate integer,
bitdepth integer,
duration integer,
samplerate integer,
updated datetime default current_timestamp not null);

create table if not exists sample_features_lowlevel (
sample_id integer unique,
average_loudness real,
barkbands_kurtosis blob,
barkbands_skewness blob,
barkbands_spread blob,
barkbands blob,
dissonance blob,
hfc blob,
pitch blob,
pitch_instantaneous_confidence blob,
pitch_salience blob,
silence_rate_20db blob,
silence_rate_30db blob,
silence_rate_60db blob,
spectral_centroid blob,
spectral_complexity blob,
spectral_crest blob,
spectral_decrease blob,
spectral_energy blob,
spectral_energyband_high blob,
spectral_energyband_low blob,
spectral_energyband_middle_high blob,
spectral_energyband_middle_low blob,
spectral_flatness_db blob,
spectral_flux blob,
spectral_kurtosis blob,
spectral_rms blob,
spectral_rolloff blob,
spectral_skewness blob,
spectral_spread blob,
spectral_strongpeak blob,
zerocrossingrate blob,
mfcc blob,
sccoeffs blob,
scvalleys blob,
foreign key(sample_id) references samples(id));

create table if not exists sample_features_rhythm (
sample_id integer unique,
bpm real,
confidence real,
onset_rate real,
beats_loudness blob,
first_peak_bpm integer,
first_peak_spread real,
first_peak_weight real,
second_peak_bpm integer,
second_peak_spread real,
second_peak_weight real,
beats_position blob,
bpm_estimates blob,
bpm_intervals blob,
onset_times blob,
beats_loudness_band_ratio blob,
histogram blob,
foreign key(sample_id) references samples(id));

create table if not exists sample_features_sfx (
sample_id integer unique,
pitch_after_max_to_before_max_energy_ratio real,
pitch_centroid real,
pitch_max_to_total real,
pitch_min_to_total real,
inharmonicity blob,
oddtoevenharmonicenergyratio blob,
tristimulus blob,
foreign key(sample_id) references samples(id));

create table if not exists sample_features_tonal (
sample_id integer unique,
chords_change_rate real,
chords_number_rate real,
key_strength real,
tuning_diatonic_strength real,
tuning_equal_tempered_deviation real,
tuning_frequency real,
tuning_nontempered_tuning_ratio real,
chords_strength blob,
chords_histogram blob,
thpcp blob,
hpcp blob,
chords_key text,
chords_scale text,
key_key text,
key_scale text,
chord_progression blob,
foreign key(sample_id) references samples(id));

create table if not exists sample_images (
sample_id integer unique,
mel_spec blob,
log_spec blob,
freq_spec blob,
foreign key(sample_id) references samples(id));

create table if not exists sample_user_data (
sample_id integer unique,
user_tags text,
notes text,
foreign key(sample_id) references samples(id));

create table if not exists projects (
id integer unique,
name text not null,
path text not null,
type text not null,
updated datetime default current_timestamp not null);

create table if not exists project_user_data (
project_id integer unique,
user_tags text,
notes text,
foreign key(project_id) references projects(id));
COMMIT;
