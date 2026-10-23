//! Mmino: South African local music player foundation (Stage 1).
//!
//! Standard-library-only contracts for local audio playback, library indexing,
//! queue management, playlists, metadata, and search.

use std::collections::BTreeMap;
use std::fmt;

/// Supported local audio formats for Mmino Stage 1.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AudioFormat {
    Mp3,
    Flac,
    Wav,
    Ogg,
    Aac,
    Unknown(String),
}

impl AudioFormat {
    pub fn from_extension(ext: &str) -> Self {
        match ext.trim().to_ascii_lowercase().as_str() {
            "mp3" => Self::Mp3,
            "flac" => Self::Flac,
            "wav" | "wave" => Self::Wav,
            "ogg" | "oga" => Self::Ogg,
            "aac" | "m4a" => Self::Aac,
            other => Self::Unknown(other.to_string()),
        }
    }

    pub fn mime_type(&self) -> &'static str {
        match self {
            Self::Mp3 => "audio/mpeg",
            Self::Flac => "audio/flac",
            Self::Wav => "audio/wav",
            Self::Ogg => "audio/ogg",
            Self::Aac => "audio/aac",
            Self::Unknown(_) => "application/octet-stream",
        }
    }

    pub fn is_supported(&self) -> bool {
        !matches!(self, Self::Unknown(_))
    }
}

impl fmt::Display for AudioFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mp3 => write!(f, "MP3"),
            Self::Flac => write!(f, "FLAC"),
            Self::Wav => write!(f, "WAV"),
            Self::Ogg => write!(f, "OGG"),
            Self::Aac => write!(f, "AAC"),
            Self::Unknown(ext) => write!(f, "UNKNOWN({ext})"),
        }
    }
}

/// Metadata and location for a local audio recording.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioTrack {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub album: Option<String>,
    pub genre: Option<String>,
    pub year: Option<u32>,
    pub duration_secs: u32,
    pub file_path: String,
    pub format: AudioFormat,
}

impl AudioTrack {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        artist: impl Into<String>,
        file_path: impl Into<String>,
        format: AudioFormat,
        duration_secs: u32,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            artist: artist.into(),
            album: None,
            genre: None,
            year: None,
            duration_secs,
            file_path: file_path.into(),
            format,
        }
    }

    pub fn with_album(mut self, album: impl Into<String>) -> Self {
        self.album = Some(album.into());
        self
    }

    pub fn with_genre(mut self, genre: impl Into<String>) -> Self {
        self.genre = Some(genre.into());
        self
    }

    pub fn with_year(mut self, year: u32) -> Self {
        self.year = Some(year);
        self
    }

    pub fn formatted_duration(&self) -> String {
        let mins = self.duration_secs / 60;
        let secs = self.duration_secs % 60;
        format!("{mins}:{secs:02}")
    }
}

/// User playlist containing ordered track IDs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Playlist {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub track_ids: Vec<String>,
}

impl Playlist {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: None,
            track_ids: Vec::new(),
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn add_track(&mut self, track_id: impl Into<String>) {
        self.track_ids.push(track_id.into());
    }

    pub fn remove_track(&mut self, track_id: &str) -> bool {
        let initial_len = self.track_ids.len();
        self.track_ids.retain(|id| id != track_id);
        self.track_ids.len() < initial_len
    }

    pub fn len(&self) -> usize {
        self.track_ids.len()
    }

    pub fn is_empty(&self) -> bool {
        self.track_ids.is_empty()
    }
}

/// Current playback state of the audio engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackState {
    Stopped,
    Playing,
    Paused,
}

/// Repeat behavior for playback queues.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RepeatMode {
    #[default]
    Off,
    All,
    One,
}

/// Queue and playback state manager for Mmino.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaybackQueue {
    tracks: Vec<AudioTrack>,
    current_index: Option<usize>,
    state: PlaybackState,
    position_secs: u32,
    volume_percent: u8,
    repeat_mode: RepeatMode,
    shuffle_enabled: bool,
}

