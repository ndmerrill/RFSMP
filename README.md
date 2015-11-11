# RFSMP

The Rust Fucking Simple Music Player that just plays your music, no frills.

Design goals:
  1. Small, neat, maintainable code base. All configuration done at compile time.
  2. Uses user directory structure to sort music; ignores metadata.
  3. Can play music files selected with simple Regular Expressions.
  4. Full support of gapless playback.

### Version
Still in Alpha

### Installation

Because Gstreamer Rust Bindings use unstable standard library features, the
following steps are required to get RFSMP to compile with stable rust.

  1. Enter src directory of RFSMP.
  2. Run following commands:

    git clone https://github.com/arturoc/gstreamer1.0-rs.git
    mv gstreamer1.0-rs/ gstreamer
  3. Add the line 'rc = "\*"' to the end of Cargo.toml in the gstreamer directory
  4. Add 'extern crate rc;' to src/lib.rs inside the gstreamer directory
  5. Replace 'use std::rc::{Rc, Weak};' with 'use rc::{Rc, Weak} in src/bus.rs in the gstreamer directory.
  6. Add the following lines to the main direcotry Cargo.toml:

    [dependencies.gstreamer]
    path = "src/gstreamer"

Then, from the main directory of RFSMP, build with

    cargo build

### Development

TODO

### Todos

TODO

### Credit
Developers:
 * Nathanael Merrill

Uses FMOD by Firelight Technologies


### License
    Copyright 2015 Nathanael Merrill

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
