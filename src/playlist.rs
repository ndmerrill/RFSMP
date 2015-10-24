use std::string;

pub struct Playlist {
    music: string::String,
}

impl Playlist {
    pub fn new(song: &str) -> Playlist {
        Playlist{music: song.to_string()}
    }

    pub fn get_next_song(&self) -> &str {
        &self.music
    }
}