impl Default for PlaybackQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl PlaybackQueue {
    pub fn new() -> Self {
        Self {
            tracks: Vec::new(),
            current_index: None,
            state: PlaybackState::Stopped,
            position_secs: 0,
            volume_percent: 80,
            repeat_mode: RepeatMode::Off,
            shuffle_enabled: false,
        }
    }

    pub fn load_tracks(&mut self, tracks: Vec<AudioTrack>, start_index: Option<usize>) {
        self.tracks = tracks;
        self.position_secs = 0;
        if self.tracks.is_empty() {
            self.current_index = None;
            self.state = PlaybackState::Stopped;
        } else {
            let idx = start_index.unwrap_or(0);
            self.current_index = Some(idx.min(self.tracks.len().saturating_sub(1)));
            self.state = PlaybackState::Playing;
        }
    }

    pub fn play(&mut self) -> Result<(), &'static str> {
        if self.tracks.is_empty() {
            return Err("Queue is empty");
        }
        if self.current_index.is_none() {
            self.current_index = Some(0);
        }
        self.state = PlaybackState::Playing;
        Ok(())
    }

    pub fn pause(&mut self) -> Result<(), &'static str> {
        if self.state != PlaybackState::Playing {
            return Err("Not currently playing");
        }
        self.state = PlaybackState::Paused;
        Ok(())
    }

    pub fn resume(&mut self) -> Result<(), &'static str> {
        if self.state != PlaybackState::Paused {
            return Err("Not currently paused");
        }
        self.state = PlaybackState::Playing;
        Ok(())
    }

    pub fn stop(&mut self) {
        self.state = PlaybackState::Stopped;
        self.position_secs = 0;
    }

    pub fn next(&mut self) -> Option<&AudioTrack> {
        if self.tracks.is_empty() {
            return None;
        }

        let cur = self.current_index.unwrap_or(0);
        match self.repeat_mode {
            RepeatMode::One => {
                self.position_secs = 0;
                self.state = PlaybackState::Playing;
            }
            RepeatMode::All => {
                self.current_index = Some((cur + 1) % self.tracks.len());
                self.position_secs = 0;
                self.state = PlaybackState::Playing;
            }
            RepeatMode::Off => {
                if cur + 1 < self.tracks.len() {
                    self.current_index = Some(cur + 1);
                    self.position_secs = 0;
                    self.state = PlaybackState::Playing;
                } else {
                    self.state = PlaybackState::Stopped;
                    self.position_secs = 0;
                    return None;
                }
            }
        }

        self.current_track()
    }

    pub fn previous(&mut self) -> Option<&AudioTrack> {
        if self.tracks.is_empty() {
            return None;
        }

        let cur = self.current_index.unwrap_or(0);
        if self.position_secs > 3 {
            // If playing more than 3 seconds into the track, restart it
            self.position_secs = 0;
        } else if cur > 0 {
            self.current_index = Some(cur - 1);
            self.position_secs = 0;
        } else if self.repeat_mode == RepeatMode::All {
            self.current_index = Some(self.tracks.len().saturating_sub(1));
            self.position_secs = 0;
        } else {
            self.position_secs = 0;
        }

        self.state = PlaybackState::Playing;
        self.current_track()
    }

    pub fn seek(&mut self, position_secs: u32) {
        if let Some(track) = self.current_track() {
            self.position_secs = position_secs.min(track.duration_secs);
        }
    }

    pub fn set_volume(&mut self, volume: u8) {
        self.volume_percent = volume.min(100);
    }

    pub fn set_repeat_mode(&mut self, mode: RepeatMode) {
        self.repeat_mode = mode;
    }

    pub fn toggle_shuffle(&mut self) {
        self.shuffle_enabled = !self.shuffle_enabled;
    }

    pub fn current_track(&self) -> Option<&AudioTrack> {
        self.current_index.and_then(|idx| self.tracks.get(idx))
    }

    pub fn current_index(&self) -> Option<usize> {
        self.current_index
    }

    pub fn state(&self) -> PlaybackState {
        self.state
    }

    pub fn position_secs(&self) -> u32 {
        self.position_secs
    }

    pub fn volume_percent(&self) -> u8 {
        self.volume_percent
    }

    pub fn repeat_mode(&self) -> RepeatMode {
        self.repeat_mode
    }

    pub fn is_shuffle_enabled(&self) -> bool {
        self.shuffle_enabled
    }

    pub fn len(&self) -> usize {
        self.tracks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tracks.is_empty()
    }

    pub fn tracks(&self) -> &[AudioTrack] {
        &self.tracks
    }

    pub fn append(&mut self, track: AudioTrack) {
        self.tracks.push(track);
        if self.current_index.is_none() {
            self.current_index = Some(0);
        }
    }

    pub fn clear(&mut self) {
        self.tracks.clear();
        self.current_index = None;
        self.state = PlaybackState::Stopped;
        self.position_secs = 0;
    }
}

