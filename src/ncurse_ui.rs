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
extern crate ncurses;

use playlist;

use self::ncurses::*;

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
    prev_time: i32,
    prev_song_num: i32,
}

fn split_time(time: i32) -> String {
    return format!("{}:{:0>2}", time / 60, time % 60);
}

impl Drop for UI {
    fn drop (&mut self) {
        endwin();
    }
}

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
            prev_time: -1,
            prev_song_num: -1,
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
            self.cols = COLS;
            self.lines = LINES;
            self.prev_time = -1;
            self.prev_song_num = -1;
        }

        if playlist.song_index != self.prev_song_num {
            // redraw everything on song change
            self.draw_times_and_bar(time, totaltime);

            // song name
            let mut song_name = String::from(playlist.get_curr_song().unwrap_or(""));
            if song_name.len() > COLS as usize {
                song_name = format!("{}...", &song_name[..(COLS-3) as usize]);
            }
            mvprintw(LINES-4, 0, &*format!("{:^1$}", song_name, COLS as usize));

            // list of songs

            // handle scrolling
            let mut song_list;
            if playlist.songs.len() > (LINES-4) as usize {
                let mut to_take = playlist.song_index - (LINES-4)/2;
                if playlist.songs.len() as i32 - to_take < LINES-4 {
                    to_take = playlist.songs.len() as i32 - (LINES-4);
                }
                if to_take < 0 {
                    to_take = 0;
                }
                song_list = playlist.songs.iter().skip(to_take as usize);
            }
            else {
                song_list = playlist.songs.iter().skip(0);
            }

            // print the songs
            for i in 0..LINES-4 {
                let tmp = String::new();
                let x = song_list.next().unwrap_or(&tmp);
                match x == playlist.get_curr_song().unwrap_or("") {
                    false => {},
                    true => {attron(COLOR_PAIR(2));},
                };
                let visable;
                if x.len() > COLS as usize {
                    visable = format!("{}...", &x[..(COLS-3) as usize]);
                }
                else {
                    visable = format!("{:<1$}", x, COLS as usize);
                }

                mvprintw(i, 0, &*visable);
                attron(COLOR_PAIR(1));
            }

            self.prev_song_num = playlist.song_index;
        }

        else if time != self.prev_time {
            // redraw the progress bar and times
            self.draw_times_and_bar(time, totaltime);
        }

        refresh();
        return UIResult::NA;
    }

    fn draw_times_and_bar(&mut self, time: i32, totaltime: i32) {
            let squares;

            if time == 0 || totaltime == 0 {
                squares = 0;
            }
            else {
                squares = (time as f64 / totaltime as f64 * COLS as f64) as usize;
            }

            // end time
            mvprintw(LINES-3, 0, &*format!("{:>1$}", split_time(totaltime), COLS as usize));
            // current time
            mvprintw(LINES-3, 0, &*split_time(time));
            self.prev_time = time;

            // progress bar
            mvprintw(LINES-2, 0, &*format!("{:#<1$}{0:<2$}", "", squares, COLS as usize-squares));
    }
}
