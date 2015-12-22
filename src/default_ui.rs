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

use playlist;

use rustty::{
    Terminal,
    Event,
    HasSize,
    CellAccessor,
    Cell,
    Attr,
    Color,
};
use rustty::ui::{
    Painter,
    Dialog,
    Widget,
    Alignable,
    HorizontalAlign,
    VerticalAlign,
};

use std::error::Error;

// Create the dialog that shows the keybindings. Should probably be removed.
fn create_info_dlg(length: usize) -> Dialog {
    let mut optiondlg = Dialog::new(length as usize, 4);

    let inc_label = "space --> play/pause";
    let q_label   = "x     --> exit program";
    let n_label   = "n --> next song";
    let pp_label  = "p --> prev song";

    let inc_pos = optiondlg.window().halign_line(inc_label, HorizontalAlign::Left, 1);
    let q_pos = optiondlg.window().halign_line(q_label, HorizontalAlign::Left, 1);
    let n_pos = optiondlg.window().halign_line(n_label, HorizontalAlign::Middle, 1);
    let pp_pos = optiondlg.window().halign_line(pp_label, HorizontalAlign::Middle, 1);

    optiondlg.window_mut().printline(inc_pos, 1, inc_label);
    optiondlg.window_mut().printline(q_pos, 2, q_label);
    optiondlg.window_mut().printline(n_pos, 1, n_label);
    optiondlg.window_mut().printline(pp_pos, 2, pp_label);
    optiondlg.window_mut().draw_box();
    optiondlg
}

// Tells main what happened to the UI and if the user sent any input.
pub enum UIResult {
    PlayPause,
    Next,
    Previous,
    Exit,
    Error(String),
    NA,
}

pub struct UI {
    term: Terminal,
    optiondlg: Dialog,
    canvas: Widget,
    list_canvas: Widget,
    length: usize,
    height: usize,
    time: (i32, i32),
}

impl UI {
    pub fn new(playlist: &playlist::Playlist) -> UI {
        // Create our terminal, dialog window and main canvasa
        let term = Terminal::new().expect("Failed to make Rustty terminal");
        let length = term.cols();
        let height = term.rows();

        // aligns everything
        let mut optiondlg = create_info_dlg(length);
        let mut canvas = Widget::new(length as usize, 2);
        let mut list_canvas = Widget::new(length as usize,playlist.songs.len());
        optiondlg
            .window_mut()
            .align(&term, HorizontalAlign::Middle, VerticalAlign::Bottom, 0);
        canvas.align(&term, HorizontalAlign::Middle, VerticalAlign::Bottom, 4);
        list_canvas.align(&term, HorizontalAlign::Middle, VerticalAlign::Top, 0);

        UI {term: term,
            optiondlg: optiondlg,
            canvas: canvas,
            list_canvas: list_canvas,
            length: length,
            height: height,
            time: (-1,-1),
        }
    }