/// Local music library index containing tracks and user playlists.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LocalLibrary {
    tracks: BTreeMap<String, AudioTrack>,
    playlists: BTreeMap<String, Playlist>,
}

impl LocalLibrary {
    pub fn new() -> Self {
        Self {
            tracks: BTreeMap::new(),
            playlists: BTreeMap::new(),
        }
    }

    pub fn add_track(&mut self, track: AudioTrack) {
        self.tracks.insert(track.id.clone(), track);
    }

    pub fn remove_track(&mut self, track_id: &str) -> Option<AudioTrack> {
        let removed = self.tracks.remove(track_id);
        // Also remove track from all playlists
        for playlist in self.playlists.values_mut() {
            playlist.remove_track(track_id);
        }
        removed
    }

    pub fn get_track(&self, track_id: &str) -> Option<&AudioTrack> {
        self.tracks.get(track_id)
    }

    pub fn track_count(&self) -> usize {
        self.tracks.len()
    }

    pub fn all_tracks(&self) -> Vec<&AudioTrack> {
        self.tracks.values().collect()
    }

    pub fn search(&self, query: &str) -> Vec<&AudioTrack> {
        let q = query.trim().to_ascii_lowercase();
        if q.is_empty() {
            return self.all_tracks();
        }
        self.tracks
            .values()
            .filter(|t| {
                t.title.to_ascii_lowercase().contains(&q)
                    || t.artist.to_ascii_lowercase().contains(&q)
                    || t.album.as_deref().unwrap_or("").to_ascii_lowercase().contains(&q)
                    || t.genre.as_deref().unwrap_or("").to_ascii_lowercase().contains(&q)
            })
            .collect()
    }

    pub fn filter_by_artist(&self, artist: &str) -> Vec<&AudioTrack> {
        let a = artist.trim().to_ascii_lowercase();
        self.tracks
            .values()
            .filter(|t| t.artist.to_ascii_lowercase() == a)
            .collect()
    }

    pub fn filter_by_album(&self, album: &str) -> Vec<&AudioTrack> {
        let alb = album.trim().to_ascii_lowercase();
        self.tracks
            .values()
            .filter(|t| t.album.as_deref().unwrap_or("").to_ascii_lowercase() == alb)
            .collect()
    }

    pub fn filter_by_genre(&self, genre: &str) -> Vec<&AudioTrack> {
        let g = genre.trim().to_ascii_lowercase();
        self.tracks
            .values()
            .filter(|t| t.genre.as_deref().unwrap_or("").to_ascii_lowercase() == g)
            .collect()
    }

    pub fn create_playlist(&mut self, id: impl Into<String>, name: impl Into<String>) -> &mut Playlist {
        let p_id = id.into();
        let playlist = Playlist::new(p_id.clone(), name);
        self.playlists.entry(p_id).or_insert(playlist)
    }

    pub fn get_playlist(&self, playlist_id: &str) -> Option<&Playlist> {
        self.playlists.get(playlist_id)
    }

