// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub mod package;

use std::io::{self, Write};

use pbr;
use depot_client::DisplayProgress;

/// A moving progress bar to track progress of a sized event, similar to wget, curl, npm, etc.
///
/// This is designed to satisfy a generic behavior which sets the size of the task (usually a
/// number of bytes representing the total download/upload/transfer size) and will be a generic
/// writer (i.e. implementing the `Write` trait) as a means to increase progress towards
/// completion.
pub struct ProgressBar {
    bar: pbr::ProgressBar,
    total: u64,
    current: u64,
}

impl Default for ProgressBar {
    fn default() -> Self {
        ProgressBar {
            bar: pbr::ProgressBar::new(0),
            total: 0,
            current: 0,
        }
    }
}

impl DisplayProgress for ProgressBar {
    fn size(&mut self, size: u64) {
        self.bar = pbr::ProgressBar::new(size);
        self.bar.set_units(pbr::Units::Bytes);
        self.bar.show_tick = true;
        self.bar.message("    ");
        self.total = size;
    }

    fn finish(&mut self) {
        println!("");
        io::stdout().flush().ok().expect("flush() fail");
    }
}

impl Write for ProgressBar {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.bar.write(buf) {
            Ok(n) => {
                self.current += n as u64;
                if self.current == self.total {
                    self.finish();
                }
                Ok(n)
            }
            Err(e) => Err(e),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        self.bar.flush()
    }
}