    // Updates the User Interface and gets user input.
    pub fn manage_ui(&mut self, playlist: &playlist::Playlist,
                     time: i32, totaltime: i32) -> UIResult {
        //TODO: The rest of this function won't run if there is input
        //this is not really ideal
        while let Some(Event::Key(ch)) = self.term.get_event(0).unwrap() {
            match ch {
                ' ' => return UIResult::PlayPause,
                'p' => return UIResult::Previous,
                'n' => return UIResult::Next,
                'x' => return UIResult::Exit,
                _  => return UIResult::NA,
            }
        }

        if self.time != (time, totaltime) ||
            (self.length, self.height) != (self.term.cols(), self.term.rows()){

            let mut a = self.length_checker();
            if a != "" {
                return UIResult::Error(a);
            }

            self.time = (time, totaltime);

            if totaltime/60 > 99 {
                return UIResult::Error(String::from(
                    "RFSMP does not support songs longer than 100 minutes"));
            }

            let curr_song = playlist.get_curr_song().unwrap_or("");
            let length_i32 = self.length as i32;
            let mut spaces = (curr_song.len() +
                ((totaltime/60).to_string().len()+3)*2) as i32;
            spaces = length_i32 - spaces - 8;
            spaces = spaces / 2;

            let spaces_l = vec![' '; spaces as usize].into_iter().collect::<String>();
            let spaces_r = match (length_i32-curr_song.len() as i32).wrapping_rem(2) {
                0 => spaces_l.clone(),
                1 => spaces_l.clone() + " ",
                _ => unreachable!(),
            };
            let status_chars: Vec<char> = format!("--{:0>7$}:{:0>2}{}--{}--{}{:0>7$}:{:0>2}--",
                                  time/60, time%60, spaces_l, curr_song, spaces_r,
                                  totaltime/60, totaltime%60,
                                  (totaltime/60).to_string().len())
                                  .chars().collect();

            let (cols, rows) = self.canvas.size();
            let (cols, rows) = (cols as isize, rows as isize);
            let number_of_x;

            if totaltime == 0 {
                number_of_x = 0;
            }
            else {
                number_of_x = (time as f32 / totaltime as f32
                         * length_i32 as f32).round() as i32;
            }

            let mut load_chars = vec!['x'; number_of_x as usize];
            load_chars.append(&mut vec!['-'; (length_i32 - number_of_x) as usize]);

            for x in 0..cols {
                for y in 0..rows {
                    let fep ='*';
                    let mut cell = match self.canvas.get_mut(x as usize, y as usize) {
                        Some(a) => a,
                        None => return UIResult::Error(
                            "Could not draw to screen".to_string()),
                    };
                    match y {
                        0 => cell.set_ch(*status_chars.get(x as usize).unwrap_or_else(|| &fep)),
                        1 => cell.set_ch(*load_chars.get(x as usize).unwrap_or_else(|| &fep)),
                        _ => unreachable!(),
                    };
                }
            }

            a = self.second_panel(playlist, curr_song);
            if a != "" {
                return UIResult::Error(a);
            }
            self.canvas.draw_into(&mut self.term);
            self.list_canvas.draw_into(&mut self.term);
            self.optiondlg.window().draw_into(&mut self.term);
            match self.term.swap_buffers() {
                Ok(_) => {},
                Err(e) => return UIResult::Error(
                    String::from("Swap buffers error: ".to_string() + e.description())),
            }
        }
        return UIResult::NA;
    }

    // Draws the list of songs.
    fn second_panel(&mut self, playlist: &playlist::Playlist, curr_song: &str) 
                    -> String {
        let cell = Cell::with_style(Color::Black, Color::Red, Attr::Default);
        let cellother = Cell::with_style(Color::Default,
                                         Color::Default,
                                         Attr::Default);
        let (_, rows) = self.list_canvas.size();

        for i in 0..rows {
            let song = match playlist.songs.get(i) {
                Some(a) => a,
                None => return String::from("Failed to look through songs"),
            };
            self.list_canvas
                .printline_with_cell(0, i as usize, song,
                                     match curr_song == song {
                                         true => cell,
                                         false => cellother,
                                     });
        }
        return String::from("");
    }

    // Checks to see if the terminal has changed size.
    fn length_checker(&mut self) -> String {
        let last_pos = (self.length, self.height);
        self.length = self.term.cols();
        self.height =  self.term.rows();
        if last_pos != (self.term.cols(), self.term.rows()){
            match self.term.clear() {
                Ok(_) => {},
                Err(e) => {
                    return "Terminal clear error: ".to_string() + e.description()
                }
            }
            self.optiondlg = create_info_dlg(self.length);
            self.canvas = Widget::new(self.length as usize, 2);
            self.optiondlg
                .window_mut()
                .align(&self.term,
                       HorizontalAlign::Middle,
                       VerticalAlign::Bottom, 0);
            self.canvas.align(&self.term,
                              HorizontalAlign::Middle,
                              VerticalAlign::Bottom, 4);
            self.list_canvas.align(&self.term, HorizontalAlign::Left,
                                   VerticalAlign::Top, 0);
        }
        return "".to_string();
    }
}