    pub fn get_playlist_mut(&mut self, playlist_id: &str) -> Option<&mut Playlist> {
        self.playlists.get_mut(playlist_id)
    }

    pub fn remove_playlist(&mut self, playlist_id: &str) -> Option<Playlist> {
        self.playlists.remove(playlist_id)
    }

    pub fn all_playlists(&self) -> Vec<&Playlist> {
        self.playlists.values().collect()
    }

    pub fn get_playlist_tracks(&self, playlist_id: &str) -> Vec<&AudioTrack> {
        self.playlists
            .get(playlist_id)
            .map(|p| {
                p.track_ids
                    .iter()
                    .filter_map(|id| self.tracks.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Export a playlist as M3U formatted text.
    pub fn export_m3u(&self, playlist_id: &str) -> Result<String, &'static str> {
        let playlist = self.playlists.get(playlist_id).ok_or("Playlist not found")?;
        let mut lines = vec!["#EXTM3U".to_string()];
        lines.push(format!("#PLAYLIST:{}", playlist.name));

        for id in &playlist.track_ids {
            if let Some(track) = self.tracks.get(id) {
                lines.push(format!(
                    "#EXTINF:{},{} - {}",
                    track.duration_secs, track.artist, track.title
                ));
                lines.push(track.file_path.clone());
            }
        }

        Ok(lines.join("\n") + "\n")
    }

    /// Import an M3U playlist file content into the library.
    pub fn import_m3u(&mut self, playlist_id: &str, playlist_name: &str, m3u_text: &str) {
        let mut playlist = Playlist::new(playlist_id, playlist_name);
        for line in m3u_text.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            // Match against existing tracks by file_path
            if let Some((id, _)) = self.tracks.iter().find(|(_, t)| t.file_path == trimmed) {
                playlist.add_track(id.clone());
            }
        }
        self.playlists.insert(playlist_id.to_string(), playlist);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_tracks() -> Vec<AudioTrack> {
        vec![
            AudioTrack::new(
                "track-1",
                "Jerusalema",
                "Master KG",
                "/music/jerusalema.mp3",
                AudioFormat::Mp3,
                342,
            )
            .with_album("Jerusalema")
            .with_genre("Afropop"),
            AudioTrack::new(
                "track-2",
                "Imbizo",
                "Phuzekhemisi",
                "/music/imbizo.flac",
                AudioFormat::Flac,
                280,
            )
            .with_album("Imbizo")
            .with_genre("Maskandi"),
            AudioTrack::new(
                "track-3",
                "Soweto Blues",
                "Hugh Masekela",
                "/music/soweto_blues.wav",
                AudioFormat::Wav,
                315,
            )
            .with_album("Afrobeat")
            .with_genre("Jazz"),
        ]
    }

    #[test]
    fn audio_format_parsing() {
        assert_eq!(AudioFormat::from_extension("mp3"), AudioFormat::Mp3);
        assert_eq!(AudioFormat::from_extension("FLAC"), AudioFormat::Flac);
        assert_eq!(AudioFormat::from_extension("wav"), AudioFormat::Wav);
        assert_eq!(AudioFormat::from_extension("ogg"), AudioFormat::Ogg);
        assert_eq!(AudioFormat::from_extension("m4a"), AudioFormat::Aac);
        assert_eq!(AudioFormat::from_extension("xyz"), AudioFormat::Unknown("xyz".to_string()));

        assert_eq!(AudioFormat::Mp3.mime_type(), "audio/mpeg");
        assert_eq!(AudioFormat::Flac.mime_type(), "audio/flac");
        assert!(AudioFormat::Mp3.is_supported());
        assert!(!AudioFormat::Unknown("xyz".into()).is_supported());
    }

    #[test]
    fn track_formatting() {
        let track = AudioTrack::new("1", "Song", "Artist", "/path.mp3", AudioFormat::Mp3, 125);
        assert_eq!(track.formatted_duration(), "2:05");
    }

    #[test]
    fn queue_navigation_and_states() {
        let mut queue = PlaybackQueue::new();
        assert_eq!(queue.state(), PlaybackState::Stopped);

        let tracks = sample_tracks();
        queue.load_tracks(tracks, Some(0));
        assert_eq!(queue.state(), PlaybackState::Playing);
        assert_eq!(queue.current_track().unwrap().title, "Jerusalema");

        // Pause
        assert!(queue.pause().is_ok());
        assert_eq!(queue.state(), PlaybackState::Paused);

        // Resume
        assert!(queue.resume().is_ok());
        assert_eq!(queue.state(), PlaybackState::Playing);

        // Next track
        let next = queue.next();
        assert!(next.is_some());
        assert_eq!(next.unwrap().title, "Imbizo");
        assert_eq!(queue.current_index(), Some(1));

        // Next to Hugh Masekela
        let next = queue.next();
        assert!(next.is_some());
        assert_eq!(next.unwrap().title, "Soweto Blues");

        // Next at end with RepeatMode::Off stops playback
        assert_eq!(queue.repeat_mode(), RepeatMode::Off);
        let end = queue.next();
        assert!(end.is_none());
        assert_eq!(queue.state(), PlaybackState::Stopped);
    }

    #[test]
    fn queue_repeat_modes() {
        let mut queue = PlaybackQueue::new();
        queue.load_tracks(sample_tracks(), Some(2)); // Start at last track
        queue.set_repeat_mode(RepeatMode::All);

        // Next should loop to track 0
        let next = queue.next();
        assert!(next.is_some());
        assert_eq!(next.unwrap().title, "Jerusalema");
        assert_eq!(queue.current_index(), Some(0));

        // Repeat One keeps current track
        queue.set_repeat_mode(RepeatMode::One);
        let same = queue.next();
        assert!(same.is_some());
        assert_eq!(same.unwrap().title, "Jerusalema");
        assert_eq!(queue.current_index(), Some(0));
    }

    #[test]
    fn queue_volume_bounds() {
        let mut queue = PlaybackQueue::new();
        queue.set_volume(50);
        assert_eq!(queue.volume_percent(), 50);

        queue.set_volume(150); // should clamp to 100
        assert_eq!(queue.volume_percent(), 100);
    }

    #[test]
    fn library_indexing_and_search() {
        let mut lib = LocalLibrary::new();
        for t in sample_tracks() {
            lib.add_track(t);
        }
        assert_eq!(lib.track_count(), 3);

        // Search
        let results = lib.search("master");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Jerusalema");

        let results = lib.search("maskandi");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Imbizo");

        // Filter by artist
        let artist_tracks = lib.filter_by_artist("hugh masekela");
        assert_eq!(artist_tracks.len(), 1);
        assert_eq!(artist_tracks[0].title, "Soweto Blues");
    }

    #[test]
    fn playlist_creation_and_m3u_export() {
        let mut lib = LocalLibrary::new();
        for t in sample_tracks() {
            lib.add_track(t);
        }

        let playlist = lib.create_playlist("mzansi-vibes", "Mzansi Classics");
        playlist.add_track("track-1");
        playlist.add_track("track-3");
        assert_eq!(playlist.len(), 2);

        let tracks = lib.get_playlist_tracks("mzansi-vibes");
        assert_eq!(tracks.len(), 2);
        assert_eq!(tracks[0].title, "Jerusalema");
        assert_eq!(tracks[1].title, "Soweto Blues");

        // Export M3U
        let m3u = lib.export_m3u("mzansi-vibes").unwrap();
        assert!(m3u.starts_with("#EXTM3U"));
        assert!(m3u.contains("Master KG - Jerusalema"));
        assert!(m3u.contains("/music/jerusalema.mp3"));

        // Import M3U
        lib.import_m3u("imported-pl", "Imported List", &m3u);
        let imported = lib.get_playlist("imported-pl").unwrap();
        assert_eq!(imported.len(), 2);
    }
}
