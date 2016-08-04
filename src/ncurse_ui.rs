//  Copyright 2015 Nathanael Merrill
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.


//uifeature:
// press / to start filter/seatch
// change playlist to only display the things using regex filter of vector
// refresh/serach only when ch thread stops hanging for maximum performance
use playlist;

use ncurses::*;

pub enum UIResult {
    PlayPause,
    Next,
    Previous,
    Exit,
    Error(String),
    NA,
}

pub struct UI {
    cols: i32,
    lines: i32,
}
//there are three threads, one that hangs on the keyinput (the ncurse one), one that hangs on retrieving the data from main(which prints the output), and the gstreamer loop
//the cool part about this is that there is no looping in the ui, only hanging threads that send messages to awake other hanging threads

//current bug is in a stack thread in mobile bookmarks folder
impl UI {
    pub fn new() -> UI {
        initscr();
        noecho();
        refresh();
        start_color();
        use_default_colors();
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        init_pair(1, -1, -1);
        init_pair(2, COLOR_YELLOW, -1);

        nodelay(stdscr, true);

        UI {
            cols: COLS,
            lines: LINES,
        }
    }

    pub fn manage_ui(&mut self, playlist: &playlist::Playlist,
                     time: i32, totaltime: i32) -> UIResult {
        let ch = getch() as u8 as char;

        match ch {
            'n' => return UIResult::Next,
            'p' => return UIResult::Previous,
            'x' => return UIResult::Exit,
            ' ' => return UIResult::PlayPause,
            _ => {},
        }

        if self.cols != COLS || self.lines != LINES {
            //redraw
        }

//if self.time != (time, totaltime) ||
            //(self.length, self.height) != (self.term.cols(), self.term.rows()){

            //self.time = (time, totaltime);

            //if totaltime/60 > 99 {
                //return UIResult::Error(String::from(
                    //"RFSMP does not support songs longer than 100 minutes"));
            //}

            //let curr_song = playlist.get_curr_song().unwrap_or("");
            //let mut curr_song_visable = String::from(curr_song);
            //let length_i32 = self.length as i32;
            //let mut spaces = (curr_song_visable.len() +
                //((totaltime/60).to_string().len()+3)*2) as i32;
            //spaces = length_i32 - spaces - 8;
            //if spaces < 0 {
                //curr_song_visable = String::new();
                //curr_song_visable.push_str(&curr_song[0..(curr_song.len() as i32+(spaces)-5) as usize]);
                //curr_song_visable.push_str("...");
                //spaces = 2;
            //}
            //spaces = spaces / 2;

            //let spaces_l = vec![' '; spaces as usize].into_iter().collect::<String>();
            //let spaces_r = match (length_i32-curr_song_visable.len() as i32).wrapping_rem(2) {
                //0 => spaces_l.clone(),
                //1 => spaces_l.clone() + " ",
                //_ => unreachable!(),
            //};
            //let status_chars: Vec<char> = format!("--{:0>7$}:{:0>2}{}--{}--{}{:0>7$}:{:0>2}--",
                                  //time/60, time%60, spaces_l, curr_song_visable, spaces_r,
                                  //totaltime/60, totaltime%60,
                                  //(totaltime/60).to_string().len())
                                  //.chars().collect();

            //let (cols, rows) = self.canvas.size();
            //let (cols, rows) = (cols as isize, rows as isize);
            //let number_of_x;

            //if totaltime == 0 {
                //number_of_x = 0;
            //}
            //else if totaltime < time {
                //number_of_x = length_i32;
            //}
            //else {
                //number_of_x = (time as f32 / totaltime as f32
                         //* length_i32 as f32).round() as i32;
            //}

            //let mut load_chars = vec!['x'; number_of_x as usize];
            //load_chars.append(&mut vec!['-'; (length_i32 - number_of_x) as usize]);

            //for x in 0..cols {
                //for y in 0..rows {
                    //let fep ='*';
                    //let mut cell = match self.canvas.get_mut(x as usize, y as usize) {
                        //Some(a) => a,
                        //None => return UIResult::Error(
                            //"Could not draw to screen".to_string()),
                    //};
                    //match y {
                        //0 => cell.set_ch(*status_chars.get(x as usize).unwrap_or_else(|| &fep)),
                        //1 => cell.set_ch(*load_chars.get(x as usize).unwrap_or_else(|| &fep)),
                        //_ => unreachable!(),
                    //};
                //}
            //}

        for (x, i) in playlist.songs.iter().zip(0..) {
            match x == playlist.get_curr_song().unwrap_or("") {
                false => {},
                true => {attron(COLOR_PAIR(2));},
            };
            mvprintw(i, 0, x);
            attron(COLOR_PAIR(1));
        }
        refresh();
        return UIResult::NA;
    }

}
