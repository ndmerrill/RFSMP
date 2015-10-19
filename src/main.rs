extern crate rfmod;

struct MusicPlayer {
    fmod: rfmod::Sys,
    //current_music: rfmod::Sound,
}

impl MusicPlayer {
    pub fn new() -> MusicPlayer {
        let fmod = match rfmod::Sys::new() {
            Ok(f) => f,
            Err(e) => {
                panic!("Error Code: {:?}", e);
            }
        };

        match fmod.init() {
            rfmod::Result::Ok => {},
            e => {
                panic!("Error Code: {:?}", e);
            }
        };

        let player = MusicPlayer{fmod: fmod};
        player
    }
}

fn main() {
    let player = MusicPlayer::new();
    let music = match player.fmod.create_sound("/home/nathan/Downloads/left.mp3", None, None) {
        Ok(f) => f,
        Err(e) => panic!("Error Code: {:?}", e),
    };

    music.play_to_the_end();
    println!("Hello, world!");
}
